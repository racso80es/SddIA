//! Handler nativo `system-vitality-probe`.

use super::super::daemons::{iso_now, load_eda_pending, load_cumulo, state_dir, write_json_atomic};
use crate::envelope::OrchestratorEnvelope;
use serde_json::{json, Value};
use std::fs;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::path::Path;
use std::process::Command;
use std::time::Duration;
use uuid::Uuid;

const STATE_FILE: &str = "vitality-probe.json";

#[derive(Clone)]
struct ProbeResult {
    id: &'static str,
    ok: bool,
    cause: String,
}

fn state_path(repo: &Path) -> Result<std::path::PathBuf, String> {
    Ok(state_dir(repo)?.join(STATE_FILE))
}

fn load_probe_state(repo: &Path) -> Value {
    let Ok(path) = state_path(repo) else {
        return json!({"probes": {}});
    };
    if !path.is_file() {
        return json!({"probes": {}});
    }
    fs::read_to_string(&path)
        .ok()
        .and_then(|t| serde_json::from_str(&t).ok())
        .unwrap_or_else(|| json!({"probes": {}}))
}

fn save_probe_state(repo: &Path, state: &Value) -> Result<(), String> {
    write_json_atomic(&state_path(repo)?, state)
}

fn telemetry_dir(repo: &Path) -> std::path::PathBuf {
    load_cumulo(repo)
        .ok()
        .and_then(|m| m.get("eda_fractal").cloned())
        .and_then(|f| f.get("telemetry").and_then(|v| v.as_str()).map(str::to_string))
        .map(|rel| {
            let t = rel.trim().trim_start_matches("./");
            repo.join(t)
        })
        .unwrap_or_else(|| repo.join(".events/telemetry"))
}

fn probe_bus_topology(repo: &Path) -> ProbeResult {
    let cumulo = repo.join("SddIA/core/cumulo.paths.json");
    if !cumulo.is_file() {
        return ProbeResult {
            id: "bus.topology",
            ok: false,
            cause: "ausente SddIA/core/cumulo.paths.json".into(),
        };
    }
    let raw = match fs::read_to_string(&cumulo) {
        Ok(t) => t,
        Err(e) => {
            return ProbeResult {
                id: "bus.topology",
                ok: false,
                cause: format!("no readable cumulo.paths.json: {e}"),
            };
        }
    };
    let cfg: Value = match serde_json::from_str(&raw) {
        Ok(v) => v,
        Err(e) => {
            return ProbeResult {
                id: "bus.topology",
                ok: false,
                cause: format!("cumulo.paths.json JSON inválido: {e}"),
            };
        }
    };
    let Some(fractal) = cfg.get("eda_fractal").and_then(|v| v.as_object()) else {
        return ProbeResult {
            id: "bus.topology",
            ok: false,
            cause: "eda_fractal ausente en cumulo.paths.json".into(),
        };
    };
    for (key, val) in fractal {
        if key.ends_with("_subscriptions") {
            continue;
        }
        let Some(rel) = val.as_str() else { continue };
        if rel.ends_with(".json") || rel.ends_with(".md") {
            continue;
        }
        let path = repo.join(rel.trim().trim_start_matches("./"));
        if !path.exists() {
            return ProbeResult {
                id: "bus.topology",
                ok: false,
                cause: format!("hoja eda_fractal.{key} ausente: {rel}"),
            };
        }
    }
    let overlay = repo.join(".SddIA/local.paths.json");
    if overlay.is_file() {
        match fs::read_to_string(&overlay).ok().and_then(|t| serde_json::from_str::<Value>(&t).ok()) {
            Some(Value::Object(m)) if m.is_empty() => {
                return ProbeResult {
                    id: "bus.topology",
                    ok: false,
                    cause: "overlay .SddIA/local.paths.json es {}".into(),
                };
            }
            Some(_) => {}
            None => {
                return ProbeResult {
                    id: "bus.topology",
                    ok: false,
                    cause: "overlay .SddIA/local.paths.json JSON inválido".into(),
                };
            }
        }
    }
    ProbeResult {
        id: "bus.topology",
        ok: true,
        cause: "topología parseable".into(),
    }
}

fn qa_bin(repo: &Path) -> std::path::PathBuf {
    for rel in ["SddIA/target/debug/sddia-qa", "SddIA/target/release/sddia-qa"] {
        let p = repo.join(rel);
        if p.is_file() {
            return p;
        }
    }
    repo.join("SddIA/target/debug/sddia-qa")
}

