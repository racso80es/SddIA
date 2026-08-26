//! Centinela IMAP read-only. Emite Email_Received. Ceguera lógica: no orquesta.

use chrono::Utc;
use sddia_daemon_runtime::{
    find_repo_root, load_bus_topology, write_fractal_event, BusTopology, DaemonRuntime,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use uuid::Uuid;

const DEFAULT_PORT: u16 = 993;
const DEFAULT_MAILBOX: &str = "INBOX";
const DEFAULT_POLL: u64 = 60;
const DEFAULT_SNIPPET: usize = 512;
const DEFAULT_INITIAL_LOOKBACK_DAYS: u64 = 60;
const DEFAULT_MAX_UIDS_PER_POLL: usize = 50;
const STATE_FILE: &str = "email-watcher.json";

struct EmailWatcherState {
    last_uid: u32,
    _mailbox: String,
    imap_identity_sha256: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WatermarkResetReason {
    IdentityChanged,
    AboveCeiling,
}

struct ImapCfg {
    host: String,
    port: u16,
    user: String,
    secret: String,
    mailbox: String,
    poll_secs: u64,
    snippet_chars: usize,
    initial_lookback_days: u64,
    max_uids_per_poll: usize,
}

fn iso_now() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

fn env_or(key: &str, default: &str) -> String {
    env::var(key)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| default.to_string())
}

fn require_cfg() -> Result<ImapCfg, String> {
    let host = env::var("SDDIA_EMAIL_IMAP_HOST")
        .map_err(|_| "SDDIA_EMAIL_IMAP_HOST requerido")?
        .trim()
        .to_string();
    if host.is_empty() {
        return Err("SDDIA_EMAIL_IMAP_HOST vacío".into());
    }
    let user = env::var("SDDIA_EMAIL_IMAP_USER")
        .map_err(|_| "SDDIA_EMAIL_IMAP_USER requerido")?;
    let secret = env::var("SDDIA_EMAIL_IMAP_SECRET")
        .map_err(|_| "SDDIA_EMAIL_IMAP_SECRET requerido")?;
    let port = env_or("SDDIA_EMAIL_IMAP_PORT", "993")
        .parse::<u16>()
        .unwrap_or(DEFAULT_PORT);
    Ok(ImapCfg {
        host,
        port,
        user: user.trim().to_string(),
        secret: secret.trim().to_string(),
        mailbox: env_or("SDDIA_EMAIL_MAILBOX", DEFAULT_MAILBOX),
        poll_secs: env_or("SDDIA_EMAIL_POLL_SECONDS", "60")
            .parse::<u64>()
            .unwrap_or(DEFAULT_POLL)
            .max(5),
        snippet_chars: env_or("SDDIA_EMAIL_SNIPPET_CHARS", "512")
            .parse::<usize>()
            .unwrap_or(DEFAULT_SNIPPET)
            .max(32),
        initial_lookback_days: env_or("SDDIA_EMAIL_INITIAL_LOOKBACK_DAYS", "60")
            .parse::<u64>()
            .unwrap_or(DEFAULT_INITIAL_LOOKBACK_DAYS)
            .max(1),
        max_uids_per_poll: env_or("SDDIA_EMAIL_MAX_UIDS_PER_POLL", "50")
            .parse::<usize>()
            .unwrap_or(DEFAULT_MAX_UIDS_PER_POLL)
            .max(1),
    })
}

fn imap_identity_sha256(cfg: &ImapCfg) -> String {
    imap_identity_sha256_from(&cfg.host, cfg.port, &cfg.mailbox, &cfg.user)
}

fn imap_identity_sha256_from(host: &str, port: u16, mailbox: &str, user: &str) -> String {
    let normalized = format!(
        "{}|{}|{}|{}",
        host.trim().to_ascii_lowercase(),
        port,
        mailbox.trim().to_ascii_lowercase(),
        user.trim().to_ascii_lowercase(),
    );
    let digest = Sha256::digest(normalized.as_bytes());
    format!("{:x}", digest)
}

fn uid_search_criterion(last: u32, lookback_days: u64) -> String {
    if last == 0 {
        // F-07: bootstrap = ALL; el lote se recorta a los N UIDs más recientes en plan_bootstrap_uids.
        // lookback_days queda reservado para opt-in legado (SDDIA_EMAIL_BOOTSTRAP_SINCE=1).
        let _ = lookback_days;
        if matches!(
            std::env::var("SDDIA_EMAIL_BOOTSTRAP_SINCE")
                .unwrap_or_default()
                .trim()
                .to_lowercase()
                .as_str(),
            "1" | "true" | "yes"
        ) {
            let since = Utc::now() - chrono::Duration::days(lookback_days as i64);
            return format!("SINCE {}", since.format("%d-%b-%Y"));
        }
        "ALL".to_string()
    } else {
        format!("UID {}:*", last + 1)
    }
}

fn instance_root(repo: &Path) -> PathBuf {
    repo.join(".SddIA")
}

fn state_path(repo: &Path) -> PathBuf {
    sddia_daemon_runtime::state_dir(repo).join(STATE_FILE)
}

fn inbox_dir(repo: &Path) -> PathBuf {
    instance_root(repo).join("inbox")
}

fn load_state(repo: &Path) -> EmailWatcherState {
    let path = state_path(repo);
    if !path.is_file() {
        return EmailWatcherState {
            last_uid: 0,
            _mailbox: DEFAULT_MAILBOX.to_string(),
            imap_identity_sha256: None,
        };
    }
    let Ok(raw) = fs::read_to_string(&path) else {
        return EmailWatcherState {
            last_uid: 0,
            _mailbox: DEFAULT_MAILBOX.to_string(),
            imap_identity_sha256: None,
        };
    };
    let Ok(v) = serde_json::from_str::<Value>(&raw) else {
        return EmailWatcherState {
            last_uid: 0,
            _mailbox: DEFAULT_MAILBOX.to_string(),
            imap_identity_sha256: None,
        };
    };
    EmailWatcherState {
        last_uid: v
            .get("last_uid")
            .and_then(|x| x.as_u64())
            .map(|n| n as u32)
            .unwrap_or(0),
        _mailbox: v
            .get("mailbox")
            .and_then(|x| x.as_str())
            .unwrap_or(DEFAULT_MAILBOX)
            .to_string(),
        imap_identity_sha256: v
            .get("imap_identity_sha256")
            .and_then(|x| x.as_str())
            .map(str::to_string),
    }
}

fn load_last_uid(repo: &Path) -> u32 {
    load_state(repo).last_uid
}

fn resolve_watermark(
    last: u32,
    stored_identity: Option<&str>,
    identity_now: &str,
    mailbox_max_uid: Option<u32>,
) -> (u32, Option<WatermarkResetReason>) {
    if let Some(stored) = stored_identity {
        if stored != identity_now {
            return (0, Some(WatermarkResetReason::IdentityChanged));
        }
    }
    if last > 0 {
        if let Some(max_uid) = mailbox_max_uid {
            if max_uid < last {
                return (0, Some(WatermarkResetReason::AboveCeiling));
            }
        }
    }
    (last, None)
}

fn uids_after(uids: impl IntoIterator<Item = u32>, last: u32) -> Vec<u32> {
    let mut ordered: Vec<u32> = uids.into_iter().filter(|u| *u > last).collect();
    ordered.sort_unstable();
    ordered
}

/// Bootstrap F-07: los `max` UIDs más altos del mailbox (orden ascendente para procesar).
fn plan_bootstrap_uids(uids: impl IntoIterator<Item = u32>, max: usize) -> Vec<u32> {
    if max == 0 {
        return Vec::new();
    }
    let mut all: Vec<u32> = uids.into_iter().collect();
    all.sort_unstable();
    if all.len() > max {
        all = all[all.len() - max..].to_vec();
    }
    all
}

/// Prioriza UNSEEN (más recientes primero) y limita el lote; el catch-up continúa en sondeos siguientes.
fn plan_poll_uids(incremental: &[u32], unseen: &[u32], last: u32, max: usize) -> Vec<u32> {
    if max == 0 {
        return Vec::new();
    }
    let unseen_set: std::collections::HashSet<u32> =
        unseen.iter().copied().filter(|u| *u > last).collect();
    let mut batch = Vec::with_capacity(max.min(incremental.len() + unseen_set.len()));

    let mut unseen_desc: Vec<u32> = unseen_set.iter().copied().collect();
    unseen_desc.sort_unstable_by(|a, b| b.cmp(a));
    for uid in unseen_desc {
        if batch.len() >= max {
            break;
        }
        batch.push(uid);
    }

    for uid in incremental {
        if batch.len() >= max {
            break;
        }
        if !unseen_set.contains(uid) {
            batch.push(*uid);
        }
    }

    batch.sort_unstable();
    batch
}

/// Avanza watermark solo en secuencia contigua desde `last` (evita saltar huecos del catch-up).
fn advance_contiguous_watermark(last: u32, processed: &[u32]) -> u32 {
    if processed.is_empty() {
        return last;
    }
    let mut sorted = processed.to_vec();
    sorted.sort_unstable();
    let set: std::collections::HashSet<u32> = sorted.iter().copied().collect();
    let mut next = last;
    while set.contains(&(next + 1)) {
        next += 1;
    }
    next
}

fn save_state(
    repo: &Path,
    mailbox: &str,
    last_uid: u32,
    imap_identity_sha256: &str,
) -> Result<(), String> {
    let path = state_path(repo);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("mkdir state: {e}"))?;
    }
    let body = json!({
        "mailbox": mailbox,
        "last_uid": last_uid,
        "imap_identity_sha256": imap_identity_sha256,
        "updated_at": iso_now(),
    });
    fs::write(&path, format!("{body}\n")).map_err(|e| format!("write watermark: {e}"))
}

