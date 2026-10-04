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

pub fn lab_reset_store() {
    let mut s = store().lock().unwrap();
    s.issues.clear();
    s.comments.clear();
    s.audit_log.clear();
    s.next_id = 100;
    persist_store(&s);
    let _ = fs::remove_file(store_path());
}

pub fn lab_audit_append(fragment: &str) {
    let mut s = store().lock().unwrap();
    s.audit_log.push(fragment.to_string());
    persist_store(&s);
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

pub fn lab_create_issue(
    team_key: &str,
    title: &str,
    labels: &[String],
    parent_ref: Option<&str>,
    state_name: &str,
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
    });
    s.issues.insert(ident.clone(), issue);
    if let Some(parent) = parent_ref {
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

pub fn lab_fetch(issue_ref: &str) -> Option<Value> {
    let s = store().lock().unwrap();
    s.issues.get(issue_ref).cloned()
}
