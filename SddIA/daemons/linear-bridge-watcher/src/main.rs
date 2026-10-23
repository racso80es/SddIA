use chrono::Utc;
use sddia_daemon_runtime::linear_bridge::assimilate_list_issues;
use sddia_daemon_runtime::linear_bridge_webhook::{
    spawn_lab_webhook_listener, WebhookProjectTarget,
};
use sddia_daemon_runtime::{find_repo_root, load_bus_topology, BusTopology, DaemonRuntime};
use serde_json::{json, Value};
use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};

const DAEMON_NAME: &str = "linear-bridge-watcher";
const HEARTBEAT_TICK_SECONDS: u64 = 10;

fn webhook_bind_host() -> String {
    env::var("SDDIA_LINEAR_WEBHOOK_BIND").unwrap_or_else(|_| "127.0.0.1".into())
}

fn webhook_port() -> u16 {
    env::var("SDDIA_LINEAR_WEBHOOK_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
}

fn webhook_secret() -> String {
    env::var("LINEAR_WEBHOOK_SECRET").unwrap_or_default()
}

fn poll_seconds() -> u64 {
    env::var("SDDIA_LINEAR_BRIDGE_POLL_SECONDS")
        .ok()
        .and_then(|s| s.parse().ok())
        .map(|n: u64| n.max(5))
        .unwrap_or(60)
}

fn lifecycle_tool_name(repo: &Path) -> Option<String> {
    let text = fs::read_to_string(repo.join("SddIA/core/capability-bindings.md")).ok()?;
    let mut seen = false;
    for line in text.lines() {
        if line.contains("capability_id:") && line.contains("tracker:lifecycle") {
            seen = true;
            continue;
        }
        if seen && line.contains("provider:") {
            let raw = line.split("provider:").nth(1)?.trim().trim_matches('"');
            let name = raw.split(':').nth(1).unwrap_or(raw).trim().trim_matches('"');
            if name.is_empty() {
                return None;
            }
            return Some(name.to_string());
        }
    }
    None
}

fn core_provider_file(repo: &Path) -> bool {
    let text = fs::read_to_string(repo.join(".SddIA/project.md")).unwrap_or_default();
    let Some(rest) = text.split("tracker:").nth(1) else {
        return true;
    };
    let block = rest.split("docs_layout:").next().unwrap_or(rest);
    !block.contains("provider: linear")
}

fn resolve_adapter_bin(repo: &Path) -> Option<std::path::PathBuf> {
    let name = lifecycle_tool_name(repo)?;
    for profile in ["release", "debug"] {
        let p = repo.join("SddIA/target").join(profile).join(&name);
        if p.is_file() {
            return Some(p);
        }
    }
    None
}

fn invoke_list_issues(repo: &Path, team_key: &str) -> Option<Vec<Value>> {
    let bin = resolve_adapter_bin(repo)?;
    let req = json!({
        "request": {
            "operation": "list_issues",
            "team_key": team_key,
        }
    });
    let mut cmd = Command::new(&bin);
    cmd.current_dir(repo)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if env::var("SDDIA_LAB_MOCK_OUTBOUND")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
    {
        cmd.env("SDDIA_LAB_MOCK_OUTBOUND", "1");
    }
    let mut child = cmd.spawn().ok()?;
    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        let _ = stdin.write_all(req.to_string().as_bytes());
    }
    let out = child.wait_with_output().ok()?;
    if !out.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    let line = stdout.lines().last().unwrap_or("").trim();
    let body: Value = serde_json::from_str(line).ok()?;
    let inner = body.get("result").or(Some(&body)).unwrap();
    if inner.get("success") == Some(&json!(false)) {
        return None;
    }
    let items = inner
        .pointer("/data/items")
        .or_else(|| inner.get("items"))
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    Some(items)
}

