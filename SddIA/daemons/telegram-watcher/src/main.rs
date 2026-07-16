use sddia_daemon_runtime::{find_repo_root, load_bus_topology, BusTopology, DaemonRuntime};
use serde_json::{json, Value};
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};

/// Margen bajo el intervalo de heartbeat (30s) para no bloquear el hilo principal en el límite.
const POLL_TIMEOUT: u64 = 25;
const HEARTBEAT_TICK_SECONDS: u64 = 10;
const CONFLICT_BACKOFF_SECONDS: u64 = 5;
const STATE_REL: &str = ".SddIA/daemons/state/telegram-watcher.json";
const LEGACY_STATE_REL: &str = ".SddIA/.state/telegram_last_id";
const SEEN_REL: &str = ".SddIA/daemons/state/telegram-seen-update-ids.json";
const SEEN_MAX: usize = 512;

fn execute_process_bin(repo: &Path) -> PathBuf {
    if let Ok(p) = env::var("SDDIA_EXECUTE_PROCESS_BIN") {
        if !p.trim().is_empty() {
            return PathBuf::from(p);
        }
    }
    for rel in [
        "SddIA/target/debug/execute-process",
        "SddIA/target/release/execute-process",
    ] {
        let candidate = repo.join(rel);
        if candidate.is_file() {
            return candidate;
        }
    }
    repo.join("SddIA/target/debug/execute-process")
}

fn state_path(repo: &Path) -> PathBuf {
    repo.join(STATE_REL)
}

fn load_last_update_id(repo: &Path) -> i64 {
    let path = state_path(repo);
    if path.is_file() {
        if let Ok(raw) = fs::read_to_string(&path) {
            let trimmed = raw.trim();
            if let Ok(body) = serde_json::from_str::<Value>(trimmed) {
                if let Some(id) = body.get("last_update_id").and_then(|v| v.as_i64()) {
                    return id.max(0);
                }
            }
            if trimmed.chars().all(|c| c.is_ascii_digit()) {
                return trimmed.parse::<i64>().unwrap_or(0).max(0);
            }
        }
    }
    let legacy = repo.join(LEGACY_STATE_REL);
    if legacy.is_file() {
        if let Ok(raw) = fs::read_to_string(&legacy) {
            let trimmed = raw.trim();
            if trimmed.chars().all(|c| c.is_ascii_digit()) {
                return trimmed.parse::<i64>().unwrap_or(0).max(0);
            }
            if let Ok(body) = serde_json::from_str::<Value>(trimmed) {
                if let Some(id) = body.get("last_update_id").and_then(|v| v.as_i64()) {
                    return id.max(0);
                }
            }
        }
    }
    0
}

fn save_last_update_id(repo: &Path, last_update_id: i64) -> Result<(), String> {
    let path = state_path(repo);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("mkdir state: {e}"))?;
    }
    let body = json!({ "last_update_id": last_update_id });
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, format!("{body}\n")).map_err(|e| format!("write state tmp: {e}"))?;
    fs::rename(&tmp, &path).map_err(|e| format!("rename state: {e}"))?;
    // Espejo legacy para operadores que lean la ruta antigua.
    let legacy = repo.join(LEGACY_STATE_REL);
    if let Some(parent) = legacy.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(&legacy, format!("{last_update_id}"));
    Ok(())
}

fn load_seen(repo: &Path) -> HashSet<i64> {
    let path = repo.join(SEEN_REL);
    let Ok(raw) = fs::read_to_string(path) else {
        return HashSet::new();
    };
    let Ok(body) = serde_json::from_str::<Value>(&raw) else {
        return HashSet::new();
    };
    body.get("ids")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_i64())
                .collect::<HashSet<_>>()
        })
        .unwrap_or_default()
}

