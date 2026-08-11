use sddia_daemon_runtime::github_bridge::{load_bridge_state, process_pr};
use sddia_daemon_runtime::{find_repo_root, load_bus_topology, BusTopology, DaemonRuntime};
use serde_json::{json, Value};
use std::env;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};

const DEFAULT_REPO: &str = "racso80es/SddIA";
const SIMULATION_REL: &str = ".SddIA/.dev/remote_pr_simulation.json";
const HEARTBEAT_TICK_SECONDS: u64 = 10;
const HEARTBEAT_EMIT_FAIL_BUDGET: u32 = 5;

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
                    eprintln!(
                        "[GITHUB-BRIDGE] heartbeat keepalive: {e} (fail {fails}/{HEARTBEAT_EMIT_FAIL_BUDGET})"
                    );
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

fn lab_simulate() -> bool {
    matches!(
        env::var("SDDIA_LAB_SIMULATE_REMOTE_PR")
            .unwrap_or_default()
            .to_ascii_lowercase()
            .as_str(),
        "1" | "true" | "yes"
    )
}

fn poll_seconds() -> u64 {
    env::var("SDDIA_GITHUB_BRIDGE_POLL_SECONDS")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .map(|n| n.max(5))
        .unwrap_or(30)
}

fn github_request(url: &str, token: &str) -> Option<Value> {
    let agent = ureq::agent();
    match agent
        .get(url)
        .set("Accept", "application/vnd.github+json")
        .set("Authorization", &format!("Bearer {token}"))
        .set("User-Agent", "sddia-github-bridge-watcher")
        .timeout(Duration::from_secs(30))
        .call()
    {
        Ok(resp) => resp.into_json().ok(),
        Err(e) => {
            eprintln!("[GITHUB-BRIDGE] HTTP error: {e}");
            None
        }
    }
}

fn parse_repo_slug(slug: &str) -> Option<(String, String)> {
    let mut parts = slug.splitn(2, '/');
    let owner = parts.next()?.trim().to_string();
    let name = parts.next()?.trim().to_string();
    if owner.is_empty() || name.is_empty() {
        None
    } else {
        Some((owner, name))
    }
}

