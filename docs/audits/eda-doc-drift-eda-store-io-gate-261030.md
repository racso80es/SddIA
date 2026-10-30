---
audit_id: DOC-DRIFT-EDA-STORE-IO-GATE-261030
date: "2026-10-30"
branch: Kaicen_Auditoria_Eventos_261030
scope: "Documentación desalineada respecto a commits 9d17ece8 … ed9137a8"
related:
  - docs/features/eda-store-io-gate/
  - docs/features/eda-store-io-gate/plan-doc-drift-261030.md
  - docs/audits/eda-events-redb-vs-filesystem-20261026.md
  - docs/audits/kaizen-auditoria-eventos-261030.md
---

# Auditoría — Deriva documental (gate I/O EDA)

## Veredicto

| Severidad | Cantidad | Acción |
|-----------|----------|--------|
| **Alta** | 4 | Corregido en esta auditoría (feature pack + nota supersión auditoría 20261026) |
| **Media** | 5 | Listado; genoma vía `entity-manager` o PR documental aparte |
| **Baja / histórica** | 3 | Mantener como evidencia; banner «snapshot» |

## Commits de referencia (rama)

| Commit | Alcance |
|--------|---------|
| `9d17ece8` | Kaizen `event-bus-audit` vía store |
| `189dc7dd` | `publish_ecst`, lectores, fractal claim, installer, `--publish-ecst` |
| `ed9137a8` | Matriz integration tests pending + fractal |

## Alta — corregido aquí

1. **`objectives-test-matrix.md`** — § Problema afirmaba cobertura «puntual» tras cerrar emisores; ya existe matriz en `eda_store_io_matrix_*`.
2. **`objectives.md`** — § Problema en presente sobre emisores/lectores residuales; la rama cierra el corte (queda TM-8 parcial y genoma).
3. **`plan.md` / `plan-test-matrix.md`** — Redacción de plan futuro (`Hoy write_json`, harness en `tests/support/eda_store_lab.rs`); implementación real en `tests/common/mod.rs`.
4. **`eda-events-redb-vs-filesystem-20261026.md`** — `status: FINDINGS_OPEN` y matriz §5 declara BUGs en `accept_pr`, `system_vitality` ya mitigados en `eda-redb-lock` + `eda-store-io-gate` (ver § Supersión abajo).

## Media — pendiente (no mutar genoma a mano)

| Artefacto | Deriva | Remedio sugerido |
|-----------|--------|------------------|
| `SddIA/actions/emit-pr-*-event.md` (presented, audited, resealed) | Paso 4 «Escribir JSON en pending» sin `SDDIA_EVENT_STORE` | Actualizar ficha acción: `write_pending_event` / gate (`filesystem` vs `redb`) |
| `SddIA/daemons/event-watcher.md` v1.2.0 | No menciona `claim_store_fractal` ni `F-EDA-RESIDUAL-FRACTAL` | Bump doc daemon: pending + fractal desde store en `redb` |
| `SddIA/events/index.md` | Runtime solo `eda_bus.pending` + rutas fractal FS | Añadir párrafo SSOT `events.redb` + path lógico (coherente con `events-contract.md`) |
| `docs/fixes/eda-redb-lock/spec.md` § D3 | Cita `write_json_atomic` en vitality como bypass activo | Nota histórica: sustituido por `write_fractal_event` / gate en CA4 |
| `docs/features/eda-store-io-gate/plan.md` § Fase 4 | Scripts `docs/` / QA sin marcar `filesystem` | Inventario manual o comentario en scripts (fuera de alcance código rama) |

## Baja — histórico válido

- PBIs en `docs/todos/done/` con default `filesystem` en HU madre: archivo de programa, no runtime.
- `docs/fixes/event-bus-audit-kaizen-redb-emit/plan.md` describe el «antes» a propósito; `validacion.md` ya APTO.
- `kaizen-auditoria-eventos-261030.md`: operaciones **locales** de instancia; no contradice la rama.

## Supersión parcial — auditoría 20261026

Tras merge efectivo de esta rama en `main`, la matriz §5 de `eda-events-redb-vs-filesystem-20261026.md` debe leerse así:

| Hallazgo 20261026 | Estado en rama `Kaicen_Auditoria_Eventos_261030` |
|-------------------|--------------------------------------------------|
| Lock sin backoff (A) | `eda-redb-lock` — backoff + `F-VITALIDAD` no fractura por lock transitorio |
| `accept_pr` solo FS (B) | `scan_presented_for_branch` consulta store si `writes_store` |
| `system_vitality` bypass (C) | Emisión vía `write_fractal_event` (tests `redb_ca4_*`) |
| Escritores heartbeat, kalma2, lancedb, iota, installer (no listados en tabla) | `189dc7dd` + matriz tests |
| Fractal watcher `read_dir` only (plan fase 3) | `claim_store_fractal` + `load_dispatched_event` en route fractal |

Quedan **abiertos** a nivel producto (no documentación de esta feature):

- Tabla autogenerada § Cobertura en `spec-test-matrix.md` (TM-8: asserts por suscriptor).
- Gaps PTC / `Raw_Execution_Finished` / `Process_Execution_Completed` en matriz (spec § gaps).
- Fichas de acción y codex que describen solo FS.

## Plan de ejecución

Ver **`docs/features/eda-store-io-gate/plan-doc-drift-261030.md`** (fases P0–P4, forja vs docs, DoD).

## Checklist post-merge

- [ ] Issue Linear `eda-store-io-gate` → `done` + `pbi_archived` en validación.
- [ ] Banner `SUPERSEDED` en auditoría 20261026 o nuevo addendum con fecha merge (P0).
- [x] Forja genoma acciones + `event-watcher.md` (P1) vía `entity-manager`.
- [x] Codex P2 (`accept-pr`, DCC, PPR, `refine-item`).