fn save_last_uid(repo: &Path, mailbox: &str, last_uid: u32, identity: &str) -> Result<(), String> {
    save_state(repo, mailbox, last_uid, identity)
}

fn header_value<'a>(headers: &'a str, name: &str) -> Option<String> {
    let needle = format!("{name}:");
    let mut val = String::new();
    let mut capturing = false;
    for line in headers.lines() {
        if capturing {
            if line.starts_with(' ') || line.starts_with('\t') {
                val.push(' ');
                val.push_str(line.trim());
                continue;
            }
            break;
        }
        if line.len() >= needle.len()
            && line.as_bytes()[..needle.len()].eq_ignore_ascii_case(needle.as_bytes())
        {
            val = line[needle.len()..].trim().to_string();
            capturing = true;
        }
    }
    if capturing && !val.is_empty() {
        Some(decode_rfc2047(&val))
    } else {
        None
    }
}

fn decode_q_bytes(s: &str) -> Vec<u8> {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'_' => {
                out.push(b' ');
                i += 1;
            }
            b'=' if i + 2 < b.len() => {
                match u8::from_str_radix(std::str::from_utf8(&b[i + 1..i + 3]).unwrap_or(""), 16) {
                    Ok(v) => {
                        out.push(v);
                        i += 3;
                    }
                    Err(_) => {
                        out.push(b[i]);
                        i += 1;
                    }
                }
            }
            c => {
                out.push(c);
                i += 1;
            }
        }
    }
    out
}

