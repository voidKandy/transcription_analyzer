use std::{collections::HashMap, path::PathBuf};

use espionox::prelude::*;
use serde_json::Value;
const SUM_INIT_PROMPT: &str = r#"
    You are a transcription summarization model. You will be given a transcription of an audio file, summarize the transcription into the key points of what is said. Make sure to format your response in markdown. You should start EVERY response with a ## header
    "#;

async fn get_summary(agent: &mut Agent, content: &str) -> String {
    let message = Message::new_user(content);
    agent.cache.push(message);

    let response = agent.io_completion().await.unwrap();

    response
}

fn format_summary_and_transcription(sum: &str, trans: &str) -> String {
    format!(
        r#"
---
tags: []
---

{sum}

[^Recording]:
<small>
{trans}
</small>
"#
    )
}

pub async fn summarize_prattl_output(
    prattl_out: Value,
    target_dir: &PathBuf,
) -> HashMap<PathBuf, String> {
    let anth_key = std::env::var("ANTH_KEY").expect("Could not get api key");
    let sum_model = CompletionModel::default_anthropic(&anth_key);
    let mut sum_agent = espionox::agents::Agent::new(Some(SUM_INIT_PROMPT), sum_model);

    let obj = prattl_out
        .as_object()
        .expect("could not coerce json to object")
        .to_owned();

    let mut outmap = HashMap::new();

    for (k, v) in obj.into_iter() {
        let v = v.to_string();
        if v.trim().is_empty() {
            continue;
        }
        let filename = get_name_from_key(&k);
        let filepath = target_dir.join(filename);
        let summary = get_summary(&mut sum_agent, v.as_str()).await;
        let contents = format_summary_and_transcription(&summary, v.as_str());
        outmap.insert(filepath, contents);
    }

    outmap
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