fn probe_tools_index(repo: &Path) -> ProbeResult {
    let bin = qa_bin(repo);
    if !bin.is_file() {
        return ProbeResult {
            id: "cumulo.tools_index",
            ok: false,
            cause: format!("sddia-qa ausente: {}", bin.display()),
        };
    }
    match Command::new(&bin)
        .arg("verify-tools-index")
        .current_dir(repo)
        .output()
    {
        Ok(out) if out.status.success() => ProbeResult {
            id: "cumulo.tools_index",
            ok: true,
            cause: "verify-tools-index OK".into(),
        },
        Ok(out) => {
            let err = String::from_utf8_lossy(&out.stderr);
            let so = String::from_utf8_lossy(&out.stdout);
            ProbeResult {
                id: "cumulo.tools_index",
                ok: false,
                cause: format!(
                    "verify-tools-index FAILED: {}",
                    if !err.trim().is_empty() {
                        err.trim()
                    } else {
                        so.trim()
                    }
                ),
            }
        }
        Err(e) => ProbeResult {
            id: "cumulo.tools_index",
            ok: false,
            cause: format!("spawn sddia-qa: {e}"),
        },
    }
}

fn probe_cerbero_config(repo: &Path) -> ProbeResult {
    let ctx = repo.join("SddIA/norms/execution-contexts.md");
    if !ctx.is_file() {
        return ProbeResult {
            id: "cerbero.config",
            ok: false,
            cause: "ausente SddIA/norms/execution-contexts.md".into(),
        };
    }
    let raw = match fs::read_to_string(&ctx) {
        Ok(t) => t,
        Err(e) => {
            return ProbeResult {
                id: "cerbero.config",
                ok: false,
                cause: format!("no readable execution-contexts.md: {e}"),
            };
        }
    };
    if !raw.trim_start().starts_with("---") {
        return ProbeResult {
            id: "cerbero.config",
            ok: false,
            cause: "execution-contexts.md sin frontmatter YAML".into(),
        };
    }
    let revoked = repo.join(".SddIA/cerbero/revoked_entities.json");
    if revoked.is_file() {
        match fs::read_to_string(&revoked).ok().and_then(|t| serde_json::from_str::<Value>(&t).ok()) {
            Some(_) => {}
            None => {
                return ProbeResult {
                    id: "cerbero.config",
                    ok: false,
                    cause: "JSON inválido: .SddIA/cerbero/revoked_entities.json".into(),
                };
            }
        }
    }
    ProbeResult {
        id: "cerbero.config",
        ok: true,
        cause: "execution-contexts.md parseable".into(),
    }
}

fn kalma_port() -> u16 {
    std::env::var("SDDIA_CLIENT_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8765)
}

fn http_get_status(host: &str, port: u16, path: &str) -> Result<u16, String> {
    let addr = format!("{host}:{port}");
    let sock: SocketAddr = addr
        .to_socket_addrs()
        .map_err(|e| e.to_string())?
        .next()
        .ok_or_else(|| format!("no resolve {addr}"))?;
    let mut stream = TcpStream::connect_timeout(&sock, Duration::from_secs(2))
        .map_err(|e| format!("connect {addr}: {e}"))?;
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .map_err(|e| e.to_string())?;
    stream
        .set_write_timeout(Some(Duration::from_secs(2)))
        .map_err(|e| e.to_string())?;
    write!(
        stream,
        "GET {path} HTTP/1.0\r\nHost: {host}\r\nConnection: close\r\n\r\n"
    )
    .map_err(|e| e.to_string())?;
    let mut buf = [0u8; 96];
    let n = stream.read(&mut buf).map_err(|e| e.to_string())?;
    let head = String::from_utf8_lossy(&buf[..n]);
    let code = head
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse::<u16>().ok())
        .ok_or_else(|| format!("respuesta HTTP ilegible: {}", head.chars().take(40).collect::<String>()))?;
    Ok(code)
}

fn probe_kalma2_http() -> ProbeResult {
    let port = kalma_port();
    match http_get_status("127.0.0.1", port, "/") {
        Ok(code) if (200..400).contains(&code) => ProbeResult {
            id: "kalma2.http",
            ok: true,
            cause: format!("GET / → {code}"),
        },
        Ok(code) => ProbeResult {
            id: "kalma2.http",
            ok: false,
            cause: format!("GET http://127.0.0.1:{port}/ status {code}"),
        },
        Err(e) => ProbeResult {
            id: "kalma2.http",
            ok: false,
            cause: e,
        },
    }
}

