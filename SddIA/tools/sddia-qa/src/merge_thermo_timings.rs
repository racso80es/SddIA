//! Análisis de `hook-timings.jsonl` — PBI-MERGE-THERMO-06 / AC-7.

use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn median(vals: &mut [i64]) -> Option<i64> {
    if vals.is_empty() {
        return None;
    }
    vals.sort_unstable();
    Some(vals[vals.len() / 2])
}

pub fn run(repo: &Path, json: bool) -> i32 {
    let path = repo.join(".SddIA/proofs/hook-timings/hook-timings.jsonl");
    if !path.is_file() {
        eprintln!("missing {}", path.display());
        return 1;
    }
    let text = fs::read_to_string(&path).unwrap_or_default();
    let mut by_key: HashMap<String, Vec<i64>> = HashMap::new();
    let mut total = 0usize;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Ok(v) = serde_json::from_str::<Value>(line) else {
            continue;
        };
        if v.get("hook").and_then(|x| x.as_str()) != Some("pre-push") {
            continue;
        }
        let ms = v.get("total_ms").and_then(|x| x.as_i64()).unwrap_or(0);
        let dc = v
            .get("delta_class")
            .and_then(|x| x.as_str())
            .unwrap_or("unknown");
        let ah = match v.get("attestation_hit") {
            Some(Value::Bool(b)) => if *b { "hit" } else { "miss" },
            _ => "na",
        };
        let key = format!("{dc}|{ah}");
        by_key.entry(key).or_default().push(ms);
        total += 1;
    }
    let mut report = Vec::new();
    for (key, mut vals) in by_key {
        let count = vals.len();
        let med = median(&mut vals);
        let parts: Vec<&str> = key.split('|').collect();
        report.push(json!({
            "delta_class": parts.first().unwrap_or(&"?"),
            "attestation_hit": parts.get(1).unwrap_or(&"?"),
            "count": count,
            "median_ms": med,
        }));
    }
    let out = json!({
        "success": true,
        "pre_push_lines": total,
        "groups": report,
    });
    if json {
        println!("{}", serde_json::to_string(&out).unwrap_or_default());
    } else {
        println!("merge-thermo-timings-report: {} pre-push lines", total);
        for g in report {
            println!(
                "  {} attestation={} n={} median_ms={}",
                g["delta_class"],
                g["attestation_hit"],
                g["count"],
                g["median_ms"]
            );
        }
    }
    0
}
