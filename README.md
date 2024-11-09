# Welcome
This is a really basic program for transcribing MOV files imported from an iPhone into a target directory.

## Dependencies
+ [prattl](https://prattl.co/) - for the local transcription model
    - [ffmpeg](https://ffmpeg.org/) - you likely already have this
    - [go](https://go.dev/)
+ [rust](https://www.rust-lang.org/)

Ensure your `~/.cargo/bin` folder is in your `$PATH`.
You can do this by running the below command, if you see a result from grep, you don't need to do anything:
```bash
echo $PATH | grep ".cargo/bin"
```

## How to use
```bash
git clone https://github.com/voidKandy/transcription_analyzer.git
cd transcription_analyzer
touch .env
```
Open `.env` in your editor and add the following variables: 
```bash
ANTH_KEY="your_anthropic_key"
# where your audio files are
AUDIO_DIR=
# where you want the summaries to go
TARGET_DIR=
```
Then you build the rust binary, and add it to your path:
Now you should be able to run:
```bash
source .env
cargo run --bin transcribe_and_summarize $AUDIO_DIR $TARGET_DIR
```

> **IMPORTANT**
> I know that the program asks for user input and doesn't actually wait for it. I'm working on it

