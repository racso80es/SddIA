use sddia_daemon_runtime::{
    ensure_bus_dirs, find_repo_root, load_bus_topology, BusTopology, DaemonRuntime,
};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};

const POLL_SECONDS: u64 = 2;
const MAX_ROUTE_ATTEMPTS: u32 = 3;
const HEARTBEAT_TICK_SECONDS: u64 = 10;
/// Crash-Only (Vía D): fallos consecutivos de emisión side-channel.
const HEARTBEAT_EMIT_FAIL_BUDGET: u32 = 5;
/// Chunk físico (Dogma del Despertador): tope de paths por invocación al orquestador.
const PHYSICAL_ROUTE_CHUNK: usize = 50;

fn spawn_heartbeat_worker(
    centinela: Arc<Mutex<DaemonRuntime>>,
    top: BusTopology,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut fails = 0u32;
        loop {
            let tick_result = {
                if let Ok(mut rt) = centinela.lock() {
                    rt.tick(&top)
                } else {
                    Err("lock poisoned".into())
                }
            };
            match tick_result {
                Ok(()) => fails = 0,
                Err(e) => {
                    fails += 1;
                    eprintln!("[WATCHER] heartbeat keepalive: {e} (fail {fails}/{HEARTBEAT_EMIT_FAIL_BUDGET})");
                    if fails >= HEARTBEAT_EMIT_FAIL_BUDGET {
                        panic!(
                            "Fallo crítico termodinámico: Incapacidad de reportar telemetría (side-channel). Abortando entidad para evitar estado Zombi. last_error={e}"
                        );
                    }
                }
            }
            thread::sleep(Duration::from_secs(HEARTBEAT_TICK_SECONDS));
        }
    })
}

fn execute_process_bin(repo: &Path) -> PathBuf {
    if let Ok(p) = env::var("SDDIA_EXECUTE_PROCESS_BIN") {
        if !p.trim().is_empty() {
            return PathBuf::from(p);
        }
    }
    for rel in ["SddIA/target/debug/execute-process", "SddIA/target/release/execute-process"] {
        let candidate = repo.join(rel);
        if candidate.is_file() {
            return candidate;
        }
    }
    repo.join("SddIA/target/debug/execute-process")
}

fn rel_event_path(repo: &Path, event_path: &Path) -> String {
    event_path
        .canonicalize()
        .ok()
        .and_then(|p| p.strip_prefix(repo.canonicalize().ok()?).ok().map(|r| r.to_string_lossy().replace('\\', "/")))
        .unwrap_or_else(|| event_path.to_string_lossy().replace('\\', "/"))
}

fn fractal_watch_enabled() -> bool {
    match env::var("SDDIA_LAB_WATCH_FRACTAL").unwrap_or_else(|_| "1".into()) {
        ref s if matches!(s.to_ascii_lowercase().as_str(), "0" | "false" | "no") => false,
        _ => true,
    }
}

fn watch_targets(repo: &Path, top: &sddia_daemon_runtime::BusTopology) -> Vec<(PathBuf, String)> {
    // Vía B: telemetría (Daemon_Heartbeat) antes que pending/dominio/orquestación.
    let mut targets = Vec::new();
    if fractal_watch_enabled() {
        targets.push((top.telemetry.clone(), "route-telemetry".into()));
    }
    targets.push((top.pending.clone(), "route-domain-event".into()));
    if fractal_watch_enabled() {
        targets.push((top.domain.clone(), "route-domain".into()));
        targets.push((top.orchestration.clone(), "route-orchestration".into()));
    }
    let _ = repo;
    targets
}

fn is_daemon_heartbeat(path: &Path) -> bool {
    let Ok(raw) = fs::read_to_string(path) else {
        return false;
    };
    let Ok(body) = serde_json::from_str::<Value>(&raw) else {
        return false;
    };
    body.get("event_type").and_then(|v| v.as_str()) == Some("Daemon_Heartbeat")
}

