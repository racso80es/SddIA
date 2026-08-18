---
entity_family: "templates"
paths_ref: "SddIA/core/cumulo.paths.json → directories.templates"
contract_ref: "SddIA/templates/templates-contract.md"
indexed_by: "agent:cumulo"
catalog_version: "1.1.0"
---

# Índice — Templates (Core SddIA)

Catálogo bajo `templates-contract`. Plantillas **motor** en `SddIA/templates/<template-id>/`; plantillas de **producto** en `.SddIA/templates/<template-id>/`.

| template_id | Naturaleza | Descripción |
|-------------|------------|-------------|
| process-splus-frontmatter | motor | Plantilla de proceso S+ (ver carpeta). |
| spec-template | motor | Plantilla `spec.md` con DIA (`impacts_doc` + § Impacto en Documentación). |
| systemd/sddia-email-watcher@.service | motor | Plantilla systemd del Centinela IMAP (`WorkingDirectory=%f`, marcador `@@SDDIA_CORE_ROOT@@`). |