fn decode_base64_bytes(s: &str) -> Vec<u8> {
    fn val(c: u8) -> Option<u8> {
        match c {
            b'A'..=b'Z' => Some(c - b'A'),
            b'a'..=b'z' => Some(c - b'a' + 26),
            b'0'..=b'9' => Some(c - b'0' + 52),
            b'+' | b'-' => Some(62),
            b'/' | b'_' => Some(63),
            _ => None,
        }
    }
    let filtered: Vec<u8> = s.bytes().filter(|c| *c != b'=' && !c.is_ascii_whitespace()).collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < filtered.len() {
        let a = val(filtered[i]).unwrap_or(0) as u32;
        let b = filtered.get(i + 1).copied().and_then(val).unwrap_or(0) as u32;
        let c = filtered.get(i + 2).copied().and_then(val).unwrap_or(0) as u32;
        let d = filtered.get(i + 3).copied().and_then(val).unwrap_or(0) as u32;
        let n = (a << 18) | (b << 12) | (c << 6) | d;
        out.push((n >> 16) as u8);
        if i + 2 < filtered.len() {
            out.push(((n >> 8) & 0xff) as u8);
        }
        if i + 3 < filtered.len() {
            out.push((n & 0xff) as u8);
        }
        i += 4;
    }
    out
}

