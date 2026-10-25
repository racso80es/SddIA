//! Cápsula `tracker:lifecycle` para `provider: file`.
//! Lee el manifiesto y escribe el bucket de Cúmulo. El proceso no elige la ruta.

use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

pub fn dispatch(repo: &Path, req: &Value) -> Result<Value, String> {
    let inner = req.get("request").unwrap_or(req);
    let op = inner
        .get("operation")
        .and_then(|v| v.as_str())
        .ok_or("operation requerida")?;
    match op {
        "create_issue" => create_issue(repo, inner),
        "update_issue_description" => update_description(repo, inner),
        "fetch_issue" => fetch_issue(repo, inner),
        "list_issues" => list_issues(repo, inner),
        "update_issue_state" => update_state(repo, inner),
        "create_comment" => create_comment(repo, inner),
        "archive_issue" => archive_issue(repo, inner),
        other => Err(format!("operation no soportada: {other}")),
    }
}

struct Layout {
    root: PathBuf,
    pending: String,
    done: String,
    stories: String,
}

fn layout(repo: &Path, req: &Value) -> Result<Layout, String> {
    let (root, pending, done) = if let Some(slug) = req
        .get("project_slug")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        project_layout(repo, slug)?
    } else {
        let pending = cumulo_todo(repo, "pending").unwrap_or_else(|| "docs/todos/pending".into());
        let done = cumulo_todo(repo, "done").unwrap_or_else(|| "docs/todos/done".into());
        (repo.to_path_buf(), pending, done)
    };
    let stories = stories_rel(&pending);
    Ok(Layout {
        root,
        pending,
        done,
        stories,
    })
}

fn stories_rel(pending: &str) -> String {
    let parent = Path::new(pending)
        .parent()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "docs/todos".into());
    format!("{parent}/historias")
}

fn project_layout(repo: &Path, slug: &str) -> Result<(PathBuf, String, String), String> {
    let index = repo.join(".SddIA/projects").join(format!("{slug}.md"));
    let index_text = fs::read_to_string(&index).map_err(|e| e.to_string())?;
    let fm = frontmatter(&index_text)?;
    let project_root = fm
        .get("project_root")
        .and_then(|v| v.as_str())
        .unwrap_or(".")
        .trim()
        .to_string();
    let root = if project_root == "." || project_root.is_empty() {
        repo.to_path_buf()
    } else if Path::new(&project_root).is_absolute() {
        PathBuf::from(&project_root)
    } else {
        repo.join(&project_root)
    };
    let manifest_ref = fm
        .get("manifest_ref")
        .and_then(|v| v.as_str())
        .unwrap_or(".SddIA/project.md");
    let manifest = root.join(manifest_ref.trim_start_matches("./"));
    let (pending, done) = if manifest.is_file() {
        let text = fs::read_to_string(&manifest).unwrap_or_default();
        let m = frontmatter(&text).unwrap_or(Value::Null);
        let docs = m.get("docs_layout");
        let pending = docs
            .and_then(|d| d.get("todos_pending"))
            .and_then(|v| v.as_str())
            .unwrap_or("docs/todos/pending")
            .to_string();
        let done = docs
            .and_then(|d| d.get("todos_done"))
            .and_then(|v| v.as_str())
            .unwrap_or("docs/todos/done")
            .to_string();
        (pending, done)
    } else {
        ("docs/todos/pending".into(), "docs/todos/done".into())
    };
    Ok((root, pending, done))
}

fn cumulo_todo(repo: &Path, key: &str) -> Option<String> {
    let path = repo.join("SddIA/core/cumulo.paths.json");
    let text = fs::read_to_string(path).ok()?;
    let v: Value = serde_json::from_str(&text).ok()?;
    v.pointer(&format!("/paths/todos/{key}"))
        .and_then(|x| x.as_str())
        .map(str::to_string)
}