/// Dentro de telemetry: HB primero (fairness bajo saturación).
fn prioritize_heartbeat_paths(paths: &mut Vec<PathBuf>) {
    paths.sort_by_key(|p| {
        let hb = is_daemon_heartbeat(p);
        // false < true en sort_by_key invertido: HB (0) antes que no-HB (1)
        (!hb, p.to_string_lossy().to_string())
    });
}

fn has_dead_letter_witnesses(
    repo: &Path,
    top: &sddia_daemon_runtime::BusTopology,
    event_uuid: &str,
) -> bool {
    let folder = &top.dead_letter_subscribers;
    if !folder.is_dir() {
        return false;
    }
    let Ok(entries) = fs::read_dir(folder) else {
        return false;
    };
    let prefix = format!("{event_uuid}.");
    entries.filter_map(|e| e.ok()).any(|e| {
        e.file_name()
            .to_string_lossy()
            .starts_with(&prefix)
    }) || {
        // fallback: glob pattern uuid.*.json under dead_letter if subscribers nested differently
        let dl_root = repo.join(".events/dead-letter");
        dl_root.join("subscribers").join(format!("{event_uuid}.json")).is_file()
    }
}

fn invoke_route_process(repo: &Path, rel_path: &str, process_name: &str) -> Output {
    let runner = execute_process_bin(repo);
    let payload = json!({ "event_file_path": rel_path }).to_string();
    Command::new(&runner)
        .args(["--process", process_name, "--inputs", &payload])
        .current_dir(repo)
        .output()
        .unwrap_or_else(|e| {
            eprintln!("[WATCHER] spawn execute-process: {e}");
            Command::new("false").output().expect("false")
        })
}

fn invoke_route_process_batch(repo: &Path, rel_paths: &[String], process_name: &str) -> Output {
    let runner = execute_process_bin(repo);
    let payload = json!({ "event_file_paths": rel_paths }).to_string();
    Command::new(&runner)
        .args(["--process", process_name, "--inputs", &payload])
        .current_dir(repo)
        .output()
        .unwrap_or_else(|e| {
            eprintln!("[WATCHER] spawn execute-process batch: {e}");
            Command::new("false").output().expect("false")
        })
}

fn extract_sweep(stdout: &str) -> Option<Value> {
    let line = stdout.trim().lines().last()?;
    let body: Value = serde_json::from_str(line).ok()?;
    body.get("data")
        .and_then(|d| d.get("sweep"))
        .cloned()
}

fn log_route_outcome(
    repo: &Path,
    top: &sddia_daemon_runtime::BusTopology,
    key: &str,
    event_uuid: &str,
    out: &Output,
) {
    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr);
        let so = String::from_utf8_lossy(&out.stdout);
        eprintln!(
            "[WATCHER] route-domain-event falló ({key}): {}",
            if !err.trim().is_empty() { err.trim() } else { so.trim() }
        );
        return;
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    let sweep = extract_sweep(&stdout);
    let pending_path = top.pending.join(key);
    if sweep.as_ref().and_then(|s| s.get("status")).and_then(|v| v.as_str()) == Some("kaizen-finalized") {
        println!("[WATCHER] {key}: Kaizen terminalizado — padre retirado de pending");
        return;
    }
    if has_dead_letter_witnesses(repo, top, event_uuid) {
        println!("[WATCHER] {key}: testigo dead-letter — padre permanece en pending (Kaizen)");
        return;
    }
    if sweep.as_ref().and_then(|s| s.get("status")).and_then(|v| v.as_str()) == Some("purged")
        || !pending_path.is_file()
    {
        println!("[WATCHER] {key}: enrutado y purgado de pending");
    } else if sweep.as_ref().and_then(|s| s.get("status")).and_then(|v| v.as_str()) == Some("awaiting") {
        let subs = sweep
            .as_ref()
            .and_then(|s| s.get("pending_subscribers"))
            .cloned()
            .unwrap_or(json!([]));
        println!("[WATCHER] {key}: enrutado — suscriptores pendientes: {subs}");
    } else {
        println!("[WATCHER] {key}: enrutado — consenso pendiente (sweeper)");
    }
}

