---


feature_name: man-49-remediacion-man-43
created: "2026-11-20"
updated: "2026-11-20"
process: feature
purpose: Plan ejecutable — remediación A-01..A-04 hasta APTO_TOTAL Argos
branch_name: feat/man-49-remediacion-man-43
persist_ref: docs/features/man-49-remediacion-man-43
pbi_ref: MAN-49
parent_ref: MAN-43
document_id: PBI-7e574af7
agent: dedalo
phase: diseno-blueprint
version_plan: "1.0.0"
status: blueprint-ready
audit_ref: docs/audits/man-43-post-merge-audit-20261117.md
norm_ref: features-documentation-pattern@1.3.1
execution_id: "4c3af8a6-1acd-48d3-8a21-10959beb08e8"
---
# Plan de ejecución — man-49-remediacion-man-43

Consumir `spec.md` + `clarify.md` en solo lectura. Orden obligatorio de forja (Mayeuta D6):
**R-01 → R-02 → R-04 (gates) → R-03 (docs/genoma) → verificación AC-5 → Argos**.
Git solo vía `skill:git-manager` / `./sddia-run.sh --tool git-manager`.

## Ola 0 — Preflight

| # | Acción | Criterio |
|---|--------|----------|
| 0.1 | Confirmar rama `feat/man-49-remediacion-man-43` vía git-manager | Branch existe / creable desde `main` |
| 0.2 | Inventario vivo: `rg`/`gate` baseline de `create_dir_all`, `concat!`, tokens `.events/` | Snapshot en `execution.md` |
| 0.3 | Listar residuos raíz (`orchestration/`, `telemetry/`, …) | Confirmar existencia (evidencia Dedalo: sí) |
| 0.4 | Contar `integration_src` en `eda-unit-backend.toml` | Baseline ≈34; meta = `[]` |

## Ola 1 — R-01 (P0) cero mkdir de bus

| # | Acción | Paths |
|---|--------|-------|
| 1.1 | No-op / retirar `ensure_fractal_dirs` + `ensure_bus_dirs` | `sddia-daemon-runtime/src/lib.rs` |
| 1.2 | No-op / retirar `ensure_event_bus_topology` create_dir_all | `eda_bus.rs`, `eda_bus_topology.rs` |
| 1.3 | Auditar call sites (`emit_raw_execution_finished`, `sweep_once_port`, `run_watcher`) | Sin `create_dir_all` de topología |
| 1.4 | Borrar residuos vacíos en raíz del piloto | `orchestration/`, `telemetry/`, (+ pending/domain/dead-letter/progress si hay) |
| 1.5 | Añadir regresión en `eda_event_token_hygiene` | Assert raíz limpia / no materialización |

**Gate ola:** tras `cargo test` acotado a higiene/daemon, raíz sin dirs listados → **AC-1** parcial.

## Ola 2 — R-02 watcher/sweep solo puerto

| # | Acción | Paths |
|---|--------|-------|
| 2.1 | `prune_routed_ok` → `get_event` / `list_pending` | `event-watcher/src/main.rs` |
| 2.2 | Sustituir `Path`/`is_file`/`read_to_string` por `EventUri` + puerto | mismas funciones fractal/skip/outcome |
| 2.3 | Eliminar helpers FS de `eda_sweep.rs` | `safe_remove_path`, `ensure_state_header`, `maybe_purge_processing_header`, fallback `move_fractal_to_dead_letter` |
| 2.4 | Ampliar `gate_event_manager_port` | Prohibir `fs::read_dir`, `fs::create_dir_all`, `fs::remove_file`, `fs::rename` en watcher + `eda_*.rs` |
| 2.5 | Ajustar tests del watcher/sweep al MemoryAdapter | Sin tempdir de bus en raíz |

**Gate ola:** `gate-event-manager-port` 0 violations FS → **AC-2**.

## Ola 3 — R-04 gates honestos (antes de docs)

