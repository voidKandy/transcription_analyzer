source .env

report_output=$(prattl report)
if echo "$report_output" | grep -q "doesn't exist"; then
    echo "Preparing prattl..."
    prattl prepare
    report_output=$(prattl report)
fi

compressed=$(echo "$report_output" | awk '/Compressed/ {print tolower($2)}')
echo $compressed 

if [[ "$compressed" == "true" ]]; then 
    echo "decompressing prattl..."
    prattl decompress
fi

mkdir tmp


for input_file in "$AUDIO_DIR"/*.MOV; do
    abs_input_file=$(realpath "$input_file")
    echo $abs_input_file
    output_file="tmp/$(basename "${abs_input_file%.MOV}.mp3")"
    ffmpeg -i "$input_file" -vn -acodec libmp3lame -q:a 2 "$output_file" &
done


wait

ls "tmp"

transcription=$(find tmp -maxdepth 1 -type f | xargs prattl transcribe)
echo "$transcription" > .tmp.json

if [[ -z "$transcription" || -z "$(echo "$transcription" | tr -d '[:space:]')" ]]; then
    echo "Empty transcription"
else
    echo "transcriptions complete"
    echo "deleting mp3s.."
    rm -rf tmp

    echo "summarizing..."
    mkdir "$TARGET_DIR"
    transcription-analyzer -j .tmp.json -t "$TARGET_DIR"  

    if [[ $? -ne 0 ]]; then
        echo "Error: transcription-analyzer failed."
        exit 1 
    else
        echo "Transcription analysis completed successfully."
        echo "finished summarizing & saving to .md"
    fi

fi

while true; do 
    read -p "Delete MOVs? (Y/N): " key
    if [[ "$key" == "y" || "$key" == "Y" ]]; then
        for file in "$AUDIO_DIR"/*.MOV; do
            echo "Deleting file: $file"
            rm "$file"  
        done        
        break;
    elif [[ "$key" == "n" || "$key" == "N" ]]; then
        break;
    else 
        echo "$key is not a valid input. (Y/N)"
    fi
done


if [[ "$compressed" == "true" ]]; then 
    while true; do 
        read -p "recompress prattl? (Y/N): " key
        if [[ "$key" == "y" || "$key" == "Y" ]]; then
            prattl compress;
            break;
        elif [[ "$key" == "n" || "$key" == "N" ]]; then
            break;
        else 
            echo "$key is not a valid input. (Y/N)"
        fi
    done
fi