fn emit_vitality_event(repo: &Path, probes: &[ProbeResult], verdict: &str) -> Result<String, String> {
    let event_id = Uuid::new_v4().to_string();
    let event = json!({
        "event_id": event_id,
        "event_type": "System_Vitality_Probed",
        "timestamp": iso_now(),
        "emitter_agent": "system-vitality-probe",
        "payload": {
            "probes": probes.iter().map(|p| json!({
                "id": p.id,
                "ok": p.ok,
                "cause": p.cause,
            })).collect::<Vec<_>>(),
            "verdict": verdict,
            "red_probe_ids": probes.iter().filter(|p| !p.ok).map(|p| p.id).collect::<Vec<_>>(),
        },
    });
    let dir = telemetry_dir(repo);
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    write_json_atomic(&dir.join(format!("{event_id}.json")), &event)?;
    Ok(event_id)
}

fn emit_probe_fracture(repo: &Path, probe: &ProbeResult) -> Result<Value, String> {
    let pending = load_eda_pending(repo)?;
    let event_id = Uuid::new_v4().to_string();
    let event = json!({
        "event_id": event_id,
        "event_type": "System_Fracture_Detected",
        "timestamp": iso_now(),
        "emitter_agent": "argos",
        "payload": {
            "process_name": "system-vitality-probe",
            "error_trace": format!("sonda {} en rojo: {}", probe.id, probe.cause),
            "agent_emitter": "argos",
            "attempted_action": "system-vitality-probe",
            "friction_id": format!("F-VITALIDAD-{}", probe.id.replace('.', "-").to_uppercase()),
        },
    });
    let target = repo.join(&pending).join(format!("{event_id}.json"));
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    write_json_atomic(&target, &event)?;
    Ok(json!({
        "event_id": event_id,
        "target_path": target.strip_prefix(repo).unwrap_or(&target).to_string_lossy().replace('\\', "/"),
        "probe_id": probe.id,
    }))
}

pub fn run(repo: &Path, _inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let probes = vec![
        probe_bus_topology(repo),
        probe_tools_index(repo),
        probe_cerbero_config(repo),
        probe_kalma2_http(),
    ];
    let all_ok = probes.iter().all(|p| p.ok);
    let verdict = if all_ok { "ok" } else { "degraded" };
    let event_id = emit_vitality_event(repo, &probes, verdict)?;

    let mut state = load_probe_state(repo);
    if !state.get("probes").map(|v| v.is_object()).unwrap_or(false) {
        state["probes"] = json!({});
    }
    let mut fractures = Vec::new();
    for p in &probes {
        let already = state
            .get("probes")
            .and_then(|m| m.get(p.id))
            .and_then(|e| e.get("fracture_event_id"))
            .is_some();
        if p.ok {
            if let Some(obj) = state
                .get_mut("probes")
                .and_then(|m| m.as_object_mut())
            {
                obj.insert(
                    p.id.to_string(),
                    json!({"verdict": "green"}),
                );
            }
        } else if !already {
            let seal = emit_probe_fracture(repo, p)?;
            if let Some(obj) = state
                .get_mut("probes")
                .and_then(|m| m.as_object_mut())
            {
                obj.insert(
                    p.id.to_string(),
                    json!({
                        "verdict": "red",
                        "fracture_event_id": seal.get("event_id"),
                    }),
                );
            }
            fractures.push(seal);
        }
    }
    save_probe_state(repo, &state)?;

    let censo: Vec<Value> = probes
        .iter()
        .map(|p| {
            json!({
                "id": p.id,
                "ok": p.ok,
                "cause": p.cause,
            })
        })
        .collect();

    Ok(OrchestratorEnvelope {
        success: true,
        status_code: 0,
        data: Some(json!({
            "verdict": verdict,
            "probes": censo,
            "vitality_event_id": event_id,
            "fractures_emitted": fractures,
        })),
        error: None,
        execution_report: Some(json!({
            "process_name": "system-vitality-probe",
            "phases": [{
                "phase_name": "Sondas",
                "status": "executed",
                "handler": "system-vitality-probe-core",
            }],
        })),
        exit_code: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cerbero_red_when_execution_contexts_missing() {
        let dir = tempfile::TempDir::new().unwrap();
        let r = probe_cerbero_config(dir.path());
        assert!(!r.ok);
        assert!(r.cause.contains("execution-contexts.md"), "{}", r.cause);
    }

    #[test]
    fn cerbero_green_with_frontmatter() {
        let dir = tempfile::TempDir::new().unwrap();
        let repo = dir.path();
        std::fs::create_dir_all(repo.join("SddIA/norms")).unwrap();
        std::fs::write(
            repo.join("SddIA/norms/execution-contexts.md"),
            "---\nname: execution-contexts\n---\n# x\n",
        )
        .unwrap();
        let r = probe_cerbero_config(repo);
        assert!(r.ok, "{}", r.cause);
    }
}
