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
phases:
  - T1-rama
  - T2-ola0-inventario
  - T3-uuid-politica
  - T4-remediacion-olas
  - T5-promocion-ci
  - T6-evolution
  - T7-validacion
  - T8-cierre
---
# Plan — AUT-133

Blueprint Tekton/Argos para liquidar la línea base anémica hasta `violation_count: 0` sin plantilla. Git solo `skill:git-manager`. Mutaciones de genoma solo `action:execute-process` → `process:entity-manager`. Sin KM/`docs/todos/` desde Tekton/Argos.

## Fases

1. **T1 — Rama**
   - `delegates_to`: `skill:git-manager`
   - `intent`: Crear/actualizar `refactor/aut-133-remediacion-entidades-anemicas` desde tip de `main` post-laudo AUT-125; worktree limpio; no tocar AUT-131/132 salvo coordinación de merge.

2. **T2 — Ola 0 inventario (AC-1 / KA-DOC-1)**
   - `delegates_to`: `action:execute-process` (vía build local `sddia-qa` ya compilado en CI/dev), `skill:filesystem-manager` (solo materialización bajo `persist_ref` vía aduana autorizada)
   - `intent`: Ejecutar `sddia-qa gate-entity-no-anemia --json` (global) sobre tip de `main`; capturar `oracle-red.log` + materializar `pre-remediation-inventory.json` con `genome_tip_sha` y `violation_count > 0`. Comparar con semilla `docs/refactors/gate-anti-anemia-entidades-dominio/audit-inventory.json` (322). **Stop** si el inventario sale vacío sin justificación (falso inventario). Congelar inventario; no mutar genoma en esta fase.

3. **T3 — Política UUID (AC-3 prep)**
   - `delegates_to`: `skill:filesystem-manager` (artefacto bajo `persist_ref`)
   - `intent`: Crear `uuid-migration-register.md` listando todos los paths con `UUID_NOT_V4` / `UUID_INVALID` del inventario; preasignar `disposition` (`migrate` por defecto; `exception` solo con causa formal). No rotar UUIDs aún.

4. **T4 — Remediación por olas (AC-2 + AC-3)**
   - `delegates_to`: `action:execute-process` → `process:entity-manager` (update por entidad); sync de índices vía forjas del creator; `skill:git-manager` para commits atómicos por lote
   - `intent`: Remediación gobernada en orden **process → action → tool → skill → daemon → norm → event → suite → codex → agent**. Por cada entidad:
     1. Derivar cuerpo real (handler/cápsula/contrato) → `semantic_seed` único.
     2. `entity-manager` `lifecycle_operation: update` (DA-2).
     3. Si hay fila UUID: aplicar `migrate` (uuid new + índice familia + `eda-coverage`) o registrar `exception`.
     4. Tras cada lote (≤10): re-scan gate (global o paths tocados) — 0 plantilla duplicada; no regresión de entidades ya verdes.
   - **Gates de fricción:** abortar lote si aparece plantilla compartida, vaciado de I/O, downgrade SemVer, o UUID sin sync. Coordinar con AUT-131 si forja destruye metadatos; con AUT-132 si el oráculo es falso.

5. **T5 — Promoción CI (AC-4)**
   - `delegates_to`: `skill:filesystem-manager` (patch workflow), evidencia gate
   - `intent`: Solo cuando `sddia-qa gate-entity-no-anemia --json` → `violation_count: 0` y AC-3 cerrado: materializar `post-remediation-gate.json` y cambiar `.github/workflows/sddia-index-qa.yml` (job `wasi-runtime-smoke`) de `--if-touched --base origin/main` a gate global `--json`.

6. **T6 — Evolución (AC-5)**
   - `delegates_to`: alta evolution autorizada (fuera de EM) + `skill:filesystem-manager` para `Evolution_log.md`
   - `intent`: Traza `relacionado: AUT-133` (hija o extensión cadena AUT-125/`a8f3c2e1-…`); sin inventar hash.

7. **T7 — Verificación Argos**
   - `delegates_to`: `action:execute-process` / `sddia-qa`
   - `intent`:
     - `sddia-qa gate-entity-no-anemia --json` → `violation_count: 0`
     - `sddia-qa audit-eda-coverage --scan --json` → `orphan_count: 0` (o warn backfill documentado)
     - Simetría muestra: UUIDs migrados presentes en `index.md` correspondiente
     - Ausencia de `remediate_bulk.py` / scripts ad-hoc nuevos bajo `docs/`
     - Rellenar `validacion.md` (checks ↔ AC; CI `PENDIENTE-CI` hasta run verde del workflow promovido)

8. **T8 — Cierre**
   - `delegates_to`: `skill:git-manager`, procesos de delivery/PR del códice vigente
   - `intent`: PR único `delivery_mode: branch_pr` → `pull-request-review` → merge. No declarar APTO global sin run CI del peaje global.

## Baseline hermético Ola 0 (`execution.md`)

Antes de T4 registrar: `git status --porcelain` vacío (vía git-manager); tip SHA del inventario; conteo `by_code` / `by_family`; dependencia AUT-132/AUT-131 (merged | pendiente | riesgo).

## Dependencias / exclusiones

| Hermano | Relación |
| --- | --- |
| AUT-125 | Padre; semilla inventario 322; ratchet `--if-touched` hasta T5 |
| AUT-132 | Oráculo peaje; preferible verificado antes de T4 masivo |
| AUT-131 | Forjas/EM update seguro; no solapar superficies de peaje |

## Prohibiciones operativas

- Publicar inventario post-hoc vacío como evidencia KA-DOC-1.
- Plantilla compartida / `remediate_bulk.py`.
- Rotación silenciosa de `uuid`.
- Promover CI a global con `violation_count > 0`.
- Escribir TODOs bajo `docs/todos/` desde Tekton/Argos.
