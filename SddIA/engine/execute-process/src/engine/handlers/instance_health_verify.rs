//! Handler nativo `instance-health-verify` — verificación post-deploy de instancia (HTTP WUI + systemd).

use super::super::daemons::iso_now;
use crate::envelope::OrchestratorEnvelope;
use serde_json::{json, Value};
use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

const DAEMON_NAMES: &[&str] = &[
    "event-watcher",
    "event-sweeper",
    "kalma2-bridge",
    "email-watcher",
    "telegram-watcher",
    "github-bridge-watcher",
    "iota-publish-relay",
];

fn str_field(v: &Value, key: &str) -> Result<String, String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .ok_or_else(|| format!("input requerido: {key}"))
}

fn resolve_instance_root(repo: &Path, inputs: &Value) -> Result<PathBuf, String> {
    let raw = str_field(inputs, "instance_root")?;
    let p = PathBuf::from(&raw);
    if p.is_absolute() {
        return Ok(p);
    }
    Ok(repo.join(p))
}

fn systemd_escape(root: &Path) -> Result<String, String> {
    let out = Command::new("systemd-escape")
        .args(["-p", &root.to_string_lossy()])
        .output()
        .map_err(|e| format!("systemd-escape: {e}"))?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).trim().to_string());
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn read_env_key(env_path: &Path, key: &str) -> Option<String> {
    let text = fs::read_to_string(env_path).ok()?;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (k, v) = line.split_once('=')?;
        if k.trim() == key {
            let mut val = v.trim().trim_matches('"').to_string();
            if !val.is_empty() {
                return Some(val);
            }
        }
    }
    None
}

fn systemctl_user(args: &[&str]) -> Result<String, String> {
    let out = Command::new("systemctl")
        .arg("--user")
        .args(args)
        .output()
        .map_err(|e| format!("systemctl: {e}"))?;
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn unit_enabled(esc: &str, stem: &str) -> bool {
    let unit = format!("sddia-{stem}@{esc}.service");
    systemctl_user(&["is-enabled", &unit])
        .map(|s| s == "enabled" || s == "static")
        .unwrap_or(false)
}

fn unit_active(esc: &str, stem: &str) -> bool {
    let unit = format!("sddia-{stem}@{esc}.service");
    systemctl_user(&["is-active", &unit])
        .map(|s| s == "active")
        .unwrap_or(false)
}

fn unit_nrestarts(esc: &str, stem: &str) -> u64 {
    let unit = format!("sddia-{stem}@{esc}.service");
    let show = systemctl_user(&["show", "-p", "NRestarts", "--value", &unit]).unwrap_or_default();
    show.parse().unwrap_or(0)
}

fn http_get_status(host: &str, port: u16, path: &str) -> Result<u16, String> {
    let addr = format!("{host}:{port}");
    let mut stream = TcpStream::connect(&addr).map_err(|e| format!("connect {addr}: {e}"))?;
    stream
        .set_read_timeout(Some(Duration::from_secs(3)))
        .map_err(|e| e.to_string())?;
    stream
        .set_write_timeout(Some(Duration::from_secs(3)))
        .map_err(|e| e.to_string())?;
    write!(
        stream,
        "GET {path} HTTP/1.0\r\nHost: {host}\r\nConnection: close\r\n\r\n"
    )
    .map_err(|e| e.to_string())?;
    let mut buf = [0u8; 128];
    let n = stream.read(&mut buf).map_err(|e| e.to_string())?;
    let head = String::from_utf8_lossy(&buf[..n]);
    head.split_whitespace()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| format!("HTTP ilegible: {}", head.chars().take(48).collect::<String>()))
}

fn audit_ts() -> String {
    chrono::Utc::now().format("%Y%m%dT%H%M%SZ").to_string()
}

