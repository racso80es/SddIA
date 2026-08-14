---
contrato_version: "1.1.1"
id_cambio: "7ade2a5f-be13-41ef-8b11-deb96fd58be3"
fecha: 2026-08-09
tipo_operacion: modificacion
descripcion_breve: "Evolution — ABSTRACT-03 relocalización física process software"
hash_integrity: "sha256:841b00b4e7f30be867aafdb1f7a35ab068de66391ffe252c7c64cce65299258f"
relacionado:
  - SddIA/evolution
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-03
---

# Evolution — ABSTRACT-03 relocalización física process software

## Hito

Cierre AC-MOVE diferido de ABSTRACT-02: resolución multi-root (`directories.process_domain_roots`) + move físico de los 6 process software-lifecycle/PR cycle al packing del códice `codex-software-engineering`.

## Destino canónico

`SddIA/library/codexes/codex-software-engineering/process/{name}.md`

Miembros movidos: `feature`, `bug-fix`, `refactorization`, `pull-request-review`, `accept-pr`, `delivery-close-cycle`.

## Motor

- Cúmulo `1.6.0` + `process_domain_roots`
- `resolve_process_path` domain-first (Core fallback)
- Overlay instancia: `.SddIA/local.paths.json`

## Deuda conocida

D7 (`process-creator` escritura mono-root Core) → liquidada en feature `process-creator-process-domain-roots` / evolution `a3c7e91f-2b4d-4f8a-9c1e-7d6b0a5f3211` (L-JURIS-MEMBERSHIP-PLUS-FLAG). Creator permanece Core (L-KEEP-CORE); destino de alta es multi-root.