fn prune_routed_ok(
    routed_ok: &mut HashSet<String>,
    targets: &[(PathBuf, String)],
) {
    let mut still = HashSet::new();
    for (dir, _) in targets {
        if let Ok(entries) = fs::read_dir(dir) {
            for e in entries.flatten() {
                if e.path().extension().and_then(|x| x.to_str()) == Some("json") {
                    still.insert(e.path().file_stem().unwrap_or_default().to_string_lossy().into_owned());
                }
            }
        }
    }
    routed_ok.retain(|u| still.contains(u));
}

fn fractal_side_effect_committed(path: &Path) -> bool {
    let Ok(raw) = fs::read_to_string(path) else {
        return false;
    };
    let Ok(body) = serde_json::from_str::<Value>(&raw) else {
        return false;
    };
    let Some(ds) = body.get("delivery_state").and_then(|v| v.as_object()) else {
        return false;
    };
    // Eco Telegram: si el fallback ya entregó, no re-despachar el archivo completo
    // mientras queden fallos no-terminales (el router salta skipped-already-delivered).
    ds.iter().any(|(k, v)| {
        let st = v.as_str().unwrap_or("");
        (k.contains("telegram-fallback") || k.contains("send-telegram"))
            && (st == "success" || st == "skipped-already-delivered" || st.starts_with("skipped"))
    })
}

fn fractal_fully_terminal(path: &Path) -> bool {
    let Ok(raw) = fs::read_to_string(path) else {
        return false;
    };
    let Ok(body) = serde_json::from_str::<Value>(&raw) else {
        return false;
    };
    let Some(ds) = body.get("delivery_state").and_then(|v| v.as_object()) else {
        return false;
    };
    if ds.is_empty() {
        return false;
    }
    // Terminal = success|skipped*|failed. Con failed el sweeper/router mueven a DLQ (C2).
    ds.values().all(|v| {
        let st = v.as_str().unwrap_or("");
        st == "success"
            || st == "failed"
            || st == "skipped-already-delivered"
            || st.starts_with("skipped")
    })
}

fn watcher_skip_reason(
    event_uuid: &str,
    key: &str,
    process_name: &str,
    path: &Path,
    processing: &HashSet<String>,
    routed_ok: &HashSet<String>,
    repo: &Path,
    top: &sddia_daemon_runtime::BusTopology,
    attempts: &HashMap<String, u32>,
) -> Option<String> {
    if processing.contains(event_uuid) {
        return Some(format!("in-flight uuid={event_uuid}"));
    }
    if routed_ok.contains(event_uuid) && path.is_file() {
        return Some(format!("routed-ok pending file uuid={event_uuid}"));
    }
    if process_name != "route-domain-event" && fractal_fully_terminal(path) {
        return Some(format!("fractal-terminal uuid={event_uuid}"));
    }
    if process_name == "route-domain-event" && has_dead_letter_witnesses(repo, top, event_uuid) {
        return Some("dead-letter kaizen".into());
    }
    if attempts.get(key).copied().unwrap_or(0) >= MAX_ROUTE_ATTEMPTS {
        // Si ya hubo side-effect Telegram, no seguir reintentando eco.
        if process_name != "route-domain-event" && fractal_side_effect_committed(path) {
            return Some(format!("max attempts + side-effect committed ({MAX_ROUTE_ATTEMPTS})"));
        }
        return Some(format!("max attempts ({MAX_ROUTE_ATTEMPTS})"));
    }
    None
}

fn run_route_cli(repo: &Path, event_file_path: &str) -> i32 {
    let runner = execute_process_bin(repo);
    let payload = json!({ "event_file_path": event_file_path }).to_string();
    let out = Command::new(&runner)
        .args(["--process", "route-domain-event", "--inputs", &payload])
        .current_dir(repo)
        .output();
    match out {
        Ok(o) => {
            print!("{}", String::from_utf8_lossy(&o.stdout));
            if o.status.success() { 0 } else { 1 }
        }
        Err(e) => {
            eprintln!("{e}");
            1
        }
    }
}

