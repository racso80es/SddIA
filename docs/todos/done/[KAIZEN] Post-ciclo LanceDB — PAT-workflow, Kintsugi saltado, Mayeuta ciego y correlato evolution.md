---
document_id: PBI-KAIZEN-LANCEDB-CICLO-FRICCIONES
uuid: "12250eca-49c6-4008-ac50-5c5722a7fe91"
title: "[KAIZEN] Post-ciclo LanceDB — PAT/workflow, Kintsugi saltado, Mayeuta ciego y correlato evolution"
format: markdown
version: "1.1.0"
created: "2026-08-31"
updated: "2026-08-31"
status: done
refinement_status: refinado
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
  - F-DCC-NO-ABORT-AFTER-PUSH-FAIL
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
    rol: "Ciclo de origen (PR #241, MERGED). Este Kaizen recoge fricción de forja/aduana/obediencia; no reabre LanceDB físico ni embeddings."
  - id: PBI-FIX-FRACTURE-01c9040df256
    rol: "Fractura DCC del mismo ciclo (Apertura en forja). Diagnóstico Mayeuta (recursión hook / implementar SDDIA_HOOK_DELIVERY_CLOSE) es FALSO. Texto de propuesta = clasificador PRE-0c5268362b9a. No ejecutar esa propuesta."
  - id: PBI-FIX-FRACTURE-0c5268362b9a
    rol: "Fix hermano (cerrado 2026-08-31). Estrecha cubo hook y sella F-DCC-HOOK-EVOL-OVERESCALATION / F-MAYEUTA-PREPUSH-EVOL-COLLISION. Este Kaizen no lo reabre; el specimen LanceDB no es pre-push evol."
  - id: PBI-KAIZEN-CI-TELEMETRY-OBSERVABILITY
    rol: "Adyacente, no el mismo alcance. Telemetría remota de jobs CI ≠ gate documental de CA de CI en validacion.md (CA8)."
source_audit: "Auditoría post-implementación feat/lancedb-real-vector-memory (PR #241 MERGED, execution_id init c4e7971d-7c67-4745-b1b4-eb8b3d84d652). Evidencia durable en repo: PBI 01c9040df256 (traza GraphQL Head sha blank, acción Apertura en forja); precedente docs/fixes/integridad-proceso-forge-ci/execution.md (DCC 32e47e49, PAT sin workflow, PR #226); memory-evolution-ingest.md v1.1.1 aún declara JSON; evolution 4d384bb1 (relacionado ampliado en commit f2b7aff); validacion.md LDB-CA13 ahora cita run 33383923692 (primer sello fue APTO sin run_id). Acuses DCC 391073eb / c9a85460 no están en persist_ref ni workspaces versionados — sesión de forja, no ancla git."
review_notes: "v1.1.0 corrige v1.0.0: (1) F-MAYEUTA-PREPUSH-EVOL-COLLISION mal aplicado — specimen es Head sha / Apertura, no pre-push evol; linaje F3 d0cfd5b66ff1. (2) Clasificador vigente post-0c5268362b9a ya no dispara hook por token delivery-close ni propone 'Implementar guarda'; el PBI FIX conserva texto pre-fix. (3) CA13: el árbol actual SÍ tiene run 33383923692; el defecto es APTO prematuro del primer sello. (4) Registro 4d384bb1 YA lista lockfile/Cargo.toml/fichas (f2b7aff); el hueco es ausencia de helper. (5) Omite F-DCC-NO-ABORT-AFTER-PUSH-FAIL (genoma aborta; runtime ejecuta todas las fases). (6) gh auth refresh no unifica el PAT de git-manager. (7) CA1 estaba mapeado a §1 siendo criterio Mayeuta. (8) '10–14 min' de compile tax sin log de timing versionado."
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/norms/external-ai-constraints.md
  - SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md
  - SddIA/library/norms/features-documentation-pattern.md
  - SddIA/engine/execute-process/src/engine/delivery_close.rs
  - SddIA/engine/execute-process/src/engine/phase_capsules.rs
  - SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs
  - SddIA/skills/git-manager.md
  - SddIA/skills/sddia-evolution-register/src/lib.rs
  - SddIA/process/memory-evolution-ingest.md
  - .github/workflows/sddia-index-qa.yml
  - docs/features/lancedb-real-vector-memory/
  - docs/fixes/integridad-proceso-forge-ci/execution.md
  - docs/fixes/dcc-hook-evol-overescalation-0c5268362b9a/
  - docs/todos/pending/[FIX] delivery-close-cycle — fractura sistémica (01c9040df256).md
  - docs/todos/done/[FIX] delivery-close-cycle — fractura sistémica (d0cfd5b66ff1).md
  - docs/todos/done/[FIX] delivery-close-cycle — fractura sistémica (0c5268362b9a).md