fn charset_bytes(charset: &str, bytes: &[u8]) -> String {
    match charset.trim().to_ascii_lowercase().as_str() {
        "utf-8" | "utf8" | "us-ascii" => String::from_utf8_lossy(bytes).into_owned(),
        _ => bytes.iter().map(|&b| char::from(b)).collect(),
    }
}

fn take_encoded_word(s: &str) -> Option<(usize, String)> {
    if !s.starts_with("=?") {
        return None;
    }
    let rest = &s[2..];
    let c_end = rest.find('?')?;
    let charset = &rest[..c_end];
    let rest = &rest[c_end + 1..];
    let e_end = rest.find('?')?;
    let enc = rest[..e_end].to_ascii_uppercase();
    let rest = &rest[e_end + 1..];
    let b_end = rest.find("?=")?;
    let body = &rest[..b_end];
    let total = 2 + c_end + 1 + e_end + 1 + b_end + 2;
    let raw = match enc.as_str() {
        "Q" => decode_q_bytes(body),
        "B" => decode_base64_bytes(body),
        _ => return None,
    };
    Some((total, charset_bytes(charset, &raw)))
}

fn decode_rfc2047(input: &str) -> String {
    let mut out = String::new();
    let mut i = 0;
    let chars = input;
    while i < chars.len() {
        if chars[i..].starts_with("=?") {
            if let Some((consumed, decoded)) = take_encoded_word(&chars[i..]) {
                out.push_str(&decoded);
                i += consumed;
                let rest = &chars[i..];
                let ws = rest
                    .chars()
                    .take_while(|c| *c == ' ' || *c == '\t' || *c == '\r' || *c == '\n')
                    .map(|c| c.len_utf8())
                    .sum::<usize>();
                if rest.get(ws..).is_some_and(|r| r.starts_with("=?")) {
                    i += ws;
                }
                continue;
            }
        }
        let ch = chars[i..].chars().next().unwrap();
        out.push(ch);
        i += ch.len_utf8();
    }
    out
}

fn detect_list_headers(headers: &str) -> Vec<String> {
    let mut out = Vec::new();
    for name in ["List-Id", "List-Unsubscribe", "Precedence", "Auto-Submitted"] {
        if let Some(v) = header_value(headers, name) {
            if name.eq_ignore_ascii_case("Precedence") {
                let low = v.to_ascii_lowercase();
                if !(low.contains("bulk") || low.contains("list")) {
                    continue;
                }
            }
            if name.eq_ignore_ascii_case("Auto-Submitted") && v.eq_ignore_ascii_case("no") {
                continue;
            }
            out.push(format!("{name}: {v}"));
        }
    }
    out
}

fn decode_snippet(raw: &str, max_chars: usize) -> String {
    let flat: String = raw
        .chars()
        .map(|c| if c == '\n' || c == '\r' { ' ' } else { c })
        .collect();
    let trimmed = flat.split_whitespace().collect::<Vec<_>>().join(" ");
    trimmed.chars().take(max_chars).collect()
}