fn run_watcher(repo: PathBuf, once: bool) -> Result<(), String> {
    let top = load_bus_topology(&repo);
    ensure_bus_dirs(&top)?;
    let mut centinela = DaemonRuntime::new(repo.clone(), "event-watcher");
    centinela.bootstrap(&top)?;
    let shared = Arc::new(Mutex::new(centinela));
    let _hb = if once {
        None
    } else {
        Some(spawn_heartbeat_worker(Arc::clone(&shared), top.clone()))
    };
    let mut attempts: HashMap<String, u32> = HashMap::new();
    let mut processing: HashSet<String> = HashSet::new();
    let mut routed_ok: HashSet<String> = HashSet::new();
    let targets = watch_targets(&repo, &top);
    if once {
        println!(
            "[WATCHER] Iniciado. roots= {:?}",
            targets.iter().map(|(p, _)| p.display().to_string()).collect::<Vec<_>>()
        );
    } else {
        println!(
            "[WATCHER] Iniciado (keepalive heartbeat cada {HEARTBEAT_TICK_SECONDS}s). roots= {:?}",
            targets.iter().map(|(p, _)| p.display().to_string()).collect::<Vec<_>>()
        );
    }
    loop {
        prune_routed_ok(&mut routed_ok, &targets);
        for (watch_dir, process_name) in &targets {
            if !watch_dir.is_dir() {
                continue;
            }
            let Ok(entries) = fs::read_dir(watch_dir) else {
                continue;
            };
            let mut paths: Vec<PathBuf> = entries
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("json"))
                .collect();
            if process_name == "route-telemetry" {
                prioritize_heartbeat_paths(&mut paths);
            } else {
                paths.sort();
            }
            if process_name == "route-domain-event" {
                let mut eligible: Vec<(PathBuf, String, String, String)> = Vec::new();
                for path in paths {
                    let file_name = path.file_name().unwrap_or_default().to_string_lossy().into_owned();
                    let key = format!(
                        "{}/{}",
                        watch_dir.file_name().unwrap_or_default().to_string_lossy(),
                        file_name
                    );
                    let event_uuid = path
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .into_owned();
                    if let Some(skip) = watcher_skip_reason(
                        &event_uuid,
                        &key,
                        process_name,
                        &path,
                        &processing,
                        &routed_ok,
                        &repo,
                        &top,
                        &attempts,
                    ) {
                        if skip.starts_with("in-flight")
                            || skip.starts_with("routed-ok")
                            || skip.starts_with("fractal-terminal")
                        {
                            println!("[WATCHER] skip {skip}");
                        } else if skip.starts_with("max attempts") {
                            println!("[WATCHER] Skip {key}: {skip}");
                        }
                        continue;
                    }
                    let rel = rel_event_path(&repo, &path);
                    eligible.push((path, event_uuid, key, rel));
                }
                for chunk in eligible.chunks(PHYSICAL_ROUTE_CHUNK) {
                    if chunk.is_empty() {
                        continue;
                    }
                    let rels: Vec<String> = chunk.iter().map(|(_, _, _, r)| r.clone()).collect();
                    println!(
                        "[WATCHER] Lote físico {} eventos → route-domain-event (semantic batch in-engine)",
                        rels.len()
                    );
                    if let Ok(mut rt) = shared.lock() {
                        rt.note_stimulus();
                    }
                    for (_, event_uuid, key, _) in chunk {
                        processing.insert(event_uuid.clone());
                        *attempts.entry(key.clone()).or_insert(0) += 1;
                    }
                    let out = invoke_route_process_batch(&repo, &rels, process_name);
                    for (path, event_uuid, key, _) in chunk {
                        processing.remove(event_uuid);
                        let file_name = path.file_name().unwrap_or_default().to_string_lossy();
                        if out.status.success() {
                            if path.is_file() {
                                routed_ok.insert(event_uuid.clone());
                            } else {
                                routed_ok.remove(event_uuid);
                                attempts.remove(key);
                            }
                            if !has_dead_letter_witnesses(&repo, &top, event_uuid) && !path.is_file()
                            {
                                attempts.remove(key);
                            }
                            log_route_outcome(&repo, &top, &file_name, event_uuid, &out);
                        } else {
                            routed_ok.remove(event_uuid);
                            log_route_outcome(&repo, &top, &file_name, event_uuid, &out);
                        }
                    }
                }
                continue;
            }
            for path in paths {
                let file_name = path.file_name().unwrap_or_default().to_string_lossy();
                let key = format!("{}/{}", watch_dir.file_name().unwrap_or_default().to_string_lossy(), file_name);
                let event_uuid = path.file_stem().unwrap_or_default().to_string_lossy().into_owned();
                if let Some(skip) = watcher_skip_reason(
                    &event_uuid,
                    &key,
                    process_name,
                    &path,
                    &processing,
                    &routed_ok,
                    &repo,
                    &top,
                    &attempts,
                ) {
                    if skip.starts_with("in-flight")
                        || skip.starts_with("routed-ok")
                        || skip.starts_with("fractal-terminal")
                    {
                        println!("[WATCHER] skip {skip}");
                    } else if skip.starts_with("max attempts") {
                        println!("[WATCHER] Skip {key}: {skip}");
                    }
                    continue;
                }
                let rel = rel_event_path(&repo, &path);
                println!("[WATCHER] Detectado nuevo evento: {key} → {process_name}");
                if let Ok(mut rt) = shared.lock() {
                    rt.note_stimulus();
                }
                processing.insert(event_uuid.clone());
                *attempts.entry(key.clone()).or_insert(0) += 1;
                let out = invoke_route_process(&repo, &rel, process_name);
                processing.remove(&event_uuid);
                if out.status.success() {
                    if path.is_file() {
                        routed_ok.insert(event_uuid.clone());
                    } else {
                        routed_ok.remove(&event_uuid);
                        attempts.remove(&key);
                    }
                } else if !(process_name != "route-domain-event"
                    && path.is_file()
                    && fractal_side_effect_committed(&path))
                {
                    routed_ok.remove(&event_uuid);
                }
                if out.status.success() {
                    if !path.is_file() {
                        attempts.remove(&key);
                    }
                    let note = if path.is_file() {
                        " (archivo persiste — consenso incompleto)"
                    } else {
                        " (purgado)"
                    };
                    println!("[WATCHER] {key}: enrutado ({process_name}){note}");
                } else {
                    let err = String::from_utf8_lossy(&out.stderr);
                    let so = String::from_utf8_lossy(&out.stdout);
                    eprintln!(
                        "[WATCHER] {process_name} falló ({key}): {}",
                        if !err.trim().is_empty() {
                            err.trim()
                        } else {
                            so.trim()
                        }
                    );
                }
            }
        }
        {
            let mut rt = shared
                .lock()
                .map_err(|_| "lock poisoned".to_string())?;
            rt.tick(&top)?;
        }
        if once {
            println!("[WATCHER] Ciclo único (--once). Fin.");
            break;
        }
        thread::sleep(Duration::from_secs(POLL_SECONDS));
    }
    if let Ok(mut rt) = shared.lock() {
        rt.shutdown();
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let repo = match find_repo_root() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };
    if let Some(idx) = args.iter().position(|a| a == "--event-file-path") {
        let path = args.get(idx + 1).map(|s| s.as_str()).unwrap_or("");
        std::process::exit(run_route_cli(&repo, path));
    }
    let once = args.iter().any(|a| a == "--once");
    if let Err(e) = run_watcher(repo, once) {
        eprintln!("[WATCHER] {e}");
        std::process::exit(1);
    }
}
