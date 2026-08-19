use serde::Deserialize;
use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};
use std::net::Ipv4Addr;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tiny_http::{Header, Method, Response, Server, StatusCode};
use uuid::Uuid;

#[derive(Deserialize)]
struct InteractReq {
    prompt: String,
    #[serde(default)]
    mode: Option<String>,
    #[serde(default)]
    process: Option<String>,
}

#[derive(Deserialize)]
struct SyncAssetsReq {
    asset_id: String,
    #[serde(default)]
    asset_family: Option<String>,
}

fn repo_root() -> PathBuf {
    if let Ok(p) = std::env::var("SDDIA_REPO_ROOT") {
        let trimmed = p.trim();
        if !trimmed.is_empty() {
            return PathBuf::from(trimmed);
        }
    }
    if let Ok(cwd) = std::env::current_dir() {
        for ancestor in cwd.ancestors() {
            if ancestor.join("SddIA/core/cumulo.paths.json").is_file() {
                return ancestor.to_path_buf();
            }
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        for ancestor in exe.ancestors() {
            if ancestor.join("SddIA/core/cumulo.paths.json").is_file() {
                return ancestor.to_path_buf();
            }
        }
    }
    PathBuf::from(".")
}

fn is_native_elf(path: &Path) -> bool {
    let mut magic = [0; 4];
    std::fs::File::open(path)
        .and_then(|mut file| file.read_exact(&mut magic))
        .is_ok_and(|_| magic == *b"\x7fELF")
}

fn resolve_orchestrator(repo: &Path) -> Result<PathBuf, String> {
    if let Ok(o) = std::env::var("SDDIA_EXECUTE_PROCESS_BIN") {
        let trimmed = o.trim();
        if !trimmed.is_empty() {
            let candidate = PathBuf::from(trimmed);
            return is_native_elf(&candidate)
                .then_some(candidate)
                .ok_or_else(|| "orquestador configurado no es un binario ELF nativo".into());
        }
    }
    for rel in [
        "SddIA/target/debug/execute-process",
        "SddIA/target/release/execute-process",
    ] {
        let candidate = repo.join(rel);
        if is_native_elf(&candidate) {
            return Ok(candidate);
        }
    }
    Err("orquestador nativo no encontrado en SddIA/target/{release,debug}".into())
}

fn client_timeout_secs() -> u64 {
    std::env::var("SDDIA_CLIENT_TIMEOUT_SECONDS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(120)
}

fn json_header() -> Header {
    Header::from_bytes(
        &b"Content-Type"[..],
        &b"application/json; charset=utf-8"[..],
    )
    .unwrap()
}

fn reply(req: tiny_http::Request, code: u16, body: String) {
    let _ = req.respond(
        Response::from_string(body)
            .with_status_code(code)
            .with_header(json_header()),
    );
}

fn serve_static(req: tiny_http::Request, ui_root: &Path) {
    let path = req.url().split('?').next().unwrap_or("/");
    let rel = path.trim_start_matches('/');
    let rel = if rel.is_empty() { "index.html" } else { rel };

    let target = ui_root.join(rel);
    let ui_canon = match ui_root.canonicalize() {
        Ok(p) => p,
        Err(_) => {
            reply(
                req,
                404,
                r#"{"success":false,"message":"asset no encontrado","exit_code":1}"#.into(),
            );
            return;
        }
    };

    let resolved = match target.canonicalize() {
        Ok(p) if p.starts_with(&ui_canon) && p.is_file() => p,
        _ => {
            reply(
                req,
                404,
                r#"{"success":false,"message":"asset no encontrado","exit_code":1}"#.into(),
            );
            return;
        }
    };

    let ctype = match resolved.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html",
        Some("js") => "text/javascript",
        Some("css") => "text/css",
        _ => "application/octet-stream",
    };

    let body = match std::fs::read(&resolved) {
        Ok(b) => b,
        Err(_) => {
            reply(
                req,
                404,
                r#"{"success":false,"message":"asset no encontrado","exit_code":1}"#.into(),
            );
            return;
        }
    };

    let hdr = Header::from_bytes(
        &b"Content-Type"[..],
        format!("{ctype}; charset=utf-8").as_bytes(),
    )
    .unwrap();
    let _ = req.respond(Response::from_data(body).with_header(hdr));
}

/// Acuse fire-and-forget: UUID preasignado + spawn de kalma2-interact sin join HTTP.
struct AcceptedAck {
    correlation_id: String,
    duration_ms: u64,
}

enum AcceptSyncError {
    Orchestrator(String),
    Spawn(String),
}

fn accept_execute(
    repo: &Path,
    prompt: &str,
    process: Option<&str>,
) -> Result<AcceptedAck, AcceptSyncError> {
    let t0 = Instant::now();
    let bin = resolve_orchestrator(repo).map_err(AcceptSyncError::Orchestrator)?;
    let correlation_id = Uuid::new_v4().to_string();

    let mut inputs = serde_json::json!({
        "prompt": prompt.trim(),
        "mode": "execute",
        "correlation_id": &correlation_id,
    });
    if let Some(proc) = process.map(str::trim).filter(|s| !s.is_empty()) {
        inputs["process"] = serde_json::json!(proc);
    }
    let inputs_json = inputs.to_string();

    let mut child = Command::new(&bin)
        .args(["--process", "kalma2-interact", "--inputs", &inputs_json])
        .current_dir(repo)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| AcceptSyncError::Spawn(e.to_string()))?;

    // Reaper: evita zombies; no bloquea el socket HTTP (L1/L8).
    thread::spawn(move || {
        let _ = child.wait();
    });

    Ok(AcceptedAck {
        correlation_id,
        duration_ms: t0.elapsed().as_millis() as u64,
    })
}

fn accepted_body(ack: &AcceptedAck) -> String {
    serde_json::json!({
        "success": true,
        "status": "accepted",
        "correlation_id": ack.correlation_id,
        "event_id": ack.correlation_id,
        "message": "intención aceptada; consultar GET /api/status",
        "duration_ms": ack.duration_ms,
    })
    .to_string()
}

fn reply_accept_result(req: tiny_http::Request, result: Result<AcceptedAck, AcceptSyncError>) {
    match result {
        Ok(ack) => reply(req, 202, accepted_body(&ack)),
        Err(AcceptSyncError::Orchestrator(message)) | Err(AcceptSyncError::Spawn(message)) => {
            reply(
                req,
                500,
                serde_json::json!({
                    "success": false,
                    "message": message,
                    "exit_code": 1
                })
                .to_string(),
            );
        }
    }
}

fn run_orchestrator(repo: &Path, bin: &Path, prompt: &str) -> Result<String, String> {
    run_orchestrator_inputs(
        repo,
        bin,
        &serde_json::json!({ "prompt": prompt }),
    )
}

