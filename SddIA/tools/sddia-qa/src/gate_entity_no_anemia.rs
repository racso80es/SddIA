//! Gate anti-anemia de entidades `.md` de dominio (AUT-125).

use execute_process::engine::entity_no_anemia::{scan_all, scan_changed, ScanReport};
use serde_json::json;
use std::path::Path;

fn has_flag(args: &[String], flag: &str) -> bool {
    args.iter().any(|a| a == flag)
}

pub fn run(repo: &Path, args: &[String]) -> i32 {
    let json_out = has_flag(args, "--json");
    let if_touched = has_flag(args, "--if-touched");
    let base = args
        .iter()
        .position(|a| a == "--base")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
        .unwrap_or("origin/main");

    let report: ScanReport = if if_touched {
        match scan_changed(repo, base) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("gate-entity-no-anemia: {e}");
                return 1;
            }
        }
    } else {
        match scan_all(repo) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("gate-entity-no-anemia: {e}");
                return 1;
            }
        }
    };

    if json_out {
        println!(
            "{}",
            json!({
                "success": report.ok,
                "exitCode": if report.ok { 0 } else { 1 },
                "result": report,
            })
        );
    } else if report.ok {
        println!(
            "gate-entity-no-anemia: OK (scanned={})",
            report.scanned
        );
    } else {
        eprintln!(
            "gate-entity-no-anemia: FAIL violations={} scanned={}",
            report.violation_count, report.scanned
        );
        for v in report.violations.iter().take(40) {
            eprintln!("  {} [{}] {} — {}", v.path, v.family, v.code, v.detail);
        }
        if report.violations.len() > 40 {
            eprintln!("  ... +{} más", report.violations.len() - 40);
        }
    }
    if report.ok { 0 } else { 1 }
}