---
feature_name: jurisdiccion-deuda-tecnica-todos
created: "2026-08-28"
process: feature
purpose: Estabilización Mayeuta — jurisdicción docs/todos y portador de deuda no-fractura
version_clarify: "1.0.0"
---

# Clarificación — jurisdiccion-deuda-tecnica-todos

Transcript Mayeuta. Semilla: PBI `PBI-OPER-DEUDA-TECNICA-KINTSUGI-001` v2.0.0. Init lab `execution_id` `a3050468-df71-4922-bac9-3743bef2e54d`.

## D0 — Apertura

| Pregunta | Decisión |
|----------|----------|
| Proceso | `feature` v1.3.2 |
| `feature_name` | `jurisdiccion-deuda-tecnica-todos` |
| Rama | `feat/jurisdiccion-deuda-tecnica-todos` |
| `persist_ref` | `docs/features/jurisdiccion-deuda-tecnica-todos` |
| Norma | `SddIA/library/norms/` vía `entity-manager` → `norm-creator` (no `SddIA/norms/` motor) |

## D1 — Censo 2026-08-28 (corrige CA3 del PBI)

El PBI v2 cita `Optimizacion_BioIA.md` como tercer ítem de `DeudaTecnica/`. **Ausente** en disco. Tercer habitante real:

| Path | `document_id` | Laudo |
|------|---------------|-------|
| `DeudaTecnica/[DEUDA] Paciente 0 — prompt y proceso de despliegue.md` | `PBI-DT-PACIENTE0-DEPLOY-PROCESS` | `process_candidate` (`paciente0-deploy`). Mover a `pending/` con `dispatch: false`. No es deuda accionable de este ciclo. |
| `DeudaTecnica/[DEUDA] Paciente 0 — prompt de teardown.md` | `PBI-DT-PACIENTE0-UNDEPLOY-PROCESS` | `process_candidate` (`paciente0-undeploy`). Ídem. |
| `DeudaTecnica/[DEUDA] Escaneo lineal…md` | `PBI-DT-FRACTURE-RESOLVER-SCAN-LINEAL` | Primer habitante CA4 (`type: deuda` + `tech_debt_ids`). Mover a `pending/` sin cambiar alcance (depende del PBI de fan-out). |
| `Optimizacion_BioIA.md` | — | **Descarte por ausencia.** No recrear. |

## D2 — Buckets huérfanos (14 docs) — no inundar `pending/`

| Bucket | Docs | Laudo |
|--------|------|-------|
| `DeudaTecnica/` | 3 | Vaciar vía D1. Destino **eliminado** (no formalizar tercer cola). |
| `kitchen/` | 5 | **Inerte** (incubación humana). `dispatch: false` implícito: TQM no ve el prefijo. Promoción = copia/move **manual** a `pending/` con frontmatter de PBI. |
| `tmp/` | 5 | Los 5 tienen `status: consolidado` + `superseded_by` telemetría unificada. **Descarte** (borrar o dejar inerte deprecado; no promover). Distinto de `.tmp/` runtime (`git-operations`). |
| `historias/` | 1 | `CODEX-MVP-AUTOLOOP-KAIZEN` status `fundacional`. **Inerte** (narrativa/códice, no PBI). |

`Tokenomics.md` en kitchen: sin `document_id`/uuid → fósil; permanece inerte hasta forja atómica o descarte explícito en ejecución.

## D3 — Runtime: no ampliar anclajes

TQM y archivado **siguen** con dos prefijos. La norma no obliga a indexar `kitchen/` ni `historias/` en Core. Consumir `paths.todos.*` de Cúmulo (ya presentes). Resolutor de fractura: **fuera** (CA6 / `PBI-KAIZEN-FRACTURE-FANOUT-IDEMPOTENCIA`).

## D4 — Portador no-fractura (CA4)

| Campo | Contrato |
|-------|----------|
| Ubicación | Solo `docs/todos/pending/` (despachable) o `done/` (archivado) |
| `type` | `deuda` |
| `tech_debt_ids` | Array no vacío; prefijo `DT-` |
| `friction_ids` | Opcional; prefijo `F-` |
| `dispatch` | `false` hasta que un proceso (`feature`/`bug-fix`/`refactorization`) lo tome |
| `process` | Proceso de resolución **o** `null` si `process_candidate` |
| `process_candidate` | Semilla de proceso; no despachar como feature |

Enum de prefijos (norma): `F-` fricción estructural; `DT-` deuda catalogada. Prohibido IDs sin prefijo en frontmatter nuevo.

## D5 — Done (CA2)

Sin estado `deuda_tecnica` como cierre. `status: deuda_tecnica` en Paciente 0 es **clasificación de contenido**, no bucket. Cierre = `done/` + `validacion.md` APTO + `pbi_archived: true`.

## D6 — CA5

Evidencia: `cargo test -p execute-process extract_pbi_path` (positivos `pending/`, negativos `DeudaTecnica/`/`kitchen/`) más aserción del predicado de archivado (`rel.contains("docs/todos/pending/")`). Salida del CLI en `validacion.md`.
