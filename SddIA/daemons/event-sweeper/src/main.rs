use sddia_daemon_runtime::eda_sweep::{sweep_once, POLL_SECONDS, SweepReport};
use sddia_daemon_runtime::{find_repo_root, load_bus_topology, BusTopology, DaemonRuntime};
use std::env;
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};

const HEARTBEAT_TICK_SECONDS: u64 = 10;

fn print_report(report: &SweepReport, json: bool) {
    if json {
        println!(
            "{}",
            serde_json::json!({
                "purged": report.purged,
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
    thread::spawn(move || loop {
        if let Ok(mut rt) = centinela.lock() {
            if let Err(e) = rt.tick(&top) {
                eprintln!("[SWEEPER] heartbeat keepalive: {e}");
            }
        }
        thread::sleep(Duration::from_secs(HEARTBEAT_TICK_SECONDS));
    })
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
    loop {
        let report = sweep_once(&repo)?;
        print_report(&report, json);
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
