use execute_process::core::env::{emit_shell_env, load_hierarchical_env};
use execute_process::core::parser::parse_frontmatter;
use execute_process::core::repo::find_repo_root;
use execute_process::core::resolver::normalize_request;
use execute_process::engine::capsules::{invoke_action, invoke_tool_capsule_json};
use execute_process::engine::eda_coverage;
use execute_process::engine::verify_process_integrity;
use execute_process::envelope::{emit, OrchestratorEnvelope};
use execute_process::engine::run_process;
use execute_process::forges::materialize_by_inputs;
use serde_json::Value;
use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::panic;
use std::process;

fn parse_inputs_arg(raw: Option<&str>) -> Result<Value, String> {
    if let Some(s) = raw {
        return serde_json::from_str(s).map_err(|e| format!("JSON inválido: {e}"));
    }
    let mut stdin = String::new();
    io::stdin()
        .read_to_string(&mut stdin)
        .map_err(|e| e.to_string())?;
    if stdin.trim().is_empty() {
        return Ok(Value::Object(Default::default()));
    }
    serde_json::from_str(&stdin).map_err(|e| format!("JSON inválido: {e}"))
}

fn emit_frontmatter_json(md_path: &str) -> Result<(), String> {
    let path = std::path::Path::new(md_path);
    let fm = parse_frontmatter(path)?;
    let json_value = serde_json::to_value(&fm).map_err(|e| e.to_string())?;
    println!(
        "{}",
        serde_json::to_string(&json_value).map_err(|e| e.to_string())?
    );
    Ok(())
}

fn parse_args() -> Result<(Option<String>, Option<String>, Option<String>, bool, bool, bool, Value, Option<String>), String> {
    let args: Vec<String> = env::args().collect();
    let mut process_name: Option<String> = None;
    let mut tool_name: Option<String> = None;
    let mut action_name: Option<String> = None;
    let mut forge_mode = false;
    let mut prefer_native = false;
    let mut emit_shell_env_format: Option<String> = None;
    let mut inputs_raw: Option<String> = None;
    let mut inputs_file: Option<String> = None;
    let mut frontmatter_path: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--emit-shell-env" => {
                let fmt = args
                    .get(i + 1)
                    .filter(|s| !s.starts_with('-'))
                    .map(|s| s.as_str())
                    .unwrap_or("bash");
                if fmt != "bash" && fmt != "bat" {
                    return Err("--emit-shell-env acepta bash|bat".into());
                }
                emit_shell_env_format = Some(fmt.to_string());
                if args.get(i + 1).is_some_and(|s| !s.starts_with('-')) {
                    i += 1;
                }
            }
            "--parse-frontmatter" => {
                i += 1;
                frontmatter_path = Some(
                    args.get(i)
                        .ok_or("--parse-frontmatter requiere ruta .md")?
                        .clone(),
                );
            }
            "--process" => {
                i += 1;
                process_name = Some(args.get(i).ok_or("--process requiere valor")?.clone());
            }
            "--tool" => {
                i += 1;
                tool_name = Some(args.get(i).ok_or("--tool requiere valor")?.clone());
            }
            "--action" => {
                i += 1;
                action_name = Some(args.get(i).ok_or("--action requiere valor")?.clone());
            }
            "--prefer-native" => {
                prefer_native = true;
            }
            "--forge" => {
                forge_mode = true;
            }
            "--detach" => {
                env::set_var("SDDIA_CLI_DETACH", "1");
            }
            "--foreground" => {
                env::set_var("SDDIA_CLI_FOREGROUND", "1");
            }
            "--inputs" => {
                i += 1;
                inputs_raw = Some(args.get(i).ok_or("--inputs requiere valor")?.clone());
            }
            "--inputs-file" => {
                i += 1;
                inputs_file = Some(args.get(i).ok_or("--inputs-file requiere valor")?.clone());
            }
            flag => return Err(format!("argumento desconocido: {flag}")),
        }
        i += 1;
    }

    let process_inputs = if let Some(path) = inputs_file {
        let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let text = text.trim_start_matches('\u{feff}');
        serde_json::from_str(text).map_err(|e| format!("JSON inválido: {e}"))?
    } else {
        parse_inputs_arg(inputs_raw.as_deref())?
    };

    if emit_shell_env_format.is_some() {
        return Ok((
            process_name,
            tool_name,
            action_name,
            forge_mode,
            prefer_native,
            true,
            process_inputs,
            frontmatter_path,
        ));
    }

    if forge_mode {
        if !process_inputs.is_object() {
            return Err("--inputs debe ser objeto JSON".into());
        }
        return Ok((
            None,
            None,
            None,
            true,
            prefer_native,
            false,
            process_inputs,
            frontmatter_path,
        ));
    }

    if let Some(name) = action_name {
        if process_name.is_some() || tool_name.is_some() {
            return Err("--action es mutuamente excluyente con --process/--tool".into());
        }
        if !process_inputs.is_object() {
            return Err("--inputs debe ser objeto JSON".into());
        }
        return Ok((
            None,
            None,
            Some(name.trim().to_string()),
            false,
            prefer_native,
            false,
            process_inputs,
            frontmatter_path,
        ));
    }

    if let Some(name) = tool_name {
        if process_name.is_some() {
            return Err("--tool y --process son mutuamente excluyentes".into());
        }
        if !process_inputs.is_object() {
            return Err("--inputs debe ser objeto JSON".into());
        }
        return Ok((
            None,
            Some(name.trim().to_string()),
            None,
            false,
            prefer_native,
            false,
            process_inputs,
            frontmatter_path,
        ));
    }

    if let Some(name) = process_name {
        if !process_inputs.is_object() {
            return Err("--inputs debe ser objeto JSON".into());
        }
        return Ok((
            Some(name.trim().to_string()),
            None,
            None,
            false,
            prefer_native,
            false,
            process_inputs,
            frontmatter_path,
        ));
    }

    if process_inputs.is_object() && !process_inputs.as_object().unwrap().is_empty() {
        let (name, inputs) = normalize_request(&process_inputs)?;
        Ok((
            Some(name),
            None,
            None,
            false,
            prefer_native,
            false,
            inputs,
            frontmatter_path,
        ))
    } else if frontmatter_path.is_some() {
        Ok((
            None,
            None,
            None,
            false,
            prefer_native,
            false,
            process_inputs,
            frontmatter_path,
        ))
    } else {
        Err(
            "Indique --process/--tool/--action y --inputs, --forge --inputs, --emit-shell-env, --parse-frontmatter, o stdin JSON"
                .into(),
        )
    }
}

