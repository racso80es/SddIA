use serde_json::{json, Value};
use std::io::{self, Read};
use std::path::PathBuf;

fn main() {
    let mut raw = String::new();
    let _ = io::stdin().read_to_string(&mut raw);
    let req: Value = serde_json::from_str(&raw).unwrap_or(json!({}));
    let repo = std::env::var("SDDIA_REPO_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    match file_tracker_adapter::dispatch(&repo, &req) {
        Ok(result) => {
            println!(
                "{}",
                json!({
                    "success": true,
                    "exitCode": 0,
                    "message": "file-tracker-adapter ok",
                    "result": result,
                })
            );
        }
        Err(err) => {
            println!(
                "{}",
                json!({
                    "success": false,
                    "exitCode": 1,
                    "message": err,
                })
            );
            std::process::exit(1);
        }
    }
}