fn frontmatter(text: &str) -> Result<Value, String> {
    let rest = text.strip_prefix("---").ok_or("sin cabecera")?;
    let end = rest.find("\n---").ok_or("cabecera abierta")?;
    serde_yaml::from_str(&rest[..end]).map_err(|e| e.to_string())
}

fn find_by_ref(layout: &Layout, issue_ref: &str) -> Option<PathBuf> {
    if issue_ref.contains('/') {
        let p = layout.root.join(issue_ref.trim_start_matches("./"));
        return p.is_file().then_some(p);
    }
    for rel in [&layout.pending, &layout.done, &layout.stories] {
        let dir = layout.root.join(rel);
        let Ok(rd) = fs::read_dir(&dir) else {
            continue;
        };
        for ent in rd.flatten() {
            let path = ent.path();
            if path.extension().and_then(|e| e.to_str()) != Some("md") {
                continue;
            }
            let Ok(text) = fs::read_to_string(&path) else {
                continue;
            };
            if let Ok(fm) = frontmatter(&text) {
                for key in ["tracker_ref", "document_id"] {
                    if fm.get(key).and_then(|v| v.as_str()).map(str::trim) == Some(issue_ref) {
                        return Some(path);
                    }
                }
            }
        }
    }
    None
}

fn create_issue(repo: &Path, req: &Value) -> Result<Value, String> {
    let layout = layout(repo, req)?;
    let description = req
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let title = req.get("title").and_then(|v| v.as_str()).unwrap_or("item");
    let fm = frontmatter(description).unwrap_or(Value::Null);
    let issue_ref = fm
        .get("document_id")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or(title)
        .to_string();
    let file_name = if title.ends_with(".md") {
        title.to_string()
    } else {
        format!("{issue_ref}.md")
    };
    let dir = layout.root.join(&layout.pending);
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let path = dir.join(&file_name);
    fs::write(&path, description).map_err(|e| e.to_string())?;
    let rel = path
        .strip_prefix(&layout.root)
        .unwrap_or(&path)
        .to_string_lossy()
        .replace('\\', "/");
    Ok(json!({
        "issue_ref": issue_ref,
        "artifact_path": rel,
        "title": title,
    }))
}

fn update_description(repo: &Path, req: &Value) -> Result<Value, String> {
    let layout = layout(repo, req)?;
    let issue_ref = req
        .get("issue_ref")
        .or_else(|| req.get("markdown_path"))
        .and_then(|v| v.as_str())
        .ok_or("issue_ref requerido")?;
    let description = req
        .get("description")
        .and_then(|v| v.as_str())
        .ok_or("description requerida")?;
    let path = find_by_ref(&layout, issue_ref).unwrap_or_else(|| layout.root.join(issue_ref.trim_start_matches("./")));
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    fs::write(&path, description).map_err(|e| e.to_string())?;
    Ok(json!({
        "issue_ref": issue_ref,
        "description_bytes": description.len(),
    }))
}

fn fetch_issue(repo: &Path, req: &Value) -> Result<Value, String> {
    let layout = layout(repo, req)?;
    let issue_ref = req
        .get("issue_ref")
        .and_then(|v| v.as_str())
        .ok_or("issue_ref requerido")?;
    let path = find_by_ref(&layout, issue_ref).ok_or_else(|| format!("no está en el bucket: {issue_ref}"))?;
    let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let fm = frontmatter(&text).unwrap_or(Value::Null);
    let artifact_path = path
        .strip_prefix(&layout.root)
        .unwrap_or(&path)
        .to_string_lossy()
        .replace('\\', "/");
    Ok(json!({
        "issue_ref": issue_ref,
        "description": text,
        "artifact_path": artifact_path,
        "title": fm.get("title").and_then(|v| v.as_str()).unwrap_or(issue_ref),
        "state": fm.get("status").and_then(|v| v.as_str()).unwrap_or("pending"),
    }))
}

