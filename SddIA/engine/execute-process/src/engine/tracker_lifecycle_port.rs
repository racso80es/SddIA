//! Puerto de `tracker:lifecycle`.
//! La rama de sustrato y los buckets de gestión viven aquí.
//! Los procesos piden una operación; este módulo lee Cúmulo y el manifiesto.

use crate::core::paths::load_paths_config;
use super::project_binding::{self, DoneGate, TrackerConfig};
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

/// Sustrato resuelto. La lectura del manifiesto vive aquí.
pub enum Substrate {
    Local,
    Remote(TrackerConfig),
}

pub fn substrate(repo: &Path, inputs: &Value) -> Result<Substrate, String> {
    match project_binding::tracker_config_for_inputs(repo, inputs)? {
        Some(tc) => Ok(Substrate::Remote(tc)),
        None => Ok(Substrate::Local),
    }
}

/// La réplica local se cierra cuando el manifiesto pide el sello en el tracker.
pub fn local_replica_closes(gate: DoneGate) -> bool {
    matches!(gate, DoneGate::Linear)
}

pub struct TodoLayout {
    pub pending: String,
    pub done: String,
    pub stories: String,
}

pub fn todo_layout(repo: &Path) -> Result<TodoLayout, String> {
    let cfg = load_paths_config(repo).unwrap_or(serde_json::json!({}));
    let pending = cfg
        .pointer("/paths/todos/pending")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .unwrap_or("docs/todos/pending")
        .trim_start_matches("./")
        .to_string();
    let done = cfg
        .pointer("/paths/todos/done")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .unwrap_or("docs/todos/done")
        .trim_start_matches("./")
        .to_string();
    let parent = Path::new(&pending)
        .parent()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "docs/todos".into());
    Ok(TodoLayout {
        pending,
        done,
        stories: format!("{parent}/historias"),
    })
}

/// `true` cuando el manifiesto del `project_slug` declara el proveedor remoto.
pub fn is_remote(repo: &Path, inputs: &Value) -> Result<bool, String> {
    Ok(super::project_binding::tracker_config_for_inputs(repo, inputs)?.is_some())
}