fn persist_eml(repo: &Path, uid: u32, raw: &[u8]) -> Result<String, String> {
    let dir = inbox_dir(repo);
    fs::create_dir_all(&dir).map_err(|e| format!("mkdir inbox: {e}"))?;
    let name = format!("{uid}.eml");
    fs::write(dir.join(&name), raw).map_err(|e| format!("write eml: {e}"))?;
    let inst = ".SddIA";
    Ok(format!("{inst}/inbox/{name}"))
}

fn emit_received(
    repo: &Path,
    top: &BusTopology,
    uid: u32,
    mailbox: &str,
    headers: &str,
    snippet: &str,
    body_ref: &str,
) -> Result<(), String> {
    let from = header_value(headers, "From").unwrap_or_default();
    let subject = header_value(headers, "Subject").unwrap_or_default();
    let received_at = header_value(headers, "Date").unwrap_or_else(iso_now);
    let list_headers = detect_list_headers(headers);
    let mut payload = json!({
        "message_uid": uid.to_string(),
        "mailbox": mailbox,
        "from": from,
        "subject": subject,
        "received_at": received_at,
        "snippet": snippet,
        "body_ref": body_ref,
    });
    if !list_headers.is_empty() {
        payload["list_headers"] = json!(list_headers);
    }
    let event = json!({
        "event_id": Uuid::new_v4().to_string(),
        "event_type": "Email_Received",
        "event_family": "domain",
        "timestamp": iso_now(),
        "emitter_agent": "email-watcher",
        "payload": payload,
    });
    write_fractal_event(repo, top, &event, "domain")
}

