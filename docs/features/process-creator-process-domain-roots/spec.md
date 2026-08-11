---
feature_name: process-creator-process-domain-roots
created: "2026-08-10"
process: feature
base: main
scope: process-creator-process-domain-roots
branch_name: feat/process-creator-process-domain-roots
persist_ref: docs/features/process-creator-process-domain-roots
pbi_ref: docs/todos/pending/[ARQUITECTURA] process-creator — jurisdicción process_domain_roots (ABSTRACT-03 D7).md
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-03-D7-PROCESS-CREATOR
parent_document_id: PBI-SDDIA-DOMAIN-ABSTRACT-03
source_feature: docs/features/sddia-domain-abstract-03-relocalizacion
version_spec: "1.0.0"
status: dedalo_locked
laudo: L-JURIS-MEMBERSHIP-PLUS-FLAG
agents: dedalo
correlation_id: ""
---

# Especificación — process-creator-process-domain-roots

## 1. Misión técnica

Liquidar deuda **D7** (ABSTRACT-03 / PPR #163): tras packing de los 6 process software-lifecycle en `directories.process_domain_roots` y resolución multi-root de **lectura** (Cúmulo **1.6.0**), alinear la **escritura** de forja (`process-creator` + `run_process_forge`) a la misma topología + política de jurisdicción — sin filas fantasma en índice Core ni ejecutables de dominio bajo `directories.process`.

## 2. Auditoría empírica (touchpoints)

| ID | Hecho | Implicación |
|----|-------|-------------|
| **T1** | `SddIA/process/process-creator.md` v1.1.0: fases 1–3 y outputs cableados a `{paths.directories.process}` + índice Core | Contrato documental de escritura mono-root |
| **T2** | `engine/.../forges/factory.rs` → `run_process_forge`: hardcode `repo.join("SddIA/process")` para create/update/índice | **Touchpoint ejecutable nativo**; sin esto el genoma `.md` no basta |
| **T3** | `resolver.rs` / `process_search_roots` ya lee `process_domain_roots` ∪ Core vía `load_paths_config` | Reutilizar para unicidad, update-locate y post-alta resolve |
| **T4** | `load_paths_config` fusiona `.SddIA/local.paths.json` (merge objeto; clave `directories.process_domain_roots` **reemplaza** array) | Overlay ya existe; creator/forge deben consumir cfg fusionada |
| **T5** | SSOT membresía: `codex-software-engineering.md` → `process_membership` (6 nombres) | Classifier base sin hardcode de cliente |
| **T6** | `process-creator` habita Core; EM/daemons/routes L-KEEP-CORE | Solo política de destino; no relocate creator |

## 3. Laudos Dedalo

| Ref | Pregunta | Laudo |
|-----|----------|-------|
| **D1** | ¿Señal de jurisdicción? | **`L-JURIS-MEMBERSHIP-PLUS-FLAG`:** (1) input opcional `process_jurisdiction`: `domain` \| `core`; (2) si ausente, `process_name ∈ process_membership` del códice `codex-software-engineering` → `domain`; (3) else → `core`. Alta **nueva** de dominio **exige** `process_jurisdiction: domain` (membresía sola no cubre nombres aún no listados). |
| **D2** | ¿Root destino si `domain`? | Con `process_domain_roots` longitud **1** (estado Cúmulo 1.6.0): ese único root. Si longitud **>1**: input opcional `process_domain_root` (relpath que **debe** coincidir con un elemento del array fusionado); si falta → **default `[0]`** y registrar en outputs el root elegido. Prohibido inventar path fuera de Cúmulo ± overlay. |
| **D3** | ¿Touchpoint primario? | Mutar **ambos**: (A) genoma `process-creator.md` (inputs/outputs/fases) y (B) `run_process_forge` en factory. El runtime nativo es SSOT de materialización; el `.md` es contrato obrero/IDE. |
| **D4** | ¿Unicidad? | **`L-UNIQ-MULTI`:** antes de escribir, escanear **unión** Core + todos `process_domain_roots` (name + aliases, misma exclusión `index`/`process-contract` que el resolver). Colisión → abort sin write. |
| **D5** | ¿Índice? | **`L-INDEX-TARGET`:** solo `{root_destino}/index.md`. Alta domain → **cero** fila nueva en `SddIA/process/index.md`. Alta Core → cero fila en índice de códice. |
| **D6** | ¿Update / idempotencia? | Localizar process existente vía `process_search_roots` (domain-first); mutar **in situ** en el root hallado; actualizar índice de **ese** root. Prohibido recrear en Core un process que ya vive en dominio. |
| **D7** | ¿Overlay? | **`AC-OVERLAY = N/A (nuevo esquema)`** con evidencia: no se introduce schema overlay nuevo. Escritura usa `load_paths_config` (misma fusión que resolve). Documentar en esta spec el contrato heredado ABSTRACT-03 (§3.3 allí): si instancia declara `directories.process_domain_roots`, **reemplaza** el array Core. |
| **D8** | ¿process-contract? | Ajuste documental mínimo: unicidad de `aliases`/`name` refiere a catálogo multi-root (unión roots), no solo `directories.process`. Sin bump de major si el contrato ya es v1.4.0; nota en cuerpo + evolución si Tekton toca el archivo. |
| **D9** | ¿Membresía códice al alta domain? | **Fuera de forja automática** en este ciclo: no mutar `process_membership` desde factory. Operador/ciclo códice actualiza membresía si el process nuevo debe ser packing canónico. Destino físico no depende de esa mutación si `process_jurisdiction: domain`. |
| **D10** | ¿Los 6 packing? | **`L-NO-REMOVE`:** no mover, no re-forjar, no alterar UUID. Unicidad debe detectarlos en domain root y abortar create homónimo en Core. |
| **D11** | ¿EM / EDA / Kalma2 Shell? | Fuera de alcance (L-KEEP-CORE / L-DEDUP-136). Handoff UUID/hash del creator hacia EM sin relocate EM. |

## 4. Contrato de inputs / outputs (process-creator)

### 4.1 Inputs (delta)

| Input | Obligatorio | Semántica |
|-------|-------------|-----------|
| *(existentes)* | según contrato vigente | Sin cambio de semántica base |
| `process_jurisdiction` | No | `domain` \| `core`. Ver D1. |
| `process_domain_root` | No | Relpath ∈ `directories.process_domain_roots` fusionado; solo si jurisdiction=domain y multi-root. |

### 4.2 Outputs (delta)

| Output | Semántica nueva |
|--------|-----------------|
| `artifact_process_md` | `{root_destino_resuelto}/{process_name}.md` (no asumir Core) |
| `artifact_process_index` | `{root_destino_resuelto}/index.md` |
| `resolved_process_root` *(nuevo)* | Relpath canónico del root usado (Core o domain) |
| `process_jurisdiction_applied` *(nuevo)* | `domain` \| `core` efectivamente aplicado |
| handoff_* | Sin cambio de forma hacia EM |

### 4.3 Algoritmo de destino (escritura)

```text
cfg = load_paths_config(repo)   # Cúmulo + overlay
core = cfg.directories.process
roots_domain = cfg.directories.process_domain_roots || []

juris = input.process_jurisdiction
si juris ausente:
  si process_name ∈ process_membership(codex-software-engineering) → juris = domain
  else → juris = core

si juris == core:
  dest = core
sino:
  si roots_domain vacío → FAIL controlado (no hay packing domain)
  si process_domain_root set:
    debe ∈ roots_domain; dest = ese
  else:
    dest = roots_domain[0]

# L-UNIQ-MULTI (create):
scan name+aliases en process_search_roots; colisión → abort

# write:
persist dest/{name}.md
update solo dest/index.md
# never append Core index when dest != core
```

## 5. Touchpoints de implementación

| Path | Cambio |
|------|--------|
| `SddIA/process/process-creator.md` | Inputs/outputs/fases; unicidad multi-root; persistencia e índice en root resuelto; bump version proceso |
| `SddIA/engine/execute-process/src/forges/factory.rs` | `run_process_forge`: eliminar hardcode Core-only; classifier D1–D2; uniq; index target; update multi-root |
| `SddIA/engine/execute-process/src/forges/*` (tests) | Fixtures: alta domain → path packing + índice domain; sin fila Core; colisión cross-root aborta; Core alta intacta |
| `SddIA/process/process-contract.md` | Nota unicidad multi-root (D8) |
| `SddIA/norms/external-ai-constraints.md` | Una línea: creators process respetan `process_domain_roots` en escritura (si aún solo documenta lectura) |
| Evolution | Entrada vinculando uuid `process-creator` `7c2d9e41-…` + PBI `a3c7e91f-…` |

**Prohibido en diff:** re-move de los 6; migrar EM/daemons/routes; tocar residual Kalma2 Shell/git-manager.

## 6. Criterios ↔ evidencia

| AC | Evidencia |
|----|-----------|
| **AC-JURIS** | Test/forge: `process_jurisdiction=domain` → `.md` bajo `process_domain_roots[0]`; sin `process_jurisdiction` + nombre no-membresía → Core |
| **AC-INDEX** | Índice destino con fila alineada YAML; `SddIA/process/index.md` sin fila nueva para alta domain |
| **AC-SMOKE** | Dry-run/lab forge (tempdir o workspace lab) de process software **no** deja ejecutable bajo `SddIA/process/`; documentar comando en `execution.md` |
| **AC-UNIQ** | Fixture: nombre igual a process en packing → create Core aborta; alias cross-root aborta |
| **AC-RESOLVE-COMPAT** | Post-alta domain, `resolve_process_path` encuentra el nuevo; los 6 packing + process Core de regresión siguen resolviendo |
| **AC-OVERLAY** | N/A schema nuevo; párrafo §3 D7 + referencia ABSTRACT-03 overlay en `implementation.md`/`spec` |
| **AC-BUILD** | `cargo test`/`cargo build -p execute-process` OK si se toca Rust |
| **AC-DOC** | Cascada + PBI → `done/` + `validacion.md` APTO + `pbi_archived: true` en esta rama |
| **AC-NONSCOPE** | Diff review: no move de los 6; no EM/daemons/routes; no PPR #136 |

## 7. Fuera de alcance

- Re-mover / re-forjar los 6 packing.
- Migrar `entity-manager`, daemons, routes EDA.
- Auto-actualizar `process_membership` del códice en cada alta domain.
- Residual Kalma2 Shell / `git-manager` (OPERATIVO #136).
- Semillas Kaizen bajo `docs/todos/` (solo Cumulo / `Kaizen_Alert_Required`).

## 8. Handoff Tekton

1. Implementar classifier + destino en **factory** primero (evidencia ejecutable); luego alinear genoma `process-creator.md`.
2. Tests AC-JURIS / AC-INDEX / AC-UNIQ / regresión resolve **antes** de declarar smoke.
3. Smoke AC-SMOKE materializado (no inventar éxito); preferir fixture tempdir en test + smoke lab opcional.
4. Docs: `implementation.md` + `execution.md`; evolution; Argos.
5. Git solo vía `skill:git-manager` / `./sddia-run.sh --tool git-manager`.

## 9. Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Solo actualizar `.md` creator y dejar factory Core-only | D3: ambos obligatorios |
| Update busca solo Core → “pérdida” de process domain | D6: locate multi-root |
| Overlay mal configurado vacía roots domain | FAIL controlado si jurisdiction=domain y array vacío |
| Alta domain sin flag en IDE | Documentar input; default membership solo para nombres ya packing (anti-recreate) |
