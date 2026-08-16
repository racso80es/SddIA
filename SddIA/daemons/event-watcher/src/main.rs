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
/// Tope de rutas en vuelo (evita inanición: pending largo no bloquea domain/telemetry).
const MAX_IN_FLIGHT_ROUTES: usize = 16;

struct RouteBook {
    processing: HashSet<String>,
    routed_ok: HashSet<String>,
    attempts: HashMap<String, u32>,
}

impl RouteBook {
    fn new() -> Self {
        Self {
            processing: HashSet::new(),
            routed_ok: HashSet::new(),
            attempts: HashMap::new(),
        }
    }

    fn in_flight(&self) -> usize {
        self.processing.len()
    }
}

type SharedBook = Arc<Mutex<RouteBook>>;

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
        .env("SDDIA_CLI_FOREGROUND", "1")
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
        .env("SDDIA_CLI_FOREGROUND", "1")
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

fn prune_routed_ok(book: &mut RouteBook, targets: &[(PathBuf, String)]) {
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
    book.routed_ok.retain(|u| still.contains(u));
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
    book: &RouteBook,
    repo: &Path,
    top: &sddia_daemon_runtime::BusTopology,
) -> Option<String> {
    if book.processing.contains(event_uuid) {
        return Some(format!("in-flight uuid={event_uuid}"));
    }
    if book.routed_ok.contains(event_uuid) && path.is_file() {
        return Some(format!("routed-ok pending file uuid={event_uuid}"));
    }
    if process_name != "route-domain-event" && fractal_fully_terminal(path) {
        return Some(format!("fractal-terminal uuid={event_uuid}"));
    }
    if process_name == "route-domain-event" && has_dead_letter_witnesses(repo, top, event_uuid) {
        return Some("dead-letter kaizen".into());
    }
    if book.attempts.get(key).copied().unwrap_or(0) >= MAX_ROUTE_ATTEMPTS {
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
        .env("SDDIA_CLI_FOREGROUND", "1")
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

fn apply_route_outcome_pending(
    book: &mut RouteBook,
    repo: &Path,
    top: &sddia_daemon_runtime::BusTopology,
    path: &Path,
    event_uuid: &str,
    key: &str,
    out: &Output,
) {
    book.processing.remove(event_uuid);
    let file_name = path.file_name().unwrap_or_default().to_string_lossy();
    if out.status.success() {
        if path.is_file() {
            book.routed_ok.insert(event_uuid.to_string());
        } else {
            book.routed_ok.remove(event_uuid);
            book.attempts.remove(key);
        }
        if !has_dead_letter_witnesses(repo, top, event_uuid) && !path.is_file() {
            book.attempts.remove(key);
        }
        log_route_outcome(repo, top, &file_name, event_uuid, out);
    } else {
        book.routed_ok.remove(event_uuid);
        log_route_outcome(repo, top, &file_name, event_uuid, out);
    }
}

fn apply_route_outcome_fractal(
    book: &mut RouteBook,
    process_name: &str,
    path: &Path,
    event_uuid: &str,
    key: &str,
    out: &Output,
) {
    book.processing.remove(event_uuid);
    if out.status.success() {
        if path.is_file() {
            book.routed_ok.insert(event_uuid.to_string());
        } else {
            book.routed_ok.remove(event_uuid);
            book.attempts.remove(key);
        }
    } else if !(process_name != "route-domain-event"
        && path.is_file()
        && fractal_side_effect_committed(path))
    {
        book.routed_ok.remove(event_uuid);
    }
    if out.status.success() {
        if !path.is_file() {
            book.attempts.remove(key);
        }
        let note = if path.is_file() {
            " (archivo persiste — consenso incompleto)"
        } else {
            " (purgado)"
        };
        let file_name = path.file_name().unwrap_or_default().to_string_lossy();
        println!(
            "[WATCHER] {file_name}: enrutado ({process_name}){note}"
        );
    } else {
        let err = String::from_utf8_lossy(&out.stderr);
        let so = String::from_utf8_lossy(&out.stdout);
        let file_name = path.file_name().unwrap_or_default().to_string_lossy();
        eprintln!(
            "[WATCHER] {process_name} falló ({file_name}): {}",
            if !err.trim().is_empty() {
                err.trim()
            } else {
                so.trim()
            }
        );
    }
}

fn run_watcher(repo: PathBuf, once: bool) -> Result<(), String> {
    let top = load_bus_topology(&repo);
    ensure_bus_dirs(&top)?;
    let mut centinela = DaemonRuntime::new(repo.clone(), "event-watcher");
    centinela.bootstrap(&top)?;
    let shared = Arc::new(Mutex::new(centinela));
    let book: SharedBook = Arc::new(Mutex::new(RouteBook::new()));
    let _hb = if once {
        None
    } else {
        Some(spawn_heartbeat_worker(Arc::clone(&shared), top.clone()))
    };
    let targets = watch_targets(&repo, &top);
    if once {
        println!(
            "[WATCHER] Iniciado. roots= {:?}",
            targets.iter().map(|(p, _)| p.display().to_string()).collect::<Vec<_>>()
        );
    } else {
        println!(
            "[WATCHER] Iniciado (keepalive heartbeat cada {HEARTBEAT_TICK_SECONDS}s; rutas async max={MAX_IN_FLIGHT_ROUTES}). roots= {:?}",
            targets.iter().map(|(p, _)| p.display().to_string()).collect::<Vec<_>>()
        );
    }
    loop {
        {
            let mut b = book.lock().map_err(|_| "route book lock poisoned".to_string())?;
            prune_routed_ok(&mut b, &targets);
        }
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
                {
                    let b = book.lock().map_err(|_| "route book lock poisoned".to_string())?;
                    if b.in_flight() >= MAX_IN_FLIGHT_ROUTES {
                        continue;
                    }
                    for path in paths {
                        if b.in_flight() + eligible.len() >= MAX_IN_FLIGHT_ROUTES {
                            break;
                        }
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
                            &b,
                            &repo,
                            &top,
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
                }
                for chunk in eligible.chunks(PHYSICAL_ROUTE_CHUNK) {
                    if chunk.is_empty() {
                        continue;
                    }
                    let rels: Vec<String> = chunk.iter().map(|(_, _, _, r)| r.clone()).collect();
                    let owned: Vec<(PathBuf, String, String)> = chunk
                        .iter()
                        .map(|(p, u, k, _)| (p.clone(), u.clone(), k.clone()))
                        .collect();
                    println!(
                        "[WATCHER] Lote físico {} eventos → route-domain-event async (semantic batch in-engine)",
                        rels.len()
                    );
                    if let Ok(mut rt) = shared.lock() {
                        rt.note_stimulus();
                    }
                    {
                        let mut b = book.lock().map_err(|_| "route book lock poisoned".to_string())?;
                        for (_, event_uuid, key) in &owned {
                            b.processing.insert(event_uuid.clone());
                            *b.attempts.entry(key.clone()).or_insert(0) += 1;
                        }
                    }
                    let book_c = Arc::clone(&book);
                    let repo_c = repo.clone();
                    let top_c = top.clone();
                    let pname = process_name.clone();
                    thread::spawn(move || {
                        let out = invoke_route_process_batch(&repo_c, &rels, &pname);
                        if let Ok(mut b) = book_c.lock() {
                            for (path, event_uuid, key) in &owned {
                                apply_route_outcome_pending(
                                    &mut b, &repo_c, &top_c, path, event_uuid, key, &out,
                                );
                            }
                        }
                    });
                }
                continue;
            }
            for path in paths {
                {
                    let b = book.lock().map_err(|_| "route book lock poisoned".to_string())?;
                    if b.in_flight() >= MAX_IN_FLIGHT_ROUTES {
                        break;
                    }
                }
                let file_name = path.file_name().unwrap_or_default().to_string_lossy();
                let key = format!("{}/{}", watch_dir.file_name().unwrap_or_default().to_string_lossy(), file_name);
                let event_uuid = path.file_stem().unwrap_or_default().to_string_lossy().into_owned();
                {
                    let b = book.lock().map_err(|_| "route book lock poisoned".to_string())?;
                    if let Some(skip) = watcher_skip_reason(
                        &event_uuid,
                        &key,
                        process_name,
                        &path,
                        &b,
                        &repo,
                        &top,
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
                }
                let rel = rel_event_path(&repo, &path);
                println!("[WATCHER] Detectado nuevo evento: {key} → {process_name} (async)");
                if let Ok(mut rt) = shared.lock() {
                    rt.note_stimulus();
                }
                {
                    let mut b = book.lock().map_err(|_| "route book lock poisoned".to_string())?;
                    b.processing.insert(event_uuid.clone());
                    *b.attempts.entry(key.clone()).or_insert(0) += 1;
                }
                let book_c = Arc::clone(&book);
                let repo_c = repo.clone();
                let pname = process_name.clone();
                let path_c = path.clone();
                let uuid_c = event_uuid.clone();
                let key_c = key.clone();
                thread::spawn(move || {
                    let out = invoke_route_process(&repo_c, &rel, &pname);
                    if let Ok(mut b) = book_c.lock() {
                        apply_route_outcome_fractal(
                            &mut b, &pname, &path_c, &uuid_c, &key_c, &out,
                        );
                    }
                });
            }
        }
        {
            let mut rt = shared
                .lock()
                .map_err(|_| "lock poisoned".to_string())?;
            rt.tick(&top)?;
        }
        if once {
            // Modo once: espera breve a que vuelen los spawns (lab/CI).
            let deadline = std::time::Instant::now() + Duration::from_secs(120);
            while std::time::Instant::now() < deadline {
                let inflight = book
                    .lock()
                    .map(|b| b.in_flight())
                    .unwrap_or(0);
                if inflight == 0 {
                    break;
                }
                thread::sleep(Duration::from_millis(200));
            }
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
