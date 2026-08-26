//! Handler nativo `instance-creator` — despliegue hermético consumidor (Kaizen Filtro C).

use super::super::capsules::invoke_tool_capsule_json;
use super::super::workspace::bootstrap_workspace;
use crate::envelope::OrchestratorEnvelope;
use chrono::Utc;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
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

fn starter_local_paths_json() -> &'static str {
    r#"{
  "directories": {
    "local_tools": ".SddIA/tools",
    "local_norms": ".SddIA/norms",
    "local_security": ".SddIA/security",
    "local_constitution": ".SddIA/constitution",
    "local_principles": ".SddIA/principles",
    "local_patterns": ".SddIA/patterns",
    "local_templates": ".SddIA/templates",
    "local_evolution": ".SddIA/evolution",
    "library_codexes": ".SddIA/library/codexes/",
    "library_norms": ".SddIA/library/norms/"
  },
  "paths": {
    "localLibraryCodexes": ".SddIA/library/codexes/",
    "localLibraryNorms": ".SddIA/library/norms/"
  },
  "files": {
    "interaction_triggers_override": ".SddIA/interaction-triggers.override.json",
    "local_security_contract": ".SddIA/local-security-contract.json",
    "local_evolution_log": ".SddIA/evolution/Evolution_log.md",
    "featuresDocumentationPattern": ".SddIA/library/norms/features-documentation-pattern.md",
    "features_documentation_pattern": ".SddIA/library/norms/features-documentation-pattern.md"
  }
}
"#
}

fn local_paths_needs_replace(path: &Path) -> bool {
    if !path.is_file() {
        return true;
    }
    let Ok(raw) = fs::read_to_string(path) else {
        return false;
    };
    let t = raw.trim();
    if t.is_empty() || t == "{}" {
        return true;
    }
    serde_json::from_str::<Value>(t)
        .ok()
        .and_then(|v| v.as_object().map(|o| o.is_empty()))
        .unwrap_or(false)
}

