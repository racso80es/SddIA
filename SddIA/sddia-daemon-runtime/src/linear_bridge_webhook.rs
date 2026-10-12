//! Webhook lab Linear (HU-B 03): firma HMAC, sin cuerpo bruto en bus.

use crate::linear_bridge::assimilate_tracker_items;
use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use serde_json::{json, Value};
use sha2::Sha256;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WebhookReject {
    InvalidSignature,
    InvalidPayload,
}

#[derive(Debug, Clone)]
pub struct WebhookProjectTarget {
    pub slug: String,
    pub label_hu: String,
    pub label_pbi: String,
}

pub fn linear_signature_hex(secret: &str, body: &[u8]) -> String {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .expect("HMAC puede usar cualquier longitud de clave");
    mac.update(body);
    hex::encode(mac.finalize().into_bytes())
}

pub fn verify_linear_signature(secret: &str, body: &[u8], header: &str) -> bool {
    let expected = linear_signature_hex(secret, body);
    let provided = header.trim();
    let provided = provided.strip_prefix("sha256=").unwrap_or(provided);
    expected.eq_ignore_ascii_case(provided)
}

/// Normaliza payload webhook Linear → ítem compatible con `snap_from_item`.
pub fn issue_item_from_webhook(doc: &Value) -> Option<Value> {
    let data = doc.get("data")?;
    let issue = if data.get("identifier").is_some() || data.get("issue_ref").is_some() {
        data
    } else {
        data.get("issue")?
    };
    Some(normalize_linear_issue(issue))
}

fn normalize_linear_issue(data: &Value) -> Value {
    let mut item = data.clone();
    if let Some(labels) = item.get("labels").and_then(|v| v.as_array()) {
        let names: Vec<Value> = labels
            .iter()
            .filter_map(|l| {
                if let Some(s) = l.as_str() {
                    return Some(json!(s));
                }
                l.get("name")
                    .and_then(|n| n.as_str())
                    .map(|s| json!(s))
            })
            .collect();
        item["labels"] = json!(names);
    }
    if let Some(state) = item.get("state") {
        if state.is_object() {
            if let Some(name) = state.get("name").and_then(|v| v.as_str()) {
                item["state"] = json!(name);
            }
        }
    }
    item
}

pub fn process_signed_webhook(
    repo: &Path,
    projects: &[WebhookProjectTarget],
    body: &[u8],
    secret: &str,
    signature: Option<&str>,
    now: DateTime<Utc>,
) -> Result<Vec<PathBuf>, WebhookReject> {
    let sig = signature.ok_or(WebhookReject::InvalidSignature)?;
    if !verify_linear_signature(secret, body, sig) {
        return Err(WebhookReject::InvalidSignature);
    }
    let doc: Value =
        serde_json::from_slice(body).map_err(|_| WebhookReject::InvalidPayload)?;
    let item = issue_item_from_webhook(&doc).ok_or(WebhookReject::InvalidPayload)?;
    let mut emitted = Vec::new();
    for proj in projects {
        let (paths, _) = assimilate_tracker_items(
            repo,
            &proj.slug,
            &proj.label_hu,
            &proj.label_pbi,
            &[item.clone()],
            now,
            "webhook",
        )
        .map_err(|_| WebhookReject::InvalidPayload)?;
        emitted.extend(paths);
    }
    Ok(emitted)
}

/// Listener HTTP mínimo (solo POST, bind local). No registra el cuerpo en stderr.
pub fn spawn_lab_webhook_listener(
    repo: PathBuf,
    bind_host: String,
    port: u16,
    secret: String,
    projects: Vec<WebhookProjectTarget>,
) -> Option<thread::JoinHandle<()>> {
    if port == 0 || secret.is_empty() {
        return None;
    }
    let addr = format!("{bind_host}:{port}");
    let listener = match TcpListener::bind(&addr) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("[LINEAR-BRIDGE] webhook bind {addr}: {e}");
            return None;
        }
    };
    eprintln!("[LINEAR-BRIDGE] webhook lab en {addr}");
    Some(thread::spawn(move || {
        for stream in listener.incoming().flatten() {
            let _ = handle_connection(stream, &repo, &secret, &projects);
        }
    }))
}

fn handle_connection(
    mut stream: TcpStream,
    repo: &Path,
    secret: &str,
    projects: &[WebhookProjectTarget],
) -> std::io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf)?;
    let (headers, body) = split_http_request(&buf)?;
    let method = headers
        .lines()
        .next()
        .and_then(|l| l.split_whitespace().next())
        .unwrap_or("");
    if !method.eq_ignore_ascii_case("POST") {
        write_http_response(&mut stream, 405, "Method Not Allowed")?;
        return Ok(());
    }
    let signature = header_value(&headers, "Linear-Signature");
    let now = Utc::now();
    match process_signed_webhook(repo, projects, &body, secret, signature.as_deref(), now) {
        Ok(paths) => {
            for p in paths {
                if let Ok(rel) = p.strip_prefix(repo) {
                    eprintln!("[LINEAR-BRIDGE] webhook Tracker_Issue_Changed: {}", rel.display());
                }
            }
            write_http_response(&mut stream, 200, "OK")?;
        }
        Err(WebhookReject::InvalidSignature) => {
            write_http_response(&mut stream, 401, "Unauthorized")?;
        }
        Err(WebhookReject::InvalidPayload) => {
            write_http_response(&mut stream, 400, "Bad Request")?;
        }
    }
    Ok(())
}

