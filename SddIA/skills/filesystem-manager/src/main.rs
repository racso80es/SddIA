use sddia_io::{emit_error, emit_success, read_stdin_json};
use serde_json::{json, Value};
use std::fs;
use std::path::{Component, Path, PathBuf};

fn fail(msg: &str) -> ! {
    emit_error(msg, 1);
    std::process::exit(1);
}

fn ok_data(data: Value) {
    emit_success(Some(json!({
        "success": true,
        "exitCode": 0,
        "data": data,
    })));
}

fn err(msg: &str) -> ! {
    fail(msg);
}

fn find_repo_root() -> PathBuf {
    if let Ok(mut cur) = std::env::current_dir() {
        loop {
            if cur.join("SddIA/core/cumulo.paths.json").is_file() {
                return cur;
            }
            if !cur.pop() {
                break;
            }
        }
    }
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn resolve_workspace_root(doc: &Value) -> PathBuf {
    if let Some(s) = doc
        .get("workspace_root")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        return PathBuf::from(s);
    }
    if let Ok(s) = std::env::var("SDDIA_WORKSPACE_ROOT") {
        let t = s.trim();
        if !t.is_empty() {
            return PathBuf::from(t);
        }
    }
    find_repo_root()
}

fn resolve_under_root(root: &Path, rel: &str) -> Result<PathBuf, String> {
    let rel = rel.trim().trim_start_matches("./");
    if rel.is_empty() {
        return Err("PROJECT_SCOPE_ESCAPE: empty path".into());
    }
    if Path::new(rel).is_absolute() {
        return Err(format!("PROJECT_SCOPE_ESCAPE: absolute '{rel}'"));
    }
    let mut joined = root.to_path_buf();
    for comp in Path::new(rel).components() {
        match comp {
            Component::ParentDir => return Err(format!("PROJECT_SCOPE_ESCAPE: '{rel}'")),
            Component::RootDir | Component::Prefix(_) => {
                return Err(format!("PROJECT_SCOPE_ESCAPE: '{rel}'"));
            }
            Component::CurDir => {}
            Component::Normal(p) => joined.push(p),
        }
    }
    let root_canon = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    if joined.exists() {
        let canon = joined
            .canonicalize()
            .map_err(|e| format!("PROJECT_SCOPE_ESCAPE: {e}"))?;
        if !canon.starts_with(&root_canon) {
            return Err(format!("PROJECT_SCOPE_ESCAPE: '{rel}'"));
        }
        return Ok(canon);
    }
    if !joined.starts_with(root) {
        return Err(format!("PROJECT_SCOPE_ESCAPE: '{rel}'"));
    }
    Ok(joined)
}

fn read_file(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("File not found or unreadable: {e}"))
}

fn write_file(path: &Path, content: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    fs::write(path, content).map_err(|e| e.to_string())
}

fn list_dir(path: &Path) -> Result<Vec<String>, String> {
    let mut names = Vec::new();
    for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry
            .file_name()
            .to_string_lossy()
            .to_string();
        names.push(name);
    }
    names.sort();
    Ok(names)
}

fn apply_hunks_strict(original: &str, patch: &str) -> Result<String, String> {
    let orig_lines: Vec<&str> = original.lines().collect();
    let mut result: Vec<String> = Vec::new();
    let mut orig_idx = 0usize;
    let mut in_hunk = false;
    let mut hunk_old: Vec<String> = Vec::new();
    let mut hunk_new: Vec<String> = Vec::new();

    for line in patch.lines() {
        if line.starts_with("@@") {
            if in_hunk {
                if !flush_hunk(&orig_lines, &mut orig_idx, &hunk_old, &hunk_new, &mut result)? {
                    return Err("PATCH_FILE: hunk mismatch".into());
                }
                hunk_old.clear();
                hunk_new.clear();
            }
            in_hunk = true;
            continue;
        }
        if !in_hunk {
            continue;
        }
        if line.starts_with("---") || line.starts_with("+++") || line.starts_with("diff ") {
            continue;
        }
        if let Some(rest) = line.strip_prefix(' ') {
            let t = rest.to_string();
            hunk_old.push(t.clone());
            hunk_new.push(t);
        } else if let Some(rest) = line.strip_prefix('-') {
            hunk_old.push(rest.to_string());
        } else if let Some(rest) = line.strip_prefix('+') {
            hunk_new.push(rest.to_string());
        }
    }
    if in_hunk && !flush_hunk(&orig_lines, &mut orig_idx, &hunk_old, &hunk_new, &mut result)? {
        return Err("PATCH_FILE: hunk mismatch".into());
    }
    while orig_idx < orig_lines.len() {
        result.push(orig_lines[orig_idx].to_string());
        orig_idx += 1;
    }
    let mut out = result.join("\n");
    if original.ends_with('\n') {
        out.push('\n');
    }
    Ok(out)
}

