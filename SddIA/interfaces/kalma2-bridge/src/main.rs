use serde::Deserialize;
use std::io::Read;
use std::net::Ipv4Addr;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use tiny_http::{Header, Method, Response, Server};

#[derive(Deserialize)]
struct InteractReq {
    prompt: String,
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

fn run_orchestrator(repo: &Path, bin: &Path, prompt: &str) -> Result<String, String> {
    let inputs = serde_json::json!({ "prompt": prompt }).to_string();
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

fn find_domain_event(domain_dir: &Path, dead_dir: &Path, event_id: &str) -> Option<(PathBuf, serde_json::Value, bool)> {
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
    None
}

fn find_pec_by_correlation(orch_dir: &Path, correlation_id: &str) -> Option<serde_json::Value> {
    let Ok(entries) = std::fs::read_dir(orch_dir) else {
        return None;
    };
    let mut best: Option<serde_json::Value> = None;
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
        if cid == correlation_id {
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
        if st == "success" {
            return (
                "completed",
                format!("Proceso «{pname}» completado (PEC correlacionado)."),
            );
        }
        return (
            "failed",
            format!("Proceso «{pname}» falló (status={st})."),
        );
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
    let domain_hit = find_domain_event(&domain_dir, &dead_dir, event_id);
    let pec = find_pec_by_correlation(&orch_dir, event_id);

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
        })
    } else {
        serde_json::json!({
            "found": false,
            "event_id": null,
            "process_name": null,
            "process_status": null
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
    match run_orchestrator(repo, &bin, &prompt) {
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

fn dispatch(req: tiny_http::Request, repo: Arc<PathBuf>, ui_root: Arc<PathBuf>) {
    let path = req.url().split('?').next().unwrap_or("/");
    match (req.method(), path) {
        (Method::Post, "/api/interact") => handle_interact(req, &repo),
        (Method::Get, "/api/status") => handle_status(req, &repo),
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
}