fn run_orchestrator_inputs(
    repo: &Path,
    bin: &Path,
    inputs: &serde_json::Value,
) -> Result<String, String> {
    let inputs = inputs.to_string();
    let timeout = Duration::from_secs(client_timeout_secs());
    let repo = repo.to_path_buf();
    let bin = bin.to_path_buf();

    let handle = thread::spawn(move || {
        Command::new(&bin)
            .args(["--process", "kalma2-interact", "--inputs", &inputs])
            .current_dir(&repo)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
    });

    let started = Instant::now();
    loop {
        if handle.is_finished() {
            break;
        }
        if started.elapsed() >= timeout {
            return Err(r#"{"success":false,"message":"timeout motor","exit_code":1}"#.into());
        }
        thread::sleep(Duration::from_millis(50));
    }

    match handle.join() {
        Ok(Ok(output)) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            match stdout.lines().rev().find(|l| !l.trim().is_empty()) {
                Some(line) => Ok(line.to_string()),
                None => {
                    let err = String::from_utf8_lossy(&output.stderr);
                    let msg = if err.trim().is_empty() {
                        "sin salida del motor"
                    } else {
                        err.trim()
                    };
                    Ok(serde_json::json!({
                        "success": false,
                        "message": msg,
                        "exit_code": output.status.code().unwrap_or(1)
                    })
                    .to_string())
                }
            }
        }
        Ok(Err(e)) => Ok(serde_json::json!({
            "success": false,
            "message": e.to_string(),
            "exit_code": 1
        })
        .to_string()),
        Err(_) => Err(r#"{"success":false,"message":"subproceso falló","exit_code":1}"#.into()),
    }
}

fn normalize_rel(path: &str) -> String {
    let p = path.replace('\\', "/");
    p.strip_prefix("./").unwrap_or(&p).to_string()
}

fn load_fractal_paths(repo: &Path) -> (PathBuf, PathBuf, PathBuf) {
    let defaults = (
        repo.join(".events/domain"),
        repo.join(".events/orchestration"),
        repo.join(".events/dead-letter"),
    );
    let cfg_path = repo.join("SddIA/core/cumulo.paths.json");
    let Ok(text) = std::fs::read_to_string(&cfg_path) else {
        return defaults;
    };
    let Ok(cfg) = serde_json::from_str::<serde_json::Value>(&text) else {
        return defaults;
    };
    let fractal = cfg.get("eda_fractal");
    let domain = fractal
        .and_then(|f| f.get("domain"))
        .and_then(|v| v.as_str())
        .map(normalize_rel)
        .map(|r| repo.join(r))
        .unwrap_or(defaults.0);
    let orch = fractal
        .and_then(|f| f.get("orchestration"))
        .and_then(|v| v.as_str())
        .map(normalize_rel)
        .map(|r| repo.join(r))
        .unwrap_or(defaults.1);
    let dead = fractal
        .and_then(|f| f.get("dead_letter"))
        .and_then(|v| v.as_str())
        .map(normalize_rel)
        .map(|r| repo.join(r))
        .unwrap_or(defaults.2);
    (domain, orch, dead)
}

fn load_proofs_dir(repo: &Path) -> PathBuf {
    let default = repo.join(".SddIA/proofs");
    let cfg_path = repo.join("SddIA/core/cumulo.paths.json");
    let Ok(text) = std::fs::read_to_string(&cfg_path) else {
        return default;
    };
    let Ok(cfg) = serde_json::from_str::<serde_json::Value>(&text) else {
        return default;
    };
    cfg.get("eda_instance")
        .and_then(|e| e.get("proofs"))
        .and_then(|v| v.as_str())
        .map(normalize_rel)
        .map(|r| repo.join(r))
        .unwrap_or(default)
}

fn pec_view_from_proof(proof: &serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "event_id": proof.get("pec_event_id"),
        "event_type": "Process_Execution_Completed",
        "timestamp": proof.get("timestamp"),
        "payload": proof.get("payload"),
    })
}

fn find_pec_proof(proofs_dir: &Path, correlation_id: &str) -> Option<serde_json::Value> {
    let path = proofs_dir
        .join("pec-correlation")
        .join(format!("{correlation_id}.json"));
    let body = read_json_file(&path)?;
    if body.get("kind").and_then(|v| v.as_str()) != Some("pec-correlation-proof") {
        return None;
    }
    Some(pec_view_from_proof(&body))
}

fn is_uuid_v4ish(s: &str) -> bool {
    let s = s.trim();
    if s.len() != 36 {
        return false;
    }
    let b = s.as_bytes();
    for (i, c) in b.iter().enumerate() {
        match i {
            8 | 13 | 18 | 23 => {
                if *c != b'-' {
                    return false;
                }
            }
            _ => {
                if !c.is_ascii_hexdigit() {
                    return false;
                }
            }
        }
    }
    true
}

fn terminal_ok(status: &str) -> bool {
    status == "success" || status == "skipped" || status.starts_with("skipped")
}

fn terminal_failed(status: &str) -> bool {
    status == "failed" || status.starts_with("failed")
}

fn read_json_file(path: &Path) -> Option<serde_json::Value> {
    let text = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&text).ok()
}

fn find_domain_event_with_proofs(
    domain_dir: &Path,
    dead_dir: &Path,
    proofs_dir: Option<&Path>,
    event_id: &str,
) -> Option<(PathBuf, serde_json::Value, bool)> {
    let name = format!("{event_id}.json");
    let in_domain = domain_dir.join(&name);
    if in_domain.is_file() {
        if let Some(v) = read_json_file(&in_domain) {
            return Some((in_domain, v, false));
        }
    }
    let in_dead = dead_dir.join(&name);
    if in_dead.is_file() {
        if let Some(v) = read_json_file(&in_dead) {
            return Some((in_dead, v, true));
        }
    }
    if let Some(proofs) = proofs_dir {
        let in_proof = proofs.join("email-triaged").join(&name);
        if in_proof.is_file() {
            if let Some(v) = read_json_file(&in_proof) {
                let projected = serde_json::json!({
                    "event_id": v.get("event_id").cloned().unwrap_or(serde_json::json!(event_id)),
                    "event_type": v.get("event_type").cloned().unwrap_or(serde_json::json!("Email_Triaged")),
                    "timestamp": v.get("timestamp"),
                    "payload": v.get("payload"),
                });
                return Some((in_proof, projected, false));
            }
        }
    }
    None
}

fn find_pec_by_correlation(orch_dir: &Path, correlation_id: &str) -> Option<serde_json::Value> {
    let Ok(entries) = std::fs::read_dir(orch_dir) else {
        return None;
    };
    let mut best: Option<serde_json::Value> = None;
    let mut best_ts = String::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let Some(body) = read_json_file(&path) else {
            continue;
        };
        if body.get("event_type").and_then(|v| v.as_str()) != Some("Process_Execution_Completed") {
            continue;
        }
        let cid = body
            .get("payload")
            .and_then(|p| p.get("correlation_id"))
            .and_then(|v| v.as_str())
            .unwrap_or("");
        if cid != correlation_id {
            continue;
        }
        let ts = body
            .get("timestamp")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        if best.is_none() || ts >= best_ts {
            best_ts = ts;
            best = Some(body);
        }
    }
    best
}

