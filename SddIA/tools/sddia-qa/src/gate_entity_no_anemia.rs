//! Gate anti-anemia de entidades `.md` de dominio (AUT-125).

use execute_process::engine::entity_no_anemia::{default_base_ref, scan_all, scan_changed, ScanReport};
use serde_json::{json, Value};
use std::path::Path;

#[derive(Debug, PartialEq)]
struct Options {
    json: bool,
    if_touched: bool,
    base: Option<String>,
}

fn parse_args(args: &[String]) -> Result<Options, String> {
    let mut opts = Options {
        json: false,
        if_touched: false,
        base: None,
    };
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--json" => opts.json = true,
            "--if-touched" => opts.if_touched = true,
            "--base" => match it.next() {
                Some(v) if !v.starts_with("--") && !v.trim().is_empty() => {
                    opts.base = Some(v.clone());
                }
                _ => return Err("--base requiere una ref git".into()),
            },
            other => return Err(format!("argumento desconocido: {other}")),
        }
    }
    if opts.base.is_some() && !opts.if_touched {
        return Err("--base solo aplica con --if-touched".into());
    }
    Ok(opts)
}

/// Ejecuta el gate y devuelve (exit code, sobre JSON).
fn evaluate(repo: &Path, args: &[String]) -> (i32, bool, Value) {
    let json_out = args.iter().any(|a| a == "--json");
    let opts = match parse_args(args) {
        Ok(o) => o,
        Err(e) => {
            return (
                2,
                json_out,
                json!({"success": false, "exitCode": 2, "error": e}),
            )
        }
    };
    let (mode, base, scanned): (&str, Option<String>, Result<ScanReport, String>) = if opts.if_touched {
        let base = opts.base.clone().unwrap_or_else(|| default_base_ref(repo));
        let rep = scan_changed(repo, &base);
        ("if-touched", Some(base), rep)
    } else {
        ("all", None, scan_all(repo))
    };
    match scanned {
        Ok(report) => {
            let code = if report.ok { 0 } else { 1 };
            (
                code,
                opts.json,
                json!({
                    "success": report.ok,
                    "exitCode": code,
                    "mode": mode,
                    "base_ref": base,
                    "result": report,
                }),
            )
        }
        Err(e) => (
            1,
            opts.json,
            json!({
                "success": false,
                "exitCode": 1,
                "mode": mode,
                "base_ref": base,
                "error": e,
            }),
        ),
    }
}

pub fn run(repo: &Path, args: &[String]) -> i32 {
    let (code, json_out, env) = evaluate(repo, args);
    if json_out {
        println!("{env}");
        return code;
    }
    if let Some(err) = env.get("error").and_then(|v| v.as_str()) {
        eprintln!("gate-entity-no-anemia: {err}");
        return code;
    }
    let result = &env["result"];
    let scanned = result["scanned"].as_u64().unwrap_or(0);
    if code == 0 {
        println!("gate-entity-no-anemia: OK (scanned={scanned})");
        return code;
    }
    let violations = result["violations"].as_array().cloned().unwrap_or_default();
    eprintln!(
        "gate-entity-no-anemia: FAIL violations={} scanned={scanned}",
        violations.len()
    );
    for v in violations.iter().take(40) {
        eprintln!(
            "  {} [{}] {} — {}",
            v["path"].as_str().unwrap_or(""),
            v["family"].as_str().unwrap_or(""),
            v["code"].as_str().unwrap_or(""),
            v["detail"].as_str().unwrap_or("")
        );
    }
    if violations.len() > 40 {
        eprintln!("  ... +{} más", violations.len() - 40);
    }
    code
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    fn repo() -> std::path::PathBuf {
        let mut here = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        here.pop();
        here.pop();
        here.pop();
        here
    }

    #[test]
    fn base_without_value_is_rejected() {
        assert!(parse_args(&args(&["--if-touched", "--base", "--json"])).is_err());
        assert!(parse_args(&args(&["--if-touched", "--base"])).is_err());
    }

    #[test]
    fn base_requires_if_touched() {
        assert!(parse_args(&args(&["--base", "origin/main"])).is_err());
    }

    #[test]
    fn parse_full_flags() {
        let o = parse_args(&args(&["--if-touched", "--base", "HEAD~1", "--json"])).unwrap();
        assert_eq!(
            o,
            Options {
                json: true,
                if_touched: true,
                base: Some("HEAD~1".into())
            }
        );
    }

    #[test]
    fn argument_errors_are_json_when_requested() {
        let (code, json_out, env) = evaluate(&repo(), &args(&["--json", "--base"]));
        assert_eq!(code, 2);
        assert!(json_out);
        assert_eq!(env["success"], false);
        assert!(env["error"].as_str().unwrap().contains("--base"));
    }

    #[test]
    fn scan_errors_are_json_when_requested() {
        let (code, json_out, env) = evaluate(
            &repo(),
            &args(&["--if-touched", "--base", "refs/heads/aut125-no-existe", "--json"]),
        );
        assert_eq!(code, 1);
        assert!(json_out);
        assert_eq!(env["mode"], "if-touched");
        assert!(env["error"].as_str().unwrap().contains("no resoluble"));
    }

    #[test]
    fn if_touched_empty_range_passes() {
        let (code, _, env) = evaluate(&repo(), &args(&["--if-touched", "--base", "HEAD", "--json"]));
        assert_eq!(code, 0, "{env}");
        assert_eq!(env["base_ref"], "HEAD");
        assert_eq!(env["result"]["scanned"], 0);
    }
}
