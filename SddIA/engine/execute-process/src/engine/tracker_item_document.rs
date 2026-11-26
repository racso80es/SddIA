//! Documento canónico de ítem en la descripción del tracker (AUT-15 / AUT-126).

use crate::engine::item_lifecycle_frontmatter;
use serde_json::{json, Value};
use serde_yaml::Value as YamlValue;

/// Dialecto de serialización hacia el proveedor activo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackerDescriptionDialect {
    /// Markdown con frontmatter `---` + un único H1 de título + cuerpo.
    FileMarkdown,
    /// Descripción remota Linear: frontmatter + cuerpo **sin** H1 de título (título nativo del issue).
    LinearDescription,
    /// Lectura habitual en Linear (` ```yaml `); escritura alineada con `LinearDescription`.
    LinearYamlFence,
}

/// Cuerpo lógico tras parseo: propiedades de ciclo + título + cuerpo + saco opaco del proveedor.
#[derive(Debug, Clone)]
pub struct TrackerItemDocument {
    pub properties: YamlValue,
    pub title: Option<String>,
    pub body: String,
    pub provider_extras: Value,
}

pub fn dialect_for_tracker_provider(provider: &str) -> TrackerDescriptionDialect {
    if provider.eq_ignore_ascii_case("linear") {
        TrackerDescriptionDialect::LinearDescription
    } else {
        TrackerDescriptionDialect::FileMarkdown
    }
}