fn project_status(
    domain: Option<&serde_json::Value>,
    in_dead_letter: bool,
    pec: Option<&serde_json::Value>,
) -> (&'static str, String) {
    if let Some(pec) = pec {
        let st = pec
            .get("payload")
            .and_then(|p| p.get("status"))
            .and_then(|v| v.as_str())
            .unwrap_or("success");
        let pname = pec
            .get("payload")
            .and_then(|p| p.get("process_name"))
            .and_then(|v| v.as_str())
            .unwrap_or("?");
        if st != "success" {
            return (
                "failed",
                format!("Proceso «{pname}» falló (status={st})."),
            );
        }
        // Slice A kalma2-full-cycle: cycle_phase distingue arranque vs cierre de negocio.
        // Legacy sin campo → completed (compat L3).
        let cycle = pec
            .get("payload")
            .and_then(|p| p.get("cycle_phase"))
            .and_then(|v| v.as_str())
            .unwrap_or("completed");
        return match cycle {
            "initialized" => (
                "initialized",
                format!(
                    "Ciclo «{pname}» arrancado (init OK). Fases de agentes pendientes o simuladas en lab."
                ),
            ),
            "awaiting_agents" => (
                "awaiting_agents",
                format!("Ciclo «{pname}» en espera de agentes IDE."),
            ),
            _ => (
                "completed",
                format!("Proceso «{pname}» completado (PEC correlacionado)."),
            ),
        };
    }

    if in_dead_letter {
        return (
            "failed",
            "Evento en dead-letter del bus fractal.".into(),
        );
    }

    if let Some(body) = domain {
        let ds = body
            .get("delivery_state")
            .and_then(|v| v.as_object());
        if let Some(ds) = ds {
            if !ds.is_empty() {
                let any_fail = ds.values().any(|v| {
                    v.as_str().map(terminal_failed).unwrap_or(false)
                });
                if any_fail {
                    return (
                        "failed",
                        "Algún suscriptor del dominio falló.".into(),
                    );
                }
                let all_ok = ds
                    .values()
                    .all(|v| v.as_str().map(terminal_ok).unwrap_or(false));
                if all_ok {
                    return (
                        "routed",
                        "Sistema Nervioso aceptó el evento (delivery_state OK).".into(),
                    );
                }
            }
        }
        return (
            "pending",
            "Evento en dominio; consenso de suscriptores incompleto.".into(),
        );
    }

    (
        "pending",
        "Sin rastro durable aún (posible purge post-route); seguir sondeo.".into(),
    )
}

fn build_status_body(repo: &Path, event_id: &str) -> (u16, String) {
    if !is_uuid_v4ish(event_id) {
        return (
            400,
            serde_json::json!({
                "success": false,
                "message": "event_id inválido",
                "exit_code": 1
            })
            .to_string(),
        );
    }

    let (domain_dir, orch_dir, dead_dir) = load_fractal_paths(repo);
    let proofs_dir = load_proofs_dir(repo);
    let domain_hit = find_domain_event_with_proofs(&domain_dir, &dead_dir, Some(&proofs_dir), event_id);
    let pec = find_pec_by_correlation(&orch_dir, event_id)
        .or_else(|| find_pec_proof(&proofs_dir, event_id));

    if domain_hit.is_none() && pec.is_none() {
        return (
            404,
            serde_json::json!({
                "success": false,
                "event_id": event_id,
                "message": "evento no encontrado",
                "exit_code": 1
            })
            .to_string(),
        );
    }

    let in_dead = domain_hit.as_ref().map(|h| h.2).unwrap_or(false);
    let domain_val = domain_hit.as_ref().map(|h| &h.1);
    let (status, message) = project_status(domain_val, in_dead, pec.as_ref());

    let delivery_status = domain_val
        .and_then(|b| b.get("delivery_state"))
        .cloned()
        .unwrap_or(serde_json::json!({}));

    let event_type = domain_val
        .and_then(|b| b.get("event_type"))
        .and_then(|v| v.as_str())
        .unwrap_or("Kalma2_Process_Requested");

    let orch_obj = if let Some(ref pec) = pec {
        serde_json::json!({
            "found": true,
            "event_id": pec.get("event_id"),
            "process_name": pec.get("payload").and_then(|p| p.get("process_name")),
            "process_status": pec.get("payload").and_then(|p| p.get("status")),
            "cycle_phase": pec.get("payload").and_then(|p| p.get("cycle_phase")),
        })
    } else {
        serde_json::json!({
            "found": false,
            "event_id": null,
            "process_name": null,
            "process_status": null,
            "cycle_phase": null
        })
    };

    (
        200,
        serde_json::json!({
            "success": true,
            "event_id": event_id,
            "correlation_id": event_id,
            "status": status,
            "domain": {
                "found": domain_hit.is_some(),
                "event_type": event_type,
                "delivery_status": delivery_status,
                "dead_letter": in_dead
            },
            "orchestration": orch_obj,
            "message": message,
            "exit_code": 0
        })
        .to_string(),
    )
}

fn load_progress_path(repo: &Path) -> PathBuf {
    let cfg_path = repo.join("SddIA/core/cumulo.paths.json");
    let default = repo.join(".events/progress");
    let Ok(text) = std::fs::read_to_string(&cfg_path) else {
        return default;
    };
    let Ok(cfg) = serde_json::from_str::<serde_json::Value>(&text) else {
        return default;
    };
    cfg.get("eda_fractal")
        .and_then(|f| f.get("progress"))
        .and_then(|v| v.as_str())
        .map(normalize_rel)
        .map(|r| repo.join(r))
        .unwrap_or(default)
}

fn query_param<'a>(url: &'a str, key: &str) -> Option<String> {
    url.split('?')
        .nth(1)?
        .split('&')
        .find_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            let k = parts.next()?;
            let v = parts.next().unwrap_or("");
            (k == key).then(|| percent_decode(v))
        })
        .filter(|s| !s.is_empty())
}

fn list_trace_files(corr_dir: &Path) -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(corr_dir) else {
        return vec![];
    };
    let mut paths: Vec<PathBuf> = entries
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("json"))
        .collect();
    paths.sort_by(|a, b| trace_sort_key(a).cmp(&trace_sort_key(b)));
    paths
}

fn trace_sort_key(path: &Path) -> String {
    read_json_file(path)
        .and_then(|v| v.get("timestamp").and_then(|t| t.as_str()).map(str::to_string))
        .unwrap_or_else(|| {
            path.file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default()
        })
}

fn sse_progress_frame(body: &str) -> Vec<u8> {
    format!("event: progress\ndata: {body}\n\n").into_bytes()
}

struct ProgressStreamReader {
    corr_dir: PathBuf,
    seen: HashSet<String>,
    pending: Vec<u8>,
    finished: bool,
    last_poll: Instant,
    last_ping: Instant,
}

impl ProgressStreamReader {
    fn new(corr_dir: PathBuf) -> Self {
        Self {
            corr_dir,
            seen: HashSet::new(),
            pending: Vec::new(),
            finished: false,
            last_poll: Instant::now() - Duration::from_secs(1),
            last_ping: Instant::now(),
        }
    }

    fn collect_new_frames(&mut self) {
        for path in list_trace_files(&self.corr_dir) {
            let key = path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            if key.is_empty() || self.seen.contains(&key) {
                continue;
            }
            if let Some(body) = read_json_file(&path) {
                if let Ok(compact) = serde_json::to_string(&body) {
                    self.pending.extend_from_slice(&sse_progress_frame(&compact));
                    self.seen.insert(key);
                }
            }
        }
    }
}

impl Read for ProgressStreamReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        loop {
            if !self.pending.is_empty() {
                let n = self.pending.len().min(buf.len());
                buf[..n].copy_from_slice(&self.pending[..n]);
                self.pending.drain(..n);
                return Ok(n);
            }
            if self.finished {
                return Ok(0);
            }
            if self.last_poll.elapsed() >= Duration::from_millis(400) {
                self.collect_new_frames();
                self.last_poll = Instant::now();
                continue;
            }
            if self.last_ping.elapsed() >= Duration::from_secs(15) {
                self.pending.extend_from_slice(b": ping\n\n");
                self.last_ping = Instant::now();
                continue;
            }
            thread::sleep(Duration::from_millis(100));
        }
    }
}

