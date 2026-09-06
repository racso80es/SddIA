use serde_json::{json, Value};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn git(repo: &Path, args: &[&str]) {
    let st = Command::new("git")
        .args(args)
        .current_dir(repo)
        .status()
        .expect("git");
    assert!(st.success(), "git {args:?} failed");
}

fn init_repo() -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "gm-cs-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).unwrap();
    git(&dir, &["init"]);
    git(&dir, &["config", "user.email", "test@example.com"]);
    git(&dir, &["config", "user.name", "test"]);
    git(&dir, &["commit", "--allow-empty", "-m", "root"]);
    dir
}

fn gm(_repo: &Path, payload: Value) -> (i32, Value) {
    let exe = env!("CARGO_BIN_EXE_git-manager");
    let mut child = Command::new(exe)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn git-manager");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(serde_json::to_string(&payload).unwrap().as_bytes())
        .unwrap();
    let out = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    let line = stdout.lines().last().unwrap_or("").trim();
    let v: Value = serde_json::from_str(line).unwrap_or_else(|_| json!({"raw": line}));
    (out.status.code().unwrap_or(1), v)
}

fn summary_payload(repo: &Path, extra: Value) -> Value {
    json!({
        "operation_type": "commit_summary",
        "repository_path": repo.to_string_lossy(),
        "operation_payload_json": extra
    })
}

#[test]
fn commit_summary_rejects_extra_keys() {
    let repo = init_repo();
    let (code, body) = gm(
        &repo,
        summary_payload(
            &repo,
            json!({"ref": "HEAD", "max_files": 30, "max_subject_chars": 200, "extra": true}),
        ),
    );
    assert_ne!(code, 0);
    let msg = body.get("error").and_then(|v| v.as_str()).unwrap_or("");
    assert!(msg.contains("payload keys must exactly match"), "{msg}");
    let _ = fs::remove_dir_all(&repo);
}

#[test]
fn commit_summary_rejects_unsafe_ref() {
    let repo = init_repo();
    let (code, body) = gm(
        &repo,
        summary_payload(
            &repo,
            json!({"ref": "HEAD;rm", "max_files": 30, "max_subject_chars": 200}),
        ),
    );
    assert_ne!(code, 0);
    let msg = body.get("error").and_then(|v| v.as_str()).unwrap_or("");
    assert!(msg.contains("forbidden shell metacharacters"), "{msg}");
    let _ = fs::remove_dir_all(&repo);
}

#[test]
fn commit_summary_rejects_max_files_out_of_range() {
    let repo = init_repo();
    let (code, body) = gm(
        &repo,
        summary_payload(
            &repo,
            json!({"ref": "HEAD", "max_files": 0, "max_subject_chars": 200}),
        ),
    );
    assert_ne!(code, 0);
    let msg = body.get("error").and_then(|v| v.as_str()).unwrap_or("");
    assert!(msg.contains("max_files must be between"), "{msg}");
    let _ = fs::remove_dir_all(&repo);
}

#[test]
fn commit_summary_subject_and_files_first_parent() {
    let repo = init_repo();
    fs::write(repo.join("a.txt"), "a").unwrap();
    fs::write(repo.join("b.txt"), "b").unwrap();
    git(&repo, &["add", "a.txt", "b.txt"]);
    git(&repo, &["commit", "-m", "add two files"]);
    let (code, body) = gm(
        &repo,
        summary_payload(
            &repo,
            json!({"ref": "HEAD", "max_files": 30, "max_subject_chars": 200}),
        ),
    );
    assert_eq!(code, 0, "{body}");
    let data = body.pointer("/result/data").unwrap();
    assert_eq!(data["subject"], json!("add two files"));
    assert_eq!(data["truncated"], json!(false));
    assert_eq!(data["totalFilesChanged"], json!(2));
    let files = data["files"].as_array().unwrap();
    let names: Vec<&str> = files.iter().filter_map(|v| v.as_str()).collect();
    assert!(names.contains(&"a.txt"));
    assert!(names.contains(&"b.txt"));
    let hash = data["commitHash"].as_str().unwrap();
    assert_eq!(hash.len(), 40);
    let _ = fs::remove_dir_all(&repo);
}

#[test]
fn commit_summary_truncates_subject_and_files() {
    let repo = init_repo();
    for i in 0..5 {
        let name = format!("f{i}.txt");
        fs::write(repo.join(&name), "x").unwrap();
        git(&repo, &["add", &name]);
    }
    git(&repo, &["commit", "-m", "ABCDEFGHIJKLMNOPQRSTUVWXYZ"]);
    let (code, body) = gm(
        &repo,
        summary_payload(
            &repo,
            json!({"ref": "HEAD", "max_files": 2, "max_subject_chars": 8}),
        ),
    );
    assert_eq!(code, 0, "{body}");
    let data = body.pointer("/result/data").unwrap();
    assert_eq!(data["subject"], json!("ABCDEFGH"));
    assert_eq!(data["truncated"], json!(true));
    assert_eq!(data["totalFilesChanged"], json!(5));
    assert_eq!(data["files"].as_array().unwrap().len(), 2);
    let _ = fs::remove_dir_all(&repo);
}

#[test]
fn commit_summary_root_commit_inner_failure() {
    let repo = init_repo();
    let (code, body) = gm(
        &repo,
        summary_payload(
            &repo,
            json!({"ref": "HEAD", "max_files": 30, "max_subject_chars": 200}),
        ),
    );
    assert_eq!(code, 0, "envelope always 0 on emit_success: {body}");
    let inner = body.pointer("/result/success").and_then(|v| v.as_bool());
    assert_eq!(inner, Some(false), "{body}");
    let _ = fs::remove_dir_all(&repo);
}
