use serde::Deserialize;
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

fn resolve_orchestrator(repo: &Path) -> Option<PathBuf> {
    if let Ok(o) = std::env::var("SDDIA_EXECUTE_PROCESS_BIN") {
        let trimmed = o.trim();
        if !trimmed.is_empty() {
            return Some(PathBuf::from(trimmed));
        }
    }
    for rel in [
        "SddIA/target/debug/execute-process",
        "SddIA/target/release/execute-process",
    ] {
        let candidate = repo.join(rel);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}

fn client_timeout_secs() -> u64 {
    std::env::var("SDDIA_CLIENT_TIMEOUT_SECONDS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(120)
}

fn json_header() -> Header {
    Header::from_bytes(&b"Content-Type"[..], &b"application/json; charset=utf-8"[..]).unwrap()
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
        Some(b) => b,
        None => {
            reply(
                req,
                500,
                r#"{"success":false,"message":"orquestador no encontrado","exit_code":1}"#.into(),
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
        assert!(bin.is_some(), "execute-process debe existir tras cargo build");
    }
}