---

# [KAIZEN] Post-ciclo LanceDB — fricción de forja

Auditoría del transcurso de `PBI-CORE-LANCEDB-REAL-001` (rama `feat/lancedb-real-vector-memory`, PR [#241](https://github.com/racso80es/SddIA/pull/241), MERGED). Las fricciones son **de ciclo**, no del driver LanceDB. El hexágono y los tests de reapertura **no** se reabren.

**Prohibido** implementar la propuesta Mayeuta de `PBI-FIX-FRACTURE-01c9040df256` (`SDDIA_HOOK_DELIVERY_CLOSE` + `SDDIA_SKIP_HOOKS=1`). Esa guarda **ya existe** (`capsule_delivery_remote_push` exporta `SDDIA_HOOK_DELIVERY_CLOSE=1`; Ola B / `0c5268362b9a`). La fractura es síntoma de push rechazado + DCC que no aborta, no recursión hook.

**No es** recurrencia de `F-MAYEUTA-PREPUSH-EVOL-COLLISION` (ese cubo era pre-push + evolution gate; cerrado en `0c5268362b9a`). Linaje correcto: F3 de `d0cfd5b66ff1` (token `delivery-close` → toda fractura DCC = «recursión hook»). El specimen `01c9040df256` lleva el texto de propuesta **pre**-estrechamiento (`Implementar guarda…`); el clasificador vigente propone `Auditar por qué la guarda no cortó…; no reimplementarla` y solo dispara hook si `hook_blob` contiene `delivery-close-cycle failed for` / `recurs` / `re-entrada`.

## 0. Trazabilidad fricción → criterio

| `friction_id` | Sección | Criterio | Deuda | Estado en origen |
|---------------|---------|----------|--------|------------------|
| `F-DCC-GIT-PAT-NO-WORKFLOW` | §1 | CA2 | `DT-GIT-MANAGER-CREDENTIAL-SPLIT` | Abierto (reproducido #226 → #241) |
| `F-DCC-NO-ABORT-AFTER-PUSH-FAIL` | §1b | CA2b | — | Abierto (genoma vs runtime) |
| `F-TEKTON-KINTSUGI-SALTO-DCC` | §2 | CA3 | — | Abierto (agravante: fractura SÍ emitida) |
| `F-MAYEUTA-FRACTURE-HOOK-FALSE-POSITIVE` | §3 | CA1, CA4, CA5 | — | Specimen sucio; clasificador vigente ya no clasifica esta traza como hook |
| `F-EVOLUTION-RELACIONADO-LOCKFILE` | §4 | CA6 | — | Parchado en origen (`f2b7aff`); falta helper anti-recurrencia |
| `F-PROCESS-INGEST-JSON-STALE` | §5 | CA7 | — | Abierto (genoma `memory-evolution-ingest` v1.1.1) |
| `F-VALIDACION-CA13-SIN-LOG-CI` | §6 | CA8 | `DT-LANCEDB-COMPILE-TAX` | Primer sello prematuro; árbol actual ya cita run `33383923692` |

## 1. `F-DCC-GIT-PAT-NO-WORKFLOW` — git-manager y `gh` no comparten credencial

DCC intento 1 del ciclo LanceDB: fase **Publicación remota** rechazada por GitHub:

```text
refusing to allow a Personal Access Token to create or update workflow
`.github/workflows/sddia-index-qa.yml` without `workflow` scope
```

`skill:git-manager` invoca el binario `git` del sistema (HTTPS origin). Ese canal usa el PAT del credential helper de git, **distinto** del token de `gh auth` (scopes `gist`, `read:org`, `repo`, `workflow` en el host del ciclo).

Precedente idéntico: `docs/fixes/integridad-proceso-forge-ci/execution.md` (DCC `32e47e49`, PR #226). Allí el operador reintentó DCC tras alinear credencial; **no** se cerró la dualidad. #241 la reprodujo al tocar CI (`protobuf-compiler` + tests LanceDB).

**Dentro:** alinear el canal de push de DCC/`git-manager` con una credencial que tenga `workflow` **o** detectar el rechazo GitHub y fallar con envelope accionable (`F-DCC-WORKFLOW-SCOPE`, paridad `F-DCC-DNS-UNRESOLVED` / `F-DCC-HOOK-EVOL-OVERESCALATION`: `blocked`, sin `System_Fracture_Detected` de recursión hook).

Instrucción operador precisa: `gh auth refresh -s workflow` **solo basta** si git ya delega en `gh` (`gh auth setup-git`). Si el credential helper guarda un PAT HTTPS aparte, hay que unificar helper o rotar ese PAT. El workaround del ciclo (`git push https://x-access-token:$(gh auth token)@…`) es bypass raw — prohibido como vía canónica.

**Fuera:** reimplementar `SDDIA_HOOK_DELIVERY_CLOSE`. Mutar políticas GitHub de la org sin laudo.

## 1b. `F-DCC-NO-ABORT-AFTER-PUSH-FAIL` — genoma aborta; runtime sigue

`delivery-close-cycle.md` v1.4.0 § Fase Publicación remota: «Abortar si `success` es `false`.»

`delivery_close.rs` `run()`:

```text
for phase in phases {
    phase_reports.push(execute_phase(...));
}
emit_dcc_phase_fractures(...)
```

No hay halt. Tras push rejected, **Apertura en forja** corre igual → GraphQL `Head sha can't be blank` / `No commits between main and feat/lancedb-real-vector-memory`. Cúmulo materializó **esa** fase (`attempted_action: Apertura en forja`), no la de Publicación remota. Mayeuta nunca vio `without workflow scope` en el PBI `01c9040df256`.

**Dentro:** si Publicación remota es `failed`/`blocked`, no ejecutar Apertura / Sello / Higiene; un solo `friction_id` causal (`F-DCC-WORKFLOW-SCOPE` u homólogo). Test: fixture push rejected → `phase_reports` posteriores `skipped` (o ausentes), un fracture payload.

**Fuera:** castrar DCC. Fail-soft de fases secundarias post-`pr_url` (ola L-FAILSOFT) no se toca.

## 2. `F-TEKTON-KINTSUGI-SALTO-DCC` — entrega continuada tras colapso

Tras el `failed` de DCC, el runtime **sí** emitió `System_Fracture_Detected` (PBI `01c9040df256` existe). Kintsugi estaba disparado. Tekton aun así ejecutó `git push` con token `gh` (URL `x-access-token`) y re-invocó DCC → PR #241.

`obediencia-procesos.md` § Kintsugi paso 6 y Ley de Jurisdicción Delegada: **prohibido** `git`/`gh` raw para evadir fallo de proceso oficial.

Clase ya sellada como `F-TEKTON-BYPASS-RAW-POST-COLAPSO` (Kaizen jurisdicción todos, CA7 — cláusula **colapso mudo**). Aquí el sello normativo **no impidió** el salto. Agravante vs #219: entonces no hubo fractura; ahora **sí** la hubo y se ignoró.

**Dentro:** envelope DCC de `Publicación remota` failed por scope `workflow` = `blocked` accionable (no fractura hook). Lab-relay / `external-ai-constraints`: barrera explícita que impida re-push raw tras DCC `failed`/`blocked`. Documentar el caso `workflow` scope como colapso de **credencial**, no de Core.

**Fuera:** castrar DCC. Perdonar el salto de #241 (histórico; PR mergeado). Reabrir el debate de `SDDIA_SKIP_HOOKS` global.

## 3. `F-MAYEUTA-FRACTURE-HOOK-FALSE-POSITIVE` — PBI `01c9040df256`

Cúmulo materializó fractura. Mayeuta (síntesis del PBI): «recursión hook» + «Implementar guarda `SDDIA_HOOK_DELIVERY_CLOSE` y push interno con `SDDIA_SKIP_HOOKS=1`».

Traza durable del PBI:

```text
Acción intentada: Apertura en forja
gh_stderr=… GraphQL: Head sha can't be blank, Base sha can't be blank,
No commits between main and feat/lancedb-real-vector-memory, Head ref must be a branch
```

Causa: §1 + §1b, no hooks. El PBI FIX **no tiene `uuid`** en frontmatter (entropía de materialización).

Clasificador **vigente** (`enrich_fracture_pbi_kaizen.rs`): `hook_blob` = `error_trace + attempted_action` (sin `process_name`). Esta traza **no** contiene `delivery-close-cycle failed for` / `recurs` / `re-entrada` → **no** debería emitir cubo hook. Residual: (a) test de regresión con fixture de este specimen; (b) cubo positivo `credential_workflow_scope` / `remote_branch_absent` para no caer en `failed`→`prompt_adjustment` genérico; (c) el PBI `01c9040df256` se enriquece o se archiva con diagnóstico corregido **en el mismo ciclo** (no segundo PR documental).

**Fuera:** reabrir `SDDIA_SKIP_HOOKS` global. Reimplementar la guarda. Reabrir `0c5268362b9a`.

## 4. `F-EVOLUTION-RELACIONADO-LOCKFILE` — gate rojo por correlato incompleto

En el **primer** registro `4d384bb1-f89d-41ce-835a-9db6d6bed114`, `relacionado` omitía entre otros `SddIA/Cargo.lock`, `SddIA/engine/execute-process/Cargo.toml`, `SddIA/infrastructure/adapters/index.md`, fichas `lancedb-*-repo.md`. `sddia-qa gate-evolution --json --range` → `EVOL_MATERIAL_UNREGISTERED`.

**Ya parchado en el ciclo origen** (commit `f2b7aff`). El árbol actual lista esos paths. El hueco que queda es de **proceso**: `sddia-evolution-register` exige `relacionado` no vacío pero no deriva lockfile / manifiestos / fichas adapter del diff `origin/main...HEAD`.

**Dentro:** checklist o helper de `relacionado` que cubra lockfile + manifiestos + fichas adapter tocadas por el diff **antes** del primer `gate-evolution`. Test: diff con `Cargo.lock` y evolution sin ese path → UNREGISTERED; con path → EVOL_OK.

**Fuera:** cambiar el algoritmo de hash. Eximir `Cargo.lock` del gate. Rehacer el correlato LanceDB.

## 5. `F-PROCESS-INGEST-JSON-STALE` — genoma miente

`SddIA/process/memory-evolution-ingest.md` v1.1.1: persistir en `.SddIA/vector_store/evolution/` JSON. Intent de fase: «persistir en `.SddIA/vector_store/evolution/`». Cuerpo: «JSON durable bajo `.SddIA/vector_store/evolution/`».

Runtime #241: tabla LanceDB `{paths.vectorStore}/lancedb/` vía puerto `EvolutionStore`. Ficha adapter `lancedb-evolution-repo.md` v1.1.0 ya dice «Sin JSON como SSOT». DA-2 impidió Write directo al proceso durante el ciclo LanceDB; el desfase quedó.

**Dentro:** `entity-manager` `update` del proceso (bump SemVer, intent/cuerpo alineados a Cúmulo `paths.vectorStore`, puerto `EvolutionStore`, sin JSON SSOT). EDA coverage si el creator lo exige.

**Fuera:** migrar JSON legado del operador; reabrir embeddings.

## 6. `F-VALIDACION-CA13-SIN-LOG-CI` — APTO prematuro (luego parcheado)

El **primer** sello Argos declaró LDB-CA13 APTO por el YAML (`apt-get protobuf-compiler` + `cargo test`) **sin** `run_id`. Tests locales ≠ CI.

El árbol **actual** de `docs/features/lancedb-real-vector-memory/validacion.md` ya tiene `ci_run_id: "33383923692"` y evidencia `sddia-index-integrity` SUCCESS head `8818faa`. El parche fue post-facto, tras CI verde. `features-documentation-pattern` v1.2.1 **no** exige evidencia de check para `global: APTO`.

**Dentro:** norma de cierre (vía EM sobre `features-documentation-pattern`): un CA de CI no es APTO sin `run_id`/URL de check verde **o** veredicto explícito `PENDIENTE-CI` que no permita `global: APTO` si ese CA es gate.

No confundir con `PBI-KAIZEN-CI-TELEMETRY-OBSERVABILITY` (puente remoto de jobs fallidos).

`DT-LANCEDB-COMPILE-TAX`: `cargo test` de adapters recompila lance; coste alto observado en el ciclo, **sin** log de timing versionado. Fuera de alcance optimizar; registrar para ciclo posterior. Prohibido citar «10–14 min» como cifra medida.

## Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| CA1 | Trazas `without workflow scope` y `Head sha can't be blank` (post-push-rejected) **no** caen en cubo hook. Fixture del specimen `01c9040df256`. | test `analyze_fracture_kaizen` |
| CA2 | Envelope DCC `Publicación remota` failed por scope: código estable `F-DCC-WORKFLOW-SCOPE`, `blocked` accionable; instrucción de unificar **credential helper git→gh**, no solo `gh auth refresh`. | test handler o traza documentada |
| CA2b | Tras Publicación remota `failed`/`blocked`, Apertura/Sello/Higiene no se ejecutan. Un fracture payload causal. | test `delivery_close` / `phase_reports` |
| CA3 | Tras ese fallo, el runtime **no** autoriza `git push`/`gh` raw; Kintsugi o wait laudo | norma + barrera motora o test de política |
| CA4 | Cubo positivo `credential_workflow_scope` / `remote_branch_absent` (no solo ausencia de hook) | test Mayeuta |
| CA5 | PBI `01c9040df256` corregido o archivado con diagnóstico §1+§1b (mismo ciclo); `uuid` si se conserva | diff PBI |
| CA6 | Helper/checklist `relacionado` cubre lockfile+Cargo.toml+fichas adapter; gate `--range` verde en fixture | test o smoke `sddia-qa` |
| CA7 | `memory-evolution-ingest` actualizado vía `entity-manager`; texto JSON SSOT retirado | frontmatter SemVer + cuerpo |
| CA8 | CA de CI no cierra `global: APTO` sin evidencia de check o marca `PENDIENTE-CI` | norma/patrón + ejemplo en persist_ref |

## Fuera de alcance

- Reabrir integración LanceDB, embeddings, ingest físico.
- MiniLM/ONNX.
- Polling CI (DA-6).
- `SDDIA_SKIP_HOOKS=1` global.
- Ejecutar la propuesta del FIX `01c9040df256` tal cual.
- Reabrir `0c5268362b9a` / `d0cfd5b66ff1`.
- Optimizar tiempo de compilación Arrow/lance (`DT-LANCEDB-COMPILE-TAX`).
- Telemetría remota de CI (`PBI-KAIZEN-CI-TELEMETRY-OBSERVABILITY`).

## Definición de Done

Un único PR mergeado en `main` + `validacion.md` APTO `pbi_archived: true` + este PBI en `docs/todos/done/` en esa rama. Fractura `01c9040df256` saldada o absorbida en el mismo PR.