fn run_tool_cli(tool_name: &str, prefer_native: bool, process_inputs: &Value) -> Result<i32, String> {
    let repo = find_repo_root()?;
    load_hierarchical_env(&repo)?;
    let result = invoke_tool_capsule_json(
        &repo,
        tool_name,
        process_inputs,
        !prefer_native,
    )?;
    println!(
        "{}",
        serde_json::to_string(&result.body).map_err(|e| e.to_string())?
    );
    Ok(result.exit_code)
}

fn run_action_cli(action_name: &str, action_inputs: &Value) -> Result<(), String> {
    let repo = find_repo_root()?;
    load_hierarchical_env(&repo)?;
    match invoke_action(&repo, action_name, action_inputs) {
        Ok(data) => {
            emit(OrchestratorEnvelope {
                success: true,
                status_code: 0,
                data: Some(data),
                error: None,
                execution_report: None,
                exit_code: 0,
            });
        }
        Err(e) => {
            emit(OrchestratorEnvelope::failure(e, 1));
        }
    }
}

fn run_main() -> Result<OrchestratorEnvelope, String> {
    let (process_name, _tool_name, _action_name, forge_mode, _prefer_native, _emit_env, process_inputs, frontmatter_path) =
        parse_args()?;
    if frontmatter_path.is_some() {
        return Err("--parse-frontmatter debe usarse sin otros flags de orquestación".into());
    }
    let repo = find_repo_root()?;
    load_hierarchical_env(&repo)?;
    if forge_mode {
        let data = materialize_by_inputs(&repo, &process_inputs)?;
        return Ok(OrchestratorEnvelope {
            success: true,
            status_code: 0,
            data: Some(data),
            error: None,
            execution_report: None,
            exit_code: 0,
        });
    }
    let process_name = process_name.ok_or("process_name requerido")?;
    run_process(&repo, &process_name, &process_inputs)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(pos) = args.iter().position(|a| a == "--parse-frontmatter") {
        match args.get(pos + 1) {
            Some(md_path) => match emit_frontmatter_json(md_path) {
                Ok(()) => process::exit(0),
                Err(e) => {
                    let _ = writeln!(io::stderr(), "{e}");
                    process::exit(1);
                }
            },
            None => {
                let _ = writeln!(io::stderr(), "--parse-frontmatter requiere ruta .md");
                process::exit(1);
            }
        }
    }

    if args.iter().any(|a| a == "--verify-process-integrity") {
        match find_repo_root() {
            Ok(repo) => process::exit(verify_process_integrity::run_cli(&repo)),
            Err(e) => {
                let _ = writeln!(io::stderr(), "{e}");
                process::exit(1);
            }
        }
    }

    if args.iter().any(|a| a == "--audit-eda-coverage") {
        let scan = args.iter().any(|a| a == "--scan");
        let json_out = args.iter().any(|a| a == "--json");
        if !scan {
            let _ = writeln!(io::stderr(), "--audit-eda-coverage requiere --scan");
            process::exit(1);
        }
        match find_repo_root() {
            Ok(repo) => match eda_coverage::scan_orphans(&repo) {
                Ok(report) => {
                    if json_out {
                        match serde_json::to_string(&report) {
                            Ok(line) => println!("{line}"),
                            Err(e) => {
                                let _ = writeln!(io::stderr(), "{e}");
                                process::exit(1);
                            }
                        }
                    } else {
                        println!(
                            "orphan_count={}",
                            report.get("orphan_count").and_then(|v| v.as_u64()).unwrap_or(0)
                        );
                    }
                    process::exit(0);
                }
                Err(e) => {
                    let _ = writeln!(io::stderr(), "{e}");
                    process::exit(1);
                }
            },
            Err(e) => {
                let _ = writeln!(io::stderr(), "{e}");
                process::exit(1);
            }
        }
    }

    if args.iter().any(|a| a == "--emit-shell-env") {
        let fmt = args
            .iter()
            .position(|a| a == "--emit-shell-env")
            .and_then(|pos| args.get(pos + 1))
            .map(|s| s.as_str())
            .filter(|s| *s != "--process" && *s != "--tool" && *s != "--inputs")
            .unwrap_or("bash");
        match find_repo_root().and_then(|repo| emit_shell_env(&repo, fmt)) {
            Ok(()) => process::exit(0),
            Err(e) => {
                let _ = writeln!(io::stderr(), "{e}");
                process::exit(1);
            }
        }
    }

    if args.iter().any(|a| a == "--tool") {
        match (|| {
            let (_, tool_name, _, _, prefer_native, _, process_inputs, _) = parse_args()?;
            let tool = tool_name.ok_or("--tool requiere valor")?;
            run_tool_cli(&tool, prefer_native, &process_inputs)
        })() {
            Ok(code) => process::exit(code),
            Err(e) => {
                let _ = writeln!(io::stderr(), "{e}");
                process::exit(1);
            }
        }
    }

    if args.iter().any(|a| a == "--action") {
        match (|| {
            let (_, _, action_name, _, _, _, process_inputs, _) = parse_args()?;
            let action = action_name.ok_or("--action requiere valor")?;
            run_action_cli(&action, &process_inputs)
        })() {
            Ok(()) => {}
            Err(e) => {
                let _ = writeln!(io::stderr(), "{e}");
                process::exit(1);
            }
        }
    }

    let result = panic::catch_unwind(|| run_main());
    match result {
        Ok(Ok(envelope)) => emit(envelope),
        Ok(Err(e)) => emit(OrchestratorEnvelope::failure(e, 1)),
        Err(_) => emit(OrchestratorEnvelope::failure(
            "panic interno del orquestador",
            1,
        )),
    }
}
