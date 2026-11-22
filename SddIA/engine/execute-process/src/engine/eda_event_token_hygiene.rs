//! Oráculo O14b — higiene de tokens de evento (`event_uri` vs rutas legacy).

#[cfg(test)]
mod tests {
    use crate::core::repo::find_repo_root;
    use serde_json::json;
    use std::fs;
    use std::path::{Path, PathBuf};

    const FORBIDDEN_IN_EVENT_CONTRACT: &[&str] = &[
        "event_file_path",
        "event_file_paths",
        "orchestration_event_path",
    ];

    /// Genoma: `target_path` permitido solo en entidades de fichero/PBI (E-2).
    const GENOME_TARGET_PATH_ALLOW: &[&str] = &[
        "SddIA/actions/materialize-fracture-pbi.md",
        "SddIA/actions/materialize-kaizen-alert-doc.md",
        "SddIA/actions/enrich-fracture-pbi-kaizen.md",
        "SddIA/tools/markdown-table-editor.md",
        "SddIA/skills/filesystem-manager.md",
        "SddIA/norms/skill-io-filesystem-manager-frozen.md",
        "SddIA/events/domain/fracture-pbi-materialized.md",
        "SddIA/tools/event-bus-audit.md",
    ];

    const GENOME_SCAN_ROOTS: &[&str] = &["SddIA/process", "SddIA/actions", "SddIA/events"];

    const PORT_ROOT: &str = "SddIA/sddia-core/src/ports/event_manager";

    fn rel(repo: &Path, abs: &Path) -> String {
        abs.strip_prefix(repo)
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .unwrap_or_else(|_| abs.display().to_string())
    }

    fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(read) = fs::read_dir(dir) else {
            return;
        };
        for ent in read.flatten() {
            let path = ent.path();
            if path.is_dir() {
                collect_rs_files(&path, out);
            } else if path.extension().is_some_and(|e| e == "rs") {
                out.push(path);
            }
        }
    }

    fn file_contains_any(haystack: &str, needles: &[&str]) -> Option<String> {
        needles
            .iter()
            .find(|n| haystack.contains(**n))
            .map(|s| (*s).to_string())
    }

    fn genome_allow_target_path(rel_path: &str) -> bool {
        GENOME_TARGET_PATH_ALLOW.contains(&rel_path)
    }


    /// MAN-49 R-01: regresión — raíz del repo sin dirs de topología de bus.
    fn assert_repo_root_bus_dirs_absent(repo: &Path) -> Result<(), String> {
        for name in [
            "orchestration",
            "telemetry",
            "pending",
            "domain",
            "dead-letter",
            "progress",
            ".events",
        ] {
            let p = repo.join(name);
            if p.is_dir() {
                return Err(format!(
                    "raíz contaminada: existe directorio de topología `{name}/`"
                ));
            }
        }
        Ok(())
    }

    fn scan_genome_contract(repo: &Path) -> Result<(), String> {
        for root in GENOME_SCAN_ROOTS {
            let abs = repo.join(root);
            if !abs.is_dir() {
                continue;
            }
            for ent in walkdir_light(&abs)? {
                if !ent.ends_with(".md") {
                    continue;
                }
                let rel_path = rel(repo, &ent);
                let text = fs::read_to_string(&ent).map_err(|e| e.to_string())?;
                let fm = text
                    .split("---")
                    .nth(1)
                    .unwrap_or("")
                    .to_string();
                for token in FORBIDDEN_IN_EVENT_CONTRACT {
                    if fm.contains(token) {
                        return Err(format!(
                            "genoma {rel_path}: token prohibido `{token}` en frontmatter"
                        ));
                    }
                }
                if !genome_allow_target_path(&rel_path) && fm.contains("target_path") {
                    return Err(format!(
                        "genoma {rel_path}: `target_path` en frontmatter fuera de lista blanca E-2"
                    ));
                }
            }
        }
        Ok(())
    }

    fn walkdir_light(dir: &Path) -> Result<Vec<PathBuf>, String> {
        let mut stack = vec![dir.to_path_buf()];
        let mut out = Vec::new();
        while let Some(cur) = stack.pop() {
            let read = fs::read_dir(&cur).map_err(|e| e.to_string())?;
            for ent in read {
                let ent = ent.map_err(|e| e.to_string())?;
                let path = ent.path();
                if path.is_dir() {
                    stack.push(path);
                } else {
                    out.push(path);
                }
            }
        }
        Ok(out)
    }

    fn scan_port_sources(repo: &Path) -> Result<(), String> {
        let port_dir = repo.join(PORT_ROOT);
        let mut files = Vec::new();
        collect_rs_files(&port_dir, &mut files);
        for path in files {
            let rel_path = rel(repo, &path);
            let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
            if let Some(token) =
                file_contains_any(&text, &["target_path", "event_file_path", "logical_rel"])
            {
                return Err(format!("puerto {rel_path}: contiene `{token}`"));
            }
        }
        Ok(())
    }

    fn engine_target_path_whitelist_rel(rel: &str) -> bool {
        matches!(
            rel,
            "SddIA/engine/execute-process/src/core/fracture_pbi.rs"
                | "SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs"
                | "SddIA/engine/execute-process/src/engine/materialize_fracture_pbi.rs"
                | "SddIA/engine/execute-process/src/engine/materialize_kaizen_alert_doc.rs"
                | "SddIA/engine/execute-process/src/engine/materialize_ci_chronic_failure_pbi.rs"
        ) || rel.contains("/daemons/event-watcher/")
    }

    fn scan_engine_event_tokens(repo: &Path) -> Result<(), String> {
        let roots = [
            repo.join("SddIA/engine/execute-process/src"),
            repo.join("SddIA/daemons/event-watcher/src"),
            repo.join("SddIA/tools/sddia-qa/src"),
        ];
        for root in roots {
            if !root.is_dir() {
                continue;
            }
            for path in walkdir_light(&root)? {
                if path.extension().is_none_or(|e| e != "rs") {
                    continue;
                }
                let rel_path = rel(repo, &path);
                if rel_path.ends_with("eda_event_token_hygiene.rs") {
                    continue;
                }
                let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
                for token in FORBIDDEN_IN_EVENT_CONTRACT {
                    if text.contains(token) {
                        return Err(format!("motor {rel_path}: contiene `{token}`"));
                    }
                }
                if text.contains("target_path") && !engine_target_path_whitelist_rel(&rel_path) {
                    return Err(format!(
                        "motor {rel_path}: `target_path` fuera de lista blanca E-2"
                    ));
                }
            }
        }
        Ok(())
    }

    #[test]
    fn write_fractal_event_seal_exposes_event_uri_only() {
        let repo = find_repo_root().expect("repo");
        let _h = crate::engine::eda_port::EventManagerTestHarness::new();
        let event_id = uuid::Uuid::new_v4().to_string();
        let event = json!({
            "event_id": event_id,
            "event_type": "Lab_Event_Token_Smoke",
            "timestamp": "2026-11-14T12:00:00Z",
            "emitter_agent": "eda_event_token_hygiene",
            "payload": { "smoke": true }
        });
        let seal = crate::engine::eda_bus::write_fractal_event(&repo, &event, "telemetry")
            .expect("write_fractal_event");
        assert!(seal.get("target_path").is_none());
        let uri = seal
            .get("event_uri")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        assert!(uri.starts_with("eda://telemetry/"));
    }

    #[test]
    fn eda_event_token_hygiene_genome_frontmatter() {
        let repo = find_repo_root().expect("repo");
        scan_genome_contract(&repo).expect("genoma sin tokens legacy de evento");
    }

    #[test]
    fn eda_event_token_hygiene_repo_root_no_bus_topology_dirs() {
        let repo = find_repo_root().expect("repo");
        assert_repo_root_bus_dirs_absent(&repo).expect("raíz sin dirs de bus");
    }

    /// MAN-49 R-03: escaneo cuerpo completo (oráculo AC-3). Se activa cuando genoma/README estén limpios.
    fn scan_genome_and_readme_body(repo: &Path) -> Result<(), String> {
        const BODY_FORBIDDEN: &[&str] = &[
            ".events/",
            "event_file_path",
            "EventStore::",
            "StoreMode::",
            "filesystem/dual",
            "processing/subscribers",
        ];
        let mut files = Vec::new();
        for ent in walkdir_light(&repo.join("SddIA"))? {
            if ent.extension().is_some_and(|e| e == "md") {
                files.push(ent);
            }
        }
        let readme = repo.join("README.md");
        if readme.is_file() {
            files.push(readme);
        }
        for ent in files {
            let rel_path = rel(repo, &ent);
            if rel_path.starts_with("SddIA/evolution/")
                || rel_path == "SddIA/scripts/qa/clean-project-ephemeral.md"
            {
                continue;
            }
            let raw = fs::read_to_string(&ent).map_err(|e| e.to_string())?;
            let text = raw.replace("directories.events/", "directories_events/");
            for token in BODY_FORBIDDEN {
                if text.contains(token) {
                    return Err(format!("{rel_path}: token legacy `{token}`"));
                }
            }
        }
        Ok(())
    }

    #[test]
    fn eda_event_token_hygiene_genome_and_readme_body() {
        let repo = find_repo_root().expect("repo");
        scan_genome_and_readme_body(&repo).expect("SddIA/**/*.md + README sin tokens legacy");
    }

    #[test]
    fn eda_event_token_hygiene_genome_domain_codex_processes() {
        let repo = find_repo_root().expect("repo");
        let roots = [
            "SddIA/library/codexes/codex-software-engineering/process",
            "SddIA/library/codexes/codex-kalma2-assistant/process",
        ];
        for root in roots {
            let abs = repo.join(root);
            for path in walkdir_light(&abs).unwrap_or_default() {
                if !path.ends_with(".md") {
                    continue;
                }
                let rel_path = rel(&repo, &path);
                let text = fs::read_to_string(&path).unwrap_or_default();
                let fm = text.split("---").nth(1).unwrap_or("");
                for token in FORBIDDEN_IN_EVENT_CONTRACT {
                    assert!(
                        !fm.contains(token),
                        "codex process {rel_path}: `{token}`"
                    );
                }
                assert!(
                    !fm.contains("target_path"),
                    "codex process {rel_path}: `target_path` en frontmatter"
                );
            }
        }
    }

    #[test]
    fn eda_event_token_hygiene_port_sources_clean() {
        let repo = find_repo_root().expect("repo");
        scan_port_sources(&repo).expect("puerto limpio");
    }

    #[test]
    fn eda_event_token_hygiene_engine_sources_clean() {
        let repo = find_repo_root().expect("repo");
        scan_engine_event_tokens(&repo).expect("motor limpio");
    }

    /// Exclusiones de ámbito de escaneo G-2 (con motivo explícito).
    const SCAN_EXCLUDE: &[(&str, &str)] = &[
        (
            "eda_event_token_hygiene.rs",
            "propio oráculo con literales de patrones prohibidos",
        ),
        (
            "gate_eda_unit_no_real_backend.rs",
            "gate QA que inspecciona referencias a rutas legacy de eventos en tests",
        ),
        (
            "gate_event_manager_port.rs",
            "gate QA que veta patrones legacy y evasiones concat! en código productivo",
        ),
    ];

    /// Excepciones explícitas para literales `.events/...` en código productivo (E-2 e higiene/retención).
    const VIRTUAL_PATH_ALLOW: &[(&str, &str)] = &[
        (
            "SddIA/tools/event-bus-audit/src/main.rs",
            "herramienta de auditoría/higiene del bus de eventos",
        ),
        (
            "SddIA/tools/repo-retention-purger/src/slice_events.rs",
            "purgador de retención de artefactos efímeros",
        ),
        (
            "SddIA/tools/sddia-qa/src/eda_e2e_lab.rs",
            "laboratorio E2E de QA",
        ),
    ];

    // Literales de ruta (no campo Rust `.events`): comillas y/o barra.
    const VIRTUAL_EVENT_SUBDIRS: &[&str] = &[
        "\".events/\"",
        "\".events\"",
        "'.events/'",
        "'.events'",
        ".events/",
        ".events\\",
    ];

    fn collect_workspace_rs(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(read) = fs::read_dir(dir) else {
            return;
        };
        for ent in read.flatten() {
            let path = ent.path();
            let name = ent.file_name().to_string_lossy().to_string();
            if path.is_dir() {
                if matches!(name.as_str(), "target" | "tests" | ".git" | "node_modules") {
                    continue;
                }
                collect_workspace_rs(&path, out);
            } else if path.extension().is_some_and(|e| e == "rs") {
                out.push(path);
            }
        }
    }

    fn strip_cfg_test_and_comments(text: &str) -> Vec<(usize, String)> {
        let mut out = Vec::new();
        let mut in_cfg_test = false;
        let mut pending_cfg_test = false;
        let mut brace_depth: i32 = 0;

        for (idx, line) in text.lines().enumerate() {
            let trimmed = line.trim();
            if !in_cfg_test {
                if trimmed.starts_with("#[cfg(test)]") {
                    pending_cfg_test = true;
                    continue;
                }
                if pending_cfg_test {
                    if trimmed.contains('{') {
                        in_cfg_test = true;
                        pending_cfg_test = false;
                        brace_depth = trimmed.chars().filter(|&c| c == '{').count() as i32
                            - trimmed.chars().filter(|&c| c == '}').count() as i32;
                        if brace_depth <= 0 {
                            in_cfg_test = false;
                        }
                    }
                    continue;
                }
                if trimmed.starts_with("//") {
                    continue;
                }
                out.push((idx + 1, line.to_string()));
            } else {
                brace_depth += trimmed.chars().filter(|&c| c == '{').count() as i32;
                brace_depth -= trimmed.chars().filter(|&c| c == '}').count() as i32;
                if brace_depth <= 0 {
                    in_cfg_test = false;
                }
            }
        }
        out
    }

    #[test]
    fn eda_event_token_hygiene_no_virtual_event_paths_workspace() {
        let repo = find_repo_root().expect("repo");
        let sddia_dir = repo.join("SddIA");
        let mut files = Vec::new();
        collect_workspace_rs(&sddia_dir, &mut files);
        files.sort();

        let mut violations = Vec::new();
        for path in files {
            let rel_path = rel(&repo, &path);
            if SCAN_EXCLUDE.iter().any(|(ex, _)| rel_path.ends_with(ex)) {
                continue;
            }
            if VIRTUAL_PATH_ALLOW.iter().any(|(al, _)| rel_path == *al) {
                continue;
            }
            let text = fs::read_to_string(&path).unwrap_or_default();
            for (lineno, line) in strip_cfg_test_and_comments(&text) {
                for pat in VIRTUAL_EVENT_SUBDIRS {
                    if line.contains(pat) {
                        violations.push(format!("{rel_path}:{lineno}: contiene `{pat}`"));
                    }
                }
            }
        }
        assert!(
            violations.is_empty(),
            "rutas virtuales de eventos en código productivo:\n{}",
            violations.join("\n")
        );
    }

    #[test]
    fn eda_event_token_hygiene_cli_outputs_no_event_path() {
        let repo = find_repo_root().expect("repo");
        let sddia_dir = repo.join("SddIA");
        let mut files = Vec::new();
        collect_workspace_rs(&sddia_dir, &mut files);
        files.sort();

        let mut violations = Vec::new();
        for path in files {
            let rel_path = rel(&repo, &path);
            if SCAN_EXCLUDE.iter().any(|(ex, _)| rel_path.ends_with(ex)) {
                continue;
            }
            let text = fs::read_to_string(&path).unwrap_or_default();
            for (lineno, line) in strip_cfg_test_and_comments(&text) {
                if line.contains("\"event_path\"") {
                    violations.push(format!("{rel_path}:{lineno}: clave JSON `\"event_path\"`"));
                }
            }
        }
        assert!(
            violations.is_empty(),
            "salidas CLI con clave `\"event_path\"` en código productivo:\n{}",
            violations.join("\n")
        );
    }
}