fn collapse_internal_spaces(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn normalize_heading_text(s: &str) -> String {
    let t = s.trim();
    let unescaped = t.replace("\\[", "[").replace("\\]", "]");
    collapse_internal_spaces(&unescaped)
}

fn titles_match(a: &str, b: &str) -> bool {
    normalize_heading_text(a).eq_ignore_ascii_case(&normalize_heading_text(b))
}

/// Primera línea H1 del cuerpo (`# `, no `##`).
pub fn first_h1_heading(body: &str) -> Option<String> {
    for line in body.lines() {
        let t = line.trim();
        if t.starts_with("# ") && !t.starts_with("## ") {
            let heading = t.trim_start_matches('#').trim();
            if !heading.is_empty() {
                return Some(heading.to_string());
            }
        }
    }
    None
}

/// Elimina el H1 inicial si coincide con `title` (y líneas en blanco posteriores).
pub fn strip_leading_title_h1(body: &str, title: &str) -> String {
    let mut lines: Vec<&str> = body.lines().collect();
    while !lines.is_empty() && lines[0].trim().is_empty() {
        lines.remove(0);
    }
    if lines.is_empty() {
        return String::new();
    }
    let first = lines[0].trim();
    if first.starts_with("# ") && !first.starts_with("## ") {
        let h = first.trim_start_matches('#').trim();
        if titles_match(h, title) {
            lines.remove(0);
            while !lines.is_empty() && lines[0].trim().is_empty() {
                lines.remove(0);
            }
        }
    }
    lines.join("\n")
}

/// Colapsa H1 consecutivos idénticos al título (caso testigo AUT-125/126).
pub fn dedupe_consecutive_title_h1(body: &str, title: Option<&str>) -> String {
    let Some(title) = title else {
        return body.to_string();
    };
    let mut out: Vec<String> = Vec::new();
    let mut last_h1: Option<String> = None;
    for line in body.lines() {
        let t = line.trim();
        if t.starts_with("# ") && !t.starts_with("## ") {
            let h = t.trim_start_matches('#').trim();
            if titles_match(h, title) {
                if last_h1.as_ref().is_some_and(|prev| titles_match(prev, h)) {
                    continue;
                }
                last_h1 = Some(h.to_string());
            }
        }
        out.push(line.to_string());
    }
    out.join("\n")
}

/// Ensambla markdown de ítem con frontmatter ya serializado (sin `---` externos).
pub fn compose_item_markdown(
    frontmatter_inner: &str,
    title: &str,
    raw_body: &str,
    dialect: TrackerDescriptionDialect,
) -> String {
    let body = strip_leading_title_h1(raw_body, title);
    match dialect {
        TrackerDescriptionDialect::FileMarkdown => {
            if body.trim().is_empty() {
                format!("---\n{frontmatter_inner}\n---\n\n# {title}\n")
            } else {
                format!("---\n{frontmatter_inner}\n---\n\n# {title}\n\n{body}\n")
            }
        }
        TrackerDescriptionDialect::LinearDescription | TrackerDescriptionDialect::LinearYamlFence => {
            if body.trim().is_empty() {
                format!("---\n{frontmatter_inner}\n---\n")
            } else {
                format!("---\n{frontmatter_inner}\n---\n\n{body}\n")
            }
        }
    }
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

fn ingest_body_title(doc: &mut TrackerItemDocument) {
    if doc.title.is_none() {
        doc.title = first_h1_heading(&doc.body);
    }
    if let Some(ref t) = doc.title {
        doc.body = strip_leading_title_h1(&doc.body, t);
        doc.body = dedupe_consecutive_title_h1(&doc.body, Some(t));
    }
}

/// Representación canónica **local** (file): un único H1 de título si `native_title` está definido.
pub fn canonicalize_local_markdown(
    raw_description: &str,
    native_title: Option<&str>,
) -> Result<String, String> {
    let canonical = item_lifecycle_frontmatter::normalize_item_markdown(raw_description)?;
    let mut doc = parse_item_description(&canonical)?;
    if let Some(t) = native_title.filter(|s| !s.trim().is_empty()) {
        doc.title = Some(t.to_string());
        doc.body = strip_leading_title_h1(&doc.body, t);
        doc.body = dedupe_consecutive_title_h1(&doc.body, Some(t));
    } else {
        ingest_body_title(&mut doc);
    }
    render_item_description(&doc, TrackerDescriptionDialect::FileMarkdown)
}

/// `parse_item_document` del contrato lógico: acepta `---` o fence `yaml` de Linear.
pub fn parse_item_description(description: &str) -> Result<TrackerItemDocument, String> {
    let canonical = item_lifecycle_frontmatter::normalize_item_markdown(description)?;
    let (fm_text, body) = split_frontmatter(&canonical)?;
    let properties = serde_yaml::from_str(&fm_text).map_err(|e| e.to_string())?;
    let mut doc = TrackerItemDocument {
        properties,
        title: None,
        body,
        provider_extras: json!({}),
    };
    ingest_body_title(&mut doc);
    Ok(doc)
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

/// Sella propiedades y proyecta `refined_body` según dialecto (refine-item / AUT-126).
pub fn seal_with_refined_body(
    canonical: &str,
    patch: &YamlValue,
    refined_body: &str,
    native_title: Option<&str>,
    dialect: TrackerDescriptionDialect,
) -> Result<String, String> {
    let fm_sealed = seal_item_description(
        canonical,
        patch,
        TrackerDescriptionDialect::FileMarkdown,
    )?;
    let mut doc = parse_item_description(&fm_sealed)?;
    doc.body = refined_body.trim().to_string();
    if let Some(t) = native_title.filter(|s| !s.is_empty()) {
        doc.title = Some(t.to_string());
    } else {
        ingest_body_title(&mut doc);
    }
    if let Some(ref t) = doc.title {
        doc.body = strip_leading_title_h1(&doc.body, t);
        doc.body = dedupe_consecutive_title_h1(&doc.body, Some(t));
    }
    render_item_description(&doc, dialect)
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
    let new_fm = serde_yaml::to_string(&doc.properties).map_err(|e| e.to_string())?;
    let title = doc
        .title
        .clone()
        .or_else(|| first_h1_heading(&doc.body))
        .unwrap_or_default();
    let body = if title.is_empty() {
        doc.body.trim_start_matches('\n').to_string()
    } else {
        strip_leading_title_h1(&doc.body, &title)
    };
    match dialect {
        TrackerDescriptionDialect::FileMarkdown => {
            if title.is_empty() {
                if body.is_empty() {
                    Ok(format!("---\n{new_fm}---\n"))
                } else {
                    Ok(format!("---\n{new_fm}---\n\n{body}"))
                }
            } else if body.is_empty() {
                Ok(format!("---\n{new_fm}---\n\n# {title}\n"))
            } else {
                Ok(format!("---\n{new_fm}---\n\n# {title}\n\n{body}"))
            }
        }
        TrackerDescriptionDialect::LinearDescription | TrackerDescriptionDialect::LinearYamlFence => {
            if body.is_empty() {
                Ok(format!("---\n{new_fm}---\n"))
            } else {
                Ok(format!("---\n{new_fm}---\n\n{body}"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::item_lifecycle_frontmatter::yaml_patch_refinement_refined;

    #[test]
    fn parse_linear_fence_and_seal_refined() {
        let raw = "```yaml\ndocument_id: X\ntracker_column: backlog\nrefinement: pending\n```\n\n# Título\n\nPárrafo.\n";
        let patch = yaml_patch_refinement_refined("2026-10-26");
        let out =
            seal_item_description(raw, &patch, TrackerDescriptionDialect::LinearYamlFence).unwrap();
        assert!(out.starts_with("---"));
        assert!(out.contains("tracker_column: todo"));
        assert!(out.contains("refinement: refined"));
        assert!(!out.contains("# Título"));
        assert!(out.contains("Párrafo."));
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

    #[test]
    fn compose_forge_ac1_no_duplicate_h1_linear() {
        let raw = "# Título\n\nCuerpo.";
        let title = "Título";
        let fm = "document_id: X";
        let linear = compose_item_markdown(fm, title, raw, TrackerDescriptionDialect::LinearDescription);
        assert!(!linear.contains("# Título"));
        assert!(linear.contains("Cuerpo."));
        let file = compose_item_markdown(fm, title, raw, TrackerDescriptionDialect::FileMarkdown);
        assert_eq!(file.matches("# Título").count(), 1);
    }

    #[test]
    fn roundtrip_linear_idempotent_with_title() {
        let raw = "---\ndocument_id: Z\n---\n\n## sección\n\nTexto.\n";
        let mut doc = parse_item_description(raw).unwrap();
        doc.title = Some("Nativo".into());
        let linear = render_item_description(&doc, TrackerDescriptionDialect::LinearDescription).unwrap();
        assert!(!linear.contains("# Nativo"));
        let again = parse_item_description(&linear).unwrap();
        let linear2 =
            render_item_description(&again, TrackerDescriptionDialect::LinearDescription).unwrap();
        assert_eq!(linear.trim(), linear2.trim());
    }

    #[test]
    fn canonicalize_strips_duplicate_h1() {
        let raw = "---\ndocument_id: A\n---\n\n# Foo\n\n# Foo\n\n## sec\n";
        let out = canonicalize_local_markdown(raw, Some("Foo")).unwrap();
        assert_eq!(out.matches("# Foo").count(), 1);
    }

    #[test]
    fn dedupe_consecutive_title_h1_collapses_dupes() {
        let body = "# Foo\n\n# Foo\n\n## x\n";
        let got = super::dedupe_consecutive_title_h1(body, Some("Foo"));
        assert_eq!(got.matches("# Foo").count(), 1);
    }

    #[test]
    fn aut125_witness_duplicate_h1_canonicalized() {
        let title = "[KAIZEN] Auditoría pre-merge dotenv shell";
        let raw = format!(
            "---\ntracker_ref: AUT-125\n---\n\n# {title}\n\n# {title}\n\n## Impacto\n"
        );
        let out = canonicalize_local_markdown(&raw, Some(title)).unwrap();
        assert_eq!(out.matches(&format!("# {title}")).count(), 1);
        let linear = parse_item_description(&out).unwrap();
        let rendered =
            render_item_description(&linear, TrackerDescriptionDialect::LinearDescription).unwrap();
        assert!(!rendered.contains(&format!("# {title}")));
    }
}