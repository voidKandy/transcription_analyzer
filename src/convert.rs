use std::{
    collections::HashMap,
    io,
    path::PathBuf,
    process::{Child, Command},
};

pub const TMP_DIR: &str = "tmp";
pub fn convert_all_files_in_dir_to_audio(dir: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut map = HashMap::new();
    let tmp_dir_path = PathBuf::from(TMP_DIR);
    if !tmp_dir_path.exists() {
        std::fs::create_dir(tmp_dir_path)?;
    }
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_file() {
            let file_name = path
                .file_name()
                .expect("couldn't get a file name from path");
            if let Some((name, ext)) = file_name.to_string_lossy().rsplit_once('.') {
                if ext.to_lowercase() == "mov" {
                    let target = PathBuf::from(format!("{TMP_DIR}/{name}.mp3"));
                    let child = spawn_ffmpeg_mov_to_mp3(&path, &target);
                    map.insert(target, child);
                }
            }
        }
    }

    let mut return_paths = vec![];
    for (name, mut proc) in map.into_iter() {
        if let Ok(Some(0)) = proc.wait().and_then(|s| Ok(s.code())) {
            return_paths.push(name);
        } else {
            println!("{name:#?} had a problem being made from .MOV");
        }
    }
    Ok(return_paths)
}

fn spawn_ffmpeg_mov_to_mp3(mov: &PathBuf, target: &PathBuf) -> Child {
    Command::new("ffmpeg")
        .args([
            "-i",
            mov.to_str().unwrap(),
            "-vn",
            "-acodec",
            "libmp3lame",
            "-q:a",
            "2",
            target.to_str().unwrap(),
        ])
        .spawn()
        .expect("failed to spawn process")
}

mod tests {
    use crate::convert::convert_all_files_in_dir_to_audio;
    use std::path::PathBuf;

    #[test]
    fn convert_works() {
        convert_all_files_in_dir_to_audio(&PathBuf::from(".test")).unwrap();
        assert!(true);
    }
}
