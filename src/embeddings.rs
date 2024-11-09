// use std::{borrow::Cow, collections::HashMap};
//
// use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
//
// type TranscriptionGroups<'t> = HashMap<String, Cow<'t, str>>;
//
// fn group_transcriptions<'t>(transcriptions: &'t serde_json::Value) -> TranscriptionGroups<'t> {
//     let map = HashMap::new();
//     let obj = transcriptions
//         .as_object()
//         .expect("transcription json not object?");
//     let to_embed: Vec<&str> = obj
//         .values()
//         .into_iter()
//         .map(|v| v.as_str().expect("could not string from value"))
//         .collect();
//
//     let model = TextEmbedding::try_new(Default::default()).unwrap();
//     let embeddings = model.embed(to_embed, None).unwrap();
//
//     map
// }
//
// mod tests {
//     use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
//     #[test]
//     fn embeddings_work() {
//         // With default InitOptions
//         let model = TextEmbedding::try_new(Default::default()).unwrap();
//
//         // // With custom InitOptions
//         // let model = TextEmbedding::try_new(
//         //     InitOptions::new(EmbeddingModel::AllMiniLML6V2).with_show_download_progress(true),
//         // )
//         // .unwrap();
//
//         let documents = vec![
//             "passage: Hello, World!",
//             "query: Hello, World!",
//             "passage: This is an example passage.",
//             // You can leave out the prefix but it's recommended
//             "fastembed-rs is licensed under Apache  2.0",
//         ];
//
//         // Generate embeddings with the default batch size, 256
//         let embeddings = model.embed(documents, None).unwrap();
//
//         println!("Embeddings length: {}", embeddings.len()); // -> Embeddings length: 4
//         println!("Embedding dimension: {}", embeddings[0].len()); // -> Embedding dimension: 384
//     }
// }
