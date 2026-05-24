---
feature_name: e1-iota-ci
created: "2026-05-24"
process: feature
purpose: IOTA Rebased Testnet físico en CI — cerrar brecha E.1 backlog post-PR11
---

# Clarificación — E.1 IOTA físico en CI

Transcript de decisiones (2026-05-24).

---

## D1 — Inicio formal

| Pregunta | Decisión |
|----------|----------|
| ¿Proceso de inicio? | **`feature`** v1.3.0 |
| Rama | `feat/e1-iota-ci` |
| `persist_ref` | `docs/features/e1-iota-ci` |
| Manifiesto operativo | `docs/todos/pending/[OPERATIVO] Backlog pendiente post-PR11 — Hito 3, Ola C y laboratorio.md` § Prioridad 3 · E.1 |
| Upstream | `docs/features/pbi-005-hito3-git-hooks/` (V-P3 local), `docs/features/pull-request-automation-dlt/` (tool DLT + simulate lab) |

---

## D2 — Triaje: estado real IOTA vs CI

| Superficie | `SDDIA_LAB_SIMULATE_IOTA` | Evidencia DLT | Gap |
|------------|---------------------------|---------------|-----|
| `run-eda-e2e-lab.py` | **Default `1`** | digest `lab-sim-*` | Por diseño lab |
| Smokes feature (`*_smoke*.json`) | **Sí** | simulado | OK para lab |
| Hito 3 `validacion.md` V-P3 | **No** (local manual) | `delivery_state.cumulo: success` | **No en CI** |
| `.github/workflows/sddia-index-qa.yml` | N/A | — | **Sin IOTA ni watcher** |
| `dlt_bus_materializer.invoke_iota_publisher` | Rama `if simulate` vs `npx tsx` | físico si wallet + Node | Código listo; falta gate CI |

**Corrección PBI:** E.1 no exige eliminar simulate — exige que **CI** deje de ser ciego a la rama física.

---

## D3 — Alcance mínimo del smoke CI

| Pregunta | Decisión |
|----------|----------|
| ¿Solo publicar payload suelto? | **No** — preferir ruta **watcher** (paridad V-P3) |
| Evento mínimo | Instancia ECST acotada (p. ej. suscriptor `cumulo` / `Domain_Entity_*` o fixture `PullRequest_Presented` lab) en `pending/` |
| Comando | `event-watcher.py --once` **sin** `SDDIA_LAB_SIMULATE_IOTA` |
| Éxito | Archivo en `processed/` + `delivery_state.cumulo == "success"` + digest no `lab-sim-*` |
| Timeout | Reutilizar `SDDIA_IOTA_TIMEOUT_SECONDS` (default 45) |

---

## D4 — Estrategia workflow GitHub Actions

| Pregunta | Decisión |
|----------|----------|
| ¿Workflow nuevo o extender `sddia-index-qa`? | **Job adicional** en el mismo workflow (`eda-iota-physical`) — visibilidad unificada en PR |
| Runtime | `ubuntu-latest` |
| Node | `setup-node` + `npm ci` en `SddIA/scripts/tools/iota-immutable-publisher/` |
| Python | `3.12` (alineado workflow existente) |
| Secret | `IOTA_WALLET_SECRET` en repo secrets → env del job |
| PR desde fork | `if: github.event.pull_request.head.repo.full_name == github.repository` — **skip** job con log explícito |
| Fallo Testnet | Job **falla** (no fallback `FALLBACK_LOCAL_SIGNATURE` en CI — reservado a oráculo producción) |

---

## D5 — Script QA reutilizable

| Pregunta | Decisión |
|----------|----------|
| Nombre propuesto | `SddIA/scripts/qa/run-iota-ci-smoke.py` |
| Responsabilidad | Preparar fixture `pending/`, invocar watcher sin simulate, assert processed + digest |
| Salida | JSON stdout (`success`, `transaction_digest`, `event_id`) para consumo CI |
| Lab local | Operador puede ejecutar el mismo script **con** `SDDIA_LAB_SIMULATE_IOTA=1` para dry-run (documentar en `execution.md`) |

---

## D6 — Secretos y bóveda

| Pregunta | Decisión |
|----------|----------|
| ¿Leer `.SddIA/.dev/wallet.key` en CI? | **No** — solo `IOTA_WALLET_SECRET` inyectado por Actions |
| ¿Commitear wallet? | **Prohibido** — `.gitignore` ya cubre `.SddIA/.dev/` |
| Rotación | Documentar en `spec.md` que el operador rota secret en GitHub si se compromete |
| Logs CI | No imprimir secret ni payload completo con datos sensibles |

---

## D7 — Relación con scripts lab existentes

| Script | Cambio |
|--------|--------|
| `run-eda-e2e-lab.py` | **Sin cambio** en default simulate |
| `audit-entity-eda-coverage.py` (`anchor_merkle`) | **Sin cambio** — Merkle batch fuera de alcance E.1 |
| `dlt_bus_materializer.py` | **Sin cambio** salvo bugfix si smoke CI lo revela |
| `event-watcher.bat` hint simulate | **Sin cambio** — hint local válido |

---

## D8 — Criterios de aceptación Argos (preview)

| ID | Check |
|----|-------|
| V-E1-1 | Job CI verde en PR de esta feature con digest Testnet real |
| V-E1-2 | Job omitido/skipped en simulación fork documentada |
| V-E1-3 | `run-iota-ci-smoke.py` ejecutable localmente (simulate + físico) |
| V-E1-4 | PBI operativo § E.1 actualizable a ✅ en cierre documental |
| V-E1-5 | Regresión: `verify-process-integrity` + índices siguen verdes |

---

## D9 — Orquestación implementación

| Hito | Entrega |
|------|---------|
| H1 | `run-iota-ci-smoke.py` + fixture JSON bajo `docs/features/e1-iota-ci/` |
| H2 | Job `eda-iota-physical` + secret documentado |
| H3 | Smoke local físico (operador) + evidencia en `validacion.md` |
| H4 | Cierre documental PBI E.1 en rama PR |
