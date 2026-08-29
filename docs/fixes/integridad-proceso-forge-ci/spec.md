---
feature_name: integridad-proceso-forge-ci
created: "2026-08-29"
process: bug-fix
branch_name: fix/integridad-proceso-forge-ci
persist_ref: docs/fixes/integridad-proceso-forge-ci
pbi_ref: docs/todos/pending/[FIX] Integridad de proceso — parse_frontmatter ciego, hash forge divergente y aduana CI opaca.md
document_id: PBI-FIX-INTEGRIDAD-PROCESO-FORGE-CI
uuid: 5a049a19-29ae-4c3b-adb0-a8b4e8d042fb
scope: integridad-proceso-forge-ci
base: main
execution_id: "1dd48b02-251c-433a-85f8-bcfd7e93336e"
---

# Spec — Integridad de proceso (forge, CI, DCC)

## Problema

Cierre de `PBI-KAIZEN-CICLO-JURISDICCION-TODOS` (PR #225): el check GitHub `verify-tools-index` falló (`gh run 33256495000`). La causa no era el índice de tools: `hash_signature` de `delivery-close-cycle.md` no coincidía con `sha256_phases_integrity(phases)`. Dos defectos de forja encadenados lo produjeron; el job CI ocultó el step real; el DCC no tenía aduana local equivalente (DA-6).

## Defectos (mapa F → capa)

| ID | Hecho | Capa | Estado |
|----|-------|------|--------|
| F1 | `forges/common.rs::parse_frontmatter` (`strip_prefix` + `split_once("\n---")`) devolvía mapa vacío ante `workspace_template: …/---` | forge parser | Saldado `76be459` (delega en Core `split("---")`) |
| F2 | `run_process_forge` update + `markdown_body_replacements` sellaba `canonical_artifact_hash` (artefacto completo); aduana valida `sha256_phases_integrity` | process forge | Saldado `76be459` (`refresh_process_hash` post-replacements) |
| F3 | Job YAML `verify-tools-index` agrupa tools + process-integrity + evolution-register + compiled-capsules; el fallo se etiqueta con el nombre del job | `.github/workflows/sddia-index-qa.yml` | Abierto |
| F4 | DCC: aduanas evolution y EDA antes del push; **no** `verify-process-integrity` ni `verify-tools-index` | `delivery-close-cycle.md` + `delivery_close.rs` | Abierto |
| DT | `workspace_template` de DCC termina en `---` (colisión con delimitador YAML) | genoma process | Deuda; parser ya blinda |

## Solución

### L1 — Test CA1 (regresión F1)

Fixture `.md` con frontmatter que incluye `workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/---` (sin línea `---` extra antes del cierre). `forges::common::parse_frontmatter` debe leer `uuid` y `hash_signature`. Ubicación: tests en `SddIA/engine/execute-process/src/forges/common.rs` (módulo que delega). No reintroducir parser local.

### L2 — Test CA2 (regresión F2)

Fixture process mínimo (`phases` array). Invocar `run_process_forge` `lifecycle: update` con `markdown_body_replacements`. Tras forja: `hash_signature` del artefacto = `sha256_phases_integrity(phases)` del YAML parseado. Aserción negativa: el hash **no** es `canonical_artifact_hash` del fichero completo si ambos diferirían. Ubicación: tests de `forges/factory.rs` o `common.rs`.

### L3 — Nombre de job CI (CA3)

Renombrar el job `verify-tools-index` → **`sddia-index-integrity`**. Conservar un solo job (un toolchain, una cache). Los **steps** mantienen nombres propios (`verify-tools-index`, `verify-process-integrity`, `evolution-register unit tests`, `verify-compiled-capsules`).

**No** partir en jobs separados: duplicaría `rust-toolchain` + cache `SddIA/target`.

**Branch protection:** el required check GitHub hoy se llama `verify-tools-index`. El PR de ejecución debe documentar el rename; Racso actualiza la protección de `main` (añadir `sddia-index-integrity`, retirar el nombre viejo) **antes o al merge**. Workflow **no** es genoma Cúmulo; edición directa del YAML está permitida.

### L4 — Aduana local en DCC (CA4)

Nueva fase **después** de Aduana EDA y **antes** de Publicación remota:

| Campo | Valor |
|-------|--------|
| `name` | `Aduana integridad índices` |
| `delegates_to` | `agent:argos` |
| `intent` | Invocar `sddia-qa verify-process-integrity` y `verify-tools-index`; `block` si alguno falla |

Forja del `.md`: `./sddia-run.sh --process entity-manager` sobre `delivery-close-cycle` (domain root `codex-software-engineering/process/`). Prohibido Write directo. Recalc `hash_signature` vía forge (`refresh_process_hash`).

Handler nativo (paridad `capsule_evolution_audit_gate`):

- `SddIA/engine/execute-process/src/engine/phase_capsules.rs` — `capsule_index_integrity_audit_gate`
- Dispatch en `delivery_close.rs` **y** `residual_runner.rs` (mismo patrón que Aduana evolution)
- `status: blocked` si exit ≠ 0; **no** `fail_soft` (debe impedir el push)
- `friction_id` de fractura: `F-DCC-INDEX-INTEGRITY`
- Skip lab opt-in: `SDDIA_LAB_SKIP_INDEX_INTEGRITY` solo para tests unitarios del runner DCC que no deban spawn-ear `sddia-qa`; el camino productivo no lo usa

Smoke (ejecución, no este commit): corromper `hash_signature` de un process de fixture (o tmp) y confirmar que DCC aborta **antes** de `Publicación remota`.

### L5 — CA5 opcional (deuda)

`workspace_template` de `delivery-close-cycle.md`: quitar el `---` terminal (dejar `/` o `{execution_id}/`). Misma forja entity-manager que L4 si se ejecuta en el mismo ciclo. No es gate de Done.

## Fuera de alcance (este commit de Diseño)

- Mutación de genoma DCC, workflow, tests y handlers (Ejecución).
- Reabrir F1/F2 en código (ya en `main` vía `76be459`).
- PBI paralelo `[FIX] delivery-close-cycle — fractura sistémica`.

## Criterios (mapeo)

| CA | Capa | Este commit |
|----|------|-------------|
| CA1 test | L1 | Diseño |
| CA2 test | L2 | Diseño |
| CA3 | L3 | Diseño |
| CA4 | L4 | Diseño |
| CA5 | L5 | Opcional en Ejecución |
