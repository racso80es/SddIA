use sddia_io::{emit_error, emit_success, read_stdin_json};
use serde_json::{json, Value};
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::collections::HashSet;
use std::iter::FromIterator;

fn fail(msg: &str) -> ! {
    emit_error(msg, 1);
    std::process::exit(1);
}

fn ok(data: Value, git_exit: i32) {
    let success = git_exit == 0;
    let mut out = json!({
        "success": success,
        "exitCode": git_exit,
        "data": data
    });
    if !success {
        out["error"] = data.get("errorSummary").cloned().unwrap_or(json!("git exited with non-zero status"));
    }
    emit_success(Some(out));
}

fn assert_safe_token(token: &str, field: &str) {
    if token.starts_with('-') {
        fail(&format!("{} must not start with a dash", field));
    }
    if token.chars().any(|c| "\n\r;|&$`<>()".contains(c)) {
        fail(&format!("{} contains forbidden shell metacharacters", field));
    }
}

fn payload_exact(payload: &Value, op: &str, expected: &[&str]) {
    let obj = payload.as_object().unwrap_or_else(|| fail("payload must be an object"));
    let mut provided: HashSet<&str> = HashSet::from_iter(obj.keys().map(|k| k.as_str()));
    let expected_set: HashSet<&str> = HashSet::from_iter(expected.iter().cloned());
    if provided != expected_set {
        fail(&format!("{} payload keys must exactly match {:?}", op, expected));
    }
}

fn git_exe() -> String {
    "git".to_string() // Assumed to be on path
}

fn resolve_repo(path_str: &str) -> PathBuf {
    let p = PathBuf::from(path_str);
    // no is_absolute for WASI
    // no canonicalize for WASI
    // no is_dir for WASI
    p
}

fn verify_git_repo(repo: &Path) {
    let output = Command::new(git_exe())
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .current_dir(repo)
        .output()
        .unwrap_or_else(|e| fail(&format!("failed to execute git: {}", e)));
    if !output.status.success() {
        fail("repository_path is not a valid git workspace");
    }
}

fn run_git(repo: &Path, args: &[&str]) -> (String, String, i32) {
    let output = Command::new(git_exe())
        .args(args)
        .current_dir(repo)
        .output()
        .unwrap_or_else(|e| fail(&format!("failed to execute git: {}", e)));
    (
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
        output.status.code().unwrap_or(1)
    )
}

fn parse_commit_hash(stdout: &str, field: &str) -> String {
    let hash = stdout.trim();
    if hash.len() != 40 || !hash.chars().all(|c| c.is_ascii_hexdigit()) {
        fail(&format!("git returned invalid {} ({})", field, hash));
    }
    hash.to_string()
}