fn handle_progress_stream(req: tiny_http::Request, repo: &Path) {
    let url = req.url();
    let correlation_id = match query_param(url, "correlation_id") {
        Some(id) if is_uuid_v4ish(&id) => id,
        Some(_) | None => {
            reply(
                req,
                400,
                r#"{"success":false,"message":"correlation_id inválido o ausente","exit_code":1}"#
                    .into(),
            );
            return;
        }
    };

    let corr_dir = load_progress_path(repo).join(&correlation_id);

    let reader = ProgressStreamReader::new(corr_dir);
    let response = Response::new(
        StatusCode(200),
        vec![sse_header()],
        reader,
        None,
        None,
    );
    let _ = req.respond(response);
}

fn handle_status(req: tiny_http::Request, repo: &Path) {
    let url = req.url().to_string();
    let event_id = url
        .split('?')
        .nth(1)
        .unwrap_or("")
        .split('&')
        .find_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            let k = parts.next()?;
            let v = parts.next().unwrap_or("");
            (k == "event_id").then(|| {
                percent_decode(v)
            })
        });

    let Some(event_id) = event_id.filter(|s| !s.is_empty()) else {
        reply(
            req,
            400,
            r#"{"success":false,"message":"event_id requerido","exit_code":1}"#.into(),
        );
        return;
    };

    let (code, body) = build_status_body(repo, &event_id);
    reply(req, code, body);
}

fn percent_decode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let hex = &s[i + 1..i + 3];
            if let Ok(v) = u8::from_str_radix(hex, 16) {
                out.push(v as char);
                i += 3;
                continue;
            }
        }
        if bytes[i] == b'+' {
            out.push(' ');
        } else {
            out.push(bytes[i] as char);
        }
        i += 1;
    }
    out
}

fn sse_header() -> Header {
    Header::from_bytes(
        &b"Content-Type"[..],
        &b"text/event-stream; charset=utf-8"[..],
    )
    .unwrap()
}

fn chat_timeout_secs() -> u64 {
    std::env::var("SDDIA_LLM_SSE_TIMEOUT_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .or_else(|| {
            std::env::var("SDDIA_LLM_CLI_TIMEOUT_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
        })
        .unwrap_or(120)
}

fn new_event_id() -> String {
    let mut b = [0u8; 16];
    if let Ok(mut f) = std::fs::File::open("/dev/urandom") {
        let _ = f.read_exact(&mut b);
    } else {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        for (i, slot) in b.iter_mut().enumerate() {
            *slot = ((nanos >> (i * 8)) as u8).wrapping_add(i as u8);
        }
    }
    b[6] = (b[6] & 0x0f) | 0x40;
    b[8] = (b[8] & 0x3f) | 0x80;
    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], b[8], b[9], b[10], b[11], b[12], b[13],
        b[14], b[15]
    )
}

fn load_eda_pending(repo: &Path) -> PathBuf {
    let cfg_path = repo.join("SddIA/core/cumulo.paths.json");
    if let Ok(text) = std::fs::read_to_string(&cfg_path) {
        if let Ok(cfg) = serde_json::from_str::<serde_json::Value>(&text) {
            if let Some(rel) = cfg
                .pointer("/eda_bus/pending")
                .and_then(|v| v.as_str())
                .map(normalize_rel)
            {
                return repo.join(rel);
            }
        }
    }
    repo.join(".events/pending")
}

fn emit_system_fracture(repo: &Path, fracture_kind: &str, error_trace: &str) {
    let event_id = new_event_id();
    let pending = load_eda_pending(repo);
    let _ = std::fs::create_dir_all(&pending);
    let ts = chrono_like_now();
    let event = serde_json::json!({
        "event_id": event_id,
        "event_type": "System_Fracture_Detected",
        "event_family": "domain",
        "timestamp": ts,
        "emitter_agent": "kalma2-bridge",
        "payload": {
            "process_name": "kalma2-bridge",
            "error_trace": error_trace,
            "agent_emitter": "kalma2-bridge",
            "attempted_action": "sse_chat_stream",
            "source": "kalma2-bridge",
            "fracture_kind": fracture_kind,
        }
    });
    let target = pending.join(format!("{event_id}.json"));
    let _ = std::fs::write(
        &target,
        serde_json::to_string_pretty(&event).unwrap_or_else(|_| "{}".into()),
    );
}

fn chrono_like_now() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    // ISO-ish sin chrono crate: epoch seconds (suficiente para bus).
    format!("{secs}")
}

fn resolve_mayeuta_llm(repo: &Path) -> Result<PathBuf, String> {
    if let Ok(o) = std::env::var("SDDIA_MAYEUTA_LLM_BIN") {
        let trimmed = o.trim();
        if !trimmed.is_empty() {
            let candidate = PathBuf::from(trimmed);
            return is_native_elf(&candidate)
                .then_some(candidate)
                .ok_or_else(|| "SDDIA_MAYEUTA_LLM_BIN no es ELF nativo".into());
        }
    }
    for rel in [
        "SddIA/target/debug/mayeuta-llm",
        "SddIA/target/release/mayeuta-llm",
    ] {
        let candidate = repo.join(rel);
        if is_native_elf(&candidate) {
            return Ok(candidate);
        }
    }
    Err("mayeuta-llm no encontrado en SddIA/target/{debug,release}".into())
}

/// Reader que transforma líneas del hijo en frames SSE `data: …\n\n`.
struct SseLineReader {
    inner: BufReader<std::process::ChildStdout>,
    pending: Vec<u8>,
    finished: bool,
}

impl Read for SseLineReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        if !self.pending.is_empty() {
            let n = self.pending.len().min(buf.len());
            buf[..n].copy_from_slice(&self.pending[..n]);
            self.pending.drain(..n);
            return Ok(n);
        }
        if self.finished {
            return Ok(0);
        }
        let mut line = String::new();
        match self.inner.read_line(&mut line)? {
            0 => {
                self.finished = true;
                Ok(0)
            }
            _ => {
                let payload = line.trim_end_matches(['\r', '\n']);
                let frame = format!("data: {payload}\n\n");
                self.pending = frame.into_bytes();
                let n = self.pending.len().min(buf.len());
                buf[..n].copy_from_slice(&self.pending[..n]);
                self.pending.drain(..n);
                Ok(n)
            }
        }
    }
}

