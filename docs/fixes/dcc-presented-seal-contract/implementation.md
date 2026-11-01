---
feature_name: dcc-presented-seal-contract
created: "2026-11-01"
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/dcc-presented-seal-contract
tracker_ref: AUT-48
execution_id: "9e85f016-c9be-46de-af20-92455e7817b8"
---

# Implementación — AUT-48

## T1 — Genoma

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md` | v1.6.1; fila `pr_url` del sello ECST sin «opcional en payload ECST v1.1»; frase `blocked` / `F-DCC-PR-URL-MISSING` bajo la tabla |
| `…/process/index.md` | Versión `1.6.1` en fila `delivery-close-cycle` |

Forja: dos invocaciones `entity-manager` (`markdown_body_replacements` + `process_version`/`process_phases`). Sin cambios en `phase_capsules.rs` ni `fracture-signatures.json`.

## T2 — Evolution

Nota `SddIA/evolution/20628bfd-f3f1-4dff-9409-4ef70e82c985.md` + fila en `Evolution_log.md` (`universe_total: 139`).