fn flush_hunk(
    orig: &[&str],
    orig_idx: &mut usize,
    hunk_old: &[String],
    hunk_new: &[String],
    result: &mut Vec<String>,
) -> Result<bool, String> {
    if hunk_old.is_empty() && hunk_new.is_empty() {
        return Ok(true);
    }
    let mut oi = 0usize;
    while oi < hunk_old.len() {
        if *orig_idx >= orig.len() {
            return Ok(false);
        }
        if orig[*orig_idx] != hunk_old[oi] {
            return Ok(false);
        }
        *orig_idx += 1;
        oi += 1;
    }
    for line in hunk_new {
        result.push(line.clone());
    }
    Ok(true)
}

fn extract_inputs(doc: &Value) -> Value {
    if let Some(inner) = doc.get("inputs").filter(|v| v.is_object()) {
        return inner.clone();
    }
    doc.clone()
}

fn main() {
    let doc = read_stdin_json();
    let inputs = extract_inputs(&doc);
    let op = inputs
        .get("operation")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| fail("operation missing"));
    let root = resolve_workspace_root(&inputs);
    let target = inputs
        .get("target_path")
        .and_then(|v| v.as_str())
        .unwrap_or_else(|| fail("target_path missing"));

    match op {
        "READ_FILE" => {
            let path = match resolve_under_root(&root, target) {
                Ok(p) => p,
                Err(e) => err(&e),
            };
            let content = match read_file(&path) {
                Ok(c) => c,
                Err(e) => err(&e),
            };
            ok_data(json!(content));
        }
        "WRITE_FILE" => {
            let content = inputs
                .get("content")
                .and_then(|v| v.as_str())
                .unwrap_or_else(|| fail("content required for WRITE_FILE"));
            let path = match resolve_under_root(&root, target) {
                Ok(p) => p,
                Err(e) => err(&e),
            };
            if let Err(e) = write_file(&path, content) {
                err(&e);
            }
            ok_data(json!("written"));
        }
        "LIST_DIR" => {
            let path = match resolve_under_root(&root, target) {
                Ok(p) => p,
                Err(e) => err(&e),
            };
            let list = match list_dir(&path) {
                Ok(l) => l,
                Err(e) => err(&e),
            };
            ok_data(json!(list));
        }
        "DELETE_FILE" => {
            let path = match resolve_under_root(&root, target) {
                Ok(p) => p,
                Err(e) => err(&e),
            };
            if path.is_dir() {
                err("is a directory");
            }
            if fs::remove_file(&path).is_err() {
                err("delete failed");
            }
            ok_data(json!("deleted"));
        }
        "CREATE_DIR" => {
            let path = match resolve_under_root(&root, target) {
                Ok(p) => p,
                Err(e) => err(&e),
            };
            if fs::create_dir_all(&path).is_err() {
                err("mkdir failed");
            }
            ok_data(json!("created"));
        }
        "MOVE_FILE" => {
            let dest = inputs
                .get("destination_path")
                .and_then(|v| v.as_str())
                .unwrap_or_else(|| fail("destination_path required"));
            let from = match resolve_under_root(&root, target) {
                Ok(p) => p,
                Err(e) => err(&e),
            };
            let to = match resolve_under_root(&root, dest) {
                Ok(p) => p,
                Err(e) => err(&e),
            };
            if let Some(parent) = to.parent() {
                let _ = fs::create_dir_all(parent);
            }
            if fs::rename(&from, &to).is_err() {
                err("move failed");
            }
            ok_data(json!("moved"));
        }
        "PATCH_FILE" => {
            let patch = inputs
                .get("content")
                .and_then(|v| v.as_str())
                .unwrap_or_else(|| fail("content required for PATCH_FILE (unified diff)"));
            let path = match resolve_under_root(&root, target) {
                Ok(p) => p,
                Err(e) => err(&e),
            };
            let original = if path.is_file() {
                fs::read_to_string(&path).unwrap_or_default()
            } else {
                String::new()
            };
            let patched = match apply_hunks_strict(&original, patch) {
                Ok(s) => s,
                Err(_) => err("PATCH_FILE: hunk mismatch"),
            };
            if let Err(e) = write_file(&path, &patched) {
                err(&e);
            }
            ok_data(json!("patched"));
        }
        other => fail(&format!("unknown operation '{other}'")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn rejects_parent_dir() {
        let td = TempDir::new().unwrap();
        let root = td.path();
        assert!(resolve_under_root(root, "../etc/passwd").is_err());
    }

    #[test]
    fn patch_applies_and_rejects_mismatch() {
        let orig = "hello\n";
        let good = "@@ -1,1 +1,1 @@\n-hello\n+world\n";
        let patched = apply_hunks_strict(orig, good).unwrap();
        assert_eq!(patched, "world\n");
        let mismatch = "@@ -1,1 +1,1 @@\n-wrong\n+world\n";
        assert!(apply_hunks_strict(orig, mismatch).is_err());
    }
}