fn handle_chat(mut req: tiny_http::Request, repo: &Path) {
    let mut buf = String::new();
    if req.as_reader().read_to_string(&mut buf).is_err() {
        reply(
            req,
            400,
            r#"{"success":false,"message":"prompt requerido","exit_code":1}"#.into(),
        );
        return;
    }

    let prompt = match serde_json::from_str::<InteractReq>(&buf) {
        Ok(p) if !p.prompt.trim().is_empty() => p.prompt.trim().to_string(),
        _ => {
            reply(
                req,
                400,
                r#"{"success":false,"message":"prompt requerido","exit_code":1}"#.into(),
            );
            return;
        }
    };

    let skill = match resolve_mayeuta_llm(repo) {
        Ok(b) => b,
        Err(message) => {
            emit_system_fracture(repo, "prosthetic_collapse", &message);
            reply(
                req,
                500,
                serde_json::json!({
                    "success": false,
                    "message": message,
                    "exit_code": 1
                })
                .to_string(),
            );
            return;
        }
    };

    let stdin_payload = serde_json::json!({
        "operation": "STREAM",
        "prompt": prompt,
    })
    .to_string();

    let mut child = match Command::new(&skill)
        .current_dir(repo)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            let msg = format!("spawn mayeuta-llm: {e}");
            emit_system_fracture(repo, "prosthetic_collapse", &msg);
            reply(
                req,
                500,
                serde_json::json!({
                    "success": false,
                    "message": msg,
                    "exit_code": 1
                })
                .to_string(),
            );
            return;
        }
    };

    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        if let Err(e) = stdin.write_all(stdin_payload.as_bytes()) {
            let msg = format!("stdin mayeuta-llm: {e}");
            let _ = child.kill();
            emit_system_fracture(repo, "prosthetic_collapse", &msg);
            reply(
                req,
                500,
                serde_json::json!({
                    "success": false,
                    "message": msg,
                    "exit_code": 1
                })
                .to_string(),
            );
            return;
        }
    }

    let Some(stdout) = child.stdout.take() else {
        let _ = child.kill();
        emit_system_fracture(repo, "prosthetic_collapse", "mayeuta-llm sin stdout");
        reply(
            req,
            500,
            r#"{"success":false,"message":"mayeuta-llm sin stdout","exit_code":1}"#.into(),
        );
        return;
    };

    let timeout = Duration::from_secs(chat_timeout_secs());
    let started = Instant::now();
    let repo_watch = repo.to_path_buf();
    let child_id = child.id();

    // Watchdog: si el stream supera timeout, mata al hijo → Read EOF + fractura.
    thread::spawn(move || {
        while started.elapsed() < timeout {
            thread::sleep(Duration::from_millis(200));
            // cheap liveness: /proc
            if !Path::new(&format!("/proc/{child_id}")).exists() {
                return;
            }
        }
        let _ = Command::new("kill")
            .args(["-9", &child_id.to_string()])
            .status();
        emit_system_fracture(
            &repo_watch,
            "sse_watchdog",
            &format!("SSE chat timeout {timeout:?}; kill -9 pid={child_id}"),
        );
    });

    let reader = SseLineReader {
        inner: BufReader::new(stdout),
        pending: Vec::new(),
        finished: false,
    };

    let response = Response::new(
        StatusCode(200),
        vec![sse_header()],
        reader,
        None,
        None,
    );
    if req.respond(response).is_err() {
        let _ = child.kill();
        emit_system_fracture(
            repo,
            "sse_watchdog",
            "cliente SSE desconectado durante stream",
        );
        return;
    }

    match child.wait() {
        Ok(status) if status.success() => {}
        Ok(status) => {
            emit_system_fracture(
                repo,
                "prosthetic_collapse",
                &format!(
                    "mayeuta-llm/prótesis exit {}",
                    status.code().unwrap_or(1)
                ),
            );
        }
        Err(e) => {
            emit_system_fracture(repo, "prosthetic_collapse", &format!("wait hijo: {e}"));
        }
    }
}

fn handle_execute(mut req: tiny_http::Request, repo: &Path) {
    let mut buf = String::new();
    if req.as_reader().read_to_string(&mut buf).is_err() {
        reply(
            req,
            400,
            r#"{"success":false,"message":"prompt requerido","exit_code":1}"#.into(),
        );
        return;
    }

    let parsed = match serde_json::from_str::<InteractReq>(&buf) {
        Ok(p) if !p.prompt.trim().is_empty() => p,
        _ => {
            reply(
                req,
                400,
                r#"{"success":false,"message":"prompt requerido","exit_code":1}"#.into(),
            );
            return;
        }
    };

    let process = parsed
        .process
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty());
    reply_accept_result(req, accept_execute(repo, parsed.prompt.trim(), process));
}

fn handle_interact(mut req: tiny_http::Request, repo: &Path) {
    let mut buf = String::new();
    if req.as_reader().read_to_string(&mut buf).is_err() {
        reply(
            req,
            400,
            r#"{"success":false,"message":"prompt requerido","exit_code":1}"#.into(),
        );
        return;
    }

    let parsed = match serde_json::from_str::<InteractReq>(&buf) {
        Ok(p) if !p.prompt.trim().is_empty() => p,
        _ => {
            reply(
                req,
                400,
                r#"{"success":false,"message":"prompt requerido","exit_code":1}"#.into(),
            );
            return;
        }
    };

    // L-EP: alias por mode explícito.
    let mode = parsed
        .mode
        .as_deref()
        .map(str::trim)
        .unwrap_or("")
        .to_lowercase();
    if mode == "execute" {
        let process = parsed
            .process
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());
        reply_accept_result(req, accept_execute(repo, parsed.prompt.trim(), process));
        return;
    }
    if mode == "chat" {
        // Compat: chat síncrono (no SSE) vía mode=chat en interact.
        let bin = match resolve_orchestrator(repo) {
            Ok(b) => b,
            Err(message) => {
                reply(
                    req,
                    500,
                    serde_json::json!({
                        "success": false,
                        "message": message,
                        "exit_code": 1
                    })
                    .to_string(),
                );
                return;
            }
        };
        let inputs = serde_json::json!({
            "prompt": parsed.prompt.trim(),
            "mode": "chat",
        });
        let t0 = Instant::now();
        match run_orchestrator_inputs(repo, &bin, &inputs) {
            Ok(mut line) => {
                if let Ok(mut v) = serde_json::from_str::<serde_json::Value>(&line) {
                    if let Some(obj) = v.as_object_mut() {
                        obj.insert(
                            "duration_ms".into(),
                            serde_json::json!(t0.elapsed().as_millis() as u64),
                        );
                    }
                    line = v.to_string();
                }
                reply(req, 200, line);
            }
            Err(body) => reply(req, 500, body),
        }
        return;
    }

    let bin = match resolve_orchestrator(repo) {
        Ok(b) => b,
        Err(message) => {
            reply(
                req,
                500,
                serde_json::json!({
                    "success": false,
                    "message": message,
                    "exit_code": 1
                })
                .to_string(),
            );
            return;
        }
    };

    let t0 = Instant::now();
    match run_orchestrator(repo, &bin, parsed.prompt.trim()) {
        Ok(mut line) => {
            if let Ok(mut v) = serde_json::from_str::<serde_json::Value>(&line) {
                if let Some(obj) = v.as_object_mut() {
                    obj.insert(
                        "duration_ms".into(),
                        serde_json::json!(t0.elapsed().as_millis() as u64),
                    );
                }
                line = v.to_string();
            }
            reply(req, 200, line);
        }
        Err(body) => reply(req, 500, body),
    }
}

