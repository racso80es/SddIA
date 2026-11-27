---
feature_name: aut-133-remediacion-entidades-anemicas
created: "2026-11-27"
process: refactorization
branch_name: refactor/aut-133-remediacion-entidades-anemicas
persist_ref: docs/refactors/aut-133-remediacion-entidades-anemicas
tracker_ref: AUT-133
historia_ref: AUT-125
pbi_ref: PBI-81e91c67
execution_id: "5ea79c56-a610-4223-b2b3-7498a510d239"
verdict: blocked
---
# Implementation — AUT-133

## Touchpoints (estado este ciclo Tekton)

| Área | Path / canal | Estado |
| --- | --- | --- |
| Inventario KA-DOC-1 | `pre-remediation-inventory.json` | **congelado** (`violation_count: 322`, 142 entidades) |
| Oráculo Ola 0 | `oracle-red.log` | **presente** (JSON gate global `mode: all`, `exitCode: 1`) |
| Política UUID | `uuid-migration-register.md` | **prep** (25 filas `disposition: migrate`; sin rotación) |
| Genoma anémico | `SddIA/**/*.md` (inventario) | **sin mutar** — T4 no iniciado |
| Índices familia | `SddIA/*/index.md` | sin tocar |
| Cobertura EDA | `SddIA/core/eda-coverage.json` | sin tocar |
| CI peaje | `.github/workflows/sddia-index-qa.yml` | **sin promover** (sigue `--if-touched`) |
| Canal mutación | `process:entity-manager` (`lifecycle_operation: update`) | **no invocable** vía MCP `run_check` |
| Oráculo re-scan | `SddIA/target/debug/sddia-qa` | binario presente; Cerbero DENY en aduana MCP |
| Dependencias | AUT-131 (#481), AUT-132 (#482) | merged en tip (oráculo/forjas listos) |

## Decisiones aplicadas

1. **No remediación cosmética:** prohibido plantilla compartida / `remediate_bulk.py` (reincidencia REJECT #480).
2. **No edición directa del genoma:** DA-2 exige `entity-manager` → `*-creator` + `emit-domain-mutation`; `fs_write` MCP sobre entidades queda **fuera**.
3. **No rotación silenciosa UUID:** registro preparado; `uuid_new` vacío hasta T4 con sync índice + `eda-coverage`.
4. **No promoción CI prematura:** ratchet `--if-touched` mientras `violation_count > 0`.
5. **Inventario no vacío:** se conserva `violation_count: 322` + `oracle-red.log`; no se publica `"violations":[]` como evidencia KA-DOC-1.

## Bloqueo productivo (causa raíz)

Aduana MCP `run_check`: intersección Cerbero ∩ allowlist `shell-executor` **no** incluye `sddia-run.sh`, `execute-process`, `sddia-qa`, `cargo` ni rutas a binarios bajo `SddIA/target/debug/`. Probado en esta sesión: DENY/`not allowlisted` para esos ejecutables.

Sin `action:execute-process` → `process:entity-manager` **no** se puede remediaar ninguna de las 142 entidades (AC-2) ni aplicar migraciones UUID (AC-3). Por mandato DA-2 / plan T4, Tekton **no** muta genoma por filesystem.

## Desbloqueo requerido

1. Allowlist MCP alineada: al menos `sddia-run.sh` **o** `execute-process` **y** `sddia-qa` (o invocación nativa PPR fuera de `run_check`).
2. Reanudar T4 por olas (`process → action → tool → …`) con `semantic_seed` derivado de handler/cápsula por entidad.
3. Tras `violation_count: 0` sin plantilla: T5 promoción CI + T6 evolution + T7 Argos.
