---
document_id: PBI-KAIZEN-LANCEDB-CICLO-FRICCIONES
uuid: "12250eca-49c6-4008-ac50-5c5722a7fe91"
title: "[KAIZEN] Post-ciclo LanceDB — PAT/workflow, Kintsugi saltado, Mayeuta ciego y correlato evolution"
format: markdown
version: "1.0.0"
created: "2026-08-31"
updated: "2026-08-31"
status: pending
priority: alta
process: feature
type: kaizen
dispatch: false
suggested_branch: feat/kaizen-lancedb-ciclo-fricciones
persist_ref_suggested: docs/features/kaizen-lancedb-ciclo-fricciones
depends_on: []
derived_from:
  - PBI-CORE-LANCEDB-REAL-001
friction_ids:
  - F-DCC-GIT-PAT-NO-WORKFLOW
  - F-TEKTON-KINTSUGI-SALTO-DCC
  - F-MAYEUTA-FRACTURE-HOOK-FALSE-POSITIVE
  - F-EVOLUTION-RELACIONADO-LOCKFILE
  - F-PROCESS-INGEST-JSON-STALE
  - F-VALIDACION-CA13-SIN-LOG-CI
tech_debt_ids:
  - DT-GIT-MANAGER-CREDENTIAL-SPLIT
  - DT-LANCEDB-COMPILE-TAX
related_pbis:
  - id: PBI-CORE-LANCEDB-REAL-001
    rol: "Ciclo de origen (PR #241). Este Kaizen recoge fricción de forja/aduana/obediencia; no reabre LanceDB físico ni embeddings."
  - id: PBI-FIX-FRACTURE-01c9040df256
    rol: "Fractura DCC del mismo ciclo. Diagnóstico Mayeuta (recursión hook / SDDIA_HOOK_DELIVERY_CLOSE) es FALSO. Este PBI corrige causa raíz; no ejecutar la propuesta del FIX auto-generado."
source_audit: "Auditoría post-implementación feat/lancedb-real-vector-memory (PR #241, execution_id c4e7971d-7c67-4745-b1b4-eb8b3d84d652). Evidencia: acuse DCC execution_id 391073eb (Publicación remota rejected workflow scope; Apertura en forja GraphQL Head sha blank); PBI untracked 01c9040df256; gh auth status (scopes incluyen workflow) vs git HTTPS PAT; gate-evolution EVOL_MATERIAL_UNREGISTERED en Cargo.lock + execute-process/Cargo.toml + adapters/*.md; memory-evolution-ingest.md aún declara JSON; validacion.md LDB-CA13 APTO sin log Actions."
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/engine/execute-process/src/engine/phase_capsules.rs
  - SddIA/skills/git-manager.md
  - SddIA/skills/sddia-evolution-register/src/lib.rs
  - SddIA/process/memory-evolution-ingest.md
  - SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs
  - .github/workflows/sddia-index-qa.yml
  - docs/features/lancedb-real-vector-memory/
  - docs/fixes/integridad-proceso-forge-ci/execution.md
  - docs/todos/pending/[FIX] delivery-close-cycle — fractura sistémica (01c9040df256).md
---

# [KAIZEN] Post-ciclo LanceDB — fricción de forja