pub fn run(repo: &Path, inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let instance_root = resolve_instance_root(repo, inputs)?;
    let esc = systemd_escape(&instance_root)?;
    let sddia = instance_root.join(".SddIA");
    let env_path = sddia.join(".dev/.env");

    let mut rows: Vec<Value> = Vec::new();
    let mut apto = true;

    let vault_ok = env_path.is_file();
    if !vault_ok {
        apto = false;
    }

    let profile_ok = sddia.join("active-domain-profile.json").is_file();
    if !profile_ok {
        apto = false;
    }

    for name in DAEMON_NAMES {
        if !unit_enabled(&esc, name) {
            continue;
        }
        let active = unit_active(&esc, name);
        let restarts = unit_nrestarts(&esc, name);
        let row_ok = active && restarts <= 1;
        if !row_ok {
            apto = false;
        }
        rows.push(json!({
            "unit": format!("sddia-{name}@{esc}.service"),
            "enabled": true,
            "active": active,
            "n_restarts": restarts,
            "ok": row_ok,
        }));
    }

    let port = read_env_key(&env_path, "SDDIA_CLIENT_PORT")
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(8765);
    let http_ok = match http_get_status("127.0.0.1", port, "/api/status") {
        Ok(code) if (200..400).contains(&code) => true,
        Ok(code) => {
            apto = false;
            rows.push(json!({"check": "wui_http", "ok": false, "cause": format!("status {code}")}));
            false
        }
        Err(e) => {
            apto = false;
            rows.push(json!({"check": "wui_http", "ok": false, "cause": e}));
            false
        }
    };
    if http_ok {
        rows.push(json!({"check": "wui_http", "ok": true, "port": port}));
    }

    if !vault_ok {
        rows.push(json!({"check": "vault_env_present", "ok": false}));
    } else {
        rows.push(json!({"check": "vault_env_present", "ok": true}));
    }
    if !profile_ok {
        rows.push(json!({"check": "active_domain_profile", "ok": false}));
    } else {
        rows.push(json!({"check": "active_domain_profile", "ok": true}));
    }

    let verdict = if apto { "APTO" } else { "NO-APTO" };
    let ts = audit_ts();
    let audit_name = format!("instance-deploy-{esc}-{ts}.md");
    let audit_dir = repo.join("docs/audits");
    fs::create_dir_all(&audit_dir).map_err(|e| e.to_string())?;
    let audit_path = audit_dir.join(&audit_name);

    let mut md = String::new();
    md.push_str("---\n");
    md.push_str(&format!("title: \"Instance deploy verify {esc}\"\n"));
    md.push_str(&format!("created: \"{}\"\n", iso_now()));
    md.push_str(&format!("verdict: {verdict}\n"));
    md.push_str(&format!("instance_root: \"{}\"\n", instance_root.display()));
    md.push_str("---\n\n");
    md.push_str(&format!("# Verificación post-deploy — {esc}\n\n"));
    md.push_str(&format!("**Veredicto:** {verdict}\n\n"));
    md.push_str("| Unidad / check | Estado |\n|---|---|\n");
    for row in &rows {
        if let Some(u) = row.get("unit").and_then(|v| v.as_str()) {
            let ok = row.get("ok").and_then(|v| v.as_bool()).unwrap_or(false);
            md.push_str(&format!("| `{u}` | {} |\n", if ok { "OK" } else { "FAIL" }));
        } else if let Some(c) = row.get("check").and_then(|v| v.as_str()) {
            let ok = row.get("ok").and_then(|v| v.as_bool()).unwrap_or(false);
            md.push_str(&format!("| {c} | {} |\n", if ok { "OK" } else { "FAIL" }));
        }
    }
    fs::write(&audit_path, md).map_err(|e| e.to_string())?;

    let rel_audit = audit_path
        .strip_prefix(repo)
        .unwrap_or(&audit_path)
        .to_string_lossy()
        .replace('\\', "/");

    Ok(OrchestratorEnvelope {
        success: true,
        status_code: 0,
        data: Some(json!({
            "verdict": verdict,
            "audit_path": rel_audit,
            "esc": esc,
            "units": rows,
        })),
        error: None,
        execution_report: Some(json!({
            "process_name": "instance-health-verify",
            "phases": [{
                "phase_name": "Verificar instancia",
                "status": "executed",
                "handler": "instance-health-verify-native",
            }],
        })),
        exit_code: 0,
    })
}
