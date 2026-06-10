use sddia_io::{emit_error, emit_success, read_stdin_json};
use serde_json::{json, Value};
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::collections::HashSet;

fn fail(msg: &str) -> ! {
    emit_error(msg, 1);
    std::process::exit(1);
}

fn ok(stdout: String, stderr: String, exit_code: i32) {
    let success = exit_code == 0;
    let mut out = json!({
        "success": success,
        "exitCode": exit_code,
        "data": {
            "stdout": stdout,
            "stderr": stderr
        }
    });
    if !success {
        out["error"] = json!("command exited with non-zero status");
    }
    emit_success(Some(out));
}

fn assert_safe_token(token: &str, field: &str) {
    if token.chars().any(|c| "\n\r;|><`".contains(c)) || token.contains("&&") || token.contains("$(") || token.contains("&") {
        fail(&format!("{} contains forbidden shell metacharacters", field));
    }
}

fn resolve_working_dir(path_str: &str) -> PathBuf {
    let p = PathBuf::from(path_str);
    if !p.is_absolute() {
        fail("working_directory must be an absolute path");
    }
    let p = p.canonicalize().unwrap_or_else(|e| fail(&format!("working_directory invalid: {}", e)));
    if !p.is_dir() {
        fail("working_directory must exist and be a directory");
    }
    p
}

fn allowlist() -> HashSet<String> {
    let mut base: HashSet<String> = ["gh", "npm", "node", "python", "python3", "pwsh", "dotnet", "docker"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    if let Ok(extra) = env::var("SDDIA_SHELL_EXECUTOR_ALLOWLIST") {
        let extra = extra.trim();
        if !extra.is_empty() {
            for item in extra.split(',') {
                let it = item.trim();
                if !it.is_empty() {
                    base.insert(it.to_string());
                }
            }
        }
    }
    base
}

fn reject_if_git(executable: &str) {
    let ex = executable.trim().trim_matches('"').trim_matches('\'');
    let name = Path::new(ex).file_name().and_then(|n| n.to_str()).unwrap_or("").to_lowercase();
    if name == "git" || name == "git.exe" {
        fail("executable git is forbidden; route via git-manager");
    }
    // In Rust WASI we can't easily resolve executable paths via `shutil.which` without host access,
    // but WASI limits us to WASI anyway, so we just check the provided executable string.
}

fn main() {
    let doc = read_stdin_json();

    let executable = doc.get("executable").and_then(|v| v.as_str()).unwrap_or_else(|| fail("executable must be a string")).trim();
    let arguments = doc.get("arguments").and_then(|v| v.as_array()).unwrap_or_else(|| fail("arguments must be an array of strings"));
    let working_directory = doc.get("working_directory").and_then(|v| v.as_str()).unwrap_or_else(|| fail("working_directory must be a string"));

    let env_vars = match doc.get("environment_vars") {
        Some(Value::Null) | None => serde_json::Map::new(),
        Some(Value::Object(map)) => map.clone(),
        _ => fail("environment_vars must be an object (string->string)")
    };

    if executable.is_empty() {
        fail("executable must be non-empty");
    }

    reject_if_git(executable);
    assert_safe_token(executable, "executable");

    let mut args_vec = Vec::new();
    for (i, a) in arguments.iter().enumerate() {
        let a_str = a.as_str().unwrap_or_else(|| fail("arguments must be an array of strings"));
        assert_safe_token(a_str, &format!("arguments[{}]", i));
        args_vec.push(a_str);
    }

    let allow = allowlist();
    let exe_name = Path::new(executable).file_name().and_then(|n| n.to_str()).unwrap_or("").to_lowercase();

    if Path::new(executable).is_absolute() {
        if !allow.contains(&exe_name) && !allow.contains(&executable.to_lowercase()) {
            fail("executable is not allowlisted");
        }
    } else {
        if !allow.contains(&exe_name) && !allow.contains(&executable.to_lowercase()) {
            fail("executable is not allowlisted");
        }
    }

    let wd = resolve_working_dir(working_directory);

    // Prepare environment
    let mut command = Command::new(executable);
    command.args(&args_vec);
    command.current_dir(&wd);

    // Pass existing environment vars plus new ones
    for (k, v) in env::vars() {
        command.env(k, v);
    }
    for (k, v) in env_vars.iter() {
        if let Some(v_str) = v.as_str() {
            command.env(k, v_str);
        } else {
            fail("environment_vars must be an object (string->string)");
        }
    }

    let output = command.output().unwrap_or_else(|_| fail("executable not found on PATH"));

    ok(
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
        output.status.code().unwrap_or(1)
    );
}
