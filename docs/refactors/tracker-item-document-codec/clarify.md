---
feature_name: tracker-item-document-codec
created: "2026-10-26"
process: refactorization
purpose: Laudos de parse/render en motor vs cápsula y relevo Mayeuta
execution_id: "ff7464c1-fe32-4067-8fb5-4bf1ccd55cb2"
document_id: PBI-e1d15344
tracker_ref: AUT-15
---

# Clarificación — tracker-item-document-codec

Relevo: Mayeuta bloqueada en runtime headless sin MCP `sddia-workspace-server` en el subproceso `cursor-agent`; cascada sellada en IDE (Tekton) tras `start-sddia.sh`.

## Decisiones

| ID | Laudo |
|----|-------|
| L-PARSE | El parseo del dialecto vive en el motor (`normalize_item_markdown` + `tracker_item_document`) en esta iteración; las operaciones `parse_item_document` / `render_item_document` en `tracker:lifecycle` quedan para v1.1 del contrato. |
| L-RENDER | Escritura remota con markdown `---`; Linear re-roundtrip a fence `yaml`; lectura siempre normaliza. |
| L-OBLIG | El handler valida claves de ciclo tras parseo; `provider_extras` no mezcla con `properties`. |
| L-STAMP | `seal_markdown` delega en `seal_item_description`; `tracker-stamp` hereda el codec sin cambio de firma pública. |
| L-FUERA | AUT-6 git binding; AUT-11 WASM token; pegado crudo de descripción en `tracker-markdown-apply` con label editable. |