Auditoría del transcurso de `PBI-CORE-LANCEDB-REAL-001` (rama `feat/lancedb-real-vector-memory`, PR [#241](https://github.com/racso80es/SddIA/pull/241)). Las fricciones son **de ciclo**, no del driver LanceDB. El hexágono y los tests de reapertura **no** se reabren.

**Prohibido** implementar la propuesta Mayeuta de `PBI-FIX-FRACTURE-01c9040df256` (`SDDIA_HOOK_DELIVERY_CLOSE` / skip hooks). Esa fractura es síntoma de push fallido, no recursión hook. Recurrencia de `F-MAYEUTA-PREPUSH-EVOL-COLLISION`.

## 0. Trazabilidad fricción → criterio

| `friction_id` | Sección | Criterio | Deuda |
|---------------|---------|----------|--------|
| `F-DCC-GIT-PAT-NO-WORKFLOW` | §1 | CA1, CA2 | `DT-GIT-MANAGER-CREDENTIAL-SPLIT` |
| `F-TEKTON-KINTSUGI-SALTO-DCC` | §2 | CA3 | — |
| `F-MAYEUTA-FRACTURE-HOOK-FALSE-POSITIVE` | §3 | CA4, CA5 | — |
| `F-EVOLUTION-RELACIONADO-LOCKFILE` | §4 | CA6 | — |
| `F-PROCESS-INGEST-JSON-STALE` | §5 | CA7 | — |
| `F-VALIDACION-CA13-SIN-LOG-CI` | §6 | CA8 | `DT-LANCEDB-COMPILE-TAX` (fuera: no optimizar Arrow en este PBI) |

## 1. `F-DCC-GIT-PAT-NO-WORKFLOW` — git-manager y `gh` no comparten credencial

DCC `execution_id` `391073eb-6148-4ff4-8728-02533d787bbd`:

```text
Publicación remota failed:
refusing to allow a Personal Access Token to create or update workflow
`.github/workflows/sddia-index-qa.yml` without `workflow` scope
```

`gh auth status` del mismo host: scopes `gist`, `read:org`, `repo`, **`workflow`**. El `git push` de `skill:git-manager` (origin HTTPS) usa **otro** PAT, sin `workflow`.

Precedente idéntico: `docs/fixes/integridad-proceso-forge-ci/execution.md` (DCC intento 1, PR #226). No se cerró la dualidad; #241 la reprodujo al tocar CI (`protobuf-compiler` + tests LanceDB).

**Dentro:** alinear el canal de push de DCC/`git-manager` con una credencial que tenga `workflow` **o** detectar el rechazo GitHub y fallar con envelope accionable (`F-DCC-WORKFLOW-SCOPE`) **sin** `System_Fracture_Detected` de recursión hook, más instrucción operador (`gh auth refresh -s workflow` / unificar credential helper).

**Fuera:** reimplementar `SDDIA_HOOK_DELIVERY_CLOSE`. Mutar políticas GitHub de la org sin laudo.

## 2. `F-TEKTON-KINTSUGI-SALTO-DCC` — entrega continuada tras colapso

Tras el `failed` de DCC, Tekton ejecutó `git push` con token `gh` y re-invocó DCC (`c9a85460-…` → PR #241). `obediencia-procesos.md` § Kintsugi paso 6 y Ley de Jurisdicción Delegada: **prohibido** `git`/`gh` raw para evadir fallo de proceso oficial.

Recurrencia normativa ya sellada como `F-TEKTON-BYPASS-RAW-POST-COLAPSO` (Kaizen jurisdicción todos, CA7). Aquí el sello **no impidió** el salto.

**Dentro:** envelope DCC de `Publicación remota` failed por scope `workflow` debe ser `blocked` accionable (no fractura hook); Tekton/lab-relay: barrera explícita que impida re-push raw. Documentar en `external-ai-constraints` / `obediencia-procesos` el caso `workflow` scope como colapso de **credencial**, no de Core.

**Fuera:** castrar DCC. Perdonar el salto de #241 (histórico).

## 3. `F-MAYEUTA-FRACTURE-HOOK-FALSE-POSITIVE` — PBI `01c9040df256`

Cúmulo materializó fractura. Mayeuta: «recursión hook» + refactor `SDDIA_HOOK_DELIVERY_CLOSE`.

Traza real del PBI = `Apertura en forja` *después* de push rechazado (`Head sha can't be blank`, rama inexistente en origin). Causa: F1, no hooks.

**Dentro:** clasificador `enrich-fracture-pbi-kaizen` / `analyze_fracture_kaizen`: si la traza contiene `without workflow scope` **o** `Head sha can't be blank` consecuente a push rejected → cubo `credential_workflow_scope` / `remote_branch_absent`, **no** cubo hook. Tests unitarios con fixtures de ambas trazas. El PBI `01c9040df256` se **enriquece o se archiva** con diagnóstico corregido en el mismo ciclo (no segundo PR documental).

**Fuera:** reabrir el debate de `SDDIA_SKIP_HOOKS` global.

## 4. `F-EVOLUTION-RELACIONADO-LOCKFILE` — gate rojo por correlato incompleto

`sddia-qa gate-evolution --json --range` → `EVOL_MATERIAL_UNREGISTERED` en:

- `SddIA/Cargo.lock`
- `SddIA/engine/execute-process/Cargo.toml`
- `SddIA/infrastructure/adapters/index.md`
- `lancedb-evolution-repo.md` / `lancedb-thought-repo.md`

El registro `4d384bb1-f89d-41ce-835a-9db6d6bed114` no los listaba. Commit parche `f2b7aff`.

**Dentro:** checklist o helper de `relacionado` que cubra lockfile + manifiestos + fichas adapter tocadas por el diff (`origin/main...HEAD`) **antes** del primer `gate-evolution`. Test: diff con `Cargo.lock` y evolution sin ese path → UNREGISTERED; con path → EVOL_OK.

**Fuera:** cambiar el algoritmo de hash. Eximir `Cargo.lock` del gate.

## 5. `F-PROCESS-INGEST-JSON-STALE` — genoma miente

`SddIA/process/memory-evolution-ingest.md` v1.1.1 sigue: persistir en `.SddIA/vector_store/evolution/` JSON. Runtime #241: tabla LanceDB `{paths.vectorStore}/lancedb/`. DA-2 impidió Write directo; el desfase quedó.

**Dentro:** `entity-manager` `update` del proceso (bump SemVer, intent/cuerpo alineados a Cúmulo `paths.vectorStore`, puerto `EvolutionStore`, sin JSON SSOT). EDA coverage si el creator lo exige.

**Fuera:** migrar JSON legado del operador; reabrir embeddings.

## 6. `F-VALIDACION-CA13-SIN-LOG-CI` — APTO prematuro

`validacion.md` de LanceDB declara LDB-CA13 APTO por el YAML (`apt-get protobuf-compiler` + `cargo test`). No hay log de GitHub Actions en el artefacto. Tests locales ≠ CI.

**Dentro:** norma de cierre: un CA de CI no es APTO sin `run_id`/URL de check verde **o** veredicto explícito `PENDIENTE-CI` que no permita `global: APTO` si ese CA es gate. Alinear `features-documentation-pattern` (vía EM) y/o checklist Argos en `validacion.md`.

`DT-LANCEDB-COMPILE-TAX`: `cargo test` de adapters recompila lance (~10–14 min). Fuera de alcance optimizar; registrar para ciclo posterior.

## Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| CA1 | Rechazo GitHub `workflow` scope en push DCC no se clasifica como recursión hook | fixture + `analyze_fracture_kaizen` / enrich |
| CA2 | Envelope DCC `Publicación remota` failed por scope: código estable `F-DCC-WORKFLOW-SCOPE`, `blocked` o `failed` accionable; instrucción de unificar credencial | test handler o traza documentada |
| CA3 | Tras ese fallo, el runtime **no** autoriza `git push`/`gh` raw; Kintsugi o wait laudo | norma + barrera motora o test de política |
| CA4 | Traza `Head sha can't be blank` post-push-rejected ≠ cubo hook | test Mayeuta |
| CA5 | PBI `01c9040df256` corregido o archivado con diagnóstico F1 (mismo ciclo) | diff PBI |
| CA6 | Helper/checklist `relacionado` cubre lockfile+Cargo.toml+fichas adapter; gate --range verde en fixture | test o smoke `sddia-qa` |
| CA7 | `memory-evolution-ingest` actualizado vía `entity-manager`; texto JSON SSOT retirado | frontmatter SemVer + cuerpo |
| CA8 | CA de CI no cierra `global: APTO` sin evidencia de check o marca `PENDIENTE-CI` | norma/patrón + ejemplo en persist_ref |

## Fuera de alcance

- Reabrir integración LanceDB, embeddings, ingest físico.
- MiniLM/ONNX.
- Polling CI (DA-6).
- `SDDIA_SKIP_HOOKS=1` global.
- Ejecutar la propuesta del FIX `01c9040df256` tal cual.

## Definición de Done

Un único PR mergeado en `main` + `validacion.md` APTO `pbi_archived: true` + este PBI en `docs/todos/done/` en esa rama. Fractura `01c9040df256` saldada o absorbida en el mismo PR.
