use std::path::Path;
use std::process::Command;

use crate::resolve::{has_flag, print_json_report};

pub fn run(repo: &Path, args: &[String]) -> i32 {
    let json_out = has_flag(args, "--json");
    let mut cmd = Command::new("cargo");
    cmd.args([
        "test",
        "-p",
        "execute-process",
        "linear_direct_cycle_e2e",
        "--",
        "--test-threads=1",
    ])
    .current_dir(repo.join("SddIA"));
    let out = cmd.output();
    match out {
        Ok(o) => {
            let success = o.status.success();
            if json_out {
                print_json_report(
                    &serde_json::json!({
                        "success": success,
                        "exitCode": if success { 0 } else { 1 },
                        "operation": "run-linear-direct-cycle-e2e-lab",
                    }),
                    false,
                );
            } else if !success {
                eprintln!("{}", String::from_utf8_lossy(&o.stderr));
                eprintln!("{}", String::from_utf8_lossy(&o.stdout));
            }
            if success {
                0
            } else {
                1
            }
        }
        Err(e) => {
            if json_out {
                print_json_report(
                    &serde_json::json!({
                        "success": false,
                        "error": e.to_string(),
                    }),
                    false,
                );
            }
            1
        }
    }
}