fn handle_sync_assets(mut req: tiny_http::Request, repo: &Path) {
    let mut buf = String::new();
    if req.as_reader().read_to_string(&mut buf).is_err() {
        reply(
            req,
            400,
            r#"{"success":false,"message":"body requerido","exit_code":1}"#.into(),
        );
        return;
    }

    let parsed: SyncAssetsReq = match serde_json::from_str::<SyncAssetsReq>(&buf) {
        Ok(p) if !p.asset_id.trim().is_empty() => p,
        _ => {
            reply(
                req,
                400,
                r#"{"success":false,"message":"asset_id requerido","exit_code":1}"#.into(),
            );
            return;
        }
    };

    let asset_family = parsed
        .asset_family
        .as_deref()
        .unwrap_or("library_codexes")
        .trim()
        .to_string();

    let correlation_id = Uuid::new_v4().to_string();
    let inputs = serde_json::json!({
        "asset_id": parsed.asset_id.trim(),
        "asset_family": asset_family,
        "correlation_id": correlation_id,
        "execution_id": correlation_id,
    })
    .to_string();

    let bin = match resolve_orchestrator(repo) {
        Ok(b) => b,
        Err(message) => {
            reply(
                req,
                500,
                serde_json::json!({
                    "success": false,
                    "message": message,
                    "exit_code": 1
                })
                .to_string(),
            );
            return;
        }
    };

    let t0 = Instant::now();

    let mut child = match Command::new(&bin)
        .args([
            "--process",
            "sync-client-assets",
            "--inputs",
            &inputs,
        ])
        .current_dir(repo)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            reply(
                req,
                500,
                serde_json::json!({
                    "success": false,
                    "message": e.to_string(),
                    "exit_code": 1
                })
                .to_string(),
            );
            return;
        }
    };

    // Reaper: evita zombies; no bloquea el socket HTTP (DA-5 fire-and-forget).
    thread::spawn(move || {
        let _ = child.wait();
    });

    reply(
        req,
        202,
        serde_json::json!({
            "accepted": true,
            "success": true,
            "status": "accepted",
            "correlation_id": correlation_id,
            "event_id": correlation_id,
            "message": "sync-client-assets encolado; consultar GET /api/status",
            "duration_ms": t0.elapsed().as_millis() as u64,
        })
        .to_string(),
    );
}

fn list_actionable_email_items(repo: &Path) -> Vec<serde_json::Value> {
    let dir = load_proofs_dir(repo).join("email-triaged");
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return Vec::new();
    };
    let mut items: Vec<(String, serde_json::Value)> = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let Some(v) = read_json_file(&path) else {
            continue;
        };
        let payload = v.get("payload").cloned().unwrap_or(serde_json::json!({}));
        if payload.get("verdict").and_then(|x| x.as_str()) != Some("actionable") {
            continue;
        }
        let ts = v
            .get("timestamp")
            .and_then(|t| t.as_str())
            .unwrap_or("")
            .to_string();
        let event_id = v
            .get("event_id")
            .and_then(|x| x.as_str())
            .map(str::to_string)
            .unwrap_or_else(|| {
                path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string()
            });
        items.push((
            ts.clone(),
            serde_json::json!({
                "event_id": event_id,
                "message_uid": payload.get("message_uid"),
                "from": payload.get("from"),
                "subject": payload.get("subject"),
                "verdict": "actionable",
                "timestamp": ts,
                "agenda_entry_id": payload.get("agenda_entry_id"),
            }),
        ));
    }
    items.sort_by(|a, b| b.0.cmp(&a.0));
    items.into_iter().take(20).map(|(_, v)| v).collect()
}

fn handle_email_inbox(req: tiny_http::Request, repo: &Path) {
    let items = list_actionable_email_items(repo);
    reply(
        req,
        200,
        serde_json::json!({
            "success": true,
            "items": items,
            "exit_code": 0
        })
        .to_string(),
    );
}

#[derive(Deserialize)]
struct EmailQuickActionReq {
    message_uid: String,
    action: String,
    #[serde(default)]
    source_event_id: Option<String>,
}

fn handle_email_quick_action(mut req: tiny_http::Request, repo: &Path) {
    let mut buf = String::new();
    if req.as_reader().read_to_string(&mut buf).is_err() {
        reply(
            req,
            400,
            r#"{"success":false,"message":"body requerido","exit_code":1}"#.into(),
        );
        return;
    }
    let parsed = match serde_json::from_str::<EmailQuickActionReq>(&buf) {
        Ok(p) => p,
        Err(_) => {
            reply(
                req,
                400,
                r#"{"success":false,"message":"message_uid y action requeridos","exit_code":1}"#.into(),
            );
            return;
        }
    };
    let uid = parsed.message_uid.trim();
    let action = parsed.action.trim().to_ascii_lowercase();
    if uid.is_empty() || !matches!(action.as_str(), "archive" | "draft" | "delegate") {
        reply(
            req,
            400,
            r#"{"success":false,"message":"action inválida","exit_code":1}"#.into(),
        );
        return;
    }
    let event_id = new_event_id();
    let (domain_dir, _, _) = load_fractal_paths(repo);
    if std::fs::create_dir_all(&domain_dir).is_err() {
        reply(
            req,
            500,
            r#"{"success":false,"message":"mkdir domain","exit_code":1}"#.into(),
        );
        return;
    }
    let mut payload = serde_json::json!({
        "message_uid": uid,
        "action": action,
        "channel": "kalma2",
    });
    if let Some(src) = parsed
        .source_event_id
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        payload["source_event_id"] = serde_json::json!(src);
    }
    let event = serde_json::json!({
        "event_id": event_id,
        "event_type": "Email_Quick_Action_Requested",
        "event_family": "domain",
        "timestamp": chrono_like_now(),
        "emitter_agent": "kalma2-bridge",
        "payload": payload,
    });
    let target = domain_dir.join(format!("{event_id}.json"));
    if std::fs::write(&target, format!("{event}\n")).is_err() {
        reply(
            req,
            500,
            r#"{"success":false,"message":"write domain event","exit_code":1}"#.into(),
        );
        return;
    }
    reply(
        req,
        202,
        serde_json::json!({
            "success": true,
            "accepted": true,
            "status": "accepted",
            "event_id": event_id,
            "message": "acción rápida encolada",
            "exit_code": 0
        })
        .to_string(),
    );
}

fn dispatch(req: tiny_http::Request, repo: Arc<PathBuf>, ui_root: Arc<PathBuf>) {
    let path = req.url().split('?').next().unwrap_or("/");
    match (req.method(), path) {
        (Method::Post, "/api/chat") => handle_chat(req, &repo),
        (Method::Post, "/api/execute") => handle_execute(req, &repo),
        (Method::Post, "/api/interact") => handle_interact(req, &repo),
        (Method::Post, "/api/sync-assets") => handle_sync_assets(req, &repo),
        (Method::Post, "/api/email-quick-action") => handle_email_quick_action(req, &repo),
        (Method::Get, "/api/status") => handle_status(req, &repo),
        (Method::Get, "/api/email-inbox") => handle_email_inbox(req, &repo),
        (Method::Get, "/api/progress/stream") => handle_progress_stream(req, &repo),
        (Method::Get, _) => serve_static(req, &ui_root),
        _ => reply(
            req,
            404,
            r#"{"success":false,"message":"ruta desconocida","exit_code":1}"#.into(),
        ),
    }
}