fn save_seen(repo: &Path, seen: &HashSet<i64>) -> Result<(), String> {
    let path = repo.join(SEEN_REL);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("mkdir seen: {e}"))?;
    }
    let mut ids: Vec<i64> = seen.iter().copied().collect();
    ids.sort_unstable();
    if ids.len() > SEEN_MAX {
        ids = ids.split_off(ids.len() - SEEN_MAX);
    }
    let body = json!({ "ids": ids });
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, format!("{body}\n")).map_err(|e| format!("write seen tmp: {e}"))?;
    fs::rename(&tmp, &path).map_err(|e| format!("rename seen: {e}"))
}

fn require_env() -> Result<(String, String), i32> {
    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap_or_default();
    let chat_id = env::var("TELEGRAM_ALLOWED_CHAT_ID").unwrap_or_default();
    let token = token.trim().to_string();
    let chat_id = chat_id.trim().to_string();
    if token.is_empty() || chat_id.is_empty() {
        eprintln!(
            "[telegram-watcher] TELEGRAM_BOT_TOKEN / TELEGRAM_ALLOWED_CHAT_ID no configurados"
        );
        return Err(2);
    }
    Ok((token, chat_id))
}

fn delete_webhook(token: &str) {
    let url = format!("https://api.telegram.org/bot{token}/deleteWebhook?drop_pending_updates=false");
    let agent = ureq::agent();
    match agent.get(&url).timeout(Duration::from_secs(15)).call() {
        Ok(resp) => {
            let body: Value = resp.into_json().unwrap_or(Value::Null);
            if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
                eprintln!("[telegram-watcher] deleteWebhook: respuesta no ok");
            }
        }
        Err(e) => eprintln!("[telegram-watcher] deleteWebhook error: {e}"),
    }
}

fn get_updates(token: &str, offset: i64) -> Vec<Value> {
    let url = format!(
        "https://api.telegram.org/bot{token}/getUpdates?timeout={POLL_TIMEOUT}&offset={offset}"
    );
    let agent = ureq::agent();
    match agent
        .get(&url)
        .timeout(Duration::from_secs(POLL_TIMEOUT + 10))
        .call()
    {
        Ok(resp) => {
            let body: Value = resp.into_json().unwrap_or(Value::Null);
            if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
                return vec![];
            }
            body.get("result")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default()
        }
        Err(ureq::Error::Status(409, _)) => {
            eprintln!(
                "[telegram-watcher] getUpdates conflict (409): otra instancia o webhook activo; backoff {CONFLICT_BACKOFF_SECONDS}s"
            );
            thread::sleep(Duration::from_secs(CONFLICT_BACKOFF_SECONDS));
            vec![]
        }
        Err(e) => {
            eprintln!("[telegram-watcher] getUpdates error: {e}");
            vec![]
        }
    }
}

fn spawn_heartbeat_worker(
    centinela: Arc<Mutex<DaemonRuntime>>,
    top: BusTopology,
) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
        if let Ok(mut rt) = centinela.lock() {
            if let Err(e) = rt.tick(&top) {
                eprintln!("[telegram-watcher] heartbeat keepalive: {e}");
            }
        }
        thread::sleep(Duration::from_secs(HEARTBEAT_TICK_SECONDS));
    })
}

