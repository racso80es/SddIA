---
feature_name: process-creator-process-domain-roots
created: "2026-08-10"
purpose: Estabilización PBI-SDDIA-DOMAIN-ABSTRACT-03-D7-PROCESS-CREATOR — jurisdicción process_domain_roots en forja process-creator (deuda D7 post L-PACK-MULTIROOT-SIX-MOVE)
process: feature
branch_name: feat/process-creator-process-domain-roots
persist_ref: docs/features/process-creator-process-domain-roots
pbi_ref: docs/todos/pending/[ARQUITECTURA] process-creator — jurisdicción process_domain_roots (ABSTRACT-03 D7).md
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-03-D7-PROCESS-CREATOR
phase: mayeuta-stabilization
agents: mayeuta
source_feature: docs/features/sddia-domain-abstract-03-relocalizacion
parent_document_id: PBI-SDDIA-DOMAIN-ABSTRACT-03
correlation_id: ""
---

# Clarificación — PBI-SDDIA-DOMAIN-ABSTRACT-03-D7-PROCESS-CREATOR

## D0 — Semilla

- **PBI:** `docs/todos/pending/[ARQUITECTURA] process-creator — jurisdicción process_domain_roots (ABSTRACT-03 D7).md` (`document_id: PBI-SDDIA-DOMAIN-ABSTRACT-03-D7-PROCESS-CREATOR`; `uuid: a3c7e91f-2b4d-4f8a-9c1e-7d6b0a5f3211`; `status: abierto`).
- **Ciclo:** `feature` · rama `feat/process-creator-process-domain-roots` · `persist_ref` arriba.
- **Origen:** PPR #163 / ABSTRACT-03 — deuda **D7** diferida bajo laudo `L-PACK-MULTIROOT-SIX-MOVE`; `source_feature` `docs/features/sddia-domain-abstract-03-relocalizacion`; CID hermano Cosecha `3211daac-…`.
- **Prerrequisito satisfecho:** packing de 6 process software-lifecycle en `directories.process_domain_roots[0]` = `SddIA/library/codexes/codex-software-engineering/process` + Cúmulo `1.6.0` con `process_domain_roots`; índice códice y resolución multi-root del orquestador ya APTO en ABSTRACT-03.
- **Huecos inyección:** `pbi_ref` / `correlation_id` vacíos en payload runtime; SSOT PBI = path pending arriba.
- **Normas / SSOT:** `SddIA/core/cumulo.paths.json` (`directories.process` + `process_domain_roots`), `process-creator.md` (Core), `features-documentation-pattern` v1.2.1, `external-ai-constraints`, `process-contract`, laudo padre `L-KEEP-CORE`.

## D1 — Entropía de la semilla

| Defecto | Corrección |
|---------|------------|
| Semilla PBI aún marca `branch_name` ABSTRACT-03 | Operar en `feat/process-creator-process-domain-roots`; alinear frontmatter PBI en diseño/cierre (Dedalo/Tekton), no bloquear estabilización |
| `correlation_id` vacío | No inventar; Dedalo/runtime rellenan si el orquestador emite CID |
| Criterio «software-lifecycle → códice» sin classifier explícito en creator | Laudo **L-JURIS**: membresía canónica = los 6 ya packing (y futuros miembros del packing códice software-engineering); Dedalo fija señal de entrada (flag / membership / catálogo) sin hardcode de cliente |
| outputs de `process-creator.md` cableados a `{directories.process}` | Actualizar contrato de outputs al root **destino resuelto** (Core o domain) |

## D2 — Congruencia empírica (post-ABSTRACT-03)

| ID | Hecho | Implicación |
|----|-------|-------------|
| **I1** | Cúmulo `1.6.0` declara `process_domain_roots: ["SddIA/library/codexes/codex-software-engineering/process"]` y `directories.process: SddIA/process` | Resolución de **lectura** multi-root ya existe; **escritura** de creator no |
| **I2** | Los 6 (`feature`, `bug-fix`, `refactorization`, `pull-request-review`, `accept-pr`, `delivery-close-cycle`) viven en packing + `…/process/index.md` de códice; ausentes como ejecutables en Core | Re-move **fuera de alcance** |
| **I3** | `process-creator.md` fases 1–3: unicidad, persistencia e índice solo bajo `{paths.directories.process}` | Alta post-packing de miembro software → path Core = **entropía R1** vs packing |
| **I4** | `process-creator` permanece entidad Core (`SddIA/process/process-creator.md`); L-KEEP-CORE padre | Jurisdicción de **escritura** ≠ mover el creator al códice |
| **I5** | `entity-manager` / daemons / routes EDA no migrados (L-KEEP-CORE) | Fuera de alcance; handoff UUID/hash del creator hacia EM no exige relocate EM |
| **I6** | Residual Kalma2 Shell/`git-manager` = OPERATIVO PPR #136 **done** | Dedup; no reabrir en este ciclo |
| **I7** | Overlay instancia (`.SddIA/local.paths.json`) puede extender roots | Documentar si Dedalo toca fusión local; no inventar overlay sin diseño |

## D3 — Laudos Mayeuta

