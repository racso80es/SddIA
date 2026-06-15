use sddia_daemon_runtime::eda_sweep::{sweep_once, POLL_SECONDS, SweepReport};
use sddia_daemon_runtime::{find_repo_root, load_bus_topology, DaemonRuntime};
use std::env;
use std::{thread, time::Duration};

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

fn run_sweeper(repo: std::path::PathBuf, once: bool, json: bool) -> Result<(), String> {
    let top = load_bus_topology(&repo);
    let mut centinela = DaemonRuntime::new(repo.clone(), "event-sweeper");
    centinela.bootstrap(&top)?;
    println!("[SWEEPER] Iniciado.");
    loop {
        let report = sweep_once(&repo)?;
        print_report(&report, json);
        centinela.tick(&top)?;
        if once {
            println!("[SWEEPER] Ciclo único (--once). Fin.");
            break;
        }
        thread::sleep(Duration::from_secs(POLL_SECONDS));
    }
    centinela.shutdown();
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