fn extract_text(update: &Value) -> Option<String> {
    let msg = update
        .get("message")
        .or_else(|| update.get("edited_message"))?;
    let text = msg.get("text")?.as_str()?;
    let trimmed = text.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn chat_id(update: &Value) -> Option<String> {
    let msg = update
        .get("message")
        .or_else(|| update.get("edited_message"))?;
    let chat = msg.get("chat")?;
    chat.get("id").map(|v| {
        if let Some(n) = v.as_i64() {
            n.to_string()
        } else {
            v.to_string()
        }
    })
}

fn invoke_gateway(repo: &Path, text: &str, dry_run: bool) -> i32 {
    if dry_run {
        return 0;
    }
    let runner = execute_process_bin(repo);
    let payload = serde_json::json!({ "text": text }).to_string();
    let out = Command::new(&runner)
        .args(["--process", "telegram-gateway", "--inputs", &payload])
        .current_dir(repo)
        .output();
    match out {
        Ok(o) => {
            if o.status.success() {
                0
            } else {
                let err = String::from_utf8_lossy(&o.stderr);
                if !err.trim().is_empty() {
                    eprintln!("{}", err.trim());
                }
                1
            }
        }
        Err(e) => {
            eprintln!("[telegram-watcher] spawn execute-process: {e}");
            1
        }
    }
}

fn process_updates(
    repo: &Path,
    updates: &[Value],
    allowed_chat: &str,
    dry_run: bool,
    centinela: &mut DaemonRuntime,
) -> Result<i64, String> {
    let mut max_id = load_last_update_id(repo);
    let mut seen = load_seen(repo);

    // ACK PRIMERO: avanzar offset antes de side-effects (Telegram no reentrega).
    for upd in updates {
        if let Some(uid) = upd.get("update_id").and_then(|v| v.as_i64()) {
            max_id = max_id.max(uid);
        }
    }
    if !updates.is_empty() && max_id > 0 {
        save_last_update_id(repo, max_id)?;
    }

    for upd in updates {
        let Some(uid) = upd.get("update_id").and_then(|v| v.as_i64()) else {
            continue;
        };
        if !seen.insert(uid) {
            continue;
        }
        if chat_id(upd).as_deref() != Some(allowed_chat) {
            continue;
        }
        let Some(text) = extract_text(upd) else {
            continue;
        };
        centinela.note_stimulus();
        let rc = invoke_gateway(repo, &text, dry_run);
        if rc != 0 {
            eprintln!("[telegram-watcher] gateway rc={rc} update_id={uid}");
        }
    }
    save_seen(repo, &seen)?;
    Ok(max_id)
}

fn run_once(
    repo: &Path,
    top: &sddia_daemon_runtime::BusTopology,
    dry_run: bool,
    centinela: &mut DaemonRuntime,
) -> Result<(), i32> {
    let (token, allowed) = require_env()?;
    let last = load_last_update_id(repo);
    let offset = if last > 0 { last + 1 } else { 0 };
    let updates = get_updates(&token, offset);
    if let Err(e) = process_updates(repo, &updates, &allowed, dry_run, centinela) {
        eprintln!("[telegram-watcher] {e}");
    }
    let _ = centinela.tick(top);
    Ok(())
}

fn run_loop(repo: PathBuf, dry_run: bool) -> Result<(), i32> {
    let (token, _) = require_env()?;
    delete_webhook(&token);
    let top = load_bus_topology(&repo);
    let mut centinela = DaemonRuntime::new(repo.clone(), "telegram-watcher");
    centinela.bootstrap(&top).map_err(|e| {
        eprintln!("[telegram-watcher] {e}");
        1
    })?;
    let shared = Arc::new(Mutex::new(centinela));
    let _hb = spawn_heartbeat_worker(Arc::clone(&shared), top.clone());
    println!(
        "[telegram-watcher] bucle iniciado (keepalive heartbeat cada {HEARTBEAT_TICK_SECONDS}s)"
    );
    loop {
        let mut centinela = shared.lock().map_err(|_| {
            eprintln!("[telegram-watcher] lock poisoned");
            1
        })?;
        run_once(&repo, &top, dry_run, &mut centinela)?;
        drop(centinela);
        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let once = args.iter().any(|a| a == "--once");
    let dry_run = args.iter().any(|a| a == "--dry-run");

    let repo = match find_repo_root() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };

    if once {
        let top = load_bus_topology(&repo);
        let mut centinela = DaemonRuntime::new(repo.clone(), "telegram-watcher");
        if let Err(e) = centinela.bootstrap(&top) {
            eprintln!("[telegram-watcher] {e}");
            std::process::exit(1);
        }
        if let Err(code) = run_once(&repo, &top, dry_run, &mut centinela) {
            centinela.shutdown();
            std::process::exit(code);
        }
        centinela.shutdown();
    } else if let Err(code) = run_loop(repo, dry_run) {
        std::process::exit(code);
    }
}