| ID | Decisión |
|----|----------|
| **L-PROCESS** | Ciclo = **`feature`**. Rama `feat/process-creator-process-domain-roots`. |
| **L-PARENT** | ABSTRACT-03 / `L-PACK-MULTIROOT-SIX-MOVE` **cerrado** como prerrequisito de packing+resolve; este ciclo liquida solo **D7** (escritura/índice de forja). |
| **L-KEEP-CORE** | No migrar `entity-manager`, daemons, routes EDA. `process-creator` sigue habitando Core; se le añade **política de destino**. |
| **L-JURIS** | Destino de alta: **software-lifecycle / miembros packing códice software-engineering** → root en `process_domain_roots` (default `[0]` salvo Dedalo multi-root); **resto** → `directories.process` (Core). Classifier exacto = Dedalo (catálogo membresía / input explícito); prohibido hardcode de path de cliente fuera de Cúmulo ± overlay. |
| **L-NO-REMOVE** | Prohibido re-mover o re-forjar los 6 ya packing (preservar UUID). |
| **L-INDEX-TARGET** | Actualizar **solo** el `index.md` del root destino. Alta de dominio: **cero** fila fantasma en `SddIA/process/index.md`. Alta Core: sin fila en índice de códice. |
| **L-UNIQ-MULTI** | Unicidad de `process_name` / aliases debe considerar **unión** Core + todos `process_domain_roots` (anti-colisión cross-root). |
| **L-FORGE** | Preferente: mutar genoma `process-creator` (y cápsula/factory asociada si existe) vía forja gobernada / `entity-manager`. Docs bajo `persist_ref` fuera de gate EDA. |
| **L-OVERLAY-DOC** | Si la solución depende de fusión `.SddIA/local.paths.json`, documentar contrato de overlay en `spec.md`/norma mínima; si no aplica, Declara N/A con evidencia. |
| **L-DEDUP-136** | Residual Kalma2 Shell/`git-manager` **fuera**; evidencia git vía `skill:git-manager` / `./sddia-run.sh --tool git-manager` sin bypass raw destructivo. |
| **L-TRUTH** | No inventar éxito de smoke/APTO sin artefacto/materialización. |
| **L-PBI** | Cierre Done = PBI → `docs/todos/done/` + `validacion.md` `global: APTO`, `pbi_archived: true` en **esta** rama (un PR). |

## D4 — Criterios de aceptación

| AC | Enunciado |
|----|-----------|
| **AC-JURIS** | Alta clasificada software-lifecycle / packing códice persiste bajo root de `process_domain_roots` (no bajo `SddIA/process/` como ejecutable). Alta no-dominio permanece en Core. |
| **AC-INDEX** | Índice del root destino actualizado y alineado al YAML fuente; sin fila fantasma en índice Core para altas de dominio. |
| **AC-SMOKE** | Smoke reproducible: alta (o dry-run/forja lab equivalente dictaminada por Dedalo) de process software **no** deja artefacto ejecutable bajo `SddIA/process/`. |
| **AC-UNIQ** | Colisión de nombre/alias cross-root detectada y aborta sin escribir. |
| **AC-RESOLVE-COMPAT** | Post-alta, `resolve_process_path` (u orquestador) encuentra el process en el root destino; sin regresión de los 6 packing ni process Core. |
| **AC-OVERLAY** | Overlay instancia documentado **o** N/A explícito con justificación. |
| **AC-BUILD** | Build/cápsulas tocadas OK si aplica cambio Rust/Python. |
| **AC-DOC** | Cascada `features-documentation-pattern`; PBI en `done/`; `validacion.md` APTO + `pbi_archived: true` en la rama del PR. |
| **AC-NONSCOPE** | Diff no re-mueve los 6 packing; no migra EM/daemons/routes EDA; no reabre OPERATIVO #136. |

## D5 — Ambigüedades resueltas / abiertas a Dedalo

| Tema | Estado |
|------|--------|
| ¿Re-mover los 6? | **Resuelto:** no (L-NO-REMOVE). |
| ¿Mover process-creator al códice? | **Resuelto:** no (L-KEEP-CORE); solo política de escritura. |
| ¿Migrar entity-manager? | **Resuelto:** no. |
| ¿Señal de jurisdicción (flag vs membership)? | **Abierta → Dedalo** (L-JURIS). |
| ¿Multi-root si crece `process_domain_roots`? | **Abierta → Dedalo** (default `[0]` aceptable si único root). |
| ¿Touchpoint: solo `.md` process-creator vs factory/cápsula? | **Abierta → Dedalo** tras audit empírico. |
| ¿Overlay local obligatorio? | **Condicional** (L-OVERLAY-DOC). |

## D6 — Handoff Dedalo

1. Auditar touchpoints reales de escritura (`process-creator.md`, factory/cápsula, tests) vs outputs cableados a `directories.process`.
2. Diseñar classifier L-JURIS + resolución de path destino desde Cúmulo (`process` ∪ `process_domain_roots`) ± overlay.
3. Spec de índice L-INDEX-TARGET + unicidad L-UNIQ-MULTI; plan de smoke AC-SMOKE sin inventar éxito.
4. Emitir `spec.md` + `plan.md`; no autorizar Tekton a escribir altas de dominio en Core «por compat».
