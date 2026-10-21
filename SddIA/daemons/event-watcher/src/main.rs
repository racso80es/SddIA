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
use std::time::Instant;
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
/// Misma cadencia que el sweeper. El watcher la dispara aunque ese proceso no corra.
const HEARTBEAT_AUDIT_SWEEP_SECONDS: u64 = 30;

struct ProbeClock {
    last_audit: Instant,
    last_vitality: Instant,
    vitality_secs: u64,
}

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
    sddia_daemon_runtime::resolve_execute_process_elf(repo)
        .unwrap_or_else(|| repo.join("SddIA/target/debug/execute-process"))
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

fn parse_vitality_probe_seconds(raw: Option<&str>) -> u64 {
    match raw.map(str::trim).filter(|s| !s.is_empty()) {
        None => 300,
        Some(s) => match s.parse::<u64>() {
            Ok(n) if n >= 30 => n,
            _ => 30,
        },
    }
}

fn probe_clock_due_now() -> ProbeClock {
    let vitality_secs =
        parse_vitality_probe_seconds(env::var("SDDIA_VITALITY_PROBE_SECONDS").ok().as_deref());
    ProbeClock {
        last_audit: Instant::now() - Duration::from_secs(HEARTBEAT_AUDIT_SWEEP_SECONDS),
        last_vitality: Instant::now() - Duration::from_secs(vitality_secs),
        vitality_secs,
    }
}

fn invoke_named_probe(repo: &Path, process: &str, inputs: &str, label: &str) {
    let runner = execute_process_bin(repo);
    match Command::new(&runner)
        .args(["--process", process, "--inputs", inputs])
        .current_dir(repo)
        .output()
    {
        Ok(out) if out.status.success() => {}
        Ok(out) => {
            let err = String::from_utf8_lossy(&out.stderr);
            let so = String::from_utf8_lossy(&out.stdout);
            eprintln!(
                "[WATCHER] {label}: {}",
                if !err.trim().is_empty() {
                    err.trim()
                } else {
                    so.trim()
                }
            );
        }
        Err(e) => eprintln!("[WATCHER] {label} spawn: {e}"),
    }
}

fn tick_probes(repo: &Path, clock: &mut ProbeClock) {
    if clock.last_audit.elapsed() >= Duration::from_secs(HEARTBEAT_AUDIT_SWEEP_SECONDS) {
        invoke_named_probe(
            repo,
            "daemon-heartbeat-audit",
            r#"{"sweep":true}"#,
            "daemon-heartbeat-audit",
        );
        clock.last_audit = Instant::now();
    }
    if clock.last_vitality.elapsed() >= Duration::from_secs(clock.vitality_secs) {
        invoke_named_probe(repo, "system-vitality-probe", "{}", "system-vitality-probe");
        clock.last_vitality = Instant::now();
    }
}

