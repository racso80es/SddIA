use execute_process::core::env::load_hierarchical_env;
use execute_process::core::parser::parse_frontmatter;
use execute_process::core::repo::find_repo_root;
use execute_process::core::resolver::normalize_request;
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

fn parse_args() -> Result<(Option<String>, bool, Value, Option<String>), String> {
    let args: Vec<String> = env::args().collect();
    let mut process_name: Option<String> = None;
    let mut forge_mode = false;
    let mut inputs_raw: Option<String> = None;
    let mut inputs_file: Option<String> = None;
    let mut frontmatter_path: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
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
            "--forge" => {
                forge_mode = true;
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

    if forge_mode {
        if !process_inputs.is_object() {
            return Err("--inputs debe ser objeto JSON".into());
        }
        return Ok((None, true, process_inputs, frontmatter_path));
    }

    if let Some(name) = process_name {
        if !process_inputs.is_object() {
            return Err("--inputs debe ser objeto JSON".into());
        }
        return Ok((Some(name.trim().to_string()), false, process_inputs, frontmatter_path));
    }

    if process_inputs.is_object() && !process_inputs.as_object().unwrap().is_empty() {
        let (name, inputs) = normalize_request(&process_inputs)?;
        Ok((Some(name), false, inputs, frontmatter_path))
    } else if frontmatter_path.is_some() {
        Ok((None, false, process_inputs, frontmatter_path))
    } else {
        Err(
            "Indique --process y --inputs, --forge --inputs, --parse-frontmatter, o stdin JSON"
                .into(),
        )
    }
}

fn run_main() -> Result<OrchestratorEnvelope, String> {
    let (process_name, forge_mode, process_inputs, frontmatter_path) = parse_args()?;
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
