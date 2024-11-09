use serde_json::Value;
use std::{
    borrow::Cow,
    io::{self},
    process::{Command, Output},
};

use crate::get_user_confirmation;

pub fn run_prattl_transcribe(args: &[&str]) -> io::Result<Value> {
    let prattl_out = Command::new("prattl")
        .arg("transcribe")
        .args(args)
        .output()
        .expect("prattl failed");
    Ok(transcribe_output_to_json(&prattl_out))
}

pub fn check_prattl_and_handle_status() {
    let status = prattl_status();
    status.handle().expect("")
}

fn prattl_status() -> PrattlStatus {
    if Command::new("which")
        .arg("prattl")
        .output()
        .expect("failed to run prattl report")
        .status
        .code()
        != Some(0)
    {
        return PrattlStatus::NotInstalled;
    }
    let report_cmd = Command::new("prattl")
        .arg("report")
        .output()
        .expect("failed to run prattl report");

    let report = String::from_utf8_lossy(&report_cmd.stdout).to_lowercase();

    if report.contains("doesn't exist") {
        return PrattlStatus::NotPrepared;
    } else if report.contains("compressed: true") {
        return PrattlStatus::Compressed;
    }

    PrattlStatus::Ready
}

enum PrattlStatus {
    NotInstalled,
    NotPrepared,
    Compressed,
    Ready,
}

impl PrattlStatus {
    /// Consumes status and returns whether the program should continue or not
    fn handle(self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Ready => {
                println!("prattl is ready");
                Ok(())
            }
            Self::Compressed => {
                println!("prattl is compressed, run prattl decompress? (Y/n)");
                if get_user_confirmation() {
                    Command::new("prattl")
                        .arg("decompress")
                        .output()
                        .expect("failed to run prepare");
                    return Ok(());
                }
                Err("User did not want program to decompress prattl".into())
            }
            Self::NotPrepared => {
                println!("prattl is not prepared, run prattl prepare? (Y/n)");
                if get_user_confirmation() {
                    Command::new("prattl")
                        .arg("prepare")
                        .output()
                        .expect("failed to run prepare");
                    return Ok(());
                }
                Err("User did not want program to prepare prattl".into())
            }
            Self::NotInstalled => {
                println!("prattl is not installed, you can find instructions on installation here: https://www.prattl.co");
                Err("prattl not installed".into())
            }
        }
    }
}

fn transcribe_output_to_json(out: &Output) -> Value {
    let stdout: Cow<'_, str> = String::from_utf8_lossy(&out.stdout);
    assert!(
        !stdout.as_ref().trim().is_empty(),
        "empty transcription from prattl"
    );

    let first_curly_pos = stdout
        .chars()
        .position(|ch| ch == '{')
        .expect("prattl did not output json");

    let last_curly_pos = stdout
        .chars()
        .rev()
        .position(|ch| ch == '}')
        .expect("prattl did not output json")
        .abs_diff(stdout.chars().count());

    let output_to_json = &stdout.as_ref()[first_curly_pos..=last_curly_pos];
    // println!("OUTPUT: {output_to_json}");

    match serde_json::from_str(output_to_json) {
        serde_json::Result::Err(e) => {
            panic!("failed to convert to json {e:#?}",)
        }

        serde_json::Result::Ok(v) => v,
    }
}

mod tests {

    use crate::prattl::run_prattl_transcribe;

    const TEST_AUDIO_FILE: &str = ".test/.test.mp3";
    #[test]
    fn into_json_works() {
        let _ = run_prattl_transcribe(&[TEST_AUDIO_FILE]);
        assert!(true);
    }
}
