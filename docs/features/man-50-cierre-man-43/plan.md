---

feature_name: man-50-cierre-man-43
created: "2026-11-20"
updated: "2026-11-20"
process: feature
purpose: Plan ejecutable — cierre B-01..B-03 residuales hasta APTO Argos
branch_name: feat/man-50-cierre-man-43
persist_ref: docs/features/man-50-cierre-man-43
pbi_ref: MAN-50
parent_ref: MAN-43
related_ref: MAN-49
document_id: PBI-91b7ec84
agent: dedalo
phase: diseno-blueprint
version_plan: "1.0.0"
status: blueprint-ready
audit_ref: docs/audits/man-49-man-43-post-merge-audit-20261120.md
norm_ref: features-documentation-pattern@1.3.1
execution_id: "ad89137d-966b-4076-8d75-e299e3f3b137"
---
# Plan de ejecución — man-50-cierre-man-43

Consumir `spec.md` + `clarify.md` en solo lectura. Orden obligatorio de forja (Mayeuta D6):
**R-01 → R-03a (gates) → R-02 (higiene binarios/raíz) → R-03b (docs/genoma) → verificación AC-5 → Argos**.
Git solo vía `skill:git-manager` / `./sddia-run.sh --tool git-manager`.

## Ola 0 — Preflight

| # | Acción | Criterio |
|---|--------|----------|
| 0.1 | Confirmar/crear rama `feat/man-50-cierre-man-43` vía git-manager | Branch desde `main` @ ≥ `ee17d953` |
| 0.2 | Baseline: callers de `write_json_atomic` + hits `".events"` en productivo | Snapshot en `execution.md` |
| 0.3 | Confirmar ternaria `integration_src` / `INTEGRATION_SRC` | Exactamente 3 paths (evidencia Dedalo) |
| 0.4 | Inventario hits `.events/` / `SDDIA_EVENT_STORE` en paths AC-4 | Lista viva para R-03b |
| 0.5 | Comprobar existencia `.events/` en raíz y mtimes de binarios target | Baseline B-02 |

## Ola 1 — R-01 (P1) DLQ solo puerto

| # | Acción | Paths |
|---|--------|-------|
| 1.1 | Introducir helper interno único (p. ej. `publish_dead_letter_via_port`) que llame `move_to_dlq` y/o `record_delivery(Nack)` según D-DLQ-API | Preferible en `eda_bus_topology` o `eda_port` bridge |
| 1.2 | Migrar los 6 `write_*_dead_letter` al helper | 6 módulos citados en `spec.md` §5 |
| 1.3 | Eliminar fallback `bus_defaults_from_root(".events")`; default sin literal | `eda_bus_topology.rs` `load_event_bus_topology` |
| 1.4 | Eliminar o acotar `write_json_atomic` (inventario callers) | Sin uso productivo DLQ |
| 1.5 | Ampliar higiene: veto `".events"` ± `/` | `eda_event_token_hygiene.rs` |
| 1.6 | Ajustar tests unitarios de los 6 módulos a MemoryAdapter / list_dlq | Sin tempdir `.events/` |

**Gate ola:** higiene sin `".events"` en productivo + tests acotados verdes → **AC-1** parcial.

## Ola 2 — R-03a allowlist vacía

| # | Acción | Paths |
|---|--------|-------|
| 2.1 | Identificar tests Redb en la ternaria | `route_domain_core`, `system_vitality`, `daemon_heartbeat` |
| 2.2 | Migrar a `execute-process/tests/*.rs` (+ `common`/fixtures) | Árbol `tests/` ya allowlisteado |
| 2.3 | Exponer helpers mínimos `pub(crate)` si hace falta | Sin API pública nueva |
| 2.4 | Vaciar `integration_src = []` y `INTEGRATION_SRC = &[]` | TOML + gate Rust |
| 2.5 | Correr `gate-eda-unit-no-real-backend` | 0 violaciones |

**Gate ola:** allowlist vacía + gate verde → **AC-3**.

### Checklist migración Redb

