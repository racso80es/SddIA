---
feature_name: argos-pr-audited
process: feature
created: 2026-06-09T00:00:00Z
branch: feature/argos-domain-event-audited-8966834805803533351
global: APTO
checks:
  - id: F-DOC-1
    name: implementation.md presente
    status: APTO
  - id: F-DOC-2
    name: validacion.md convención APTO
    status: APTO
  - id: F-DOC-3
    name: branch coherente
    status: APTO
  - id: F-PBI-B
    name: ECST pull-request-audited + index
    status: APTO
  - id: F-PARIDAD
    name: argos.md frontmatter/cuerpo
    status: APTO
  - id: F-PBI-C
    name: emit-pr-audited-event integrado aduana
    status: APTO
  - id: F-PBI-A
    name: fósil TODO erradicado merge
    status: APTO
  - id: F-ENTROPIA
    name: mock-argos-output.json eliminado
    status: APTO
git_changes: true
pbi_archived: true
---

# Validación

- Contrato ECST `pull-request-audited`: APTO
- Códice `domain/index.md` actualizado: APTO
- Argos `outputs` alineados con §2: APTO
- Emisión `PullRequest_Audited` vía `emit-pr-audited-event`: APTO
- `audit_event_reference` real en `emit-pr-merged-event`: APTO
- Suscriptor `PullRequest_Audited` registrado: APTO