fn pr_record_from_github(item: &Value, repository: &str) -> Value {
    let head = item.get("head").and_then(|v| v.as_object());
    let branch = head
        .and_then(|h| h.get("ref"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    json!({
        "repository": repository,
        "branch": branch,
        "pr_url": item.get("html_url").and_then(|v| v.as_str()).unwrap_or(""),
        "origin_agent": "jules",
        "github_number": item.get("number"),
    })
}

fn validate_pr_against_github(repo_slug: &str, pr: &Value, token: &str) -> bool {
    let Some((owner, name)) = parse_repo_slug(repo_slug) else {
        return false;
    };
    let url = if let Some(num) = pr.get("github_number") {
        format!("https://api.github.com/repos/{owner}/{name}/pulls/{num}")
    } else if let Some(pr_url) = pr.get("pr_url").and_then(|v| v.as_str()) {
        if !pr_url.contains("/pull/") {
            return false;
        }
        let num = pr_url.trim_end_matches('/').rsplit('/').next().unwrap_or("");
        format!("https://api.github.com/repos/{owner}/{name}/pulls/{num}")
    } else {
        return false;
    };
    let Some(remote) = github_request(&url, token) else {
        return false;
    };
    let head = remote.get("head").and_then(|v| v.as_object());
    let remote_branch = head.and_then(|h| h.get("ref")).and_then(|v| v.as_str());
    remote.get("html_url").and_then(|v| v.as_str()) == pr.get("pr_url").and_then(|v| v.as_str())
        && remote_branch == pr.get("branch").and_then(|v| v.as_str())
}

fn fetch_open_prs(_repo: &Path, repository: &str, token: &str) -> Vec<Value> {
    let Some((owner, name)) = parse_repo_slug(repository) else {
        return vec![];
    };
    let url = format!("https://api.github.com/repos/{owner}/{name}/pulls?state=open&per_page=30");
    let Some(data) = github_request(&url, token) else {
        return vec![];
    };
    let Some(items) = data.as_array() else {
        return vec![];
    };
    let mut records = Vec::new();
    for item in items {
        let pr = pr_record_from_github(item, repository);
        let branch = pr.get("branch").and_then(|v| v.as_str()).unwrap_or("");
        let pr_url = pr.get("pr_url").and_then(|v| v.as_str()).unwrap_or("");
        if branch.is_empty() || pr_url.is_empty() {
            continue;
        }
        if validate_pr_against_github(repository, &pr, token) {
            records.push(pr);
        } else {
            eprintln!("[GITHUB-BRIDGE] Filtro A: descartado PR corrupto {pr_url}");
        }
    }
    records
}

fn fetch_lab_simulation(repo: &Path) -> Vec<Value> {
    let path = repo.join(SIMULATION_REL);
    if !path.is_file() {
        eprintln!("[GITHUB-BRIDGE] Lab: sin fixture {SIMULATION_REL}");
        return vec![];
    }
    let Ok(raw) = fs::read_to_string(&path) else {
        return vec![];
    };
    let Ok(data) = serde_json::from_str::<Value>(&raw) else {
        return vec![];
    };
    if let Some(obj) = data.as_object() {
        return vec![Value::Object(obj.clone())];
    }
    if let Some(arr) = data.as_array() {
        return arr.iter().filter(|x| x.is_object()).cloned().collect();
    }
    vec![]
}

fn run_cycle(
    repo: &Path,
    shared: &Arc<Mutex<DaemonRuntime>>,
    top: &sddia_daemon_runtime::BusTopology,
) -> i32 {
    let mut state = load_bridge_state(repo);
    let repository = env::var("SDDIA_GITHUB_REPOSITORY")
        .unwrap_or_else(|_| DEFAULT_REPO.into())
        .trim()
        .to_string();

    let candidates = if lab_simulate() {
        fetch_lab_simulation(repo)
    } else {
        let token = env::var("GITHUB_TOKEN").unwrap_or_default();
        let token = token.trim();
        if token.is_empty() {
            eprintln!(
                "[GITHUB-BRIDGE] GITHUB_TOKEN ausente; use SDDIA_LAB_SIMULATE_REMOTE_PR=1 en lab"
            );
            return 1;
        }
        fetch_open_prs(repo, repository.trim(), token)
    };

    if candidates.is_empty() {
        if let Ok(mut rt) = shared.lock() {
            let _ = rt.tick(top);
        }
        return 0;
    }

    for pr in candidates {
        let pr_url = pr.get("pr_url").and_then(|v| v.as_str()).unwrap_or("");
        if pr_url.is_empty() || state.processed_pr_urls.iter().any(|u| u == pr_url) {
            continue;
        }
        match process_pr(repo, &pr, &mut state) {
            Ok(true) => {
                if let Ok(mut rt) = shared.lock() {
                    rt.note_stimulus();
                }
            }
            Ok(false) => {}
            Err(e) => eprintln!("[GITHUB-BRIDGE] process_pr error: {e}"),
        }
    }
    if let Ok(mut rt) = shared.lock() {
        let _ = rt.tick(top);
    }
    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let once = args.iter().any(|a| a == "--once");

    let repo = match find_repo_root() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };

    let top = load_bus_topology(&repo);
    let mut centinela = DaemonRuntime::new(repo.clone(), "github-bridge-watcher");
    if let Err(e) = centinela.bootstrap(&top) {
        eprintln!("[GITHUB-BRIDGE] {e}");
        std::process::exit(1);
    }
    let shared = Arc::new(Mutex::new(centinela));

    if once {
        let code = run_cycle(&repo, &shared, &top);
        if let Ok(mut rt) = shared.lock() {
            rt.shutdown();
        }
        std::process::exit(code);
    }

    let _hb = spawn_heartbeat_worker(Arc::clone(&shared), top.clone());
    println!(
        "[GITHUB-BRIDGE] Iniciado (poll={}s, keepalive heartbeat cada {HEARTBEAT_TICK_SECONDS}s)",
        poll_seconds()
    );
    loop {
        run_cycle(&repo, &shared, &top);
        thread::sleep(Duration::from_secs(poll_seconds()));
    }
}
