use std::{
    collections::HashMap,
    io::{self, Write},
    path::PathBuf,
    sync::{Arc, RwLock},
};

use clap::Parser;
use espionox::{language_models::completions::functions::Function, prelude::*};
use serde_json::Value;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'j', long)]
    pub transcription_json: PathBuf,
    #[arg(short = 't', long)]
    pub target_dir: PathBuf,
}
const GET_NAME_FUNCTION: &str = r#"get_name_of_file(filename!: string) 
        i = 'Based on a summary of a transcription, infer a good name for the summary'
        "#;
const SUM_INIT_PROMPT: &str = r#"
    You are a transcription summarization model. You will be given a transcription of an audio file, summarize the transcription into the key points of what is said. Make sure to format your response in markdown. You should start EVERY response with a ## header
    "#;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let args = Args::parse();
    let json_path = args
        .transcription_json
        .canonicalize()
        .expect("could not canonicalize filepath");
    let target_dir_path = Arc::new(
        args.target_dir
            .canonicalize()
            .expect("could not canonicalize target dir"),
    );

    // let openai_key = std::env::var("OPENAI_KEY").expect("Could not get api key");
    // let fn_model = CompletionModel::default_openai(&openai_key);
    // let fn_agent = espionox::agents::Agent::new(None, fn_model);

    let anth_key = std::env::var("ANTH_KEY").expect("Could not get api key");
    let sum_model = CompletionModel::default_anthropic(&anth_key);
    let mut sum_agent = espionox::agents::Agent::new(Some(SUM_INIT_PROMPT), sum_model);

    let filecontent = std::fs::read_to_string(json_path).expect("could not read to string");

    let json: Value =
        serde_json::from_str(&filecontent).expect("failed to get json from filecontent");
    let obj = json
        .as_object()
        .expect("could not coerce json to object")
        .to_owned();
    println!("{obj:#?}");

    let write_files = Arc::new(RwLock::new(None));
    let mut handles = vec![];
    for (k, v) in obj.into_iter() {
        let v = v.to_string();
        if v.trim().is_empty() {
            continue;
        }
        let summary = get_summary(&mut sum_agent, v.as_str()).await;
        let thread_to_write_files = write_files.clone();
        let thread_dir_path = target_dir_path.clone();

        let handle = std::thread::spawn(move || {
            let content = format!(
                r#"
{summary}

[^Recording]:
<small>
{v}
</small>"#
            );
            let filename = get_name_from_key(&k);
            let filepath = thread_dir_path.join(filename);
            loop {
                let thread_check = thread_to_write_files
                    .read()
                    .expect("could not get read guard");
                match *thread_check {
                    Some(write) => {
                        if write {
                            std::fs::write(filepath, content).expect("could not write to file");
                        }
                        return;
                    }
                    None => {}
                }
            }
        });
        println!("Spawned all tasks");
        handles.push(handle);
    }

    io::stdout().flush().unwrap();
    print!(
        "Finished transcribing, would you like to save to {:#?}?",
        *target_dir_path
    );

    if verify_write_files() {
        let mut should = write_files.write().expect("failed to get write lock");
        *should = Some(true);
    }

    handles
        .into_iter()
        .for_each(|h| h.join().expect("Failed to join a thread"))
}

fn verify_write_files() -> bool {
    loop {
        println!("  (Y/N)?  ");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input
            .trim()
            .to_lowercase()
            .chars()
            .next()
            .and_then(|ch| Some(ch == 'y'))
        {
            Some(valid_input) => return valid_input,
            None => println!("{input} is not valid input"),
        }
    }
}

fn get_name_from_key(key: &str) -> String {
    let split0 = match key.rsplit_once('.') {
        Some((split0, _)) => split0,
        None => key,
    };

    let name = match split0.split_once(std::path::MAIN_SEPARATOR) {
        Some((_, name)) => name,
        None => split0,
    };
    format!("{}.md", name.to_lowercase().to_string())
}

async fn get_summary(agent: &mut Agent, content: &str) -> String {
    let message = Message::new_user(content);
    agent.cache.push(message);

    let response = agent.io_completion().await.unwrap();

    response
}

async fn get_filename(agent: &mut Agent, summary: &str) -> String {
    let function = Function::try_from(GET_NAME_FUNCTION).unwrap();
    let message = Message::new_user(summary);
    agent.cache.push(message);

    let json: Value = agent.function_completion(function).await.unwrap();
    println!("got json response: {:#?}", json);

    json.get("filename")
        .and_then(|v| Some(v.as_str().unwrap()))
        .expect("could not get filename from fn return")
        .to_string()
}