fn poll_once(
    repo: &Path,
    top: &BusTopology,
    cfg: &ImapCfg,
    centinela: &mut DaemonRuntime,
) -> Result<(), String> {
    let tls = native_tls::TlsConnector::builder()
        .build()
        .map_err(|e| format!("tls: {e}"))?;
    let client = imap::connect((cfg.host.as_str(), cfg.port), cfg.host.as_str(), &tls)
        .map_err(|e| format!("imap connect: {e}"))?;
    let mut session = client
        .login(&cfg.user, &cfg.secret)
        .map_err(|e| format!("imap login: {}", e.0))?;
    session
        .examine(&cfg.mailbox)
        .map_err(|e| format!("imap examine: {e}"))?;

    let state = load_state(repo);
    let identity_now = imap_identity_sha256(cfg);
    let mailbox_max_uid = session
        .uid_search("ALL")
        .ok()
        .and_then(|uids| uids.into_iter().max().map(|u| u as u32));
    let (last, reset_reason) = resolve_watermark(
        state.last_uid,
        state.imap_identity_sha256.as_deref(),
        &identity_now,
        mailbox_max_uid,
    );
    match reset_reason {
        Some(WatermarkResetReason::IdentityChanged) => {
            eprintln!("[email-watcher] imap identity changed; resetting watermark");
        }
        Some(WatermarkResetReason::AboveCeiling) => {
            eprintln!("[email-watcher] watermark above mailbox ceiling; resetting watermark");
        }
        None => {}
    }

    let bootstrap = last == 0;
    let criterion = uid_search_criterion(last, cfg.initial_lookback_days);
    let uids = session
        .uid_search(criterion)
        .map_err(|e| format!("imap search: {e}"))?;

    let ordered = if bootstrap {
        plan_bootstrap_uids(uids, cfg.max_uids_per_poll)
    } else {
        let incremental = uids_after(uids, last);
        let unseen: Vec<u32> = session
            .uid_search("UNSEEN")
            .map(|set| set.into_iter().collect())
            .unwrap_or_default();
        plan_poll_uids(&incremental, &unseen, last, cfg.max_uids_per_poll)
    };
    let mut processed = Vec::new();

    for uid in &ordered {
        let uid = *uid;
        if inbox_dir(repo).join(format!("{uid}.eml")).is_file() {
            processed.push(uid);
            continue;
        }
        let fetched = match session.uid_fetch(uid.to_string(), "BODY.PEEK[]") {
            Ok(f) => f,
            Err(e) => {
                eprintln!("[email-watcher] uid {uid} fetch: {e}");
                continue;
            }
        };
        let Some(msg) = fetched.iter().next() else {
            continue;
        };
        let raw = msg.body().unwrap_or(&[]);
        let text = String::from_utf8_lossy(raw);
        let (headers, body) = match text.split_once("\r\n\r\n") {
            Some(pair) => pair,
            None => text.split_once("\n\n").unwrap_or((text.as_ref(), "")),
        };
        let snippet = decode_snippet(body, cfg.snippet_chars);
        let body_ref = match persist_eml(repo, uid, raw) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("[email-watcher] uid {uid} persist: {e}");
                continue;
            }
        };
        if let Err(e) = emit_received(repo, top, uid, &cfg.mailbox, headers, &snippet, &body_ref) {
            eprintln!("[email-watcher] uid {uid} emit: {e}");
            let _ = fs::remove_file(inbox_dir(repo).join(format!("{uid}.eml")));
            continue;
        }
        processed.push(uid);
        centinela.note_stimulus();
    }

    if bootstrap {
        // F-07: watermark = techo del lote bootstrap (abandona UIDs antiguos no seleccionados).
        let _ = ordered.iter().copied().max();
    }

    let final_last = if bootstrap {
        ordered.iter().copied().max().unwrap_or(last)
    } else if !processed.is_empty() {
        advance_contiguous_watermark(last, &processed)
    } else {
        last
    };

    let identity_needs_persist = state.imap_identity_sha256.as_deref() != Some(identity_now.as_str());
    let last_changed = final_last != state.last_uid;
    if identity_needs_persist || last_changed {
        save_last_uid(repo, &cfg.mailbox, final_last, &identity_now)?;
    }

    let _ = session.logout();
    Ok(())
}

fn run_loop(running: Arc<AtomicBool>) -> Result<(), i32> {
    let repo = find_repo_root().map_err(|e| {
        eprintln!("{e}");
        1
    })?;
    let cfg = require_cfg().map_err(|e| {
        eprintln!("[email-watcher] {e}");
        1
    })?;
    let top = load_bus_topology(&repo);
    let mut centinela = DaemonRuntime::new(repo.clone(), "email-watcher");
    centinela.bootstrap(&top).map_err(|e| {
        eprintln!("[email-watcher] {e}");
        1
    })?;
    while running.load(Ordering::SeqCst) {
        if let Err(e) = poll_once(&repo, &top, &cfg, &mut centinela) {
            eprintln!("[email-watcher] poll: {e}");
        }
        let _ = centinela.tick(&top);
        let step = Duration::from_secs(1);
        let mut waited = 0u64;
        while waited < cfg.poll_secs && running.load(Ordering::SeqCst) {
            thread::sleep(step);
            waited += 1;
            let _ = centinela.tick(&top);
        }
    }
    centinela.shutdown();
    Ok(())
}

