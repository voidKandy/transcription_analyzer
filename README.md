# Welcome
This is a really basic program for transcribing MOV files imported from an iPhone into a target directory.

## Dependencies
+ [prattl](https://prattl.co/) - for the local transcription model
    - [ffmpeg](https://ffmpeg.org/) - you likely already have this
    - [go](https://go.dev/)
+ [rust](https://www.rust-lang.org/)

## How to use
```bash
git clone https://github.com/voidKandy/transcription_analyzer.git
cd transcription_analyzer
touch .env
```
Open `.env` in your editor and add the following variables: 
```bash
ANTH_KEY="your_anthropic_key"
```
Now you should be able to run:
```bash
cargo run --bin transcribe_and_summarize <dir where your .MOV files are> <output dir> 
```

> **IMPORTANT**
> I know that the program asks for user input and doesn't actually wait for it. I'm working on it

