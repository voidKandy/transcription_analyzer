use std::{collections::HashMap, path::PathBuf};

use clap::Parser;
use espionox::{language_models::completions::functions::Function, prelude::*};
use serde_json::Value;
use tokio::sync::RwLock;

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
    let target_dir_path = args
        .target_dir
        .canonicalize()
        .expect("could not canonicalize target dir");

    let anth_key = std::env::var("ANTH_KEY").expect("Could not get api key");
    let openai_key = std::env::var("OPENAI_KEY").expect("Could not get api key");

    let fn_model = CompletionModel::default_openai(&openai_key);
    let fn_agent = espionox::agents::Agent::new(None, fn_model);

    let sum_model = CompletionModel::default_anthropic(&anth_key);
    let mut sum_agent = espionox::agents::Agent::new(Some(SUM_INIT_PROMPT), sum_model);

    let filecontent = std::fs::read_to_string(json_path).expect("could not read to string");

    let json: Value =
        serde_json::from_str(&filecontent).expect("failed to get json from filecontent");
    let obj = json.as_object().expect("could not coerce json to object");
    println!("{obj:#?}");

    // let mut map = HashMap::new();
    for (k, v) in obj.into_iter() {
        let summary = get_summary(&mut sum_agent, v.to_string().as_str()).await;
        let content = format!(
            r#"
{summary}

[^Recording]:
<small>
{v}
</small>"#
        );
        let filename = get_name_from_key(k);
        let filepath = target_dir_path.join(filename);
        std::fs::write(filepath, content).expect("could not write to file");
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

    let response = agent
        .do_action(io_completion, (), Option::<ListenerTrigger>::None)
        .await
        .unwrap();

    response
}

async fn get_filename(agent: &mut Agent, summary: &str) -> String {
    let function = Function::try_from(GET_NAME_FUNCTION).unwrap();
    let message = Message::new_user(summary);
    agent.cache.push(message);

    let json: Value = agent
        .do_action(
            function_completion,
            function,
            Option::<ListenerTrigger>::None,
        )
        .await
        .unwrap();
    println!("got json response: {:#?}", json);

    json.get("filename")
        .and_then(|v| Some(v.as_str().unwrap()))
        .expect("could not get filename from fn return")
        .to_string()
}
