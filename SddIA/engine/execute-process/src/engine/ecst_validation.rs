//! Validación ECST instancia vs Clase catalogada en `SddIA/events/` (paridad `ecst_validation.py`).

use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct EventClassSchema {
    pub required: Vec<String>,
    pub optional: Vec<String>,
    pub forbidden: Vec<String>,
}

fn parse_payload_fields(body: &str, section: &str) -> Vec<String> {
    let marker = format!("### {section}");
    let Some(start) = body.find(&marker) else {
        return Vec::new();
    };
    let rest = &body[start + marker.len()..];
    let end = rest.find("\n### ").unwrap_or(rest.len());
    let block = &rest[..end];
    let mut fields = Vec::new();
    for line in block.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with("- `") {
            continue;
        }
        let Some(field) = trimmed.strip_prefix("- `").and_then(|s| s.split('`').next()) else {
            continue;
        };
        if field.starts_with('*') {
            continue;
        }
        fields.push(field.to_string());
    }
    fields
}

fn event_type_from_frontmatter(front: &str) -> Option<String> {
    for line in front.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("event_type:") {
            let val = rest.trim().trim_matches('"').trim_matches('\'');
            if !val.is_empty() {
                return Some(val.to_string());
            }
        }
    }
    None
}

pub fn load_event_class_schemas(repo: &Path) -> HashMap<String, EventClassSchema> {
    let events_dir = repo.join("SddIA/events");
    let mut schemas = HashMap::new();
    if !events_dir.is_dir() {
        return schemas;
    }
    let mut paths = Vec::new();
    collect_md_files(&events_dir, &mut paths);
    paths.sort();
    for path in paths {
        if path.file_name().and_then(|n| n.to_str()) == Some("index.md")
            || path.file_name().and_then(|n| n.to_str()) == Some("events-contract.md")
        {
            continue;
        }
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        let text = text.strip_prefix('\u{feff}').unwrap_or(&text);
        if !text.starts_with("---") {
            continue;
        }
        let parts: Vec<&str> = text.splitn(3, "---").collect();
        if parts.len() < 3 {
            continue;
        }
        let Some(event_type) = event_type_from_frontmatter(parts[1]) else {
            continue;
        };
        let body = parts[2];
        schemas.insert(
            event_type,
            EventClassSchema {
                required: parse_payload_fields(body, "REQUIRED"),
                optional: parse_payload_fields(body, "OPTIONAL"),
                forbidden: parse_payload_fields(body, "FORBIDDEN"),
            },
        );
    }
    schemas
}

fn collect_md_files(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_md_files(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("md") {
            out.push(path);
        }
    }
}

pub fn validate_ecst_instance(event: &Value, schema: Option<&EventClassSchema>) -> (bool, Vec<String>) {
    let mut errors = Vec::new();
    let Some(schema) = schema else {
        return (
            false,
            vec!["event_type not cataloged in SddIA/events/ (genoma fractal)".into()],
        );
    };
    let Some(payload) = event.get("payload").and_then(|v| v.as_object()) else {
        return (false, vec!["payload must be object".into()]);
    };
    for field in &schema.required {
        match payload.get(field) {
            None | Some(Value::Null) => errors.push(format!("missing required payload.{field}")),
            _ => {}
        }
    }
    for field in &schema.forbidden {
        let Some(value) = payload.get(field) else {
            continue;
        };
        if field == "hash_signature" {
            errors.push(format!("forbidden payload.{field}"));
        } else if !value.is_null() {
            errors.push(format!(
                "forbidden payload.{field} (must be null if present)"
            ));
        }
    }
    (errors.is_empty(), errors)
}

pub fn validate_domain_mutation_event(
    repo: &Path,
    event: &Value,
) -> Result<(), String> {
    let event_type = event
        .get("event_type")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or("event_type missing or invalid")?;
    let schemas = load_event_class_schemas(repo);
    let schema = schemas.get(event_type);
    let (ok, errors) = validate_ecst_instance(event, schema);
    if !ok {
        return Err(errors.join("; "));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::repo::find_repo_root;
    use serde_json::json;

    #[test]
    fn loads_domain_entity_created_schema() {
        let repo = find_repo_root().unwrap();
        let schemas = load_event_class_schemas(&repo);
        let schema = schemas.get("Domain_Entity_Created").expect("schema");
        assert!(schema.required.contains(&"origin_topology".to_string()));
        assert!(schema.forbidden.contains(&"hash_signature_old".to_string()));
    }

    #[test]
    fn validates_domain_entity_created_payload() {
        let repo = find_repo_root().unwrap();
        let schemas = load_event_class_schemas(&repo);
        let event = json!({
            "event_type": "Domain_Entity_Created",
            "payload": {
                "entity_class": "tool",
                "entity_type": "tool",
                "entity_id": "00000000-0000-4000-8000-000000000001",
                "lifecycle_operation": "create",
                "entity_uuid": "00000000-0000-4000-8000-000000000001",
                "entity_name": "io-choke",
                "version": "1.0.0",
                "hash_signature_new": "sha256:abc",
                "hash_signature_old": null,
                "changes_summary": "create tool",
                "origin_topology": "core"
            }
        });
        let schema = schemas.get("Domain_Entity_Created").unwrap();
        let (ok, errors) = validate_ecst_instance(&event, Some(schema));
        assert!(ok, "{errors:?}");
    }
}
