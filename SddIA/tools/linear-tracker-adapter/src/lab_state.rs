//! Estado mutable del mock inline (persistido en disco entre invocaciones del binario).

use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

struct Store {
    issues: HashMap<String, Value>,
    comments: HashMap<String, Vec<String>>,
    audit_log: Vec<String>,
    next_id: u32,
}

fn store_path() -> PathBuf {
    // cwd = raíz del repo al invocar la cápsula (capsules.rs). Rutas absolutas vía
    // SDDIA_REPO_ROOT no son escribibles en WASI con preopen `--dir=.`.
    PathBuf::from(".SddIA/lab-linear-store.json")
}

fn load_store() -> Store {
    let path = store_path();
    if path.is_file() {
        if let Ok(text) = fs::read_to_string(&path) {
            if let Ok(v) = serde_json::from_str::<Value>(&text) {
                let issues = v
                    .get("issues")
                    .and_then(|x| x.as_object())
                    .map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                    .unwrap_or_default();
                let comments = v
                    .get("comments")
                    .and_then(|x| x.as_object())
                    .map(|m| {
                        m.iter()
                            .map(|(k, v)| {
                                let arr = v
                                    .as_array()
                                    .map(|a| {
                                        a.iter()
                                            .filter_map(|x| x.as_str().map(str::to_string))
                                            .collect()
                                    })
                                    .unwrap_or_default();
                                (k.clone(), arr)
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                let audit_log = v
                    .get("audit_log")
                    .and_then(|x| x.as_array())
                    .map(|a| {
                        a.iter()
                            .filter_map(|x| x.as_str().map(str::to_string))
                            .collect()
                    })
                    .unwrap_or_default();
                let next_id = v.get("next_id").and_then(|x| x.as_u64()).unwrap_or(100) as u32;
                return Store {
                    issues,
                    comments,
                    audit_log,
                    next_id,
                };
            }
        }
    }
    Store {
        issues: HashMap::new(),
        comments: HashMap::new(),
        audit_log: Vec::new(),
        next_id: 100,
    }
}

fn persist_store(s: &Store) {
    let path = store_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let body = json!({
        "issues": s.issues,
        "comments": s.comments,
        "audit_log": s.audit_log,
        "next_id": s.next_id,
    });
    let _ = fs::write(path, serde_json::to_string_pretty(&body).unwrap_or_default());
}

fn store() -> &'static Mutex<Store> {
    static S: OnceLock<Mutex<Store>> = OnceLock::new();
    S.get_or_init(|| Mutex::new(load_store()))
}

/// Sincroniza el mutex con el JSON en disco (p. ej. seeds del test vía `fs::write`).
pub fn reload_store_from_disk() {
    if !store_path().is_file() {
        return;
    }
    let fresh = load_store();
    let mut s = store().lock().unwrap();
    *s = fresh;
}

fn child_refs_by_parent(parent_ref: &str, issues: &HashMap<String, Value>) -> Vec<String> {
    let mut out: Vec<String> = issues
        .iter()
        .filter(|(_, issue)| {
            issue
                .get("parent")
                .and_then(|v| v.as_str())
                .is_some_and(|p| p == parent_ref)
        })
        .map(|(k, _)| k.clone())
        .collect();
    out.sort();
    out
}

fn merge_children_field(issue: &mut Value, derived: &[String]) {
    if derived.is_empty() {
        return;
    }
    let existing: Vec<String> = issue
        .get("children")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|node| {
                    node.as_str()
                        .map(str::to_string)
                        .or_else(|| {
                            node.get("identifier")
                                .and_then(|v| v.as_str())
                                .map(str::to_string)
                        })
                })
                .collect()
        })
        .unwrap_or_default();
    if existing.is_empty() {
        issue["children"] = json!(derived);
    }
}

pub fn lab_reset_store() {
    let mut s = store().lock().unwrap();
    s.issues.clear();
    s.comments.clear();
    s.audit_log.clear();
    s.next_id = 100;
    persist_store(&s);
    let _ = fs::remove_file(store_path());
}

pub fn lab_register_comment(issue_ref: &str, body: &str) {
    let mut s = store().lock().unwrap();
    s.comments
        .entry(issue_ref.to_string())
        .or_default()
        .push(body.to_string());
    s.audit_log.push(format!("comment:{issue_ref}:{body}"));
    persist_store(&s);
}

pub fn lab_set_description(issue_ref: &str, description: &str) {
    let mut s = store().lock().unwrap();
    if let Some(issue) = s.issues.get_mut(issue_ref) {
        issue["description"] = json!(description);
    }
    persist_store(&s);
}

pub fn lab_create_issue(
    team_key: &str,
    title: &str,
    labels: &[String],
    parent_ref: Option<&str>,
    state_name: &str,
    project_id: Option<&str>,
    description: &str,
) -> String {
    let mut s = store().lock().unwrap();
    s.next_id += 1;
    let ident = format!("{team_key}-{}", s.next_id);
    let issue = json!({
        "id": format!("issue-uuid-{ident}"),
        "identifier": ident,
        "title": title,
        "state": state_name,
        "priority": 0,
        "labels": labels,
        "parent": parent_ref,
        "children": [],
        "url": format!("https://linear.app/issue/{ident}"),
        "updated_at": "2026-10-04T00:00:00Z",
        "project_id": project_id,
        "description": description,
    });
    s.issues.insert(ident.clone(), issue);
    if let Some(parent) = parent_ref {
        if !s.issues.contains_key(parent) {
            s.issues.insert(
                parent.to_string(),
                json!({
                    "id": format!("issue-uuid-{parent}"),
                    "identifier": parent,
                    "title": format!("lab parent {parent}"),
                    "state": state_name,
                    "priority": 0,
                    "labels": ["hu"],
                    "parent": null,
                    "children": [],
                    "url": format!("https://linear.app/issue/{parent}"),
                    "updated_at": "2026-10-04T00:00:00Z",
                    "project_id": project_id,
                    "description": "",
                }),
            );
        }
        if let Some(p) = s.issues.get_mut(parent) {
            if let Some(arr) = p.get("children").and_then(|c| c.as_array()) {
                let mut kids = arr.clone();
                kids.push(json!(ident));
                p["children"] = json!(kids);
            } else {
                p["children"] = json!([ident]);
            }
        }
    }
    s.audit_log.push(format!("create:{ident}:{title}"));
    persist_store(&s);
    ident
}

pub fn lab_update_state(issue_ref: &str, state_name: &str) -> bool {
    let mut s = store().lock().unwrap();
    if let Some(issue) = s.issues.get_mut(issue_ref) {
        issue["state"] = json!(state_name);
        s.audit_log
            .push(format!("state:{issue_ref}:{state_name}"));
        persist_store(&s);
        return true;
    }
    false
}

pub fn lab_set_project(issue_ref: &str, project_id: &str) {
    let mut s = store().lock().unwrap();
    if let Some(issue) = s.issues.get_mut(issue_ref) {
        issue["project_id"] = json!(project_id);
        persist_store(&s);
    }
}

pub fn lab_archive(issue_ref: &str) -> bool {
    let mut s = store().lock().unwrap();
    if s.issues.remove(issue_ref).is_some() {
        s.audit_log.push(format!("archive:{issue_ref}"));
        persist_store(&s);
        return true;
    }
    false
}

pub fn lab_fetch(issue_ref: &str) -> Option<Value> {
    let s = store().lock().unwrap();
    let mut issue = s.issues.get(issue_ref).cloned()?;
    let derived = child_refs_by_parent(issue_ref, &s.issues);
    merge_children_field(&mut issue, &derived);
    Some(issue)
}
