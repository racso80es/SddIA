//! Handler nativo `instance-creator` — despliegue hermético consumidor (Kaizen Filtro C).

use super::super::capsules::invoke_tool_capsule_json;
use super::super::workspace::bootstrap_workspace;
use crate::envelope::OrchestratorEnvelope;
use chrono::Utc;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;
use uuid::Uuid;

fn str_opt(inputs: &Value, key: &str) -> Option<String> {
    inputs
        .get(key)
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn truthy(inputs: &Value, key: &str) -> bool {
    inputs.get(key).map_or(false, |v| {
        v.as_bool().unwrap_or_else(|| {
            v.as_str()
                .map(|s| matches!(s.trim().to_lowercase().as_str(), "1" | "true" | "yes"))
                .unwrap_or(false)
        })
    })
}

fn ensure_dir(p: &Path) -> Result<(), String> {
    fs::create_dir_all(p).map_err(|e| format!("mkdir {}: {e}", p.display()))
}

fn copy_file(src: &Path, dst: &Path) -> Result<(), String> {
    if let Some(parent) = dst.parent() {
        ensure_dir(parent)?;
    }
    fs::copy(src, dst).map_err(|e| format!("copy {} → {}: {e}", src.display(), dst.display()))?;
    Ok(())
}

fn map_vault_into_instance(vault_src: &Path, sddia: &Path, instance_root: &Path) -> Result<usize, String> {
    let mut n = 0usize;
    let inst_env = vault_src.join("instance.SddIA.dev.env");
    if inst_env.is_file() {
        copy_file(&inst_env, &sddia.join(".dev/.env"))?;
        n += 1;
    }
    let root_env = vault_src.join("root.dev.env");
    if root_env.is_file() {
        copy_file(&root_env, &instance_root.join(".dev/.env"))?;
        n += 1;
    }
    let constitution = vault_src.join("constitution");
    if constitution.is_dir() {
        n += copy_tree(&constitution, &sddia.join("constitution"))?;
    }
    let codexes = vault_src.join("codexes");
    if codexes.is_dir() {
        n += copy_tree(&codexes, &sddia.join("library/codexes"))?;
    }
    // Inventario / README: metadatos sin secretos embebidos en nombre
    for name in ["env-keys.inventory.txt", "README.md"] {
        let p = vault_src.join(name);
        if p.is_file() {
            copy_file(&p, &sddia.join(name))?;
            n += 1;
        }
    }
    // Fallback: si no hay layout preprod, copiar árbol completo bajo .SddIA/
    if n == 0 {
        n = copy_tree(vault_src, sddia)?;
    }
    Ok(n)
}

fn copy_tree(src: &Path, dst: &Path) -> Result<usize, String> {
    if !src.is_dir() {
        return Err(format!("vault_source no es directorio: {}", src.display()));
    }
    let mut n = 0usize;
    for entry in walkdir_simple(src)? {
        let rel = entry.strip_prefix(src).map_err(|e| e.to_string())?;
        let target = dst.join(rel);
        if entry.is_dir() {
            ensure_dir(&target)?;
        } else if entry.is_file() {
            copy_file(&entry, &target)?;
            n += 1;
        }
    }
    Ok(n)
}

fn walkdir_simple(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut out = Vec::new();
    fn rec(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
        for ent in fs::read_dir(dir).map_err(|e| format!("read_dir {}: {e}", dir.display()))? {
            let ent = ent.map_err(|e| e.to_string())?;
            let p = ent.path();
            out.push(p.clone());
            if p.is_dir() {
                rec(&p, out)?;
            }
        }
        Ok(())
    }
    rec(root, &mut out)?;
    Ok(out)
}

fn phase(name: &str, status: &str, extra: Value) -> Value {
    let mut m = json!({
        "phase_name": name,
        "status": status,
        "handler": "instance-creator-native",
    });
    if let Some(obj) = extra.as_object() {
        for (k, v) in obj {
            m[k] = v.clone();
        }
    }
    m
}

fn install_systemd_templates(repo: &Path, instance_root: &Path) -> Result<Value, String> {
    let templates = repo.join("SddIA/templates/systemd");
    if !templates.is_dir() {
        return Ok(json!({ "installed": false, "reason": "templates ausentes" }));
    }
    let dest = instance_root.join(".SddIA/systemd");
    ensure_dir(&dest)?;
    let mut copied = Vec::new();
    for ent in fs::read_dir(&templates).map_err(|e| e.to_string())? {
        let ent = ent.map_err(|e| e.to_string())?;
        let p = ent.path();
        if p.extension().and_then(|e| e.to_str()) == Some("template")
            || p.extension().and_then(|e| e.to_str()) == Some("service")
        {
            let name = p.file_name().unwrap().to_string_lossy().to_string();
            let body = fs::read_to_string(&p).map_err(|e| e.to_string())?;
            // Sustituye marcador de core root por ruta del genoma relativo a la instancia.
            let core = repo.display().to_string();
            let rendered = body.replace("@@SDDIA_CORE_ROOT@@", &core);
            let out = dest.join(name.trim_end_matches(".template"));
            fs::write(&out, rendered).map_err(|e| e.to_string())?;
            copied.push(out.file_name().unwrap().to_string_lossy().to_string());
        }
    }
    Ok(json!({
        "installed": true,
        "units_dir": dest.display().to_string(),
        "units": copied,
        "note": "systemctl --user enable/start queda al operador o fase Ignicion"
    }))
}

fn run_smoke(repo: &Path, instance_root: &Path) -> Value {
    // 1) Preferir cápsula eda-local-topology-test si hay binario.
    let candidates = [
        repo.join("SddIA/target/release/eda-local-topology-test"),
        repo.join("SddIA/target/debug/eda-local-topology-test"),
        instance_root.join("SddIA/target/release/eda-local-topology-test"),
    ];
    for bin in &candidates {
        if bin.is_file() {
            let payload = json!({
                "meta": {
                    "schemaVersion": "2.0",
                    "entityKind": "tool",
                    "entityId": "eda-local-topology-test"
                },
                "request": { "instance_root": instance_root.display().to_string() }
            });
            match invoke_tool_capsule_json(repo, "eda-local-topology-test", &payload, false) {
                Ok(r) => {
                    let ok = r.exit_code == 0
                        && r.body.get("success").and_then(|v| v.as_bool()).unwrap_or(false);
                    return json!({
                        "mode": "eda-local-topology-test",
                        "success": ok,
                        "exit_code": r.exit_code,
                        "body": r.body,
                    });
                }
                Err(e) => {
                    return json!({
                        "mode": "eda-local-topology-test",
                        "success": false,
                        "error": e,
                    });
                }
            }
        }
    }

    // 2) Smoke nativo de topología (tool aún stub en instancia) + estímulo Local_QA_Requested.
    let mut checks = serde_json::Map::new();
    let mut ok = true;
    let require_dir = |rel: &str, checks: &mut serde_json::Map<String, Value>, ok: &mut bool| {
        let p = instance_root.join(rel);
        let present = p.is_dir();
        checks.insert(rel.to_string(), json!(present));
        if !present {
            *ok = false;
        }
    };
    require_dir(".SddIA", &mut checks, &mut ok);
    require_dir(".SddIA/.dev", &mut checks, &mut ok);
    require_dir(".SddIA/daemons/state", &mut checks, &mut ok);
    require_dir(".events/pending", &mut checks, &mut ok);
    require_dir(".events/domain", &mut checks, &mut ok);
    require_dir(".events/orchestration", &mut checks, &mut ok);

    let env_path = instance_root.join(".SddIA/.dev/.env");
    let env_ok = env_path.is_file();
    checks.insert("vault_env_present".into(), json!(env_ok));
    // No leer ni loguear contenido de bóveda.
    if !env_ok {
        ok = false;
    }

    let local_paths = instance_root.join(".SddIA/local.paths.json");
    let lp_ok = local_paths.is_file();
    checks.insert("local_paths_present".into(), json!(lp_ok));
    if !lp_ok {
        ok = false;
    }

    let systemd_dir = instance_root.join(".SddIA/systemd");
    let unit_count = fs::read_dir(&systemd_dir)
        .map(|rd| rd.filter_map(|e| e.ok()).count())
        .unwrap_or(0);
    checks.insert("systemd_units".into(), json!(unit_count));
    if unit_count == 0 {
        ok = false;
    }

    // Cápsula eferente verificable si el genoma está en la instancia o en el repo forjador.
    let telegram_md = [
        instance_root.join("SddIA/tools/send-telegram-notification.md"),
        repo.join("SddIA/tools/send-telegram-notification.md"),
    ]
    .into_iter()
    .any(|p| p.is_file());
    checks.insert("send_telegram_notification_md".into(), json!(telegram_md));
    if !telegram_md {
        ok = false;
    }

    let pending = instance_root.join(".events/pending");
    if let Err(e) = ensure_dir(&pending) {
        return json!({
            "mode": "native-topology+local-qa",
            "success": false,
            "checks": checks,
            "error": e
        });
    }
    let eid = Uuid::new_v4().to_string();
    let event = json!({
        "event_id": eid,
        "event_type": "Local_QA_Requested",
        "event_family": "orchestration",
        "timestamp": Utc::now().to_rfc3339(),
        "emitter_agent": "instance-creator",
        "payload": {
            "blocking": false,
            "source": "instance-creator-smoke",
            "instance_root": instance_root.display().to_string(),
            "topology_ok": ok
        }
    });
    let path = pending.join(format!("{}.json", event["event_id"].as_str().unwrap()));
    let emit_ok = fs::write(&path, format!("{event}\n")).is_ok();
    checks.insert("local_qa_emitted".into(), json!(emit_ok));
    if !emit_ok {
        ok = false;
    }

    json!({
        "mode": "native-topology+local-qa",
        "success": ok,
        "checks": checks,
        "event_path": if emit_ok { json!(path.display().to_string()) } else { Value::Null },
        "note": "eda-local-topology-test binario ausente; smoke nativo + Local_QA_Requested"
    })
}

fn try_ignite(instance_root: &Path, profile: &str) -> Value {
    let script = instance_root.join("start-sddia.sh");
    if !script.is_file() {
        let alt = instance_root.join("SddIA/../start-sddia.sh");
        let _ = alt;
        return json!({
            "started": false,
            "reason": "start-sddia.sh ausente en instance_root (despliegue bundle incompleto)"
        });
    }
    // No bloquear: documentar comando; arranque largo = operador / detach futuro.
    json!({
        "started": false,
        "deferred": true,
        "command": format!(
            "cd {} && SDDIA_RUNTIME_PROFILE={} ./start-sddia.sh",
            instance_root.display(),
            profile
        ),
        "note": "Ignición interactiva diferida; use systemctl @%f o arranque manual"
    })
}

pub fn run(repo: &Path, process_inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let t0 = Instant::now();
    let instance_root_raw = str_opt(process_inputs, "instance_root")
        .ok_or_else(|| "instance_root obligatorio".to_string())?;
    let instance_root = if Path::new(&instance_root_raw).is_absolute() {
        PathBuf::from(&instance_root_raw)
    } else {
        repo.join(&instance_root_raw)
    };
    let profile = str_opt(process_inputs, "runtime_profile")
        .unwrap_or_else(|| "consumer".into());
    let vault = str_opt(process_inputs, "vault_source");
    let skip_smoke = truthy(process_inputs, "skip_smoke");
    let skip_ignition = truthy(process_inputs, "skip_ignition");
    let cid = str_opt(process_inputs, "correlation_id")
        .or_else(|| str_opt(process_inputs, "execution_id"))
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    let mut inputs = process_inputs.clone();
    if inputs.get("execution_id").is_none() {
        inputs["execution_id"] = json!(cid);
    }
    let mut state = json!({});
    let _ws = bootstrap_workspace(
        repo,
        "instance-creator",
        ".SddIA/workspaces/{process_name}/{execution_id}/",
        &mut inputs,
        &mut state,
    )?;

    let mut phases = Vec::new();

    // Topologia
    ensure_dir(&instance_root)?;
    let sddia = instance_root.join(".SddIA");
    for sub in [
        ".dev",
        "daemons/status",
        "daemons/state",
        "daemons/logs",
        "library/codexes",
        "library/norms",
        "constitution",
        "agenda",
        "systemd",
    ] {
        ensure_dir(&sddia.join(sub))?;
    }
    let local_paths = sddia.join("local.paths.json");
    if !local_paths.is_file() {
        fs::write(&local_paths, "{}\n").map_err(|e| e.to_string())?;
    }
    let events = instance_root.join(".events");
    for fam in ["pending", "domain", "orchestration", "telemetry", "dead-letter"] {
        ensure_dir(&events.join(fam))?;
    }
    phases.push(phase(
        "Topologia",
        "executed",
        json!({ "instance_root": instance_root.display().to_string() }),
    ));

    // Vault
    let mut vault_files = 0usize;
    if let Some(ref src) = vault {
        let src_path = PathBuf::from(src);
        if !src_path.is_dir() {
            return Err(format!("vault_source no es directorio: {}", src_path.display()));
        }
        vault_files = map_vault_into_instance(&src_path, &sddia, &instance_root)?;
        // No echo de secretos
        phases.push(phase(
            "Vault",
            "executed",
            json!({ "files_copied": vault_files, "vault_source": src, "secrets_logged": false }),
        ));
    } else {
        let env_path = sddia.join(".dev/.env");
        if !env_path.is_file() {
            let stub = format!(
                "# stub instance-creator — rellenar secretos fuera de git\nSDDIA_RUNTIME_PROFILE={profile}\nSDDIA_CLIENT_PORT=8766\n"
            );
            fs::write(&env_path, stub).map_err(|e| e.to_string())?;
        }
        phases.push(phase(
            "Vault",
            "executed",
            json!({ "files_copied": 0, "stub_env": true, "secrets_logged": false }),
        ));
    }

    // Systemd
    let systemd = install_systemd_templates(repo, &instance_root)?;
    phases.push(phase("Systemd", "executed", systemd));

    // Ignicion
    let ign = if skip_ignition {
        json!({ "started": false, "skipped": true })
    } else {
        try_ignite(&instance_root, &profile)
    };
    phases.push(phase(
        "Ignicion",
        if ign.get("skipped").and_then(|v| v.as_bool()).unwrap_or(false) {
            "skipped"
        } else {
            "executed"
        },
        ign,
    ));

    // Smoke
    let smoke = if skip_smoke {
        json!({ "success": true, "skipped": true })
    } else {
        run_smoke(repo, &instance_root)
    };
    let smoke_ok = smoke.get("success").and_then(|v| v.as_bool()).unwrap_or(false);
    phases.push(phase(
        "Smoke",
        if smoke_ok { "executed" } else { "failed" },
        smoke.clone(),
    ));

    let ok = smoke_ok;
    let status_code = if ok { 0 } else { 1 };

    Ok(OrchestratorEnvelope {
        success: ok,
        status_code,
        data: Some(json!({
            "instance_root": instance_root.display().to_string(),
            "runtime_profile": profile,
            "vault_files_copied": vault_files,
            "smoke": smoke,
            "correlation_id": cid,
        })),
        error: if ok {
            None
        } else {
            Some("smoke post-ignición no alcanzó success:true".into())
        },
        execution_report: Some(json!({
            "process_name": "instance-creator",
            "phases": phases,
            "duration_ms": t0.elapsed().as_millis(),
        })),
        exit_code: status_code,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn creates_topology_and_skips_smoke() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        fs::create_dir_all(repo.join("SddIA/templates/systemd")).unwrap();
        fs::write(
            repo.join("SddIA/templates/systemd/sddia-email-watcher@.service.template"),
            "[Service]\nWorkingDirectory=%f\nExecStart=@@SDDIA_CORE_ROOT@@/SddIA/daemons/email-watcher.sh\n",
        )
        .unwrap();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(repo.join("SddIA/core/cumulo.paths.json"), "{}").unwrap();
        fs::create_dir_all(repo.join("SddIA/tools")).unwrap();
        fs::write(
            repo.join("SddIA/tools/send-telegram-notification.md"),
            "---\nname: send-telegram-notification\n---\n",
        )
        .unwrap();

        let instance = repo.join("cliente-a");
        let env = run(
            repo,
            &json!({
                "instance_root": instance.display().to_string(),
                "runtime_profile": "consumer",
                "skip_smoke": true,
                "skip_ignition": true,
            }),
        )
        .unwrap();
        assert!(env.success);
        assert!(instance.join(".SddIA/.dev/.env").is_file());
        assert!(instance.join(".SddIA/local.paths.json").is_file());
        assert!(instance.join(".events/pending").is_dir());
        assert!(instance
            .join(".SddIA/systemd/sddia-email-watcher@.service")
            .is_file());
    }

    #[test]
    fn smoke_native_without_skip() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        fs::create_dir_all(repo.join("SddIA/templates/systemd")).unwrap();
        fs::write(
            repo.join("SddIA/templates/systemd/sddia-email-watcher@.service.template"),
            "[Service]\nWorkingDirectory=%f\n",
        )
        .unwrap();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(repo.join("SddIA/core/cumulo.paths.json"), "{}").unwrap();
        fs::create_dir_all(repo.join("SddIA/tools")).unwrap();
        fs::write(
            repo.join("SddIA/tools/send-telegram-notification.md"),
            "---\nname: send-telegram-notification\n---\n",
        )
        .unwrap();

        let instance = repo.join("cliente-smoke");
        let env = run(
            repo,
            &json!({
                "instance_root": instance.display().to_string(),
                "skip_ignition": true
            }),
        )
        .unwrap();
        assert!(env.success, "{:?}", env.error);
        let smoke = env.data.as_ref().unwrap().get("smoke").unwrap();
        assert_eq!(smoke["mode"], "native-topology+local-qa");
        assert_eq!(smoke["success"], true);
        assert!(instance.join(".events/pending").read_dir().unwrap().next().is_some());
    }
}
