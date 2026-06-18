//! Handler nativo `crypto-broker` — puerta RBAC hacia `cryptography-manager` (D-P6T.1).

use super::capsules::invoke_capsule_json;
use serde_json::{json, Value};
use std::path::Path;

fn resolve_crypto_request(inputs: &Value) -> Result<Value, String> {
    if let Some(req) = inputs.get("crypto_request") {
        if req.is_object() {
            return Ok(req.clone());
        }
        return Err("crypto_request debe ser objeto JSON".into());
    }
    if inputs.get("operation").and_then(|v| v.as_str()).is_some() {
        return Ok(inputs.clone());
    }
    Err("crypto_request u operation es obligatorio".into())
}

fn unwrap_crypto_response(body: &Value) -> Result<Value, String> {
    if body.get("success") == Some(&json!(false)) {
        return Err(body
            .get("error")
            .and_then(|v| v.as_str())
            .unwrap_or("cryptography-manager failed")
            .to_string());
    }
    if let Some(inner) = body.get("result") {
        if let Some(nested) = inner.get("result") {
            return Ok(nested.clone());
        }
        return Ok(inner.clone());
    }
    Ok(body.clone())
}

/// Ejecuta `crypto-broker` delegando en cápsula `cryptography-manager`.
pub fn run(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let request = resolve_crypto_request(inputs)?;
    let result = invoke_capsule_json(repo, "cryptography-manager", &request, true)?;
    let crypto_response = unwrap_crypto_response(&result.body)?;
    if result.exit_code != 0 {
        return Err(format!(
            "cryptography-manager exitCode={}",
            result.exit_code
        ));
    }
    Ok(json!({
        "success": true,
        "crypto_response": crypto_response,
        "exitCode": result.exit_code,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::repo::find_repo_root;

    #[test]
    fn crypto_broker_generate_uuid() {
        let repo = find_repo_root().unwrap();
        let inputs = json!({
            "crypto_request": {
                "operation": "GENERATE_UUID",
                "target_payload": null
            }
        });
        let out = run(&repo, &inputs).expect("crypto-broker");
        assert_eq!(out.get("success"), Some(&json!(true)));
        assert_eq!(out.get("exitCode"), Some(&json!(0)));
        let resp = out.get("crypto_response").expect("crypto_response");
        assert!(
            resp.as_str().is_some() || resp.get("result").is_some(),
            "expected uuid result"
        );
    }
}