/// Ids con entrega `pending`. No lee el ECST. Un fichero pending ya presente lo deja al barrido.
fn claim_store_pending(repo: &Path, top: &BusTopology, book: &SharedBook) {
    if !sddia_event_store::writes_store(sddia_event_store::store_mode()) {
        return;
    }
    let db = sddia_event_store::runtime_db_path(repo);
    if !db.is_file() {
        return;
    }
    let Ok(store) = sddia_event_store::EventStore::open(&db) else {
        return;
    };
    let Ok(rows) = store.list_pending_deliveries() else {
        return;
    };
    let mut seen = HashSet::new();
    for (event_id, _subscriber_id) in rows {
        if !seen.insert(event_id.clone()) {
            continue;
        }
        let path = top.pending.join(format!("{event_id}.json"));
        if path.is_file() {
            continue;
        }
        let rel = format!(".events/pending/{event_id}.json");
        let key = rel.clone();
        {
            let Ok(mut b) = book.lock() else {
                return;
            };
            if b.processing.contains(&event_id) || b.routed_ok.contains(&event_id) {
                continue;
            }
            if b.in_flight() >= MAX_IN_FLIGHT_ROUTES {
                return;
            }
            b.processing.insert(event_id.clone());
            *b.attempts.entry(key.clone()).or_insert(0) += 1;
        }
        let repo_c = repo.to_path_buf();
        let book_c = Arc::clone(book);
        let uuid_c = event_id;
        let path_c = path;
        thread::spawn(move || {
            let out = invoke_route_process(&repo_c, &rel, "route-domain-event");
            if let Ok(mut b) = book_c.lock() {
                apply_route_outcome_fractal(
                    &mut b,
                    "route-domain-event",
                    &path_c,
                    &uuid_c,
                    &key,
                    &out,
                );
            }
        });
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

/// Poll-loop skip never emits console I/O (KA-DISK-1/2/3). One-shot traces live in apply_route_outcome_*.
fn watcher_skip_emits_hot_path_trace(_skip: &str) -> bool {
    false
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
    let mut probes = probe_clock_due_now();
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
        tick_probes(&repo, &mut probes);
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
            if process_name == "route-domain-event"
                && sddia_event_store::store_mode() == sddia_event_store::StoreMode::Redb
            {
                let mut b = book.lock().map_err(|_| "route book lock poisoned".to_string())?;
                for path in &paths {
                    let event_uuid = path
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .into_owned();
                    if b.routed_ok.contains(&event_uuid) {
                        continue;
                    }
                    let rel = rel_event_path(&repo, path);
                    let event = json!({
                        "event_id": format!("residual-{event_uuid}"),
                        "event_type": "System_Fracture_Detected",
                        "emitter_agent": "event-watcher",
                        "payload": {
                            "process_name": "event-watcher",
                            "friction_id": "F-EDA-RESIDUAL-FILESYSTEM",
                            "error_trace": format!("residual {rel} con store redb; no reinyectado"),
                            "attempted_action": "route-domain-event",
                        },
                        "delivery_state": {},
                    });
                    let _ = sddia_event_store::record_ecst(&repo, "domain", &event);
                    b.routed_ok.insert(event_uuid);
                    eprintln!("[WATCHER] residual {rel}: fractura F-EDA-RESIDUAL-FILESYSTEM");
                }
                continue;
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
                            let _ = watcher_skip_emits_hot_path_trace(&skip);
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
                        let _ = watcher_skip_emits_hot_path_trace(&skip);
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
        claim_store_pending(&repo, &top, &book);
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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_tree() -> (PathBuf, sddia_daemon_runtime::BusTopology) {
        let root = std::env::temp_dir().join(format!(
            "sddia-ew-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let top = sddia_daemon_runtime::BusTopology {
            pending: root.join("pending"),
            dead_letter: root.join("dead-letter"),
            dead_letter_subscribers: root.join("dead-letter").join("subscribers"),
            telemetry: root.join("telemetry"),
            progress: root.join("progress"),
            orchestration: root.join("orchestration"),
            domain: root.join("domain"),
        };
        for d in [
            &top.pending,
            &top.dead_letter,
            &top.dead_letter_subscribers,
            &top.telemetry,
            &top.orchestration,
            &top.domain,
        ] {
            fs::create_dir_all(d).unwrap();
        }
        (root, top)
    }

    fn write_json(path: &Path, body: &str) {
        fs::write(path, body).unwrap();
    }

    #[test]
    fn skip_hot_path_never_emits_trace() {
        for skip in [
            "in-flight uuid=x",
            "routed-ok pending file uuid=x",
            "fractal-terminal uuid=x",
            "max attempts (3)",
            "max attempts + side-effect committed (3)",
            "dead-letter kaizen",
        ] {
            assert!(
                !watcher_skip_emits_hot_path_trace(skip),
                "hot path must stay silent for {skip}"
            );
        }
    }

    #[test]
    fn skip_in_flight() {
        let (root, top) = test_tree();
        let path = top.pending.join("u-if.json");
        write_json(&path, "{}");
        let mut book = RouteBook::new();
        book.processing.insert("u-if".into());
        let skip = watcher_skip_reason(
            "u-if",
            "pending/u-if.json",
            "route-domain-event",
            &path,
            &book,
            &root,
            &top,
        )
        .expect("skip");
        assert!(skip.starts_with("in-flight"));
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn skip_routed_ok_keeps_book_and_file() {
        let (root, top) = test_tree();
        let path = top.pending.join("u-ok.json");
        write_json(&path, "{}");
        let mut book = RouteBook::new();
        book.routed_ok.insert("u-ok".into());
        let skip = watcher_skip_reason(
            "u-ok",
            "pending/u-ok.json",
            "route-domain-event",
            &path,
            &book,
            &root,
            &top,
        )
        .expect("skip");
        assert!(skip.starts_with("routed-ok"));
        assert!(book.routed_ok.contains("u-ok"));
        assert!(path.is_file());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn skip_fractal_terminal() {
        let (root, top) = test_tree();
        let path = top.telemetry.join("u-ft.json");
        write_json(
            &path,
            r#"{"delivery_state":{"a":"success","b":"skipped-already-delivered"}}"#,
        );
        let book = RouteBook::new();
        let skip = watcher_skip_reason(
            "u-ft",
            "telemetry/u-ft.json",
            "route-telemetry",
            &path,
            &book,
            &root,
            &top,
        )
        .expect("skip");
        assert!(skip.starts_with("fractal-terminal"));
        let none = watcher_skip_reason(
            "u-ft",
            "telemetry/u-ft.json",
            "route-domain-event",
            &path,
            &book,
            &root,
            &top,
        );
        assert!(none.is_none(), "batch path must not classify fractal-terminal");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn skip_max_attempts() {
        let (root, top) = test_tree();
        let path = top.pending.join("u-max.json");
        write_json(&path, "{}");
        let mut book = RouteBook::new();
        book.attempts
            .insert("pending/u-max.json".into(), MAX_ROUTE_ATTEMPTS);
        let skip = watcher_skip_reason(
            "u-max",
            "pending/u-max.json",
            "route-domain-event",
            &path,
            &book,
            &root,
            &top,
        )
        .expect("skip");
        assert!(skip.starts_with("max attempts"));
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn skip_dead_letter_kaizen() {
        let (root, top) = test_tree();
        let path = top.pending.join("u-dl.json");
        write_json(&path, "{}");
        write_json(&top.dead_letter_subscribers.join("u-dl.witness.json"), "{}");
        let book = RouteBook::new();
        let skip = watcher_skip_reason(
            "u-dl",
            "pending/u-dl.json",
            "route-domain-event",
            &path,
            &book,
            &root,
            &top,
        )
        .expect("skip");
        assert_eq!(skip, "dead-letter kaizen");
        let _ = fs::remove_dir_all(&root);
    }

    static WATCHER_ENV: Mutex<()> = Mutex::new(());

    struct EnvGuard {
        key: &'static str,
        prev: Option<String>,
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            match &self.prev {
                Some(value) => env::set_var(self.key, value),
                None => env::remove_var(self.key),
            }
        }
    }

    fn push_env(key: &'static str, value: &str) -> EnvGuard {
        let prev = env::var(key).ok();
        env::set_var(key, value);
        EnvGuard { key, prev }
    }

    fn mock_bin(dir: &Path) -> PathBuf {
        let path = dir.join("mock-execute.sh");
        fs::write(
            &path,
            "#!/bin/sh\nprintf '%s\\n' \"$@\" >> \"$WATCHER_LOG\"\nprintf '\\n' >> \"$WATCHER_LOG\"\nexit 0\n",
        )
        .unwrap();
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
        path
    }

    #[test]
    fn filesystem_pending_is_routed_without_opening_the_store() {
        let _lock = WATCHER_ENV.lock().unwrap();
        let root = std::env::temp_dir().join(format!(
            "sddia-ew-fs-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(root.join(".events/pending")).unwrap();
        let body = r#"{"event_id":"ca06","secret":"ecst-fichero-no-va-en-argv"}"#;
        fs::write(root.join(".events/pending/ca06.json"), body).unwrap();
        let log = root.join("argv.log");
        let bin = mock_bin(&root);
        let _bin = push_env("SDDIA_EXECUTE_PROCESS_BIN", bin.to_str().unwrap());
        let _log = push_env("WATCHER_LOG", log.to_str().unwrap());
        sddia_event_store::override_store_mode(Some(sddia_event_store::StoreMode::Filesystem));
        run_watcher(root.clone(), true).expect("tick");
        sddia_event_store::override_store_mode(None);
        let logged = fs::read_to_string(&log).unwrap_or_default();
        assert!(
            logged.contains(".events/pending/ca06.json"),
            "argv={logged}"
        );
        assert!(!logged.contains("ecst-fichero-no-va-en-argv"));
        assert!(!root.join(".SddIA/runtime/events.redb").is_file());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn redb_claim_delegates_the_id_without_reading_the_ecst() {
        let _lock = WATCHER_ENV.lock().unwrap();
        let root = std::env::temp_dir().join(format!(
            "sddia-ew-db-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(root.join(".SddIA/runtime")).unwrap();
        let event_id = "ac4-claim";
        let secret = "ecst-secreto-no-leido";
        sddia_event_store::override_store_mode(Some(sddia_event_store::StoreMode::Redb));
        let store = sddia_event_store::EventStore::open(sddia_event_store::runtime_db_path(&root))
            .expect("open");
        store
            .insert_event(
                &sddia_event_store::StoredEvent {
                    event_id: event_id.into(),
                    family: sddia_event_store::Family::Domain,
                    event_type: "Domain_Probe".into(),
                    created_at: 1,
                    ecst: serde_json::json!({
                        "event_id": event_id,
                        "event_type": "Domain_Probe",
                        "payload": { "secret": secret }
                    }),
                },
                &["agent:argos"],
            )
            .expect("insert");
        drop(store);
        let log = root.join("argv.log");
        let bin = mock_bin(&root);
        let _bin = push_env("SDDIA_EXECUTE_PROCESS_BIN", bin.to_str().unwrap());
        let _log = push_env("WATCHER_LOG", log.to_str().unwrap());
        run_watcher(root.clone(), true).expect("tick");
        sddia_event_store::override_store_mode(None);
        let logged = fs::read_to_string(&log).unwrap_or_default();
        assert!(
            logged.contains(&format!(".events/pending/{event_id}.json")),
            "argv={logged}"
        );
        assert!(!logged.contains(secret), "el watcher leyó el ECST: {logged}");
        assert!(!root.join(".events/pending").join(format!("{event_id}.json")).is_file());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn residual_pending_json_is_a_fracture_and_is_not_routed() {
        let _lock = WATCHER_ENV.lock().unwrap();
        let root = std::env::temp_dir().join(format!(
            "sddia-ew-residual-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(root.join(".events/pending")).unwrap();
        fs::write(
            root.join(".events/pending/ca12-residual.json"),
            r#"{"event_id":"ca12-residual","secret":"no-reinject"}"#,
        )
        .unwrap();
        let log = root.join("argv.log");
        let bin = mock_bin(&root);
        let _bin = push_env("SDDIA_EXECUTE_PROCESS_BIN", bin.to_str().unwrap());
        let _log = push_env("WATCHER_LOG", log.to_str().unwrap());
        sddia_event_store::override_store_mode(Some(sddia_event_store::StoreMode::Redb));
        run_watcher(root.clone(), true).expect("tick");
        sddia_event_store::override_store_mode(None);
        let logged = fs::read_to_string(&log).unwrap_or_default();
        assert!(
            !logged.contains("ca12-residual.json"),
            "reinyectó el residual: {logged}"
        );
        assert!(root.join(".events/pending/ca12-residual.json").is_file());
        let store = sddia_event_store::EventStore::open(sddia_event_store::runtime_db_path(&root))
            .expect("open");
        let rows = store
            .list_family(sddia_event_store::Family::Domain)
            .expect("family");
        assert!(rows.iter().any(|row| {
            row.ecst.get("event_type").and_then(|v| v.as_str()) == Some("System_Fracture_Detected")
                && row.ecst.get("payload").and_then(|p| p.get("friction_id")).and_then(|v| v.as_str())
                    == Some("F-EDA-RESIDUAL-FILESYSTEM")
        }));
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn one_tick_fires_both_probes_without_the_sweeper() {
        let _lock = WATCHER_ENV.lock().unwrap();
        let root = std::env::temp_dir().join(format!(
            "sddia-ew-pr-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        let log = root.join("argv.log");
        let bin = mock_bin(&root);
        let _bin = push_env("SDDIA_EXECUTE_PROCESS_BIN", bin.to_str().unwrap());
        let _log = push_env("WATCHER_LOG", log.to_str().unwrap());
        let mut clock = probe_clock_due_now();
        tick_probes(&root, &mut clock);
        let logged = fs::read_to_string(&log).unwrap();
        assert!(logged.contains("daemon-heartbeat-audit"));
        assert!(logged.contains(r#"{"sweep":true}"#));
        assert!(logged.contains("system-vitality-probe"));
        assert!(logged.contains("{}"));
        assert!(!logged.contains("event-sweeper"));
        let _ = fs::remove_dir_all(&root);
    }
}
