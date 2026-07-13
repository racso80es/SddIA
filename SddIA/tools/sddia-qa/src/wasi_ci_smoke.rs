use serde_json::{json, Value};
use std::path::Path;
use std::process::Command;

use crate::resolve::{has_flag, print_json_report};

fn wasm_artifact(repo: &Path, name: &str) -> std::path::PathBuf {
    repo.join("SddIA/target/wasm32-wasip1/debug")
        .join(format!("{name}.wasm"))
}

fn run_wasmtime(wasm: &Path, payload: &Value, dir_mount: Option<&str>) -> Result<Value, String> {
    let mut cmd = Command::new("wasmtime");
    cmd.arg("run");
    if let Some(mount) = dir_mount {
        cmd.arg(format!("--dir={mount}"));
    }
    cmd.arg(wasm);
    cmd.stdin(std::process::Stdio::piped());
    let input = serde_json::to_string(payload).map_err(|e| e.to_string())?;
    cmd.stdin(std::process::Stdio::piped());
    let mut child = cmd
        .spawn()
        .map_err(|e| format!("wasmtime spawn: {e}"))?;
    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        let _ = stdin.write_all(input.as_bytes());
    }
    let output = child.wait_with_output().map_err(|e| e.to_string())?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout.trim().lines().last().unwrap_or("");
    if line.is_empty() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }
    let body: Value = serde_json::from_str(line).map_err(|e| e.to_string())?;
    if body.get("success") != Some(&json!(true)) {
        return Err(body
            .get("error")
            .or_else(|| body.get("message"))
            .and_then(|v| v.as_str())
            .unwrap_or("wasmtime falló")
            .to_string());
    }
    Ok(body)
}

pub fn run(repo: &Path, args: &[String]) -> i32 {
    let json_out = has_flag(args, "--json");
    let skip_e2e = has_flag(args, "--skip-e2e");
    let mut report = json!({"steps": [], "wasi_path_verified": false, "success": false});
    let mut exit_code = 1;

    let result = (|| -> Result<(), String> {
        let wasmtime_bin = which::which("wasmtime").map_err(|_| "wasmtime not in PATH".to_string())?;
        let ver = Command::new("wasmtime")
            .arg("--version")
            .output()
            .map_err(|e| e.to_string())?;
        report["steps"].as_array_mut().unwrap().push(json!({
            "wasmtime": wasmtime_bin.to_string_lossy(),
            "version": String::from_utf8_lossy(&ver.stdout).trim(),
        }));

        let crypto_wasm = wasm_artifact(repo, "cryptography-manager");
        let wasi_poc = wasm_artifact(repo, "wasi-poc");
        for (label, path) in [("cryptography-manager", &crypto_wasm), ("wasi-poc", &wasi_poc)] {
            if !path.is_file() {
                return Err(format!("artefacto WASI ausente: {}", path.display()));
            }
            report["steps"].as_array_mut().unwrap().push(json!({
                "artifact": label,
                "path": path.strip_prefix(repo).unwrap_or(path).to_string_lossy(),
            }));
        }

        let uuid_body = run_wasmtime(
            &crypto_wasm,
            &json!({"operation": "GENERATE_UUID", "target_payload": null}),
            Some("/"),
        )?;
        let uuid_val = uuid_body.get("result").and_then(|v| v.as_str());
        if uuid_val.unwrap_or("").is_empty() {
            return Err("cryptography-manager WASM no devolvió UUID".into());
        }
        report["steps"]
            .as_array_mut()
            .unwrap()
            .push(json!({"crypto_wasm": "GENERATE_UUID", "ok": true}));

        let poc_body = run_wasmtime(
            &wasi_poc,
            &json!({
                "meta": {"schemaVersion": "2.0", "entityKind": "tool", "entityId": "wasi-poc"},
                "request": {"ping": true},
            }),
            None,
        )?;
        if poc_body
            .get("result")
            .and_then(|r| r.get("echo"))
            .is_none()
        {
            return Err("wasi-poc WASM no devolvió echo esperado".into());
        }
        report["steps"]
            .as_array_mut()
            .unwrap()
            .push(json!({"wasi_poc": "echo", "ok": true}));
        report["wasi_path_verified"] = true.into();

        if !skip_e2e {
            let qa = {
                let mut found = None;
                for rel in ["SddIA/target/debug/sddia-qa", "SddIA/target/release/sddia-qa"] {
                    let c = repo.join(rel);
                    if c.is_file() {
                        found = Some(c);
                        break;
                    }
                }
                found.ok_or_else(|| "sddia-qa no encontrado para e2e".to_string())?
            };
            std::env::set_var("SDDIA_CI_REQUIRE_WASI", "1");
            std::env::set_var("SDDIA_LAB_SIMULATE_IOTA", "1");
            std::env::set_var("SDDIA_LAB_SIMULATE_SYNC_INDEX", "1");
            std::env::set_var("SDDIA_LAB_ROUTE_SYNC", "1");
            let output = Command::new(&qa)
                .args(["run-eda-e2e-lab", "--entity-class", "tool", "--json"])
                .current_dir(repo)
                .output()
                .map_err(|e| e.to_string())?;
            let stdout = String::from_utf8_lossy(&output.stdout);
            let line = stdout.trim();
            if line.is_empty() {
                return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
            }
            let e2e: Value = serde_json::from_str(line).map_err(|e| e.to_string())?;
            report["steps"].as_array_mut().unwrap().push(json!({
                "eda_e2e": e2e.get("success"),
                "cleaned": e2e.get("cleaned"),
            }));
            if e2e.get("success") != Some(&json!(true)) {
                return Err(e2e
                    .get("error")
                    .and_then(|v| v.as_str())
                    .unwrap_or("run-eda-e2e-lab falló")
                    .to_string());
            }
        }
        report["success"] = true.into();
        Ok(())
    })();

    if let Err(e) = result {
        report["error"] = e.into();
    } else {
        exit_code = 0;
    }
    print_json_report(&report, json_out);
    exit_code
}

// Minimal which without extra dep
mod which {
    use std::path::PathBuf;
    use std::process::Command;
    pub fn which(name: &str) -> Result<PathBuf, ()> {
        let output = Command::new("sh")
            .args(["-c", &format!("command -v {name}")])
            .output()
            .map_err(|_| ())?;
        if !output.status.success() {
            return Err(());
        }
        let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if path.is_empty() {
            Err(())
        } else {
            Ok(PathBuf::from(path))
        }
    }
}