fn main() {
    let repo = repo_root();
    let ui_root = repo.join("interfaces/kalma2");

    if !ui_root.is_dir() {
        eprintln!("[kalma2-bridge] falta bundle UI: {}", ui_root.display());
        std::process::exit(1);
    }

    let port = std::env::var("SDDIA_CLIENT_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8765u16);

    let server = Server::http((Ipv4Addr::LOCALHOST, port)).unwrap_or_else(|e| {
        eprintln!("[kalma2-bridge] bind {port}: {e}");
        std::process::exit(1);
    });

    eprintln!("[kalma2-bridge] activo en http://127.0.0.1:{port}");

    let repo = Arc::new(repo);
    let ui_root = Arc::new(ui_root);

    for req in server.incoming_requests() {
        let repo = Arc::clone(&repo);
        let ui_root = Arc::clone(&ui_root);
        thread::spawn(move || dispatch(req, repo, ui_root));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_orchestrator_finds_debug_binary() {
        let repo = repo_root();
        let bin = resolve_orchestrator(&repo);
        assert!(
            bin.is_ok(),
            "execute-process nativo debe existir tras cargo build"
        );
    }

    #[test]
    fn native_elf_rejects_script_content() {
        let path = std::env::temp_dir().join("kalma2-bridge-non-elf-test");
        std::fs::write(&path, "#!/usr/bin/env python3\n").unwrap();
        assert!(!is_native_elf(&path));
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn uuid_v4ish_accepts_canonical() {
        assert!(is_uuid_v4ish("458c34a8-9ad5-4a40-88c4-0be1e5d9598e"));
        assert!(!is_uuid_v4ish("not-a-uuid"));
        assert!(!is_uuid_v4ish(""));
    }

    #[test]
    fn project_status_completed_from_pec() {
        let pec = serde_json::json!({
            "payload": {"status": "success", "process_name": "task-queue-manager"}
        });
        let (st, _) = project_status(None, false, Some(&pec));
        assert_eq!(st, "completed");
    }

    #[test]
    fn project_status_initialized_from_cycle_phase() {
        let pec = serde_json::json!({
            "payload": {
                "status": "success",
                "process_name": "bug-fix",
                "cycle_phase": "initialized"
            }
        });
        let (st, msg) = project_status(None, false, Some(&pec));
        assert_eq!(st, "initialized");
        assert!(msg.contains("arrancado"));
    }

    #[test]
    fn project_status_awaiting_agents_from_cycle_phase() {
        let pec = serde_json::json!({
            "payload": {
                "status": "success",
                "process_name": "feature",
                "cycle_phase": "awaiting_agents"
            }
        });
        let (st, _) = project_status(None, false, Some(&pec));
        assert_eq!(st, "awaiting_agents");
    }

    #[test]
    fn find_pec_by_correlation_prefers_latest_timestamp() {
        let orch = std::env::temp_dir().join(format!("sddia-pec-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&orch).unwrap();
        let cid = "11111111-1111-4111-8111-111111111111";
        let early = serde_json::json!({
            "event_type": "Process_Execution_Completed",
            "timestamp": "2026-08-13T05:00:00Z",
            "payload": {"correlation_id": cid, "cycle_phase": "awaiting_agents", "status": "success"}
        });
        let late = serde_json::json!({
            "event_type": "Process_Execution_Completed",
            "timestamp": "2026-08-13T05:10:00Z",
            "payload": {"correlation_id": cid, "cycle_phase": "completed", "status": "success"}
        });
        std::fs::write(orch.join("early.json"), early.to_string()).unwrap();
        std::fs::write(orch.join("late.json"), late.to_string()).unwrap();
        let pec = find_pec_by_correlation(&orch, cid).expect("pec");
        assert_eq!(pec["payload"]["cycle_phase"], "completed");
        let _ = std::fs::remove_dir_all(&orch);
    }

    #[test]
    fn find_pec_proof_reads_namespaced_json() {
        let proofs = std::env::temp_dir().join(format!("sddia-proofs-{}", uuid::Uuid::new_v4()));
        let ns = proofs.join("pec-correlation");
        std::fs::create_dir_all(&ns).unwrap();
        let cid = "e273713c-dd91-487b-8716-1bdc8c5da741";
        std::fs::write(
            ns.join(format!("{cid}.json")),
            serde_json::json!({
                "kind": "pec-correlation-proof",
                "correlation_id": cid,
                "pec_event_id": "9ff24776-26c7-4596-8b08-7b6fc4531641",
                "payload": {
                    "process_name": "feature",
                    "status": "success",
                    "cycle_phase": "completed"
                }
            })
            .to_string(),
        )
        .unwrap();
        let pec = find_pec_proof(&proofs, cid).expect("proof");
        assert_eq!(pec["payload"]["process_name"], "feature");
        let _ = std::fs::remove_dir_all(&proofs);
    }

    #[test]
    fn build_status_body_resolves_proof_after_pec_gone() {
        let repo = std::env::temp_dir().join(format!("sddia-status-proof-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        std::fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"eda_fractal":{"domain":".events/domain","orchestration":".events/orchestration","dead_letter":".events/dead-letter"},"eda_instance":{"proofs":".SddIA/proofs"}}"#,
        )
        .unwrap();
        std::fs::create_dir_all(repo.join(".events/domain")).unwrap();
        std::fs::create_dir_all(repo.join(".events/orchestration")).unwrap();
        let cid = "e273713c-dd91-487b-8716-1bdc8c5da741";
        let proof_dir = repo.join(".SddIA/proofs/pec-correlation");
        std::fs::create_dir_all(&proof_dir).unwrap();
        std::fs::write(
            proof_dir.join(format!("{cid}.json")),
            serde_json::json!({
                "kind": "pec-correlation-proof",
                "correlation_id": cid,
                "pec_event_id": "9ff24776-26c7-4596-8b08-7b6fc4531641",
                "timestamp": "2026-08-15T12:00:00Z",
                "payload": {
                    "process_name": "feature",
                    "status": "success",
                    "cycle_phase": "completed"
                }
            })
            .to_string(),
        )
        .unwrap();
        let (code, body) = build_status_body(&repo, cid);
        assert_eq!(code, 200);
        let v: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(v["status"], "completed");
        assert_eq!(v["orchestration"]["found"], true);
        let _ = std::fs::remove_dir_all(&repo);
    }

    #[test]
    fn build_status_body_projects_email_triaged() {
        let repo = std::env::temp_dir().join(format!("sddia-email-triaged-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        std::fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"eda_fractal":{"domain":".events/domain","orchestration":".events/orchestration","dead_letter":".events/dead-letter"},"eda_instance":{"proofs":".SddIA/proofs"}}"#,
        )
        .unwrap();
        std::fs::create_dir_all(repo.join(".events/domain")).unwrap();
        std::fs::create_dir_all(repo.join(".events/orchestration")).unwrap();
        std::fs::create_dir_all(repo.join(".events/dead-letter")).unwrap();
        let eid = "aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee";
        std::fs::write(
            repo.join(".events/domain").join(format!("{eid}.json")),
            serde_json::json!({
                "event_id": eid,
                "event_type": "Email_Triaged",
                "event_family": "domain",
                "payload": {
                    "message_uid": "42",
                    "verdict": "noise",
                    "decision_path": "deterministic",
                    "thermodynamic_cost": {"tokens_in": 0, "tokens_out": 0, "duration_ms": 0}
                }
            })
            .to_string(),
        )
        .unwrap();
        let (code, body) = build_status_body(&repo, eid);
        assert_eq!(code, 200);
        let v: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(v["success"], true);
        assert_eq!(v["domain"]["found"], true);
        assert_eq!(v["domain"]["event_type"], "Email_Triaged");
        let _ = std::fs::remove_dir_all(&repo);
    }

    #[test]
    fn build_status_body_projects_email_triaged_from_proof_after_purge() {
        let repo = std::env::temp_dir().join(format!("sddia-email-triaged-proof-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        std::fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"eda_fractal":{"domain":".events/domain","orchestration":".events/orchestration","dead_letter":".events/dead-letter"},"eda_instance":{"proofs":".SddIA/proofs"}}"#,
        )
        .unwrap();
        std::fs::create_dir_all(repo.join(".events/domain")).unwrap();
        std::fs::create_dir_all(repo.join(".events/orchestration")).unwrap();
        std::fs::create_dir_all(repo.join(".events/dead-letter")).unwrap();
        let eid = "bbbbbbbb-cccc-4ddd-8eee-ffffffffffff";
        let proof_dir = repo.join(".SddIA/proofs/email-triaged");
        std::fs::create_dir_all(&proof_dir).unwrap();
        std::fs::write(
            proof_dir.join(format!("{eid}.json")),
            serde_json::json!({
                "kind": "email-triaged-proof",
                "event_id": eid,
                "event_type": "Email_Triaged",
                "timestamp": "2026-08-18T15:00:00Z",
                "payload": {
                    "message_uid": "42",
                    "verdict": "noise",
                    "decision_path": "deterministic"
                }
            })
            .to_string(),
        )
        .unwrap();
        let (code, body) = build_status_body(&repo, eid);
        assert_eq!(code, 200);
        let v: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(v["domain"]["found"], true);
        assert_eq!(v["domain"]["event_type"], "Email_Triaged");
        let _ = std::fs::remove_dir_all(&repo);
    }

    #[test]
    fn project_status_routed_from_delivery() {
        let domain = serde_json::json!({
            "delivery_state": {
                "task-queue-manager": "success",
                "iota-immutable-publisher": "skipped-lab-no-credentials"
            }
        });
        let (st, _) = project_status(Some(&domain), false, None);
        assert_eq!(st, "routed");
    }

    #[test]
    fn accepted_body_identity_correlation_event() {
        let ack = AcceptedAck {
            correlation_id: "458c34a8-9ad5-4a40-88c4-0be1e5d9598e".into(),
            duration_ms: 3,
        };
        let v: serde_json::Value = serde_json::from_str(&accepted_body(&ack)).unwrap();
        assert_eq!(v["success"], true);
        assert_eq!(v["status"], "accepted");
        assert_eq!(v["correlation_id"], ack.correlation_id);
        assert_eq!(v["event_id"], ack.correlation_id);
        assert_eq!(v["duration_ms"], 3);
    }

    #[test]
    fn progress_stream_route_before_static() {
        let src = include_str!("main.rs");
        let prod = src.split("#[cfg(test)]").next().unwrap_or(src);
        let dispatch = prod.split("fn dispatch").nth(1).expect("dispatch");
        let status_pos = dispatch.find("\"/api/status\"").expect("status route");
        let progress_pos = dispatch
            .find("\"/api/progress/stream\"")
            .expect("progress route");
        let static_pos = dispatch.find("serve_static").expect("static");
        assert!(progress_pos > status_pos);
        assert!(progress_pos < static_pos);
    }

    #[test]
    fn list_trace_files_sorted_by_timestamp() {
        let dir = std::env::temp_dir().join(format!("kalma2-traces-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("b.json"),
            r#"{"trace_id":"b","timestamp":"2026-08-15T10:00:01Z"}"#,
        )
        .unwrap();
        std::fs::write(
            dir.join("a.json"),
            r#"{"trace_id":"a","timestamp":"2026-08-15T10:00:00Z"}"#,
        )
        .unwrap();
        let files = list_trace_files(&dir);
        assert_eq!(files.len(), 2);
        assert!(files[0].to_string_lossy().contains("a.json"));
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn email_inbox_filters_actionable_only() {
        let repo = std::env::temp_dir().join(format!("sddia-inbox-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        std::fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"eda_instance":{"proofs":".SddIA/proofs"}}"#,
        )
        .unwrap();
        let dir = repo.join(".SddIA/proofs/email-triaged");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("noise.json"),
            r#"{"event_id":"n","timestamp":"2026-08-19T10:00:00Z","payload":{"verdict":"noise","message_uid":"1"}}"#,
        )
        .unwrap();
        std::fs::write(
            dir.join("act.json"),
            r#"{"event_id":"a","timestamp":"2026-08-19T11:00:00Z","payload":{"verdict":"actionable","message_uid":"2","from":"x@y","subject":"Go"}}"#,
        )
        .unwrap();
        let items = list_actionable_email_items(&repo);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0]["event_id"], "a");
        assert_eq!(items[0]["subject"], "Go");
        let _ = std::fs::remove_dir_all(repo);
    }

    #[test]
    fn email_routes_exist_in_dispatch() {
        let src = include_str!("main.rs");
        let prod = src.split("#[cfg(test)]").next().unwrap_or(src);
        assert!(prod.contains("\"/api/email-inbox\""));
        assert!(prod.contains("\"/api/email-quick-action\""));
    }

    #[test]
    fn sse_progress_frame_format() {
        let frame = String::from_utf8(sse_progress_frame(r#"{"trace_id":"x"}"#)).unwrap();
        assert!(frame.starts_with("event: progress\n"));
        assert!(frame.contains("data: {\"trace_id\":\"x\"}"));
    }

    #[test]
    fn sync_assets_route_exists_in_dispatch() {
        let src = include_str!("main.rs");
        let prod = src.split("#[cfg(test)]").next().unwrap_or(src);
        assert!(
            prod.contains("\"/api/sync-assets\""),
            "dispatch debe tener la ruta /api/sync-assets"
        );
        assert!(
            prod.contains("handle_sync_assets"),
            "handle_sync_assets debe estar definida en producción"
        );
    }

    #[test]
    fn sync_assets_handler_is_fire_and_forget() {
        let src = include_str!("main.rs");
        let prod = src.split("#[cfg(test)]").next().unwrap_or(src);
        let handler_start = prod.find("fn handle_sync_assets").expect("handle_sync_assets");
        let handler_slice = &prod[handler_start..];
        let next_fn = handler_slice[1..].find("\nfn ").unwrap_or(handler_slice.len());
        let handler_body = &handler_slice[..next_fn + 1];
        assert!(
            handler_body.contains("Stdio::null()"),
            "handle_sync_assets debe spawn con Stdio::null() (fire-and-forget DA-5)"
        );
        assert!(
            handler_body.contains("202"),
            "handle_sync_assets debe responder 202 accepted"
        );
        assert!(
            handler_body.contains("\"accepted\": true") || handler_body.contains("\"accepted\":true"),
            "response body debe incluir accepted:true"
        );
    }

    #[test]
    fn bridge_execute_path_has_no_eda_write_helpers() {
        // Audit estático AC-R2: producción (antes de #[cfg(test)]) sin sellado fractal.
        // Excluye el módulo de tests: include_str se autoincriminaría con este assert.
        let src = include_str!("main.rs");
        let prod = src.split("#[cfg(test)]").next().unwrap_or(src);
        assert!(
            !prod.contains("write_fractal_event"),
            "bridge prod no debe llamar write_fractal_event"
        );
        assert!(
            prod.contains("accept_execute") && prod.contains("Stdio::null()"),
            "camino execute debe usar spawn detach"
        );
        // Fire-and-forget: camino execute no usa join del orquestador.
        assert!(
            prod.contains("reply_accept_result") && prod.contains("AcceptedAck"),
            "execute debe responder acuse AcceptedAck"
        );
    }
}