fn rel_path_under_repo(repo: &Path, path_str: &str) -> String {
    let p = Path::new(path_str);
    if p.is_absolute() {
        fail("file path must be relative to the repository");
    }
    if p.components().any(|c| matches!(c, std::path::Component::ParentDir)) {
        fail("file path escapes the repository");
    }
    let full = repo.join(p);
    // Paths ausentes (p.ej. deletes en porcelain) deben poder pasar a `git add --`.
    if !full.exists() {
        return path_str
            .trim()
            .trim_end_matches('/')
            .replace('\\', "/");
    }
    let repo_canon = repo
        .canonicalize()
        .unwrap_or_else(|_| fail("repository path does not exist"));
    let full = full
        .canonicalize()
        .unwrap_or_else(|_| fail("file path does not exist"));
    if !full.starts_with(&repo_canon) {
        fail("file path escapes the repository");
    }
    full.strip_prefix(&repo_canon)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

fn handle(op: &str, repo: &Path, payload: &Value) -> (Value, i32) {
    match op {
        "status" => {
            payload_exact(payload, op, &[]);
            let (stdout, stderr, code) = run_git(repo, &["status", "--porcelain=v1"]);
            let err_summary = if stderr.trim().is_empty() { Value::Null } else { json!(stderr.trim()) };
            (json!({ "gitStdout": stdout, "gitStderr": stderr, "errorSummary": err_summary }), code)
        }
        "branch_list" => {
            payload_exact(payload, op, &[]);
            let (stdout, stderr, code) = run_git(repo, &["branch", "-a", "-v", "--no-abbrev"]);
            let branches: Vec<&str> = stdout.lines().filter(|l| !l.trim().is_empty()).collect();
            let err_summary = if stderr.trim().is_empty() { Value::Null } else { json!(stderr.trim()) };
            (json!({ "gitStdout": stdout, "gitStderr": stderr, "branches": branches, "errorSummary": err_summary }), code)
        }
        "get_last_commit" => {
            payload_exact(payload, op, &["ref"]);
            let r = payload.get("ref").and_then(|v| v.as_str()).unwrap_or_else(|| fail("ref must be a string"));
            if r.trim().is_empty() { fail("ref must be a non-empty string"); }
            assert_safe_token(r, "ref");
            let (stdout, stderr, code) = run_git(repo, &["rev-parse", r]);
            let err_summary = if stderr.trim().is_empty() { Value::Null } else { json!(stderr.trim()) };
            if code != 0 {
                return (json!({ "gitStdout": stdout, "gitStderr": stderr, "errorSummary": err_summary }), code);
            }
            let hash = parse_commit_hash(&stdout, "commitHash");
            (json!({ "gitStdout": stdout, "gitStderr": stderr, "commitHash": hash, "errorSummary": err_summary }), code)
        }
        "merge" => {
            payload_exact(payload, op, &["branch_name", "no_ff"]);
            let branch = payload.get("branch_name").and_then(|v| v.as_str()).unwrap_or_else(|| fail("branch_name must be a string"));
            let no_ff = payload.get("no_ff").and_then(|v| v.as_bool()).unwrap_or_else(|| fail("no_ff must be a boolean"));
            if branch.is_empty() { fail("branch_name must be a non-empty string"); }
            assert_safe_token(branch, "branch_name");

            let mut args = vec!["merge"];
            if no_ff { args.push("--no-ff"); }
            args.push(branch);

            let (stdout, stderr, code) = run_git(repo, &args);
            let err_summary = if stderr.trim().is_empty() { Value::Null } else { json!(stderr.trim()) };
            if code != 0 {
                return (json!({ "gitStdout": stdout, "gitStderr": stderr, "errorSummary": err_summary }), code);
            }

            let (r_stdout, r_stderr, r_code) = run_git(repo, &["rev-parse", "HEAD"]);
            let full_stdout = format!("{}{}", stdout, r_stdout);
            let full_stderr = format!("{}{}", stderr, r_stderr);
            let r_err_summary = if r_stderr.trim().is_empty() { json!("rev-parse HEAD failed after merge") } else { json!(r_stderr.trim()) };
            if r_code != 0 {
                return (json!({ "gitStdout": full_stdout, "gitStderr": full_stderr, "errorSummary": r_err_summary }), r_code);
            }

            let hash = parse_commit_hash(&r_stdout, "mergeCommitHash");
            (json!({ "gitStdout": full_stdout, "gitStderr": full_stderr, "mergeCommitHash": hash, "errorSummary": err_summary }), code)
        }
        "delete_branch" => {
            payload_exact(payload, op, &["branch_name", "remote", "force"]);
            let branch = payload.get("branch_name").and_then(|v| v.as_str()).unwrap_or_else(|| fail("branch_name must be a string"));
            let remote = payload.get("remote").and_then(|v| v.as_bool()).unwrap_or_else(|| fail("remote must be a boolean"));
            let force = payload.get("force").and_then(|v| v.as_bool()).unwrap_or_else(|| fail("force must be a boolean"));
            if branch.is_empty() { fail("branch_name must be a non-empty string"); }
            assert_safe_token(branch, "branch_name");

            let (stdout, stderr, code) = if remote {
                run_git(repo, &["push", "origin", "--delete", branch])
            } else {
                run_git(repo, &["branch", if force { "-D" } else { "-d" }, branch])
            };
            let err_summary = if stderr.trim().is_empty() { Value::Null } else { json!(stderr.trim()) };
            (json!({ "gitStdout": stdout, "gitStderr": stderr, "errorSummary": err_summary }), code)
        }
        "checkout" => {
            payload_exact(payload, op, &["branch_name", "create_if_not_exists"]);
            let branch = payload.get("branch_name").and_then(|v| v.as_str()).unwrap_or_else(|| fail("branch_name must be a string"));
            let create = payload.get("create_if_not_exists").and_then(|v| v.as_bool()).unwrap_or_else(|| fail("create_if_not_exists must be a boolean"));
            if branch.is_empty() { fail("branch_name must be a non-empty string"); }
            assert_safe_token(branch, "branch_name");

            let (stdout, stderr, code) = if create {
                run_git(repo, &["checkout", "-b", branch])
            } else {
                run_git(repo, &["checkout", branch])
            };
            let err_summary = if stderr.trim().is_empty() { Value::Null } else { json!(stderr.trim()) };
            (json!({ "gitStdout": stdout, "gitStderr": stderr, "errorSummary": err_summary }), code)
        }
        "commit" => {
            payload_exact(payload, op, &["message", "files"]);
            let message = payload.get("message").and_then(|v| v.as_str()).unwrap_or_else(|| fail("message must be a string"));
            let files = payload.get("files").and_then(|v| v.as_array()).unwrap_or_else(|| fail("files must be an array"));
            if message.trim().is_empty() { fail("message must be a non-empty string"); }

            let mut file_strs = Vec::new();
            for f in files {
                file_strs.push(f.as_str().unwrap_or_else(|| fail("files must be an array of strings")));
            }

            for f in file_strs {
                let rel = rel_path_under_repo(repo, f);
                let full = repo.join(&rel);
                let (a_stdout, a_stderr, a_code) = if full.exists() {
                    // Altas/mods (incl. dirs): -A cubre el pathspec.
                    run_git(repo, &["add", "-A", "--", &rel])
                } else {
                    // Deletes (worktree ausente): plain `add` falla; `rm --ignore-unmatch` es idempotente.
                    run_git(repo, &["rm", "--ignore-unmatch", "--", &rel])
                };
                if a_code != 0 {
                    let err_summary = format!("git add failed: {}", a_stderr.trim());
                    return (json!({ "gitStdout": a_stdout, "gitStderr": a_stderr, "errorSummary": err_summary }), a_code);
                }
            }

            let (stdout, stderr, code) = run_git(repo, &["commit", "-m", message]);
            let err_summary = if stderr.trim().is_empty() { Value::Null } else { json!(stderr.trim()) };
            (json!({ "gitStdout": stdout, "gitStderr": stderr, "errorSummary": err_summary }), code)
        }
        "push" => {
            payload_exact(payload, op, &["remote", "branch", "force"]);
            let remote = payload.get("remote").and_then(|v| v.as_str()).unwrap_or_else(|| fail("remote must be a string"));
            let branch = payload.get("branch").and_then(|v| v.as_str()).unwrap_or_else(|| fail("branch must be a string"));
            let force = payload.get("force").and_then(|v| v.as_bool()).unwrap_or_else(|| fail("force must be a boolean"));
            if remote.is_empty() { fail("remote must be a non-empty string"); }
            if branch.is_empty() { fail("branch must be a non-empty string"); }
            assert_safe_token(remote, "remote");
            assert_safe_token(branch, "branch");

            let mut args = vec!["push"];
            if force { args.push("--force"); }
            args.push(remote);
            args.push(branch);

            let (stdout, stderr, code) = run_git(repo, &args);
            let err_summary = if stderr.trim().is_empty() { Value::Null } else { json!(stderr.trim()) };
            (json!({ "gitStdout": stdout, "gitStderr": stderr, "errorSummary": err_summary }), code)
        }
        "pull" => {
            payload_exact(payload, op, &["remote", "branch"]);
            let remote = payload.get("remote").and_then(|v| v.as_str()).unwrap_or_else(|| fail("remote must be a string"));
            let branch = payload.get("branch").and_then(|v| v.as_str()).unwrap_or_else(|| fail("branch must be a string"));
            if remote.is_empty() { fail("remote must be a non-empty string"); }
            if branch.is_empty() { fail("branch must be a non-empty string"); }
            assert_safe_token(remote, "remote");
            assert_safe_token(branch, "branch");

            let (stdout, stderr, code) = run_git(repo, &["pull", remote, branch]);
            let err_summary = if stderr.trim().is_empty() { Value::Null } else { json!(stderr.trim()) };
            (json!({ "gitStdout": stdout, "gitStderr": stderr, "errorSummary": err_summary }), code)
        }
        "fetch" => {
            payload_exact(payload, op, &["remote", "prune"]);
            let remote = payload.get("remote").and_then(|v| v.as_str()).unwrap_or_else(|| fail("remote must be a string"));
            let prune = payload.get("prune").and_then(|v| v.as_bool()).unwrap_or_else(|| fail("prune must be a boolean"));
            if remote.is_empty() { fail("remote must be a non-empty string"); }
            assert_safe_token(remote, "remote");

            let mut args = vec!["fetch", remote];
            if prune { args.push("--prune"); }

            let (stdout, stderr, code) = run_git(repo, &args);
            let err_summary = if stderr.trim().is_empty() { Value::Null } else { json!(stderr.trim()) };
            (json!({ "gitStdout": stdout, "gitStderr": stderr, "errorSummary": err_summary }), code)
        }
        "diff_name_only" => {
            payload_exact(payload, op, &["ref_spec"]);
            let ref_spec = payload.get("ref_spec").and_then(|v| v.as_str()).unwrap_or_else(|| fail("ref_spec must be a string"));
            if ref_spec.is_empty() { fail("ref_spec must be a non-empty string"); }
            assert_safe_token(ref_spec, "ref_spec");

            let (stdout, stderr, code) = run_git(repo, &["diff", "--name-only", ref_spec]);
            let files: Vec<String> = stdout
                .lines()
                .map(|ln| ln.trim())
                .filter(|ln| !ln.is_empty())
                .map(|ln| ln.to_string())
                .collect();
            let err_summary = if stderr.trim().is_empty() { Value::Null } else { json!(stderr.trim()) };
            (json!({ "gitStdout": stdout, "gitStderr": stderr, "files": files, "errorSummary": err_summary }), code)
        }
        _ => fail(&format!("unsupported operation_type: {}", op)),
    }
}

fn main() {
    let doc = read_stdin_json();

    let op = doc.get("operation_type").and_then(|v| v.as_str()).unwrap_or_else(|| fail("operation_type missing or not a string"));
    let repo_path = doc.get("repository_path").and_then(|v| v.as_str()).unwrap_or_else(|| fail("repository_path missing or not a string"));
    let payload = doc.get("operation_payload_json").unwrap_or_else(|| fail("operation_payload_json missing"));

    let allowed_ops = vec![
        "status", "checkout", "commit", "push", "pull", "fetch",
        "branch_list", "get_last_commit", "merge", "delete_branch", "diff_name_only"
    ];
    if !allowed_ops.contains(&op) {
        fail(&format!("operation_type must be one of {:?}", allowed_ops));
    }
    if !payload.is_object() {
        fail("operation_payload_json must be a JSON object");
    }

    let repo = resolve_repo(repo_path);
    verify_git_repo(&repo);

    let (data, code) = handle(op, &repo, payload);
    ok(data, code);
}