fn split_http_request(raw: &[u8]) -> std::io::Result<(String, Vec<u8>)> {
    let sep = raw
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "headers"))?;
    let headers = String::from_utf8_lossy(&raw[..sep]).to_string();
    let body = raw[sep + 4..].to_vec();
    Ok((headers, body))
}

fn header_value(headers: &str, name: &str) -> Option<String> {
    let name_lc = name.to_ascii_lowercase();
    for line in headers.lines().skip(1) {
        let Some((k, v)) = line.split_once(':') else {
            continue;
        };
        if k.trim().eq_ignore_ascii_case(&name_lc) {
            return Some(v.trim().to_string());
        }
    }
    None
}

fn write_http_response(stream: &mut TcpStream, code: u16, msg: &str) -> std::io::Result<()> {
    let body = format!("{msg}\n");
    let resp = format!(
        "HTTP/1.1 {code} {msg}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    );
    stream.write_all(resp.as_bytes())?;
    stream.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linear_bridge::{
        assimilate_tracker_items, compose_tracker_issue_changed, save_state, IssueSnap,
    };
    use serde_json::json;
    use std::collections::HashMap;
    use std::fs;
    use uuid::Uuid;

    fn tmp_repo() -> PathBuf {
        let dir = std::env::temp_dir().join(format!("linear-wh-{}", Uuid::new_v4()));
        fs::create_dir_all(dir.join(".events/pending")).unwrap();
        fs::create_dir_all(dir.join(".SddIA/state/tracker-outbound")).unwrap();
        dir
    }

    fn webhook_body(identifier: &str, state: &str) -> Vec<u8> {
        json!({
            "action": "update",
            "data": {
                "identifier": identifier,
                "title": "t",
                "state": state,
                "labels": ["hu"],
                "updated_at": "2026-10-04T12:00:00Z",
            }
        })
        .to_string()
        .into_bytes()
    }

    #[test]
    fn ac13_invalid_signature_emits_nothing() {
        let repo = tmp_repo();
        let secret = "lab-secret";
        let body = webhook_body("OSC-21", "Todo");
        let projects = vec![WebhookProjectTarget {
            slug: "lab".into(),
            label_hu: "hu".into(),
            label_pbi: "pbi".into(),
        }];
        let r = process_signed_webhook(
            &repo,
            &projects,
            &body,
            secret,
            Some("deadbeef"),
            Utc::now(),
        );
        assert_eq!(r, Err(WebhookReject::InvalidSignature));
        let _ = fs::remove_dir_all(&repo);
    }

    #[test]
    fn ac13_valid_signature_emits_webhook_source() {
        let repo = tmp_repo();
        let secret = "lab-secret-ac13";
        let slug = "lab";
        let prev = IssueSnap {
            state: "Backlog".into(),
            title: "t".into(),
            description_len: 0,
            priority: 0,
            labels: vec!["hu".into()],
            parent: None,
            cancelled: false,
            linear_updated_at: Some("2026-10-04T11:00:00Z".into()),
            last_comment: None,
        };
        save_state(
            &repo,
            slug,
            &crate::linear_bridge::WatcherState {
                issues: HashMap::from([("OSC-22".into(), prev)]),
            },
        )
        .unwrap();
        let body = webhook_body("OSC-22", "Todo");
        let sig = linear_signature_hex(secret, &body);
        let projects = vec![WebhookProjectTarget {
            slug: slug.into(),
            label_hu: "hu".into(),
            label_pbi: "pbi".into(),
        }];
        let paths = process_signed_webhook(
            &repo,
            &projects,
            &body,
            secret,
            Some(&sig),
            Utc::now(),
        )
        .unwrap();
        assert_eq!(paths.len(), 1);
        let raw = fs::read_to_string(&paths[0]).unwrap();
        assert!(!raw.contains(secret));
        let ev: Value = serde_json::from_str(&raw).unwrap();
        assert_eq!(
            ev.pointer("/payload/source"),
            Some(&json!("webhook"))
        );
        assert!(ev.get("raw_webhook").is_none());
        assert!(ev.pointer("/payload/webhook_body").is_none());
        let _ = fs::remove_dir_all(&repo);
    }

    #[test]
    fn ac22_event_envelope_never_contains_secret() {
        let repo = tmp_repo();
        let secret = "LINEAR_WEBHOOK_SECRET_VALUE_FOR_TEST";
        let body = webhook_body("OSC-23", "Backlog");
        let sig = linear_signature_hex(secret, &body);
        let item = issue_item_from_webhook(
            &serde_json::from_slice::<Value>(&body).unwrap(),
        )
        .unwrap();
        let (paths, _) = assimilate_tracker_items(
            &repo,
            "lab",
            "hu",
            "pbi",
            &[item],
            Utc::now(),
            "webhook",
        )
        .unwrap();
        for p in paths {
            let raw = fs::read_to_string(&p).unwrap();
            assert!(!raw.contains(secret));
            assert!(!raw.contains("LINEAR_WEBHOOK_SECRET"));
        }
        let bus = compose_tracker_issue_changed("lab", "OSC-23", vec![], None, "webhook");
        let serialized = bus.to_string();
        assert!(!serialized.contains(secret));
        let _ = process_signed_webhook(
            &repo,
            &[WebhookProjectTarget {
                slug: "lab".into(),
                label_hu: "hu".into(),
                label_pbi: "pbi".into(),
            }],
            &body,
            secret,
            Some(&sig),
            Utc::now(),
        );
        let _ = fs::remove_dir_all(&repo);
    }
}
