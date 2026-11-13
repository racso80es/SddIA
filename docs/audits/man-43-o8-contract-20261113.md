---
id: man-43-o8-contract-20261113
uuid: "b2c4d6e8-f0a1-4b3c-9d5e-7f8a9b0c1d2e"
type: audit
version: 1.0.0
created: "2026-11-13"
tracker_ref: MAN-43
parent_audit: docs/audits/man-43-o6-remediation-20261113.md
baseline_commit: pending-O8
scope: "F-02 bindings/narrativa + procesos forjados"
verdict: NO_APTO_TOTAL
f02_bindings: closed
f02_schema_norm: open
---

# MAN-43 O8 — alineación contractual 2.0.0

## Dictamen

**F-02 (split bindings vs kernel):** cerrado en rama para `capability-bindings.md` y narrativa README/Constitución (`contract_version` / puerto `^2.0.0`). Permanece abierto el schema normativo `eda.event_manager.v1.schema.json` y fases YAML de `route-domain-event` que aún nombran `fs.persist` / `event_file_path` como JSON en disco.

El feature MAN-43 sigue **`NO_APTO_TOTAL`** (AC-13 abierto).

## Evidencia

| Comprobación | Resultado |
|--------------|-----------|
| `rg 'contract_version: \"1.2.0\"' SddIA/core/capability-bindings.md` | Sin coincidencias en fila `eda:event-manager` |
| `entity-manager` → `event-bus-audit` | `Domain_Entity_Updated` acusado (JSON CLI) |
| `entity-manager` → `route-domain-event` | `Domain_Entity_Updated` acusado (JSON CLI) |
| `cargo test -p sddia-core event_manager` | ver commit |

## Residual

- Frontmatter `route-domain-event` (`inputs`, fases `fs.persist`): requiere `process_phases` vía `entity-manager`, no parche de cuerpo.
- Norma `eda.event_manager.v1.schema.json`: ciclo `entity-manager` clase `norm`.
