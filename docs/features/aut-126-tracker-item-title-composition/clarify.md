---

feature_name: aut-126-tracker-item-title-composition
created: "2026-11-26"
process: feature
tracker_ref: AUT-126
execution_id: "2cddf289-b44b-4855-8339-fee4fb4397d6"
---
# Clarificación — AUT-126

| ID | Decisión |
|----|----------|
| **D-MODEL** | Modelo lógico `{ properties, title, body }` en `tracker_item_document`; título explícito en parse/render. |
| **D-LINEAR** | `LinearDescription`: `description` sin H1 de título; título solo en campo nativo del issue. |
| **D-FILE** | `FileMarkdown`: frontmatter + un único `# {title}` + cuerpo sin H1 duplicado. |
| **D-FORGE** | `compose_item_markdown` sustituye concatenación manual en `forge-pbi` / `forge-hu`. |
| **D-REFINE** | `FetchSnapshot.native_title` + `seal_with_refined_body` proyectan sellado remoto sin H1. |
| **D-125** | Saneamiento AUT-125 post-merge vía `update_issue_description` con el codec nuevo (proceso autorizado). |