fn once_envelope(success: bool, message: &str) -> Value {
    json!({
        "meta": {
            "schemaVersion": "2.0",
            "entityKind": "tool",
            "entityId": "email-watcher"
        },
        "success": success,
        "exitCode": if success { 0 } else { 1 },
        "message": message,
    })
}

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let flag = Arc::clone(&running);
    let _ = ctrlc::set_handler(move || {
        flag.store(false, Ordering::SeqCst);
    });
    let once = env::args().any(|a| a == "--once");
    if once {
        let repo = match find_repo_root() {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{e}");
                println!("{}", once_envelope(false, &e.to_string()));
                std::process::exit(1);
            }
        };
        let cfg = match require_cfg() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("[email-watcher] {e}");
                println!("{}", once_envelope(false, &e));
                std::process::exit(1);
            }
        };
        let top = load_bus_topology(&repo);
        let mut centinela = DaemonRuntime::new(repo.clone(), "email-watcher");
        if let Err(e) = centinela.bootstrap(&top) {
            eprintln!("[email-watcher] {e}");
            println!("{}", once_envelope(false, &e));
            std::process::exit(1);
        }
        let poll_result = poll_once(&repo, &top, &cfg, &mut centinela);
        let _ = centinela.tick(&top);
        centinela.shutdown();
        match poll_result {
            Ok(()) => {
                println!("{}", once_envelope(true, "poll ok"));
            }
            Err(e) => {
                eprintln!("[email-watcher] {e}");
                println!("{}", once_envelope(false, &e));
                std::process::exit(1);
            }
        }
        return;
    }
    if let Err(code) = run_loop(running) {
        std::process::exit(code);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rfc2047_q_decodes_spanish_meeting_subject() {
        let raw = "=?UTF-8?Q?Reuni=C3=B3n_con_Racso_el_21=2F08=2F2026_a_las_10=3A00?=";
        assert_eq!(
            decode_rfc2047(raw),
            "Reunión con Racso el 21/08/2026 a las 10:00"
        );
    }

    #[test]
    fn header_value_decodes_rfc2047_subject() {
        let h = "Subject: =?UTF-8?Q?Reuni=C3=B3n_con_Racso_el_21=2F08=2F2026_a_las_10=3A00?=\r\nFrom: a@b\r\n";
        assert_eq!(
            header_value(h, "Subject").as_deref(),
            Some("Reunión con Racso el 21/08/2026 a las 10:00")
        );
    }

    #[test]
    fn list_id_is_captured() {
        let h = "From: a@b\r\nList-Id: <news.example>\r\nSubject: hi\r\n";
        let v = detect_list_headers(h);
        assert!(v.iter().any(|s| s.starts_with("List-Id:")));
    }

    #[test]
    fn header_value_tolerates_replacement_char_in_line() {
        // Cabecera con U+FFFD (3 bytes) tras el prefijo ASCII — el slice str [..11] fallaría.
        let line = "Precedence:\u{FFFD} bulk";
        assert!(header_value(line, "Precedence").is_some());
    }

    #[test]
    fn snippet_truncates() {
        let s = decode_snippet("aaa\nbbb", 4);
        assert!(s.len() <= 4);
    }

    #[test]
    fn first_poll_uses_all_not_since() {
        let c = uid_search_criterion(0, 60);
        assert_eq!(c, "ALL");
    }

    #[test]
    fn bootstrap_takes_highest_n_uids() {
        let batch = plan_bootstrap_uids([1, 2, 3, 10, 11, 12, 13], 3);
        assert_eq!(batch, vec![11, 12, 13]);
    }

    #[test]
    fn incremental_poll_uses_uid_range() {
        assert_eq!(uid_search_criterion(42, 60), "UID 43:*");
    }

    #[test]
    fn plan_poll_prioritizes_unseen_and_caps_batch() {
        let incremental: Vec<u32> = (57163..=57220).collect();
        let unseen = vec![104385, 57170];
        let batch = plan_poll_uids(&incremental, &unseen, 57162, 5);
        assert!(batch.contains(&104385));
        assert!(batch.contains(&57170));
        assert_eq!(batch.len(), 5);
    }

    #[test]
    fn contiguous_watermark_skips_high_uid_gap() {
        assert_eq!(advance_contiguous_watermark(57162, &[104385]), 57162);
        assert_eq!(advance_contiguous_watermark(57162, &[57163, 57164]), 57164);
        assert_eq!(
            advance_contiguous_watermark(57162, &[57163, 104385, 57164]),
            57164
        );
    }

    #[test]
    fn watermark_skips_uids_already_seen() {
        assert_eq!(uids_after([1, 2, 3, 4], 3), vec![4]);
        assert!(uids_after([1, 2, 3], 3).is_empty());
        assert_eq!(uids_after([5, 2, 4], 0), vec![2, 4, 5]);
    }

    #[test]
    fn imap_identity_normalizes_case_and_whitespace() {
        let a = imap_identity_sha256_from("  IMAP.Gmail.COM ", 993, " INBOX ", " User@Example.com ");
        let b = imap_identity_sha256_from("imap.gmail.com", 993, "inbox", "user@example.com");
        assert_eq!(a, b);
        assert_eq!(a.len(), 64);
    }

    #[test]
    fn imap_identity_changes_when_user_changes() {
        let a = imap_identity_sha256_from("imap.example.com", 993, "inbox", "a@b.com");
        let b = imap_identity_sha256_from("imap.example.com", 993, "inbox", "c@d.com");
        assert_ne!(a, b);
    }

    #[test]
    fn resolve_watermark_identity_mismatch_resets() {
        let id_a = imap_identity_sha256_from("imap.example.com", 993, "inbox", "a@b.com");
        let id_b = imap_identity_sha256_from("imap.example.com", 993, "inbox", "c@d.com");
        let (last, reason) = resolve_watermark(104466, Some(&id_a), &id_b, Some(5799));
        assert_eq!(last, 0);
        assert_eq!(reason, Some(WatermarkResetReason::IdentityChanged));
    }

    #[test]
    fn resolve_watermark_legacy_skips_identity_check() {
        let id_b = imap_identity_sha256_from("imap.example.com", 993, "inbox", "c@d.com");
        let (last, reason) = resolve_watermark(42, None, &id_b, Some(5799));
        assert_eq!(last, 42);
        assert_eq!(reason, None);
    }

    #[test]
    fn resolve_watermark_ceiling_resets_stale_high_uid() {
        let id = imap_identity_sha256_from("imap.example.com", 993, "inbox", "a@b.com");
        let (last, reason) = resolve_watermark(104466, Some(&id), &id, Some(5799));
        assert_eq!(last, 0);
        assert_eq!(reason, Some(WatermarkResetReason::AboveCeiling));
    }

    #[test]
    fn resolve_watermark_same_identity_within_ceiling() {
        let id = imap_identity_sha256_from("imap.example.com", 993, "inbox", "a@b.com");
        let (last, reason) = resolve_watermark(5798, Some(&id), &id, Some(5799));
        assert_eq!(last, 5798);
        assert_eq!(reason, None);
    }

    #[test]
    fn watermark_roundtrip_persists_last_uid() {
        let dir = std::env::temp_dir().join(format!("sddia-ew-{}", Uuid::new_v4()));
        let repo = dir.join("repo");
        let state = repo.join(".SddIA").join("daemons").join("state");
        std::fs::create_dir_all(&state).unwrap();
        let identity = imap_identity_sha256_from("imap.example.com", 993, "inbox", "a@b.com");
        save_last_uid(&repo, "INBOX", 42, &identity).unwrap();
        assert_eq!(load_last_uid(&repo), 42);
        let loaded = load_state(&repo);
        assert_eq!(loaded.imap_identity_sha256.as_deref(), Some(identity.as_str()));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn once_envelope_json_io_contract() {
        let ok = once_envelope(true, "poll ok");
        assert_eq!(ok["success"], json!(true));
        assert_eq!(ok["exitCode"], json!(0));
        assert_eq!(ok["meta"]["schemaVersion"], json!("2.0"));
        assert_eq!(ok["meta"]["entityId"], json!("email-watcher"));
        let fail = once_envelope(false, "imap connect: timeout");
        assert_eq!(fail["success"], json!(false));
        assert_eq!(fail["exitCode"], json!(1));
    }
}