1. ¿El test abre store/FS real? → mover a `tests/`.
2. ¿Solo MemoryAdapter? → puede quedarse en unitario **fuera** de allowlist.
3. Tras mover, sync TOML y const Rust en el mismo commit lógico.
4. Si un helper privado bloquea la extracción → `pub(crate)` acotado; si imposible sin reabrir MAN-37 → **blocked** con evidencia (**L-TRUTH**).

## Ola 3 — R-02 binarios + raíz

| # | Acción | Nota |
|---|--------|------|
| 3.1 | `cargo build -p sddia-workspace-server -p event-watcher -p execute-process` | debug |
| 3.2 | `cargo build --release -p sddia-workspace-server -p event-watcher -p execute-process` | release |
| 3.3 | Añadir `".events"` a `assert_repo_root_bus_dirs_absent` | higiene |
| 3.4 | Purgar `.events/` de la raíz del piloto si existe | Tras rebuild |
| 3.5 | Documentar reinicio daemon `event-watcher` / MCP server | Higiene operativa en `execution.md` |
| 3.6 | Re-correr test raíz limpia | Confirmar AC-2 |

**Gate ola:** raíz sin `.events/` + assert verde → **AC-2**.

## Ola 4 — R-03b docs + genoma

| # | Acción | Nota |
|---|--------|------|
| 4.1 | Limpiar `.antigravity/rules/*.md` con hits | Edición permitida (no genoma códex) |
| 4.2 | `entity-manager` update `events-contract.md` | Sin `SDDIA_EVENT_STORE` / `.events/` runtime; **L-GENOME** |
| 4.3 | `entity-manager` update 4 procesos códex | accept-pr, delivery-close-cycle, pull-request-review, refine-item |
| 4.4 | Alinear nota `docs/audits/man-43-o6-remediation-20261113.md` | Consistencia APTO/NO_APTO |
| 4.5 | Escaneo evidencia cero tokens en universo AC-4 | Registrar en `execution.md` |

**Gate ola:** escaneo limpio → **AC-4**.

## Ola 5 — Verificación global + handoff Argos

| # | Acción | Criterio |
|---|--------|----------|
| 5.1 | `cargo test --workspace -- --test-threads=1` | Verde |
| 5.2 | `cargo clippy --workspace -- -D warnings` | Verde |
| 5.3 | `sddia-qa verify-process-integrity` + gates QA | Verde |
| 5.4 | Re-inspección: sin `.events/dead-letter/` nuevos post-smoke; raíz limpia | Evidencia |
| 5.5 | Materializar `execution.md` / `implementation.md` | Sin inventar éxito |
| 5.6 | `validacion.md` pre-sello **sin** `APTO` Tekton | Estado `PENDIENTE_ARGOS` |
| 5.7 | PR → `pull-request-review` (Argos) | Dictamen **`APTO`** solo Argos |

**Gate ola:** **AC-5**.

## Matriz AC ↔ olas

| AC | Olas | Owner verificación |
|----|------|--------------------|
| AC-1 | 1, 5 | Higiene + review writers |
| AC-2 | 3, 5 | Build + assert raíz |
| AC-3 | 2, 5 | Gate unit-backend |
| AC-4 | 4, 5 | Escaneo + entity-manager |
| AC-5 | 5 | Suite + Argos |

## Stop conditions (blocked)

- Imposible migrar ternaria Redb sin romper suite y sin ruta clara → **blocked** R-03a; no rellenar `INTEGRATION_SRC`.
- `entity-manager` no disponible / falla mutación genoma → **blocked** R-03b; no editar códex a mano.
- Writers DLQ sin acceso al puerto en call site → refactor mínimo de inyección; si requiere op nueva del puerto → **blocked** (**L-DOMAIN**).
- Gates/tests rojos post-fix → **NO_APTO**; registrar evidencia (**L-TRUTH**).

## Handoff Tekton

1. Leer `spec.md` §§3–5 y este plan; **no** reabrir R-01..R-03b.
2. Ejecutar olas 0→5; materializar `execution.md` / `implementation.md` según norma features.
3. No escribir `APTO`; no tocar `docs/todos/`.
4. Al verde local, invocar cierre vía delivery + Argos PPR.
5. Git solo `skill:git-manager`; artefactos feature solo MCP `sddia-workspace-server`.
