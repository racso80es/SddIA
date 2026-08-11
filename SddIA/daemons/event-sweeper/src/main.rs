use sddia_daemon_runtime::eda_sweep::{sweep_once, POLL_SECONDS, SweepReport};
use sddia_daemon_runtime::{find_repo_root, load_bus_topology, BusTopology, DaemonRuntime};
use std::env;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::{thread, time::Duration};

const HEARTBEAT_TICK_SECONDS: u64 = 10;
const HEARTBEAT_EMIT_FAIL_BUDGET: u32 = 5;
/// Cadencia mínima del ingest de régimen (Vía A) vía sweeper.
const HEARTBEAT_AUDIT_SWEEP_SECONDS: u64 = 30;

fn print_report(report: &SweepReport, json: bool) {
    if json {
        println!(
            "{}",
            serde_json::json!({
                "purged": report.purged,
                "dead_lettered": report.dead_lettered,
                "kaizen_alerts": report.kaizen_alerts,
                "kaizen_finalized": report.kaizen_finalized,
                "skipped": report.skipped,
            })
        );
        return;
    }
    if !report.purged.is_empty() {
        println!("[SWEEPER] Purgados: {:?}", report.purged);
    }
    if !report.dead_lettered.is_empty() {
        println!("[SWEEPER] Dead-letter: {:?}", report.dead_lettered);
    }
    if !report.kaizen_finalized.is_empty() {
        println!("[SWEEPER] Kaizen terminalizados: {:?}", report.kaizen_finalized);
    }
    if !report.kaizen_alerts.is_empty() {
        println!("[SWEEPER] Alertas Kaizen: {:?}", report.kaizen_alerts);
    }
}

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
                        "[SWEEPER] heartbeat keepalive: {e} (fail {fails}/{HEARTBEAT_EMIT_FAIL_BUDGET})"
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

fn execute_process_bin(repo: &std::path::Path) -> std::path::PathBuf {
    if let Ok(p) = env::var("SDDIA_EXECUTE_PROCESS_BIN") {
        if !p.trim().is_empty() {
            return std::path::PathBuf::from(p);
        }
    }
    for rel in [
        "SddIA/target/debug/execute-process",
        "SddIA/target/release/execute-process",
    ] {
        let candidate = repo.join(rel);
        if candidate.is_file() {
            return candidate;
        }
    }
    repo.join("SddIA/target/debug/execute-process")
}

/// Vía A: sweep periódico de auditoría (ingest régimen + staleness) sin fan-out.
fn invoke_heartbeat_audit_sweep(repo: &std::path::Path) {
    let runner = execute_process_bin(repo);
    let payload = r#"{"sweep":true}"#;
    match std::process::Command::new(&runner)
        .args(["--process", "daemon-heartbeat-audit", "--inputs", payload])
        .current_dir(repo)
        .output()
    {
        Ok(out) if out.status.success() => {}
        Ok(out) => {
            let err = String::from_utf8_lossy(&out.stderr);
            let so = String::from_utf8_lossy(&out.stdout);
            eprintln!(
                "[SWEEPER] daemon-heartbeat-audit sweep: {}",
                if !err.trim().is_empty() {
                    err.trim()
                } else {
                    so.trim()
                }
            );
        }
        Err(e) => eprintln!("[SWEEPER] daemon-heartbeat-audit spawn: {e}"),
    }
}

fn run_sweeper(repo: std::path::PathBuf, once: bool, json: bool) -> Result<(), String> {
    let top = load_bus_topology(&repo);
    let mut centinela = DaemonRuntime::new(repo.clone(), "event-sweeper");
    centinela.bootstrap(&top)?;
    let shared = Arc::new(Mutex::new(centinela));
    let _hb = if once {
        None
    } else {
        Some(spawn_heartbeat_worker(Arc::clone(&shared), top.clone()))
    };
    if once {
        println!("[SWEEPER] Iniciado.");
    } else {
        println!(
            "[SWEEPER] Iniciado (keepalive heartbeat cada {HEARTBEAT_TICK_SECONDS}s)."
        );
    }
    let mut last_hb_audit = Instant::now() - Duration::from_secs(HEARTBEAT_AUDIT_SWEEP_SECONDS);
    loop {
        let report = sweep_once(&repo)?;
        print_report(&report, json);
        // Vía A: ingest régimen + staleness cada HEARTBEAT_AUDIT_SWEEP_SECONDS.
        if !once && last_hb_audit.elapsed() >= Duration::from_secs(HEARTBEAT_AUDIT_SWEEP_SECONDS)
        {
            invoke_heartbeat_audit_sweep(&repo);
            last_hb_audit = Instant::now();
        }
        {
            let mut centinela = shared
                .lock()
                .map_err(|_| "lock poisoned".to_string())?;
            centinela.tick(&top)?;
        }
        if once {
            println!("[SWEEPER] Ciclo único (--once). Fin.");
            break;
        }
        thread::sleep(Duration::from_secs(POLL_SECONDS));
    }
    if let Ok(mut centinela) = shared.lock() {
        centinela.shutdown();
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let once = args.iter().any(|a| a == "--once");
    let json = args.iter().any(|a| a == "--json");
    let repo = match find_repo_root() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };
    if let Err(e) = run_sweeper(repo, once, json) {
        eprintln!("[SWEEPER] {e}");
        std::process::exit(1);
    }
}