| # | Acción | Paths |
|---|--------|-------|
| 3.1 | Quitar `concat!`; restaurar `".events/"` | `eda_event_token_hygiene.rs` |
| 3.2 | Añadir path a `path_allowlisted` | `gate_eda_unit_no_real_backend.rs` |
| 3.3 | Plan de vaciado `INTEGRATION_SRC` | Por cada entrada: migrar a `tests/` **o** MemoryAdapter; sync TOML + const Rust |
| 3.4 | Iterar hasta `integration_src = []` y gate verde | Si bloqueo real → `blocked` con evidencia (**L-TRUTH**) |

**Gate ola:** allowlist vacía + higiene allowlisteada → **AC-4**.

### Estrategia vaciado INTEGRATION_SRC (checklist)

1. Clasificar cada path: ¿usa Redb/FS real o solo MemoryAdapter?
2. Memory-only → quitar de allowlist sin mover fichero.
3. Real-backend → mover módulo test a `execute-process/tests/**` (ya omitido por `path_contains = "/tests/"`) o marcar crate integration.
4. Prefijos `handlers/kalma2`, `handlers/email_` → misma regla; no dejar substrings “comodín” si el gate deja de leer el array.
5. Correr `sddia-index-integrity --verify` tras cada lote.

## Ola 4 — R-03 docs + genoma

| # | Acción | Nota |
|---|--------|------|
| 4.1 | Actualizar `README.md` | `event_uri` / puerto |
| 4.2 | Actualizar `.antigravity/rules/tekton-fire-and-forget.md` | Sin `.events/` runtime |
| 4.3 | Escáner legacy tokens en `SddIA/**/*.md` | Lista viva de hits |
| 4.4 | `entity-manager` update por cada entidad con hit | **L-GENOME**; incluir sustitutos de emit-* ausentes |
| 4.5 | Ampliar `scan_genome_contract` a cuerpo completo + README | Oráculo AC-3 |
| 4.6 | Documentar N/A para `emit-*-event.md` ausentes sin tokens | D-GENOME-DELTA en `execution.md` |

**Gate ola:** higiene genoma/README limpia → **AC-3**.

## Ola 5 — Verificación global + handoff Argos

| # | Acción | Criterio |
|---|--------|----------|
| 5.1 | `cargo test --workspace -- --test-threads=1` | Verde |
| 5.2 | `cargo clippy --workspace -- -D warnings` | Verde |
| 5.3 | `sddia-index-integrity --verify` | Verde |
| 5.4 | Re-inspección raíz | Cero dirs topología |
| 5.5 | Materializar evidencia en `execution.md` / checks | Sin inventar éxito |
| 5.6 | `validacion.md` listo para sello **sin** `APTO_TOTAL` Tekton | Estado pre-Argos |
| 5.7 | Abrir/actualizar PR → `pull-request-review` (Argos) | Dictamen **`APTO_TOTAL`** solo Argos |
| 5.8 | Reauditoría parental MAN-43 (19/19) | Actualizar informe audit con evidencia |

**Gate ola:** **AC-5**.

## Matriz AC ↔ olas

| AC | Olas | Owner verificación |
|----|------|--------------------|
| AC-1 | 1, 5 | Tekton + higiene |
| AC-2 | 2, 5 | Gate event-manager-port |
| AC-3 | 4, 5 | Higiene + entity-manager |
| AC-4 | 3, 5 | Gate unit-backend |
| AC-5 | 5 | Suite + Argos |

## Stop conditions (blocked)

- Imposible vaciar `INTEGRATION_SRC` sin romper suite y sin ruta de migración clara → **blocked** R-04; no reintroducir `concat!` ni dirs raíz.
- `entity-manager` no disponible / falla mutación genoma → **blocked** R-03; no editar genoma a mano.
- Gates rojos post-fix → **NO_APTO**; registrar evidencia (**L-TRUTH**).

## Handoff Tekton

1. Leer `spec.md` §§3–5 y este plan; no reabrir R-*.
2. Ejecutar olas 0→5; materializar `execution.md` / `implementation.md` según norma features.
3. No escribir `APTO_TOTAL`; no tocar `docs/todos/`.
4. Al verde local, invocar cierre vía proceso delivery + Argos PPR.
