//! Paridad Suscripciones (domain `.md`) vs `event-domain-subscriptions.json`.

use serde_json::Value;
use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

fn load_subscriptions(repo: &Path) -> Result<HashMap<String, Vec<Value>>, String> {
    let path = repo.join("SddIA/core/event-domain-subscriptions.json");
    let text = fs::read_to_string(&path).map_err(|e| format!("read subscriptions: {e}"))?;
    let v: Value = serde_json::from_str(&text).map_err(|e| format!("parse subscriptions: {e}"))?;
    let obj = v
        .as_object()
        .ok_or_else(|| "subscriptions root must be object".to_string())?;
    let mut out = HashMap::new();
    for (k, subs) in obj {
        let arr = subs
            .as_array()
            .ok_or_else(|| format!("{k}: subscriptions must be array"))?;
        out.insert(k.clone(), arr.clone());
    }
    Ok(out)
}

fn parse_frontmatter_event_type(text: &str) -> Option<String> {
    if !text.starts_with("---") {
        return None;
    }
    let end = text[3..].find("\n---")?;
    let yaml = &text[3..3 + end];
    for line in yaml.lines() {
        let line = line.trim();
        if line.starts_with("event_type:") {
            let rest = line.strip_prefix("event_type:")?.trim();
            let v = rest.trim_matches('"').trim_matches('\'');
            return Some(v.to_string());
        }
    }
    None
}

fn json_handler_key(entry: &Value) -> Option<(String, String)> {
    let agent = entry
        .get("agent")
        .and_then(|v| v.as_str())
        .map(|s| s.to_lowercase())?;
    let handler = entry
        .get("process")
        .or_else(|| entry.get("tool"))
        .or_else(|| entry.get("action"))
        .and_then(|v| v.as_str())
        .map(str::to_string)?;
    Some((agent, handler))
}

fn parse_md_subscription_table(body: &str) -> BTreeSet<(String, String)> {
    let mut set = BTreeSet::new();
    let Some(idx) = body.find("## Suscripciones") else {
        return set;
    };
    let section = &body[idx..];
    for line in section.lines().skip(1) {
        let line = line.trim();
        if !line.starts_with('|') || line.contains(":---") {
            continue;
        }
        let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
        if parts.len() < 4 {
            continue;
        }
        let raw_sub = parts[1].trim();
        if !raw_sub.contains('`') {
            continue;
        }
        let sub = raw_sub.trim_matches('`');
        let agent = parts[2].trim().to_lowercase();
        if sub.is_empty() || agent.is_empty() || sub == "Suscriptor" || agent.contains('-') {
            continue;
        }
        set.insert((agent, sub.to_string()));
    }
    set
}

fn domain_event_files(repo: &Path) -> Result<HashMap<String, PathBuf>, String> {
    let dir = repo.join("SddIA/events/domain");
    let mut map = HashMap::new();
    for entry in fs::read_dir(&dir).map_err(|e| e.to_string())?.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        if let Some(et) = parse_frontmatter_event_type(&text) {
            map.insert(et, path);
        }
    }
    Ok(map)
}

pub fn run(repo: &Path, json: bool) -> i32 {
    let subs = match load_subscriptions(repo) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{e}");
            return 1;
        }
    };
    let events = match domain_event_files(repo) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{e}");
            return 1;
        }
    };

    let mut errors: Vec<String> = Vec::new();
    for (event_type, entries) in &subs {
        let Some(path) = events.get(event_type) else {
            continue;
        };
        let text = fs::read_to_string(path).unwrap_or_default();
        let table = parse_md_subscription_table(&text);
        if table.is_empty() {
            continue;
        }
        let mut expected = BTreeSet::new();
        for entry in entries {
            if let Some(pair) = json_handler_key(entry) {
                expected.insert(pair);
            }
        }
        if table != expected {
            errors.push(format!(
                "{event_type}: tabla md != JSON (md={table:?} json={expected:?})"
            ));
        }
    }

    if json {
        println!(
            "{}",
            serde_json::json!({
                "success": errors.is_empty(),
                "error_count": errors.len(),
                "errors": errors,
            })
        );
    } else if errors.is_empty() {
        println!("verify-domain-subscription-parity: OK ({} eventos)", subs.len());
    } else {
        for e in &errors {
            eprintln!("{e}");
        }
    }
    if errors.is_empty() {
        0
    } else {
        1
    }
}