fn list_issues(repo: &Path, req: &Value) -> Result<Value, String> {
    let layout = layout(repo, req)?;
    let kind = req
        .get("kind")
        .and_then(|v| v.as_str())
        .unwrap_or("all");
    let mut items = Vec::new();
    let buckets: Vec<&String> = if kind == "hu" {
        vec![&layout.stories, &layout.pending, &layout.done]
    } else if kind == "all" {
        vec![&layout.pending, &layout.done, &layout.stories]
    } else {
        vec![&layout.pending, &layout.done]
    };
    for rel in buckets {
        let dir = layout.root.join(rel);
        let Ok(rd) = fs::read_dir(&dir) else {
            continue;
        };
        for ent in rd.flatten() {
            let path = ent.path();
            if path.extension().and_then(|e| e.to_str()) != Some("md") {
                continue;
            }
            let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
            let item_kind = if name.contains("HU-") { "hu" } else { "pbi" };
            if kind != "all" && kind != item_kind {
                continue;
            }
            let rel_path = path
                .strip_prefix(&layout.root)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");
            items.push(json!({
                "kind": item_kind,
                "title": name,
                "path": rel_path,
                "issue_ref": name.trim_end_matches(".md"),
            }));
        }
    }
    Ok(json!({ "items": items }))
}

fn update_state(repo: &Path, req: &Value) -> Result<Value, String> {
    let layout = layout(repo, req)?;
    let issue_ref = req
        .get("issue_ref")
        .and_then(|v| v.as_str())
        .ok_or("issue_ref requerido")?;
    let state = req
        .get("state_name")
        .and_then(|v| v.as_str())
        .unwrap_or("pending");
    let path = find_by_ref(&layout, issue_ref).ok_or_else(|| format!("no está en el bucket: {issue_ref}"))?;
    let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let updated = if text.starts_with("---") {
        if let Some(end) = text[3..].find("\n---") {
            let head = &text[3..3 + end];
            let rest = &text[3 + end + 4..];
            if head.contains("status:") {
                let mut lines = String::new();
                for line in head.lines() {
                    if line.trim_start().starts_with("status:") {
                        lines.push_str(&format!("status: {state}\n"));
                    } else {
                        lines.push_str(line);
                        lines.push('\n');
                    }
                }
                format!("---\n{lines}---{rest}")
            } else {
                format!("---\n{head}\nstatus: {state}\n---{rest}")
            }
        } else {
            text
        }
    } else {
        text
    };
    let dest = if state.eq_ignore_ascii_case("done") || state.eq_ignore_ascii_case("cancelled") || state == "cerrado" {
        let dir = layout.root.join(&layout.done);
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        let name = path.file_name().unwrap_or_default();
        let target = dir.join(name);
        let _ = fs::rename(&path, &target);
        target
    } else {
        path.clone()
    };
    fs::write(&dest, updated).map_err(|e| e.to_string())?;
    Ok(json!({ "issue_ref": issue_ref, "state": state }))
}

fn create_comment(repo: &Path, req: &Value) -> Result<Value, String> {
    let layout = layout(repo, req)?;
    let issue_ref = req
        .get("issue_ref")
        .and_then(|v| v.as_str())
        .ok_or("issue_ref requerido")?;
    let body = req.get("body").and_then(|v| v.as_str()).unwrap_or("");
    let path = find_by_ref(&layout, issue_ref).ok_or_else(|| format!("no está en el bucket: {issue_ref}"))?;
    let mut text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    text.push_str("\n\n## Comentario\n\n");
    text.push_str(body);
    text.push('\n');
    fs::write(&path, text).map_err(|e| e.to_string())?;
    Ok(json!({ "issue_ref": issue_ref }))
}

fn archive_issue(repo: &Path, req: &Value) -> Result<Value, String> {
    let mut owned = req.clone();
    if let Some(obj) = owned.as_object_mut() {
        obj.insert("state_name".into(), json!("cancelled"));
    }
    update_state(repo, &owned)
}
