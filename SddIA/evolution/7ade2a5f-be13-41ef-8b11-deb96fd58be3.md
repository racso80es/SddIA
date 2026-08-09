---
uuid: 7ade2a5f-be13-41ef-8b11-deb96fd58be3
date: "2026-08-09"
feature_name: sddia-domain-abstract-03-relocalizacion
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-03
type: architectural-milestone
scope: L-PACK-MULTIROOT-SIX-MOVE
parent_document_id: PBI-SDDIA-DOMAIN-ABSTRACT-02
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

`process-creator` sigue escribiendo bajo `directories.process` Core (D7 / L-KEEP-CORE).