/// Proveedor de fichero declarado en el índice de proyecto del payload.
pub fn manifest_provider_is_local(repo: &Path, payload: &Value) -> Result<bool, String> {
    let Some(slug) = payload
        .get("project_slug")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    else {
        return Ok(false);
    };
    let cfg = load_paths_config(repo)?;
    let projects = cfg
        .pointer("/instance/projects")
        .and_then(|v| v.as_str())
        .unwrap_or(".SddIA/projects");
    let index = repo
        .join(projects.trim().trim_start_matches("./"))
        .join(format!("{slug}.md"));
    if !index.is_file() {
        return Ok(false);
    }
    let index_fm = crate::core::parser::parse_frontmatter(&index).map_err(|e| e.to_string())?;
    let manifest_ref = index_fm
        .get("manifest_ref")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim();
    let project_root = index_fm
        .get("project_root")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim();
    if manifest_ref.is_empty() {
        return Ok(false);
    }
    let manifest = if project_root == "." || project_root == "./" || project_root.is_empty() {
        repo.join(manifest_ref.trim_start_matches("./"))
    } else {
        Path::new(project_root).join(manifest_ref.trim_start_matches("./"))
    };
    if !manifest.is_file() {
        return Ok(false);
    }
    let fm = crate::core::parser::parse_frontmatter(&manifest).map_err(|e| e.to_string())?;
    Ok(fm
        .get("tracker")
        .and_then(|t| t.get("provider"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        == Some("file"))
}

pub fn pending_dir(repo: &Path) -> Result<PathBuf, String> {
    Ok(repo.join(&todo_layout(repo)?.pending))
}

pub fn done_dir(repo: &Path) -> Result<PathBuf, String> {
    Ok(repo.join(&todo_layout(repo)?.done))
}

pub fn stories_dir(repo: &Path) -> Result<PathBuf, String> {
    Ok(repo.join(&todo_layout(repo)?.stories))
}

pub fn management_bucket_dirs(repo: &Path) -> Result<Vec<PathBuf>, String> {
    let layout = todo_layout(repo)?;
    Ok(vec![repo.join(&layout.pending), repo.join(&layout.done)])
}

pub fn ref_is_story(rel: &str) -> bool {
    rel.split(['/', '\\']).any(|s| s == "historias")
}

pub fn management_markdowns(repo: &Path) -> Result<Vec<PathBuf>, String> {
    let layout = todo_layout(repo)?;
    let mut out = Vec::new();
    for rel in [&layout.pending, &layout.done, &layout.stories] {
        let dir = repo.join(rel);
        let Ok(rd) = fs::read_dir(&dir) else {
            continue;
        };
        for ent in rd.filter_map(|e| e.ok()) {
            let p = ent.path();
            if p.extension().and_then(|x| x.to_str()) == Some("md") {
                out.push(p);
            }
        }
    }
    out.sort();
    Ok(out)
}

pub fn backlog_items(repo: &Path, kind: &str) -> Result<Vec<Value>, String> {
    let layout = todo_layout(repo)?;
    let mut items = Vec::new();
    for bucket in [&layout.pending, &layout.done] {
        let dir = repo.join(bucket);
        let Ok(rd) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in rd.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("md") {
                continue;
            }
            let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
            let item_kind = if name.contains("HU-") { "hu" } else { "pbi" };
            if kind != "all" && kind != item_kind {
                continue;
            }
            items.push(json!({
                "kind": item_kind,
                "title": name,
                "path": path.strip_prefix(repo).unwrap_or(&path).to_string_lossy().replace('\\', "/"),
            }));
        }
    }
    Ok(items)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn layout_follows_cumulo_and_backlog_ignores_other_dirs() {
        let td = tempfile::tempdir().unwrap();
        let repo = td.path();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"paths":{"todos":{"pending":"cola/abiertos","done":"cola/cerrados"}}}"#,
        )
        .unwrap();
        let layout = todo_layout(repo).unwrap();
        assert_eq!(layout.pending, "cola/abiertos");
        assert_eq!(layout.done, "cola/cerrados");
        assert_eq!(layout.stories, "cola/historias");
        fs::create_dir_all(repo.join("cola/abiertos")).unwrap();
        fs::write(repo.join("cola/abiertos/HU-uno.md"), "# u\n").unwrap();
        fs::create_dir_all(repo.join("docs/todos/pending")).unwrap();
        fs::write(repo.join("docs/todos/pending/HU-ajeno.md"), "# x\n").unwrap();
        let items = backlog_items(repo, "all").unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0]["path"], "cola/abiertos/HU-uno.md");
    }

    fn project_pair(repo: &Path, slug: &str, tracker_yaml: &str) {
        fs::create_dir_all(repo.join(".SddIA/projects")).unwrap();
        fs::create_dir_all(repo.join(".SddIA")).unwrap();
        fs::write(
            repo.join(".SddIA/projects").join(format!("{slug}.md")),
            format!(
                "---\nid: {slug}\nuuid: \"aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee\"\nproject_root: \".\"\nmanifest_ref: .SddIA/{slug}.md\ncodex_slug: codex-software-engineering\nstatus: active\n---\n"
            ),
        )
        .unwrap();
        fs::write(
            repo.join(".SddIA").join(format!("{slug}.md")),
            format!(
                "---\nid: {slug}\nuuid: \"aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee\"\ngit_remote: https://example.invalid/lab.git\ndefault_branch: main\ndelivery_mode: branch_pr\ncontract_version: \"1.4.0\"\ncodex_slug: codex-software-engineering\n{tracker_yaml}docs_layout:\n  features: docs/features\n  fixes: docs/fixes\n  todos_pending: cola/abiertos\n  todos_done: cola/cerrados\n---\n"
            ),
        )
        .unwrap();
    }

    #[test]
    fn substrate_file_reads_buckets_remote_does_not() {
        let td = tempfile::tempdir().unwrap();
        let repo = td.path();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"paths":{"todos":{"pending":"cola/abiertos","done":"cola/cerrados"}}}"#,
        )
        .unwrap();
        fs::create_dir_all(repo.join("cola/abiertos")).unwrap();
        fs::write(repo.join("cola/abiertos/PBI-local.md"), "# l\n").unwrap();
        fs::create_dir_all(repo.join("docs/todos/pending")).unwrap();
        fs::write(repo.join("docs/todos/pending/PBI-ajeno.md"), "# a\n").unwrap();

        project_pair(repo, "lab-file", "tracker:\n  provider: file\n");
        let local = substrate(repo, &json!({"project_slug": "lab-file"})).unwrap();
        assert!(matches!(local, Substrate::Local));
        assert!(!is_remote(repo, &json!({"project_slug": "lab-file"})).unwrap());
        let items = backlog_items(repo, "pbi").unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0]["path"], "cola/abiertos/PBI-local.md");

        project_pair(
            repo,
            "lab-remote",
            "tracker:\n  provider: linear\n  team_key: MAN\n  state_map:\n    done: Done\n",
        );
        let remote = substrate(repo, &json!({"project_slug": "lab-remote"})).unwrap();
        assert!(matches!(remote, Substrate::Remote(_)));
        assert!(is_remote(repo, &json!({"project_slug": "lab-remote"})).unwrap());
        assert!(local_replica_closes(DoneGate::Linear));
        assert!(!local_replica_closes(DoneGate::Git));
    }
}
