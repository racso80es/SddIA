use serde_json::Value;
use std::io::{self, Read, Write};
use std::path::PathBuf;

fn repo_root() -> Result<PathBuf, String> {
    let mut current = std::env::current_dir().map_err(|e| e.to_string())?;
    loop {
        if current.join("SddIA/core/cumulo.paths.json").is_file() {
            return Ok(current);
        }
        current = current
            .parent()
            .ok_or_else(|| "No se encontró raíz del workspace".to_string())?
            .to_path_buf();
    }
}

fn emit(body: &Value) {
    let line = serde_json::to_string(body).unwrap_or_else(|_| {
        r#"{"success":false,"exitCode":1,"message":"serialize error"}"#.into()
    });
    let _ = writeln!(io::stdout(), "{line}");
    let code = body
        .get("exitCode")
        .and_then(|v| v.as_i64())
        .unwrap_or(1) as i32;
    std::process::exit(code);
}

fn main() {
    let repo = match repo_root() {
        Ok(p) => p,
        Err(e) => {
            emit(&serde_json::json!({
                "success": false,
                "exitCode": 1,
                "message": e
            }));
            return;
        }
    };
    let mut buf = String::new();
    if io::stdin().read_to_string(&mut buf).is_err() {
        emit(&serde_json::json!({
            "success": false,
            "exitCode": 1,
            "message": "stdin vacío"
        }));
        return;
    }
    let request: Value = match serde_json::from_str(buf.trim()) {
        Ok(v) => v,
        Err(e) => {
            emit(&serde_json::json!({
                "success": false,
                "exitCode": 1,
                "message": format!("JSON inválido: {e}")
            }));
            return;
        }
    };
    emit(&user_preference_core::run_capsule(&repo, &request));
}