fn yaml_tracker_field(manifest: &str, key: &str) -> Option<String> {
    let mut in_tracker = false;
    let mut in_labels = false;
    for line in manifest.lines() {
        if line.trim() == "tracker:" {
            in_tracker = true;
            in_labels = false;
            continue;
        }
        if !in_tracker {
            continue;
        }
        if !line.starts_with(' ') && !line.starts_with('\t') && line.contains(':') {
            break;
        }
        let trimmed = line.trim();
        if trimmed == "labels:" {
            in_labels = true;
            continue;
        }
        if in_labels {
            if let Some(rest) = trimmed.strip_prefix(&format!("{key}:")) {
                return Some(rest.trim().trim_matches('"').to_string());
            }
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix(&format!("{key}:")) {
            return Some(rest.trim().trim_matches('"').to_string());
        }
    }
    None
}

fn manifest_has_tracker(manifest: &str) -> bool {
    manifest.contains("tracker:")
}

struct ProjectTarget {
    slug: String,
    team_key: String,
    label_hu: String,
    label_pbi: String,
}

fn discover_projects(repo: &Path) -> Vec<ProjectTarget> {
    let index = repo.join(".SddIA/projects");
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir(&index) else {
        return out;
    };
    for ent in entries.flatten() {
        let path = ent.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let slug = path.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string();
        let raw = fs::read_to_string(&path).unwrap_or_default();
        let manifest_rel = raw
            .lines()
            .find_map(|l| l.strip_prefix("manifest_ref:").map(|r| r.trim().trim_matches('"')))
            .unwrap_or(".SddIA/project.md");
        let project_root = raw
            .lines()
            .find_map(|l| l.strip_prefix("project_root:").map(|r| r.trim().trim_matches('"')))
            .unwrap_or("");
        if project_root.is_empty() {
            continue;
        }
        let root_path = if Path::new(project_root).is_absolute() {
            Path::new(project_root).to_path_buf()
        } else {
            repo.join(project_root)
        };
        let manifest_path = root_path.join(manifest_rel);
        let manifest = fs::read_to_string(manifest_path).unwrap_or_default();
        if !manifest_has_tracker(&manifest) {
            continue;
        }
        let team_key = yaml_tracker_field(&manifest, "team_key").unwrap_or_else(|| "OSC".into());
        let label_hu = yaml_tracker_field(&manifest, "hu").unwrap_or_else(|| "hu".into());
        let label_pbi = yaml_tracker_field(&manifest, "pbi").unwrap_or_else(|| "pbi".into());
        out.push(ProjectTarget {
            slug,
            team_key,
            label_hu,
            label_pbi,
        });
    }
    out
}

fn poll_once(repo: &Path) {
    let now = Utc::now();
    for proj in discover_projects(repo) {
        let Some(items) = invoke_list_issues(repo, &proj.team_key) else {
            eprintln!("[LINEAR-BRIDGE] list_issues falló para {}", proj.slug);
            continue;
        };
        match assimilate_list_issues(
            repo,
            &proj.slug,
            &proj.label_hu,
            &proj.label_pbi,
            &items,
            now,
        ) {
            Ok((paths, _)) => {
                for p in paths {
                    if let Ok(rel) = p.strip_prefix(repo) {
                        eprintln!("[LINEAR-BRIDGE] Tracker_Issue_Changed: {}", rel.display());
                    }
                }
            }
            Err(e) => eprintln!("[LINEAR-BRIDGE] assimilate {}: {e}", proj.slug),
        }
    }
}

fn spawn_heartbeat(centinela: Arc<Mutex<DaemonRuntime>>, top: BusTopology) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        loop {
            if let Ok(mut rt) = centinela.lock() {
                let _ = rt.tick(&top);
            }
            thread::sleep(Duration::from_secs(HEARTBEAT_TICK_SECONDS));
        }
    })
}

fn main() {
    let repo = find_repo_root().expect("repo root");
    if core_provider_file(&repo) {
        eprintln!("[LINEAR-BRIDGE] provider file: sin poll ni webhook");
        return;
    }
    let top = load_bus_topology(&repo);
    let mut centinela = DaemonRuntime::new(repo.clone(), DAEMON_NAME);
    if let Err(e) = centinela.bootstrap(&top) {
        eprintln!("[LINEAR-BRIDGE] {e}");
        std::process::exit(1);
    }
    let shared = Arc::new(Mutex::new(centinela));
    let _hb = spawn_heartbeat(Arc::clone(&shared), top.clone());
    let wh_projects: Vec<WebhookProjectTarget> = discover_projects(&repo)
        .into_iter()
        .map(|p| WebhookProjectTarget {
            slug: p.slug,
            label_hu: p.label_hu,
            label_pbi: p.label_pbi,
        })
        .collect();
    let _wh = spawn_lab_webhook_listener(
        repo.clone(),
        webhook_bind_host(),
        webhook_port(),
        webhook_secret(),
        wh_projects,
    );
    eprintln!("[LINEAR-BRIDGE] polling cada {}s", poll_seconds());
    loop {
        poll_once(&repo);
        if let Ok(mut rt) = shared.lock() {
            let _ = rt.tick(&top);
        }
        thread::sleep(Duration::from_secs(poll_seconds()));
    }
}
