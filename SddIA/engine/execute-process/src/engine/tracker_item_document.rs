//! Documento canónico de ítem en la descripción del tracker (AUT-15 / kaizen).

use crate::engine::item_lifecycle_frontmatter;
use serde_json::{json, Value};
use serde_yaml::Value as YamlValue;

/// Dialecto de serialización hacia el proveedor activo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackerDescriptionDialect {
    /// Markdown con frontmatter `---` (file-tracker-adapter y escritura compatible con Linear).
    FileMarkdown,
    /// Lectura habitual en Linear (` ```yaml `); la escritura usa `FileMarkdown` hasta operación dedicada.
    LinearYamlFence,
}

/// Cuerpo lógico tras parseo: propiedades de ciclo + cuerpo + saco opaco del proveedor.
#[derive(Debug, Clone)]
pub struct TrackerItemDocument {
    pub properties: YamlValue,
    pub body: String,
    pub provider_extras: Value,
}

fn split_frontmatter(raw: &str) -> Result<(String, String), String> {
    let mut lines = raw.lines();
    if lines.next() != Some("---") {
        return Err("TRACKER_DOC: sin frontmatter".into());
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
    Err("TRACKER_DOC: frontmatter sin cierre".into())
}

fn strip_legacy_status(fm: &mut YamlValue) {
    if let Some(map) = fm.as_mapping_mut() {
        map.remove("status");
    }
}

/// `parse_item_document` del contrato lógico: acepta `---` o fence `yaml` de Linear.
pub fn parse_item_description(description: &str) -> Result<TrackerItemDocument, String> {
    let canonical = item_lifecycle_frontmatter::normalize_item_markdown(description)?;
    let (fm_text, body) = split_frontmatter(&canonical)?;
    let properties = serde_yaml::from_str(&fm_text).map_err(|e| e.to_string())?;
    Ok(TrackerItemDocument {
        properties,
        body,
        provider_extras: json!({}),
    })
}

fn merge_patch(properties: &mut YamlValue, patch: &YamlValue) {
    strip_legacy_status(properties);
    if let Some(map) = patch.as_mapping() {
        for (k, v) in map {
            if let Some(key) = k.as_str() {
                properties[key] = v.clone();
            }
        }
    }
}

/// Fusiona `patch` en propiedades y devuelve markdown nativo para persistir.
pub fn seal_item_description(
    description: &str,
    patch: &YamlValue,
    dialect: TrackerDescriptionDialect,
) -> Result<String, String> {
    let mut doc = parse_item_description(description)?;
    merge_patch(&mut doc.properties, patch);
    render_item_description(&doc, dialect)
}

/// `render_item_document` del contrato lógico.
pub fn render_item_description(
    doc: &TrackerItemDocument,
    dialect: TrackerDescriptionDialect,
) -> Result<String, String> {
    let _ = dialect;
    let new_fm = serde_yaml::to_string(&doc.properties).map_err(|e| e.to_string())?;
    let body = doc.body.trim_start_matches('\n');
    if body.is_empty() {
        Ok(format!("---\n{new_fm}---\n"))
    } else {
        Ok(format!("---\n{new_fm}---\n\n{body}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::item_lifecycle_frontmatter::yaml_patch_refinement_refined;

    #[test]
    fn parse_linear_fence_and_seal_refined() {
        let raw = "```yaml\ndocument_id: X\ntracker_column: backlog\nrefinement: pending\n```\n\n# cuerpo\n";
        let patch = yaml_patch_refinement_refined("2026-10-26");
        let out =
            seal_item_description(raw, &patch, TrackerDescriptionDialect::LinearYamlFence).unwrap();
        assert!(out.starts_with("---"));
        assert!(out.contains("tracker_column: todo"));
        assert!(out.contains("refinement: refined"));
        assert!(out.contains("# cuerpo"));
    }

    #[test]
    fn roundtrip_file_markdown() {
        let raw = "---\ndocument_id: Y\ntracker_column: todo\nrefinement: refined\n---\n\nbody\n";
        let doc = parse_item_description(raw).unwrap();
        let again = render_item_description(&doc, TrackerDescriptionDialect::FileMarkdown).unwrap();
        let doc2 = parse_item_description(&again).unwrap();
        assert_eq!(
            doc.properties.get("document_id"),
            doc2.properties.get("document_id")
        );
    }
}
