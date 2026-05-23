---
feature_name: pull-request-automation-dlt
created: "2026-05-23"
process: feature
branch_name: feat/pull-request-automation-dlt
persist_ref: docs/features/pull-request-automation-dlt
phases: 4
agent_planificador: dedalo
---

# Plan de implementación — Oráculo Sensor DLT

Blueprint alineado al PBI v2.0.0 (H1–H4), `clarify.md` D1–D9 y `spec.md`.

## 0. Estado de la entrega

| Bloque | Estado | Evidencia |
|--------|--------|-----------|
| Rama de trabajo | ✅ | `feat/pull-request-automation-dlt` |
| Clarificación | ✅ | `clarify.md` |
| PBI markdown | ✅ | `docs/todos/done/Activacion_Validacion_PR_DLT.md` |
| Objetivos | ✅ | `objectives.md` |
| Especificación | ✅ | `spec.md` |
| **Hito 1 — Demonio sensor H1** | ✅ | `github_bridge_watcher.py` |
| **Hito 2 — Puente DLT H2** | ✅ | `dlt_bus_materializer.publish_with_retries` |
| **Hito 3 — Materialización bus H3** | ✅ | `.events/pending/<digest>.json` |
| **Hito 4 — Smoke E2E H4** | ✅ | `simulate_remote_pr.py` + smoke manifests |
| **Hito 5 — ECST + route guard** | ✅ | `pull-request-presented` v1.2 + `skipped-pre-anchored` |
| Validación Argos | ✅ | `validacion.md` APTO |

---

## 1. Hito 1 — Demonio sensor efímero (H1)

- [x] Crear `SddIA/scripts/daemons/github_bridge_watcher.py` con CLI `--once` / loop.
- [x] Integrar `env_loader.load_hierarchical_env` al arranque.
- [x] Implementar polling GitHub REST (`GITHUB_TOKEN` desde bóveda).
- [x] Persistir estado idempotente en `.SddIA/.dev/github_bridge_state.json`.
- [x] Modo lab: rama de detección simulada cuando `SDDIA_LAB_SIMULATE_REMOTE_PR=1`.
- [x] Documentar uso en `execution.md`.

**Criterio de salida:** demonio lista PRs abiertos y registra nuevos sin re-procesar digest conocido.

---

## 2. Hito 2 — Puente firma aislada (H2)

- [x] Validación ciega Filtro A: GET PR individual y comparar `head.ref`, `html_url`.
- [x] Lectura exclusiva `.SddIA/.dev/wallet.key` (fallback a `IOTA_WALLET_SECRET` vía bóveda).
- [x] Componer payload ECST pre-anclaje (§3.2 spec).
- [x] Invocar `iota-immutable-publisher` vía `dlt_bus_materializer.invoke_iota_publisher`.
- [x] Reintentos ×3 con backoff; capturar `transaction_digest` del envelope.
- [x] Dead-letter en `.events/dead-letter/` con `FALLBACK_LOCAL_SIGNATURE` si agota reintentos.

**Criterio de salida:** digest IOTA verificable en Testnet (o simulado con `SDDIA_LAB_SIMULATE_IOTA=1` en CI).

---

## 3. Hito 3 — Materialización idempotente en bus (H3)

- [x] Tras confirmación digest, escribir `.events/pending/<digest>.json`.
- [x] `event_id` = `transaction_digest`; incluir `dlt_anchor_address` en payload.
- [x] `delivery_state`: `{ "argos": "pending", "cumulo": "success" }`.
- [x] Guard idempotente: skip si archivo digest ya existe.
- [x] Módulo `dlt_bus_materializer.py`.

**Criterio de salida:** segundo ciclo sobre mismo PR no crea duplicado en pending.

---

## 4. Hito 4 — Smoke E2E desacoplado (H4)

- [x] Crear `SddIA/scripts/qa/simulate_remote_pr.py` (sin acceso wallet).
- [x] Manifest smoke: `docs/features/pull-request-automation-dlt/_smoke-remote-pr-dlt.json`.
- [x] Secuencia reproducible documentada en `validacion.md`.
- [x] Smoke aduana: `_smoke-pr-review-from-bridge.json`.

**Criterio de salida:** exit code 0 end-to-end; evento consumido; aduana 7 fases APTO.

---

## 5. Hito 5 — Evolución genómica y guard IOTA

- [x] Evolucionar `SddIA/events/pull-request-presented.md` → v1.2.0.
- [x] Añadir guard en `route_domain_event_core.py`: skip IOTA si `dlt_anchor_address` presente.
- [x] Entrada evolución `SddIA/evolution/pull-request-automation-dlt-oraculo-20260523.md`.

**Criterio de salida:** smoke remoto no dispara segunda publicación IOTA; ruta local Cursor intacta.

---

## 6. Hito 6 — Validación y cierre documental

- [x] Tekton: `implementation.md` + `execution.md`.
- [x] Argos: `validacion.md` con `global: APTO`, checks CA-1…CA-7.
- [x] Mover PBI a `docs/todos/done/` + `pbi_archived: true` en rama PR.
- [x] `delivery-close-cycle` → PR #36 → merge vía `accept-pr`.

---

## 7. Orden de commits atómicos

| # | Contenido | Estado |
|---|-----------|--------|
| 1 | docs: PBI markdown + clarify + objectives + spec + plan | ✅ |
| 2 | feat(daemon): github_bridge_watcher H1 | ✅ |
| 3 | feat(dlt): puente firma H2 + materializer H3 | ✅ |
| 4 | feat(lab): simulate_remote_pr + smoke H4 | ✅ |
| 5 | feat(eda): ECST v1.2 + route guard + validacion | ✅ |

---

## 8. Dependencias y riesgos

| Dependencia | Estado |
|-------------|--------|
| Jerarquía bóvedas (`env_loader`) | ✅ `ampliacion-configuracion-entornos` |
| Aduana `pull-request-review` v2 | ✅ `pull-request-review-redesign` |
| `iota-immutable-publisher` | ✅ operativo |
| `event-watcher` + `route-domain-event` | ✅ Ola C V3 |

| Riesgo | Mitigación |
|--------|------------|
| Testnet IOTA inestable | Fallback dead-letter + laudo manual |
| Token GH ausente en lab | Modo `SDDIA_LAB_SIMULATE_REMOTE_PR` con fixtures |
| Doble anclaje DLT | Guard `dlt_anchor_address` en route |