fn materialize_local_paths(repo: &Path, local_paths: &Path) -> Result<(), String> {
    if !local_paths_needs_replace(local_paths) {
        return Ok(());
    }
    let starter = repo.join("SddIA/scripts/starter-kit/.SddIA/local.paths.json");
    if starter.is_file() {
        fs::copy(&starter, local_paths).map_err(|e| e.to_string())?;
        return Ok(());
    }
    fs::write(local_paths, starter_local_paths_json()).map_err(|e| e.to_string())?;
    Ok(())
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

const SYSTEMD_FACTORY_DAEMONS: &[&str] = &[
    "event-watcher",
    "event-sweeper",
    "kalma2-bridge",
    "telegram-watcher",
    "github-bridge-watcher",
];

const SYSTEMD_FACTORY_TEMPLATE: &str = "sddia-daemon@.service.template";

fn install_systemd_templates(repo: &Path, instance_root: &Path) -> Result<Value, String> {
    let templates = repo.join("SddIA/templates/systemd");
    if !templates.is_dir() {
        return Ok(json!({ "installed": false, "reason": "templates ausentes" }));
    }
    let dest = instance_root.join(".SddIA/systemd");
    ensure_dir(&dest)?;
    let mut copied = Vec::new();

    let factory = templates.join(SYSTEMD_FACTORY_TEMPLATE);
    if factory.is_file() {
        let body = fs::read_to_string(&factory).map_err(|e| e.to_string())?;
        for name in SYSTEMD_FACTORY_DAEMONS {
            let rendered = body.replace("@@DAEMON_NAME@@", name);
            let out_name = format!("sddia-{name}@.service");
            let out = dest.join(&out_name);
            fs::write(&out, rendered).map_err(|e| e.to_string())?;
            copied.push(out_name);
        }
    }

    for ent in fs::read_dir(&templates).map_err(|e| e.to_string())? {
        let ent = ent.map_err(|e| e.to_string())?;
        let p = ent.path();
        let fname = p.file_name().unwrap().to_string_lossy().to_string();
        if fname == SYSTEMD_FACTORY_TEMPLATE {
            continue;
        }
        if p.extension().and_then(|e| e.to_str()) == Some("template")
            || p.extension().and_then(|e| e.to_str()) == Some("service")
        {
            let body = fs::read_to_string(&p).map_err(|e| e.to_string())?;
            let out = dest.join(fname.trim_end_matches(".template"));
            fs::write(&out, body).map_err(|e| e.to_string())?;
            copied.push(out.file_name().unwrap().to_string_lossy().to_string());
        }
    }
    Ok(json!({
        "installed": true,
        "units_dir": dest.display().to_string(),
        "units": copied,
        "factory_daemons": SYSTEMD_FACTORY_DAEMONS,
        "note": "systemctl --user enable --now sddia-<daemon>@$(systemd-escape -p instance_root).service"
    }))
}

fn run_smoke(repo: &Path, instance_root: &Path, skip_ignition: bool) -> Value {
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

    // 2) Smoke nativo de topología. No emitir Local_QA_Requested (F-SMOKE-01 / L-QA-EMIT).
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
    let lp_ok = local_paths.is_file() && !local_paths_needs_replace(&local_paths);
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

    checks.insert("local_qa_emitted".into(), json!(false));

    if skip_ignition {
        checks.insert(
            "route_domain".into(),
            json!({ "skipped": true, "reason": "skip_ignition" }),
        );
    } else {
        let probe = probe_route_domain(instance_root);
        let rd_ok = probe.get("success").and_then(|v| v.as_bool()).unwrap_or(false);
        checks.insert("route_domain".into(), probe);
        if !rd_ok {
            ok = false;
        }
    }

    json!({
        "mode": "native-topology+local-qa",
        "success": ok,
        "checks": checks,
        "note": "eda-local-topology-test binario ausente; smoke nativo (sin Local_QA_Requested)"
    })
}

fn probe_route_domain(instance_root: &Path) -> Value {
    let bin = [
        instance_root.join("SddIA/target/release/execute-process"),
        instance_root.join("SddIA/target/debug/execute-process"),
    ]
    .into_iter()
    .find(|p| p.is_file());
    let Some(bin) = bin else {
        return json!({
            "success": false,
            "reason": "execute-process ELF ausente en instancia"
        });
    };
    let domain = instance_root.join(".events/domain");
    if let Err(e) = fs::create_dir_all(&domain) {
        return json!({ "success": false, "error": e.to_string() });
    }
    let eid = Uuid::new_v4().to_string();
    let rel = format!(".events/domain/{eid}.json");
    let event = json!({
        "event_id": eid,
        "event_type": "Instance_Creator_Smoke_Probe",
        "event_family": "domain",
        "timestamp": Utc::now().to_rfc3339(),
        "emitter_agent": "instance-creator",
        "payload": { "source": "instance-creator-smoke-route" }
    });
    let path = instance_root.join(&rel);
    if fs::write(&path, format!("{event}\n")).is_err() {
        return json!({ "success": false, "reason": "write probe event" });
    }
    let inputs = json!({ "event_file_path": rel, "blocking": true });
    match Command::new(&bin)
        .current_dir(instance_root)
        .args([
            "--process",
            "route-domain-event",
            "--inputs",
            &inputs.to_string(),
        ])
        .output()
    {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let line = stdout.lines().last().unwrap_or("");
            let parsed: Value = serde_json::from_str(line).unwrap_or(json!({}));
            let success = parsed
                .get("success")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            json!({
                "success": success,
                "exit_code": out.status.code(),
                "event_path": rel
            })
        }
        Err(e) => json!({ "success": false, "error": e.to_string() }),
    }
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
    materialize_local_paths(repo, &local_paths)?;
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
        run_smoke(repo, &instance_root, skip_ignition)
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
            "[Service]\nWorkingDirectory=%f\nExecStart=%f/SddIA/daemons/email-watcher.sh\n",
        )
        .unwrap();
        fs::write(
            repo.join("SddIA/templates/systemd/sddia-daemon@.service.template"),
            "ExecStart=%f/SddIA/scripts/daemons/@@DAEMON_NAME@@.sh\nWorkingDirectory=%f\n",
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
        let paths_txt = fs::read_to_string(instance.join(".SddIA/local.paths.json")).unwrap();
        assert_ne!(paths_txt.trim(), "{}");
        assert!(paths_txt.contains("local_tools"));
        let unit = fs::read_to_string(
            instance.join(".SddIA/systemd/sddia-email-watcher@.service"),
        )
        .unwrap();
        let inst = instance.display().to_string();
        assert!(
            unit.contains("ExecStart=%f/SddIA/daemons/email-watcher.sh"),
            "ExecStart universal %f, unit={unit}"
        );
        assert!(
            !unit.contains(&inst),
            "ExecStart no debe hornear instance_root absoluto, unit={unit}"
        );
        assert!(
            !unit.contains(&format!("{}/SddIA/daemons", repo.display())),
            "ExecStart no debe ser repo forjador"
        );
        assert!(instance.join(".events/pending").is_dir());
        assert!(instance
            .join(".SddIA/systemd/sddia-email-watcher@.service")
            .is_file());
        let ew = fs::read_to_string(
            instance.join(".SddIA/systemd/sddia-event-watcher@.service"),
        )
        .unwrap();
        assert!(ew.contains("ExecStart=%f/SddIA/scripts/daemons/event-watcher.sh"));
        assert!(ew.contains("WorkingDirectory=%f"));
        assert!(!ew.contains("@@DAEMON_NAME@@"));
        assert!(!ew.contains(&inst));
        assert!(instance
            .join(".SddIA/systemd/sddia-event-sweeper@.service")
            .is_file());
        assert!(instance
            .join(".SddIA/systemd/sddia-kalma2-bridge@.service")
            .is_file());
        assert!(!instance
            .join(".SddIA/systemd/sddia-daemon@.service")
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
        assert_eq!(smoke["checks"]["local_qa_emitted"], false);
        let pending_n = instance
            .join(".events/pending")
            .read_dir()
            .unwrap()
            .count();
        assert_eq!(pending_n, 0, "F-SMOKE-01: no Local_QA_Requested en pending");
    }

    #[test]
    fn replaces_empty_local_paths_stub() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        fs::create_dir_all(repo.join("SddIA/templates/systemd")).unwrap();
        fs::write(
            repo.join("SddIA/templates/systemd/sddia-email-watcher@.service.template"),
            "[Service]\nWorkingDirectory=%f\nExecStart=%f/SddIA/daemons/email-watcher.sh\n",
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

        let instance = repo.join("cliente-stub");
        fs::create_dir_all(instance.join(".SddIA")).unwrap();
        fs::write(instance.join(".SddIA/local.paths.json"), "{}\n").unwrap();

        let env = run(
            repo,
            &json!({
                "instance_root": instance.display().to_string(),
                "skip_smoke": true,
                "skip_ignition": true,
            }),
        )
        .unwrap();
        assert!(env.success);
        let paths_txt = fs::read_to_string(instance.join(".SddIA/local.paths.json")).unwrap();
        assert_ne!(paths_txt.trim(), "{}");
        assert!(paths_txt.contains("local_tools"));
    }
}
