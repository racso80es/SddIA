---
audit_id: man-43-o14b-event-token-20261114
feature: man-43-event-manager-desacople-total
ola: O14b
date: "2026-11-14"
veredicto: APTO_PARCIAL
branch: docs/man-43-post-merge-handoff
---

# Auditoría O14b — tokens de evento (`event_uri`)

## Alcance

Sustitución de `target_path` / `event_file_path` / `orchestration_event_path` por `event_uri` (y plurales) en motor Rust y genoma (vía `entity-manager`). Lista blanca E-2 conservada para rutas de fichero/PBI.

## Evidencia

| Comando | Resultado |
|---------|-----------|
| `cargo test -p execute-process --lib eda_event_token_hygiene` | 5/0 |
| `cargo test -p linear-tracker-adapter` | 14/0 (AUT-68 corregido) |
| `cargo test -p execute-process --lib` | 854/0 (tras restaurar `process_context` en tracker-stamp/sync-replay y fix-tool-process) |
| Oráculo `eda_event_token_hygiene` | Genoma `process/actions/events`, motor (whitelist E-2), puerto `event_manager` |

## Residual

- `cargo test --workspace` y `cargo clippy --workspace`: pendiente de ejecutar en cierre de commit.
- `gate-evolution --range`: pendiente tras registro evolution O14b.
- `memory-evolution-ingest.md`: delimitador `---` del frontmatter reparado (corrupción previa en HEAD); forja posterior OK.
- Procesos forjados sin `process_context` heredado: corregidos (`tracker-stamp`, `tracker-sync-replay`, `fix-tool-process`).

## Lista blanca E-2 (genoma/motor)

`materialize-fracture-pbi`, `materialize-kaizen-alert-doc`, `enrich-fracture-pbi-kaizen`, `fracture-pbi-materialized`, `filesystem-manager`, norma congelada, `markdown-table-editor`, `event-bus-audit` (`kaizen_target_path`), `fracture_pbi.rs` + handlers materialize en motor.

## Veredicto

**APTO_PARCIAL** — núcleo AC-3/ORACULO verde en `execute-process` y genoma contractual; falta matriz workspace completa + evolution gate en el mismo PR que O14c no bloquea.