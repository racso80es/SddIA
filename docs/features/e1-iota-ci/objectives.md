---
feature_name: e1-iota-ci
process: feature
created: "2026-05-24"
persist_ref: docs/features/e1-iota-ci
branch_name: feat/e1-iota-ci
related_todo: docs/todos/pending/[OPERATIVO] Backlog pendiente post-PR11 — Hito 3, Ola C y laboratorio.md
tracks:
  - E.1
status: implementado
updated: "2026-05-24"
feature_ref: docs/features/e1-iota-ci
---

# Objetivos — E.1 IOTA físico en CI

## Meta

Cerrar la brecha **E.1** del backlog post-PR11: el pipeline de CI debe ejercitar el anclaje DLT **real** (IOTA Rebased Testnet vía `iota-immutable-publisher`), no depender exclusivamente de `SDDIA_LAB_SIMULATE_IOTA=1`. Hoy la gobernanza EDA en producción queda validada solo en laboratorio local o en smoke manual (p. ej. Hito 3 V-P3); GitHub Actions no comprueba la ruta física.

## Contexto operativo

| Hecho | Implicación |
|-------|-------------|
| `.github/workflows/sddia-index-qa.yml` | Solo `verify-tools-index` + `verify-process-integrity` — **sin** watcher ni IOTA |
| `run-eda-e2e-lab.py` | Fuerza `SDDIA_LAB_SIMULATE_IOTA=1` por defecto |
| Hito 3 V-P3 (`pbi-005-hito3-git-hooks`) | Watcher **sin** simulate validado **localmente** — no reproducido en CI |
| `dlt_bus_materializer.py` / `event-watcher.py` | Rama física operativa si `npx tsx` + `IOTA_WALLET_SECRET` disponibles |
| Vanguadia + lab L.2–L.3 entregados | Puerta de entrada y handlers lab sellados; E.1 es siguiente cierre P3 |
| PBI operativo § E.1 | Bloquea cierre del manifiesto junto con L1-O5 |

## Objetivos medibles

### Track E.1 — IOTA en CI

| ID | Objetivo | Criterio |
|----|----------|----------|
| **E1-O1** | **Job CI físico** | Workflow GitHub Actions ejecuta anclaje Testnet **sin** `SDDIA_LAB_SIMULATE_IOTA`; `transaction_digest` no vacío y **no** prefijo `lab-sim-` |
| **E1-O2** | **Ruta watcher integrada** | Smoke CI: evento mínimo `pending/` → `event-watcher.py` → `processed/` con `delivery_state.cumulo: success` en rama física |
| **E1-O3** | **Secretos aislados** | `IOTA_WALLET_SECRET` vía GitHub Secrets; sin seed en repo, logs ni payloads ECST |
| **E1-O4** | **Lab simulate intacto** | `SDDIA_LAB_SIMULATE_IOTA=1` sigue siendo el default en scripts lab locales; CI es capa adicional, no sustituto |
| **E1-O5** | **Documentación reproducible** | `execution.md` + `validacion.md` con comandos y evidencia digest; workflow documentado en `spec.md` |
| **E1-O6** | **Contención forks** | PRs desde forks no exponen secretos: job IOTA omitido o `skipped` con razón explícita |

## Orquestación

- **Alcance único track E.1** — sin reabrir vanguardia, lab L.2–L.3 ni Oráculo DLT.
- **Precedencia:** requiere `iota-immutable-publisher` y jerarquía bóvedas ya entregadas (`ampliacion-configuracion-entornos`, `pull-request-automation-dlt`).
- **Cierre backlog:** al mergear esta feature, el PBI operativo puede marcar E.1 ✅ (L1-O5 sigue residual P1).

## No objetivos (esta feature)

- Sustituir simulación lab en desarrollo local — `SDDIA_LAB_SIMULATE_IOTA` permanece.
- Webhook productivo GitHub ni despliegue daemon permanente.
- Ola C V3 (`event-sweeper`, recibos atómicos) — P4 backlog.
- L1-O5 runbooks unificados — feature o Kaizen aparte.
- Anclaje Merkle batch en cada PR — solo smoke mínimo CI.

## Artefactos previstos

| Ámbito | Rutas principales |
|--------|-------------------|
| CI | `.github/workflows/sddia-index-qa.yml` (extensión) o workflow dedicado |
| QA | Script smoke CI reutilizable bajo `SddIA/scripts/qa/` |
| Tool | `SddIA/scripts/tools/iota-immutable-publisher/` (deps CI) |
| Feature | `clarify.md`, `spec.md`, `plan.md`, smoke JSON, `validacion.md` |

## Estado

| Fase feature | Estado |
|--------------|--------|
| Inicialización | ✅ rama `feat/e1-iota-ci` |
| Objetivos | ✅ Este documento |
| Clarificación | ✅ `clarify.md` |
| Especificación | ✅ `spec.md` |
| Plan | ✅ `plan.md` |
| Implementación | ✅ `implementation.md` |
| Validación | ✅ `validacion.md` + `execution.md` |
