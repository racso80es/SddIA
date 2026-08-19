use sddia_io::outbound_lab::{lab_mock_outbound_enabled, truthy_env};
use sddia_io::read_stdin_json;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::time::Duration;

const ENTITY_ID: &str = "github-raw-fetcher";

fn emit_v2(success: bool, exit_code: i32, result: Option<Value>, error: Option<&str>) -> ! {
    let mut body = json!({
        "meta": {
            "schemaVersion": "2.0",
            "entityKind": "tool",
            "entityId": ENTITY_ID,
        },
        "success": success,
        "exitCode": exit_code,
    });
    if let Some(r) = result {
        body["result"] = r;
    }
    if let Some(e) = error {
        body["error"] = json!({ "code": "FETCH_FAILED", "message": e });
    }
    println!("{}", body);
    process::exit(exit_code);
}

fn request_fields(req: &Value) -> Result<(String, String), String> {
    let inner = req.get("request").unwrap_or(req);
    let asset_path = inner
        .get("asset_path")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "request.asset_path obligatorio".to_string())?;
    let git_ref = inner
        .get("ref")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("main")
        .to_string();
    Ok((asset_path.to_string(), git_ref))
}

fn sha256_hex(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("sha256:{}", hex::encode(hasher.finalize()))
}

fn find_repo_root(start: &Path) -> PathBuf {
    let mut cur = start.to_path_buf();
    for _ in 0..12 {
        if cur.join("SddIA/core/cumulo.paths.json").is_file() {
            return cur;
        }
        if !cur.pop() {
            break;
        }
    }
    start.to_path_buf()
}

fn read_local(repo: &Path, asset_path: &str) -> Option<String> {
    let local = repo.join(asset_path.trim_start_matches('/'));
    fs::read_to_string(local).ok()
}

fn fetch_remote(base: &str, git_ref: &str, asset_path: &str) -> Result<String, String> {
    let base = base.trim_end_matches('/');
    let path = asset_path.trim_start_matches('/');
    let url = format!("{base}/{git_ref}/{path}");
    let agent = ureq::agent();
    let resp = agent
        .get(&url)
        .timeout(Duration::from_secs(30))
        .call()
        .map_err(|e| format!("http-fetch-failed: {e}"))?;
    if resp.status() >= 400 {
        return Err(format!("http-status-{}", resp.status()));
    }
    resp.into_string()
        .map_err(|e| format!("http-body-read-failed: {e}"))
}

fn run(req: &Value) -> Result<Value, String> {
    let (asset_path, git_ref) = request_fields(req)?;

    if lab_mock_outbound_enabled() {
        let content = format!("# mock asset\npath: {asset_path}\n");
        return Ok(json!({
            "content": content,
            "declared_hash": sha256_hex(&content),
            "origin_kind": "git-raw-mock",
        }));
    }

    let repo = find_repo_root(&env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    if let Some(content) = read_local(&repo, &asset_path) {
        return Ok(json!({
            "content": content,
            "declared_hash": sha256_hex(&content),
            "origin_kind": "git-raw-local",
        }));
    }

    if truthy_env("SDDIA_LAB_SKIP_OUTBOUND_HTTP") {
        return Err("outbound-http-skipped-lab".into());
    }

    let base = env::var("SDDIA_GITHUB_RAW_BASE")
        .unwrap_or_else(|_| "https://raw.githubusercontent.com/racso80es/SddIA".to_string());
    let content = fetch_remote(&base, &git_ref, &asset_path)?;
    Ok(json!({
        "content": content,
        "declared_hash": sha256_hex(&content),
        "origin_kind": "git-raw",
    }))
}

fn main() {
    let req = read_stdin_json();
    match run(&req) {
        Ok(result) => emit_v2(true, 0, Some(result), None),
        Err(msg) => emit_v2(false, 1, None, Some(&msg)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_prefix() {
        let h = sha256_hex("hello");
        assert!(h.starts_with("sha256:"));
        assert_eq!(h.len(), 7 + 64);
    }

    #[test]
    fn rejects_missing_asset_path() {
        let err = request_fields(&json!({})).unwrap_err();
        assert!(err.contains("asset_path"));
    }
}
