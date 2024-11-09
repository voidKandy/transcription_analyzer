// use linfa::prelude::SilhouetteScore;
// use linfa::traits::Fit;
// use linfa::traits::Predict;
// use linfa::DatasetBase;
// use linfa_clustering::KMeans;
// use linfa_datasets::generate;
// use ndarray::{array, Axis};
// use ndarray_npy::write_npy;
// use ndarray_rand::rand::SeedableRng;
// use rand_xoshiro::Xoshiro256Plus;
//
// use linfa_nn::distance::LInfDist;
//
// // A routine K-means task: build a synthetic dataset, fit the algorithm on it
// // and save both training data and predictions to disk.
// pub fn cluster_embeddings(embeddings: ndarray::Array2<f32>) {
//     // Our random number generator, seeded for reproducibility
//     let n = embeddings.nrows();
//     let k_range = 1..=n;
//     let mut sil_scores = Vec::<(usize, f32)>::with_capacity(n);
//
//     // Iterate through possible ideal clusters
//     let dataset = DatasetBase::from(embeddings);
//
//     for k in k_range {
//         let rng = Xoshiro256Plus::seed_from_u64(42);
//         let model: KMeans<f32, _> = KMeans::params_with(k, rng, LInfDist)
//             .max_n_iterations(20)
//             .tolerance(1e-1)
//             .fit(&dataset)
//             .expect("KMeans fitted");
//
//         // bad
//         let pred = model.predict(dataset.clone());
//
//         let DatasetBase {
//             ref records,
//             ref targets,
//             ..
//         } = pred;
//
//         let score: f32 = pred.silhouette_score().unwrap();
//         sil_scores.push((sil_scores.len() + 1, score));
//     }
//
//     println!("scores: {sil_scores:#?}");
//     let ideal_n_clusters = sil_scores
//         .into_iter()
//         .filter(|x| x.1 != 1.0)
//         .max_by(|a, b| a.1.total_cmp(&b.1))
//         .unwrap()
//         .0;
//     println!("ideal k: {ideal_n_clusters}");
//
//     // For each our expected centroids, generate `n` data points around it (a "blob")
//
//     // Save to disk our dataset (and the cluster label assigned to each observation)
//     // We use the `npy` format for compatibility with NumPy
//     // write_npy("clustered_dataset.npy", &records).expect("Failed to write .npy file");
//     // write_npy("clustered_memberships.npy", &targets.map(|&x| x as u64))
//     //     .expect("Failed to write .npy file");
// }
//
// mod tests {
//     use std::path::PathBuf;
//
//     use fastembed::TextEmbedding;
//     use ndarray::{array, Array, Array2};
//
//     use super::cluster_embeddings;
//
//     const OUT_DIR: &str = "out";
//
//     #[test]
//     fn cluster_test() {
//         let all_contents: Vec<String> = std::fs::read_dir(PathBuf::from(OUT_DIR))
//             .unwrap()
//             .into_iter()
//             .filter_map(|e| {
//                 let p = e.unwrap().path();
//                 if p.is_file() & p.exists() {
//                     Some(std::fs::read_to_string(p).unwrap())
//                 } else {
//                     None
//                 }
//             })
//             .collect();
//
//         // println!("all contents: {all_contents:?}");
//
//         let model = TextEmbedding::try_new(Default::default()).unwrap();
//         let embeddings = model.embed(all_contents, None).unwrap();
//         let rows = embeddings.len();
//
//         let flat: Vec<f32> = embeddings.into_iter().flatten().collect();
//         // 384 dimensions
//         let shape = (rows, 384);
//         cluster_embeddings(Array::from_shape_vec(shape, flat).unwrap());
//         assert!(false);
//     }
// }
