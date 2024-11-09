use clap::Parser;
use std::{fs, path::PathBuf};
use transcription_analyzer::{
    convert::{convert_all_files_in_dir_to_audio, TMP_DIR},
    get_user_confirmation,
    prattl::{check_prattl_and_handle_status, run_prattl_transcribe},
    summarize::summarize_prattl_output,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'a', long)]
    pub audio_dir: PathBuf,
    #[arg(short = 't', long)]
    pub target_dir: PathBuf,
    // #[arg(short = 'j', long)]
    // pub transcription_json: PathBuf,
}

fn audio_files_in_dir(dir: &PathBuf) -> Vec<PathBuf> {
    std::fs::read_dir(dir)
        .expect("could not read dir")
        .fold(vec![], |mut all_files, entry| {
            let path = entry.unwrap().path();
            if path.is_file() {
                if let Some((_, ext)) = path.file_name().unwrap().to_string_lossy().rsplit_once('.')
                {
                    if ext.to_lowercase() == "mp3" {
                        all_files.push(path)
                    }
                }
            }
            all_files
        })
}
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let args = Args::parse();
    assert!(
        args.audio_dir.is_dir(),
        "audio dir argument is not a directory"
    );
    assert!(
        args.target_dir.is_dir(),
        "target dir argument is not a directory"
    );
    // println!("y/n");
    // if get_user_confirmation() {
    //     panic!("yes!");
    // } else {
    //     panic!("no!");
    // }

    check_prattl_and_handle_status();

    let mut converted_paths = convert_all_files_in_dir_to_audio(&args.audio_dir)
        .expect("failed to convert files to mp3s");
    converted_paths.append(&mut audio_files_in_dir(&args.audio_dir));

    let path_strs: Vec<&str> = converted_paths
        .iter()
        .map(|p| p.to_str().unwrap())
        .collect();

    let prattl_out = run_prattl_transcribe(&path_strs).expect("failed to transcribe");

    println!("transcriptions gotten");
    fs::remove_dir_all(PathBuf::from(TMP_DIR)).expect("failed to remove temp dir");

    println!("Summarizing..");
    let sum_map = summarize_prattl_output(prattl_out, &args.target_dir).await;
    println!(
        "Got summaries, would you like to save your files to {:#?}?",
        args.target_dir
    );

    // if !get_user_confirmation() {
    //     println!("Exiting");
    //     return;
    // }

    for (path, contents) in sum_map {
        println!("writing to {path:#?}");
        fs::write(path, contents).expect("failed to write");
    }

    println!("Saved files, should I delete the originals?");

    // if get_user_confirmation() {
    for path in converted_paths {
        if path.exists() {
            let path = path.canonicalize().expect("could not get true path");
            fs::remove_file(path).expect("could not remove file");
        }
    }
    // }
}
