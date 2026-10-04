//! Procesos `refine-pbi` y `refine-hu` — refinamiento pre-forja y emisión ECST (HU-A 06).

use super::super::capsules::invoke_tool_for_process;
use super::super::project_binding::{self, resolve_doc_path, TrackerConfig};
use super::super::route_domain_core::materialize_pending_domain_event;
use crate::envelope::OrchestratorEnvelope;
use chrono::Utc;
use serde_json::{json, Value};
use serde_yaml::Value as YamlValue;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

const PROCESS_PBI: &str = "refine-pbi";
const PROCESS_HU: &str = "refine-hu";

fn str_field(inputs: &Value, key: &str) -> Option<String> {
    inputs
        .get(key)
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn split_frontmatter(raw: &str) -> Result<(String, String), String> {
    let mut lines = raw.lines();
    if lines.next() != Some("---") {
        return Err("REFINE_INVALID: sin frontmatter".into());
    }
    let mut fm = String::new();
    for line in lines.by_ref() {
        if line == "---" {
            let body: String = lines.collect::<Vec<_>>().join("\n");
            return Ok((fm, body));
        }
        fm.push_str(line);
        fm.push('\n');
    }
    Err("REFINE_INVALID: frontmatter sin cierre".into())
}

fn parse_fm(text: &str) -> Result<YamlValue, String> {
    let (fm, _) = split_frontmatter(text)?;
    serde_yaml::from_str(&fm).map_err(|e| format!("REFINE_INVALID: yaml {e}"))
}

fn fm_str(fm: &YamlValue, key: &str) -> Option<String> {
    fm.get(key)
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn seal_markdown(raw: &str, patch: &YamlValue) -> Result<String, String> {
    let (fm_text, body) = split_frontmatter(raw)?;
    let mut fm: YamlValue = serde_yaml::from_str(&fm_text).map_err(|e| e.to_string())?;
    if let Some(map) = patch.as_mapping() {
        for (k, v) in map {
            if let Some(key) = k.as_str() {
                fm[key] = v.clone();
            }
        }
    }
    let new_fm = serde_yaml::to_string(&fm).map_err(|e| e.to_string())?;
    let body = body.trim_start_matches('\n');
    Ok(format!("---\n{new_fm}---\n\n{body}"))
}

fn mayeuta_touch_body(body: &str) -> String {
    const MARKER: &str = "<!-- refined:mayeuta -->";
    if body.contains(MARKER) {
        return body.to_string();
    }
    if body.trim().is_empty() {
        return format!("{MARKER}\n");
    }
    format!("{body}\n\n{MARKER}\n")
}

fn title_from_hu(fm: &YamlValue, body: &str) -> String {
    if let Some(t) = fm_str(fm, "title") {
        return t;
    }
    for line in body.lines() {
        let t = line.trim();
        if t.starts_with("# ") {
            return t.trim_start_matches('#').trim().to_string();
        }
    }
    "HU".to_string()
}

fn resolve_pbi_path(bound: &project_binding::BoundProject, pbi_ref: &str) -> Result<PathBuf, String> {
    let pending_root =
        resolve_doc_path(&bound.project_root, &bound.docs.todos_pending)?;
    let full = if Path::new(pbi_ref).is_absolute() {
        let p = PathBuf::from(pbi_ref);
        if !p.starts_with(&bound.project_root) {
            return Err("PROJECT_SCOPE_ESCAPE: pbi_ref".into());
        }
        p
    } else {
        resolve_doc_path(&bound.project_root, pbi_ref)?
    };
    if !full.starts_with(&pending_root) {
        return Err(format!(
            "REFINE_SCOPE: pbi_ref debe estar bajo {}",
            bound.docs.todos_pending
        ));
    }
    if !full.is_file() {
        return Err(format!("REFINE_NOT_FOUND: {}", full.display()));
    }
    Ok(full)
}

fn resolve_hu_path(bound: &project_binding::BoundProject, hu_ref: &str) -> Result<PathBuf, String> {
    let full = if Path::new(hu_ref).is_absolute() {
        let p = PathBuf::from(hu_ref);
        if !p.starts_with(&bound.project_root) {
            return Err("PROJECT_SCOPE_ESCAPE: hu_ref".into());
        }
        p
    } else {
        resolve_doc_path(&bound.project_root, hu_ref)?
    };
    let rel = full
        .strip_prefix(&bound.project_root)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();
    if !rel.contains("historias") {
        return Err("REFINE_SCOPE: hu_ref debe residir bajo historias/".into());
    }
    if !full.is_file() {
        return Err(format!("REFINE_NOT_FOUND: {}", full.display()));
    }
    Ok(full)
}

fn rel_path(project_root: &Path, path: &Path) -> String {
    path.strip_prefix(project_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn emit_refined(
    repo: &Path,
    event_type: &str,
    source_process: &str,
    ref_field: &str,
    ref_value: &str,
    project_slug: &str,
    tracker_ref: Option<&str>,
) -> Result<String, String> {
    let event_id = Uuid::new_v4().to_string();
    let occurred_at = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let mut payload = json!({
        "event_id": event_id,
        "correlation_id": event_id,
        "source_process": source_process,
        ref_field: ref_value,
        "occurred_at": occurred_at,
        "project_slug": project_slug,
    });
    if let Some(tr) = tracker_ref.filter(|s| !s.is_empty()) {
        payload["tracker_ref"] = json!(tr);
    }
    materialize_pending_domain_event(repo, event_type, source_process, payload)
}

fn emit_create_issue_failed(
    repo: &Path,
    issue_ref: &str,
    error_code: &str,
    project_slug: &str,
) -> Result<(), String> {
    let event_id = Uuid::new_v4().to_string();
    let payload = json!({
        "issue_ref": issue_ref,
        "operation": "create_issue",
        "error_code": error_code,
        "source_process": PROCESS_HU,
        "occurred_at": Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        "project_slug": project_slug,
        "event_id": event_id,
        "correlation_id": event_id,
    });
    materialize_pending_domain_event(repo, "Tracker_Sync_Failed", PROCESS_HU, payload)?;
    Ok(())
}

fn backlog_state_name(tc: &TrackerConfig) -> Option<String> {
    tc.state_map.get("backlog").cloned()
}

fn try_create_hu_issue(
    repo: &Path,
    tc: &TrackerConfig,
    title: &str,
    description: &str,
) -> Result<String, String> {
    let mut request = json!({
        "operation": "create_issue",
        "team_key": tc.team_key,
        "title": title,
        "description": description,
        "labels": [tc.label_hu.clone()],
    });
    if let Some(pid) = &tc.project_id {
        request["project_id"] = json!(pid);
    }
    if let Some(state) = backlog_state_name(tc) {
        request["state_name"] = json!(state);
    }
    let tool_body = invoke_tool_for_process(
        repo,
        "linear-tracker-adapter",
        &json!({ "request": request }),
        Some(PROCESS_HU),
    )?;
    tool_body
        .get("issue_ref")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .ok_or_else(|| "LINEAR_CREATE_ISSUE_EMPTY: sin issue_ref".into())
}

pub fn run_pbi(repo: &Path, inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let pbi_ref = str_field(inputs, "pbi_ref").ok_or("pbi_ref requerido")?;
    let bound = project_binding::bind(repo, inputs)?
        .ok_or("project_slug requerido para refine-pbi")?;
    let path = resolve_pbi_path(&bound, &pbi_ref)?;
    let raw = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let (_, body) = split_frontmatter(&raw)?;
    let refined_body = mayeuta_touch_body(&body);
    let refined_date = Utc::now().format("%Y-%m-%d").to_string();
    let sealed = seal_markdown(
        &raw,
        &serde_yaml::from_str(&format!(
            "status: refinado\nrefined: \"{refined_date}\""
        ))
        .map_err(|e| e.to_string())?,
    )?;
    let sealed = if refined_body != body {
        let (fm, _) = split_frontmatter(&sealed)?;
        format!("---\n{fm}---\n\n{}", refined_body.trim_start())
    } else {
        sealed
    };
    let fm = parse_fm(&sealed)?;
    fs::write(&path, &sealed).map_err(|e| e.to_string())?;
    let tracker_ref = fm_str(&fm, "tracker_ref");
    let rel = rel_path(&bound.project_root, &path);
    let event_rel = emit_refined(
        repo,
        "PBI_Refined",
        PROCESS_PBI,
        "pbi_ref",
        &rel,
        &bound.slug,
        tracker_ref.as_deref(),
    )?;
    Ok(OrchestratorEnvelope {
        success: true,
        status_code: 0,
        data: Some(json!({
            "pbi_ref": rel,
            "status": "refinado",
            "event_path": event_rel,
            "tracker_ref": tracker_ref,
        })),
        error: None,
        execution_report: Some(json!({
            "phases": [
                {"phase_name": "Refinamiento Mayeuta", "status": "executed", "handler": PROCESS_PBI},
                {"phase_name": "Sellado Argos", "status": "executed", "handler": PROCESS_PBI},
            ]
        })),
        exit_code: 0,
    })
}

pub fn run_hu(repo: &Path, inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let hu_ref = str_field(inputs, "hu_ref").ok_or("hu_ref requerido")?;
    let bound = project_binding::bind(repo, inputs)?
        .ok_or("project_slug requerido para refine-hu")?;
    let path = resolve_hu_path(&bound, &hu_ref)?;
    let rel = rel_path(&bound.project_root, &path);
    let raw = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let fm = parse_fm(&raw)?;
    let (_, body) = split_frontmatter(&raw)?;
    let refined_body = mayeuta_touch_body(&body);

    let mut tracker_ref = fm_str(&fm, "tracker_ref");
    let mut create_issue_invoked = false;
    let tracker_cfg = project_binding::tracker_config_for_inputs(repo, inputs)?;

    if tracker_ref.is_none() {
        if let Some(tc) = tracker_cfg {
            let title = title_from_hu(&fm, &body);
            let description = refined_body.chars().take(8000).collect::<String>();
            match try_create_hu_issue(repo, &tc, &title, &description) {
                Ok(issue_ref) => {
                    create_issue_invoked = true;
                    tracker_ref = Some(issue_ref);
                }
                Err(code) => {
                    let err_code = if code.len() > 120 {
                        code.chars().take(120).collect()
                    } else {
                        code
                    };
                    let _ = emit_create_issue_failed(repo, &rel, &err_code, &bound.slug);
                }
            }
        }
    }

    let refined_date = Utc::now().format("%Y-%m-%d").to_string();
    let mut patch_lines = format!("status: refinada\nrefined: \"{refined_date}\"");
    if let Some(tr) = &tracker_ref {
        patch_lines.push_str(&format!("\ntracker_ref: \"{tr}\""));
    }
    let patch: YamlValue = serde_yaml::from_str(&patch_lines).map_err(|e| e.to_string())?;
    let mut sealed = seal_markdown(&raw, &patch)?;
    if refined_body != body {
        let (fm_text, _) = split_frontmatter(&sealed)?;
        sealed = format!("---\n{fm_text}---\n\n{}", refined_body.trim_start());
    }
    fs::write(&path, sealed).map_err(|e| e.to_string())?;

    let event_rel = emit_refined(
        repo,
        "HU_Refined",
        PROCESS_HU,
        "hu_ref",
        &rel,
        &bound.slug,
        tracker_ref.as_deref(),
    )?;

    Ok(OrchestratorEnvelope {
        success: true,
        status_code: 0,
        data: Some(json!({
            "hu_ref": rel,
            "status": "refinada",
            "event_path": event_rel,
            "tracker_ref": tracker_ref,
            "create_issue_invoked": create_issue_invoked,
        })),
        error: None,
        execution_report: Some(json!({
            "phases": [
                {"phase_name": "Refinamiento Mayeuta", "status": "executed", "handler": PROCESS_HU},
                {
                    "phase_name": "Registro Linear",
                    "status": if create_issue_invoked { "executed" } else { "skipped" },
                    "handler": PROCESS_HU,
                },
                {"phase_name": "Sellado Argos", "status": "executed", "handler": PROCESS_HU},
            ]
        })),
        exit_code: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::repo::find_repo_root;
    use std::sync::{Mutex, OnceLock};

    fn lab_guard() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
    }

    fn with_lab<F: FnOnce()>(f: F) {
        let _g = lab_guard();
        let repo = find_repo_root().expect("repo");
        std::env::set_var("SDDIA_REPO_ROOT", repo.to_string_lossy().as_ref());
        std::env::set_var("SDDIA_LAB_MOCK_OUTBOUND", "1");
        std::env::remove_var("LINEAR_API_TOKEN");
        let _ = fs::remove_file(repo.join(".SddIA/lab-linear-store.json"));
        f();
        std::env::remove_var("SDDIA_LAB_MOCK_OUTBOUND");
        let _ = fs::remove_file(repo.join(".SddIA/lab-linear-store.json"));
    }

    fn write_git_root(root: &Path) {
        fs::create_dir_all(root.join(".git")).unwrap();
    }

    fn index_md(uuid: &str, client: &Path, slug: &str) -> String {
        format!(
            "---\nid: {slug}\nuuid: \"{uuid}\"\nproject_root: \"{}\"\nmanifest_ref: .SddIA/project.md\ncodex_slug: codex-software-engineering\nstatus: active\n---\n\n# {slug}\n",
            client.display()
        )
    }

    fn manifest_md(uuid: &str, version: &str, tracker: bool) -> String {
        let tracker_block = if tracker {
            "tracker:\n  provider: linear\n  team_key: OSC\n  state_map:\n    backlog: Backlog\n    done: Done\n  labels:\n    hu: hu\n    pbi: pbi\n"
        } else {
            ""
        };
        format!(
            "---\nid: lab\nuuid: \"{uuid}\"\ngit_remote: https://example.invalid/lab.git\ndefault_branch: main\ndelivery_mode: branch_pr\ncontract_version: \"{version}\"\ncodex_slug: codex-software-engineering\n{tracker_block}docs_layout:\n  features: docs/features\n  fixes: docs/fixes\n  todos_pending: docs/todos/pending\n  todos_done: docs/todos/done\n---\n\n# lab\n"
        )
    }

    fn register_lab_project(core: &Path, slug: &str, tracker: bool) -> PathBuf {
        let client = core.join(format!("_refine_lab_{slug}"));
        write_git_root(&client);
        fs::create_dir_all(client.join(".SddIA")).unwrap();
        let uuid = "aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee";
        fs::write(
            client.join(".SddIA/project.md"),
            manifest_md(uuid, "1.2.0", tracker),
        )
        .unwrap();
        fs::create_dir_all(core.join(".SddIA/projects")).unwrap();
        fs::write(
            core.join(".SddIA/projects").join(format!("{slug}.md")),
            index_md(uuid, &client, slug),
        )
        .unwrap();
        client
    }

    fn cleanup_project_index(core: &Path, slug: &str) {
        let _ = fs::remove_file(core.join(".SddIA/projects").join(format!("{slug}.md")));
        let _ = fs::remove_dir_all(core.join(format!("_refine_lab_{slug}")));
    }

    #[test]
    fn ac_ref_1_refine_pbi_emits_and_seals() {
        let core = find_repo_root().expect("repo");
        let slug = "refine-ac-ref1";
        let client = register_lab_project(&core, slug, false);
        let pbi_path = client.join("docs/todos/pending/ac-ref-pbi.md");
        fs::create_dir_all(pbi_path.parent().unwrap()).unwrap();
        fs::write(
            &pbi_path,
            "---\ndocument_id: PBI-TEST\nstatus: pending\n---\n\n# PBI\n",
        )
        .unwrap();
        let hu_rel = "docs/todos/pending/ac-ref-pbi.md";
        let out = run_pbi(
            &core,
            &json!({"project_slug": slug, "pbi_ref": hu_rel}),
        )
        .expect("run_pbi");
        assert_eq!(
            out.data.as_ref().and_then(|d| d.get("status")).and_then(|v| v.as_str()),
            Some("refinado")
        );
        let text = fs::read_to_string(&pbi_path).unwrap();
        assert!(text.contains("status: refinado"));
        assert!(out.data.as_ref().and_then(|d| d.get("event_path")).is_some());
        cleanup_project_index(&core, slug);
    }

    #[test]
    fn ac_ref_2_without_tracker_no_capsule() {
        let core = find_repo_root().expect("repo");
        let slug = "refine-ac-ref2";
        let client = register_lab_project(&core, slug, false);
        let hu_path = client.join("docs/todos/historias/ac-ref-hu.md");
        fs::create_dir_all(hu_path.parent().unwrap()).unwrap();
        fs::write(
            &hu_path,
            "---\ntitle: HU lab\nstatus: pending\n---\n\n# HU\n",
        )
        .unwrap();
        let out = run_hu(
            &core,
            &json!({
                "project_slug": slug,
                "hu_ref": "docs/todos/historias/ac-ref-hu.md",
            }),
        )
        .expect("run_hu");
        assert_eq!(
            out.data
                .as_ref()
                .and_then(|d| d.get("create_issue_invoked"))
                .and_then(|v| v.as_bool()),
            Some(false)
        );
        let text = fs::read_to_string(&hu_path).unwrap();
        assert!(text.contains("status: refinada"));
        cleanup_project_index(&core, slug);
    }

    #[test]
    fn ac_0c_creates_tracker_ref_when_missing() {
        with_lab(|| {
            let core = find_repo_root().expect("repo");
            let slug = "refine-ac-0c";
            let client = register_lab_project(&core, slug, true);
            let hu_path = client.join("docs/todos/historias/ac-0c-hu.md");
            fs::create_dir_all(hu_path.parent().unwrap()).unwrap();
            fs::write(
                &hu_path,
                "---\ntitle: HU AC0c\nstatus: pending\n---\n\n# HU AC0c\n",
            )
            .unwrap();
            let out = run_hu(
                &core,
                &json!({
                    "project_slug": slug,
                    "hu_ref": "docs/todos/historias/ac-0c-hu.md",
                }),
            )
            .expect("run_hu");
            assert_eq!(
                out.data
                    .as_ref()
                    .and_then(|d| d.get("create_issue_invoked"))
                    .and_then(|v| v.as_bool()),
                Some(true)
            );
            let text = fs::read_to_string(&hu_path).unwrap();
            assert!(text.contains("tracker_ref:"));
            assert!(text.contains("tracker_ref:"));
            cleanup_project_index(&core, slug);
        });
    }

    #[test]
    fn ac_0c_skips_create_when_tracker_ref_present() {
        with_lab(|| {
            let core = find_repo_root().expect("repo");
            let slug = "refine-ac-0c-idem";
            let client = register_lab_project(&core, slug, true);
            let hu_path = client.join("docs/todos/historias/ac-0c-idem.md");
            fs::create_dir_all(hu_path.parent().unwrap()).unwrap();
            fs::write(
                &hu_path,
                "---\ntitle: HU idem\ntracker_ref: LAB-HU-EXISTING\nstatus: pending\n---\n\n# HU\n",
            )
            .unwrap();
            let out = run_hu(
                &core,
                &json!({
                    "project_slug": slug,
                    "hu_ref": "docs/todos/historias/ac-0c-idem.md",
                }),
            )
            .expect("run_hu");
            assert_eq!(
                out.data
                    .as_ref()
                    .and_then(|d| d.get("create_issue_invoked"))
                    .and_then(|v| v.as_bool()),
                Some(false)
            );
            let text = fs::read_to_string(&hu_path).unwrap();
            assert!(text.contains("LAB-HU-EXISTING"));
            assert!(!text.contains("tracker_ref:"));
            cleanup_project_index(&core, slug);
        });
    }
}
