pub mod outbound_lab;

use std::env;
use std::fs;
use std::io::{self, IsTerminal, Read};
use std::process;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct SddiaResponse<T> {
    pub success: bool,
    #[serde(rename = "exitCode")]
    pub exit_code: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

fn env_truthy(name: &str) -> bool {
    env::var(name)
        .ok()
        .is_some_and(|v| matches!(v.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on"))
}

fn extract_request_file(args: &[String]) -> Result<Option<String>, String> {
    let mut i = 0;
    while i < args.len() {
        let a = args[i].as_str();
        if let Some(rest) = a.strip_prefix("--request-file=") {
            let p = rest.trim();
            if p.is_empty() {
                return Err("flag --request-file= requiere ruta de fichero".into());
            }
            return Ok(Some(p.to_string()));
        }
        if a == "--request-file" {
            let next = args.get(i + 1).map(|s| s.trim()).unwrap_or("");
            if next.is_empty() {
                return Err("flag --request-file requiere ruta de fichero".into());
            }
            return Ok(Some(next.to_string()));
        }
        i += 1;
    }
    Ok(None)
}

/// Resuelve JSON de entrada según precedencia `capsule-json-io` v2.0:
/// 1) `--request-file <ruta>`
/// 2) `SDDIA_CAPSULE_REQUEST`
/// 3) `SDDIA_SKIP_STDIN=1`
/// 4) Guarda Fail-Fast TTY (`is_terminal()`)
/// 5) `stdin` (pipe/heredoc)
pub fn resolve_capsule_json_with<F>(
    args: &[String],
    env_request: Option<&str>,
    skip_stdin: bool,
    stdin_is_tty: bool,
    empty_stdin_as_empty_obj: bool,
    read_stdin: F,
) -> Result<Value, String>
where
    F: FnOnce() -> io::Result<String>,
{
    if let Some(path) = extract_request_file(args)? {
        let raw = fs::read_to_string(&path)
            .map_err(|e| format!("no se pudo leer --request-file '{path}': {e}"))?;
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return if empty_stdin_as_empty_obj {
                Ok(serde_json::json!({}))
            } else {
                Err(format!("--request-file '{path}' está vacío"))
            };
        }
        return serde_json::from_str(trimmed)
            .map_err(|e| format!("JSON inválido en --request-file '{path}': {e}"));
    }

    if let Some(raw_env) = env_request.map(str::trim).filter(|s| !s.is_empty()) {
        return serde_json::from_str(raw_env)
            .map_err(|e| format!("JSON inválido en SDDIA_CAPSULE_REQUEST: {e}"));
    }

    if skip_stdin {
        return if empty_stdin_as_empty_obj {
            Ok(serde_json::json!({}))
        } else {
            Err("SDDIA_SKIP_STDIN=1 activo sin --request-file ni SDDIA_CAPSULE_REQUEST".into())
        };
    }

    if stdin_is_tty {
        return Err(
            "stdin es una terminal interactiva (TTY) sin entrada JSON; use --request-file <ruta>, SDDIA_CAPSULE_REQUEST o pipe/heredoc por stdin"
                .into(),
        );
    }

    let raw = read_stdin().map_err(|e| format!("Failed to read stdin: {e}"))?;
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return if empty_stdin_as_empty_obj {
            Ok(serde_json::json!({}))
        } else {
            Err("EOF while parsing a value at line 1 column 0".into())
        };
    }

    serde_json::from_str(trimmed).map_err(|e| format!("invalid JSON stdin: {e}"))
}

pub fn read_capsule_json_from_args(
    args: &[String],
    empty_stdin_as_empty_obj: bool,
) -> Result<Value, String> {
    let env_req = env::var("SDDIA_CAPSULE_REQUEST").ok();
    let skip = env_truthy("SDDIA_SKIP_STDIN");
    let is_tty = io::stdin().is_terminal();
    resolve_capsule_json_with(
        args,
        env_req.as_deref(),
        skip,
        is_tty,
        empty_stdin_as_empty_obj,
        || {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            Ok(buffer)
        },
    )
}

pub fn read_stdin_json() -> Value {
    let args: Vec<String> = env::args().collect();
    match read_capsule_json_from_args(&args, true) {
        Ok(val) => val,
        Err(msg) => {
            emit_error(&msg, 1);
            process::exit(1);
        }
    }
}

pub fn emit_success<T: Serialize>(result: Option<T>) {
    let response = SddiaResponse {
        success: true,
        exit_code: 0,
        feedback: None,
        result,
        error: None,
    };
    emit(&response);
}

pub fn emit_error(msg: &str, exit_code: i32) {
    // `error` = contrato sddia-io; `feedback` = alias legible para route-domain / DLT.
    let response: SddiaResponse<Value> = SddiaResponse {
        success: false,
        exit_code,
        feedback: Some(msg.to_string()),
        result: None,
        error: Some(msg.to_string()),
    };
    emit(&response);
}

fn emit<T: Serialize>(response: &SddiaResponse<T>) {
    match serde_json::to_string(response) {
        Ok(json) => {
            println!("{}", json);
            process::exit(response.exit_code);
        }
        Err(_) => process::exit(1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn tty_without_alternative_fails_fast() {
        let args = vec!["bin".to_string()];
        let err = resolve_capsule_json_with(&args, None, false, true, true, || {
            panic!("no debe leer stdin en TTY");
        })
        .unwrap_err();
        assert!(err.contains("TTY"), "{err}");
        assert!(err.contains("--request-file"), "{err}");
    }

    #[test]
    fn request_file_takes_precedence_over_env_and_tty() {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("sddia-io-req-{nanos}.json"));
        fs::write(&path, r#"{"source":"file"}"#).unwrap();
        let args = vec![
            "bin".to_string(),
            "--request-file".to_string(),
            path.to_string_lossy().to_string(),
        ];
        let v = resolve_capsule_json_with(
            &args,
            Some(r#"{"source":"env"}"#),
            false,
            true,
            false,
            || panic!("no debe leer stdin cuando existe --request-file"),
        )
        .unwrap();
        let _ = fs::remove_file(&path);
        assert_eq!(v["source"], "file");
    }

    #[test]
    fn env_request_takes_precedence_over_skip_and_tty() {
        let args = vec!["bin".to_string()];
        let v = resolve_capsule_json_with(
            &args,
            Some(r#"{"source":"env"}"#),
            true,
            true,
            false,
            || panic!("no debe leer stdin con SDDIA_CAPSULE_REQUEST"),
        )
        .unwrap();
        assert_eq!(v["source"], "env");
    }

    #[test]
    fn skip_stdin_returns_empty_object_when_allowed_or_err_in_strict() {
        let args = vec!["bin".to_string()];
        let ok = resolve_capsule_json_with(&args, None, true, true, true, || {
            panic!("no debe leer stdin con SDDIA_SKIP_STDIN=1");
        })
        .unwrap();
        assert_eq!(ok, serde_json::json!({}));

        let err = resolve_capsule_json_with(&args, None, true, true, false, || {
            panic!("no debe leer stdin con SDDIA_SKIP_STDIN=1");
        })
        .unwrap_err();
        assert!(err.contains("SDDIA_SKIP_STDIN"), "{err}");
    }

    #[test]
    fn pipe_stdin_reads_json_when_not_tty() {
        let args = vec!["bin".to_string()];
        let v = resolve_capsule_json_with(&args, None, false, false, false, || {
            Ok(r#"{"source":"pipe"}"#.to_string())
        })
        .unwrap();
        assert_eq!(v["source"], "pipe");
    }
}
