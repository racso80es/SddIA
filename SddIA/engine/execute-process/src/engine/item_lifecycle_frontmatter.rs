//! Frontmatter del ítem de backlog: columna de tracker vs refinamiento (AUT-5 / MAN-14).

use serde_yaml::Value as YamlValue;

pub const FM_TRACKER_COLUMN: &str = "tracker_column";
pub const FM_REFINEMENT: &str = "refinement";
pub const FM_REFINED_DATE: &str = "refined";

pub const REFINEMENT_PENDING: &str = "pending";
pub const REFINEMENT_REFINED: &str = "refined";

pub const COLUMN_BACKLOG: &str = "backlog";
pub const COLUMN_TODO: &str = "todo";
pub const COLUMN_IN_PROGRESS: &str = "in_progress";
pub const COLUMN_IN_REVIEW: &str = "in_review";
pub const COLUMN_DONE: &str = "done";
pub const COLUMN_CANCELLED: &str = "cancelled";

/// Líneas YAML (sin `---`) para el alta `forge-pbi`.
pub fn forge_frontmatter_lines() -> String {
    format!(
        "{FM_TRACKER_COLUMN}: {COLUMN_BACKLOG}\n{FM_REFINEMENT}: {REFINEMENT_PENDING}"
    )
}

pub fn yaml_patch_refinement_refined(refined_date: &str) -> YamlValue {
    serde_yaml::from_str(&format!(
        "{FM_TRACKER_COLUMN}: {COLUMN_TODO}\n{FM_REFINEMENT}: {REFINEMENT_REFINED}\n{FM_REFINED_DATE}: \"{refined_date}\""
    ))
    .expect("patch refinado")
}

pub fn yaml_patch_tracker_column(column: &str) -> YamlValue {
    serde_yaml::from_str(&format!("{FM_TRACKER_COLUMN}: {column}"))
        .expect("patch columna")
}

pub fn yaml_patch_cancelled() -> YamlValue {
    serde_yaml::from_str(&format!(
        "{FM_TRACKER_COLUMN}: {COLUMN_CANCELLED}\n{FM_REFINEMENT}: {REFINEMENT_REFINED}"
    ))
    .expect("patch cancelado")
}

/// Convierte descripciones de tracker (p. ej. bloque `yaml` de Linear) al formato `---` / cuerpo.
pub fn normalize_item_markdown(raw: &str) -> Result<String, String> {
    let trimmed = raw.trim_start();
    if trimmed.starts_with("---") {
        return Ok(raw.to_string());
    }
    if !trimmed.starts_with("```") {
        return Err("ITEM_FM: sin frontmatter reconocible".into());
    }
    let after_ticks = trimmed
        .strip_prefix("```")
        .ok_or_else(|| "ITEM_FM: fence inválido".to_string())?;
    let yaml_start = after_ticks
        .strip_prefix("yaml")
        .or_else(|| after_ticks.strip_prefix("yml"))
        .ok_or_else(|| "ITEM_FM: fence sin yaml".to_string())?
        .trim_start_matches(['\r', '\n']);
    let close = yaml_start
        .find("```")
        .ok_or_else(|| "ITEM_FM: fence sin cierre".to_string())?;
    let fm_text = yaml_start[..close].trim();
    let body = yaml_start[close + 3..].trim_start_matches(['\r', '\n']);
    if fm_text.is_empty() {
        return Err("ITEM_FM: yaml vacío".into());
    }
    if body.is_empty() {
        Ok(format!("---\n{fm_text}\n---\n"))
    } else {
        Ok(format!("---\n{fm_text}\n---\n\n{body}"))
    }
}

fn split_frontmatter(raw: &str) -> Result<(String, String), String> {
    let mut lines = raw.lines();
    if lines.next() != Some("---") {
        return Err("ITEM_FM: sin frontmatter".into());
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
    Err("ITEM_FM: frontmatter sin cierre".into())
}

fn strip_legacy_status(fm: &mut YamlValue) {
    if let Some(map) = fm.as_mapping_mut() {
        map.remove("status");
    }
}

/// Fusiona `patch` en el frontmatter y elimina `status` legado (pending / refinado / refinada).
pub fn seal_markdown(raw: &str, patch: &YamlValue) -> Result<String, String> {
    super::tracker_item_document::seal_item_description(
        raw,
        patch,
        super::tracker_item_document::TrackerDescriptionDialect::FileMarkdown,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forge_lines_no_legacy_status() {
        let lines = forge_frontmatter_lines();
        assert!(lines.contains("tracker_column: backlog"));
        assert!(!lines.contains("status:"));
    }

    #[test]
    fn refine_strips_status_and_sets_column() {
        let raw = "---\ndocument_id: X\nstatus: pending\n---\n\n# body\n";
        let patch = yaml_patch_refinement_refined("2026-10-25");
        let out = seal_markdown(raw, &patch).unwrap();
        assert!(!out.contains("status: pending"));
        assert!(out.contains("tracker_column: todo"));
        assert!(out.contains("refinement: refined"));
    }

    #[test]
    fn column_patch_in_progress() {
        let raw = "---\ntracker_column: todo\nrefinement: refined\n---\n\n# x\n";
        let out = seal_markdown(raw, &yaml_patch_tracker_column(COLUMN_IN_PROGRESS)).unwrap();
        assert!(out.contains("tracker_column: in_progress"));
        assert!(out.contains("refinement: refined"));
    }

    #[test]
    fn seal_markdown_accepts_linear_yaml_fence() {
        let raw = "```yaml\ndocument_id: X\ntracker_column: backlog\nrefinement: pending\n```\n\n# cuerpo\n";
        let patch = yaml_patch_refinement_refined("2026-10-26");
        let out = seal_markdown(raw, &patch).unwrap();
        assert!(out.starts_with("---"));
        assert!(out.contains("tracker_column: todo"));
        assert!(out.contains("refinement: refined"));
        assert!(out.contains("# cuerpo"));
    }
}
