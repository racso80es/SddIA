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
| PBI markdown | ✅ | `docs/todos/pending/Activacion_Validacion_PR_DLT.md` |
| Objetivos | ✅ | `objectives.md` |
| Especificación | ✅ | `spec.md` |
| **Hito 1 — Demonio sensor H1** | ⏳ | `github_bridge_watcher.py` |
| **Hito 2 — Puente DLT H2** | ⏳ | Firma + `iota-immutable-publisher` |
| **Hito 3 — Materialización bus H3** | ⏳ | Idempotencia digest → pending |
| **Hito 4 — Smoke E2E H4** | ⏳ | `SDDIA_LAB_SIMULATE_REMOTE_PR=1` |
| **Hito 5 — ECST + route guard** | ⏳ | `pull-request-presented` v1.2 + skip IOTA |
| Validación Argos | ⏳ | `validacion.md` |

---

## 1. Hito 1 — Demonio sensor efímero (H1)

- [ ] Crear `SddIA/scripts/daemons/github_bridge_watcher.py` con CLI `--once` / loop.
- [ ] Integrar `env_loader.load_hierarchical_env` al arranque.
- [ ] Implementar polling GitHub REST (`GITHUB_TOKEN` desde bóveda).
- [ ] Persistir estado idempotente en `.SddIA/.dev/github_bridge_state.json`.
- [ ] Modo lab: rama de detección simulada cuando `SDDIA_LAB_SIMULATE_REMOTE_PR=1`.
- [ ] Documentar uso en `execution.md` (post-implementación).

**Criterio de salida:** demonio lista PRs abiertos y registra nuevos sin re-procesar digest conocido.

---

## 2. Hito 2 — Puente firma aislada (H2)

- [ ] Validación ciega Filtro A: GET PR individual y comparar `head.ref`, `html_url`.
- [ ] Lectura exclusiva `.SddIA/.dev/wallet.key` (fallback a `IOTA_WALLET_SECRET` vía bóveda si unificado).
- [ ] Componer payload ECST pre-anclaje (§3.2 spec).
- [ ] Invocar `iota-immutable-publisher` reutilizando patrón `_invoke_iota_publisher`.
- [ ] Reintentos ×3 con backoff; capturar `transaction_digest` del envelope.
- [ ] Dead-letter en `.events/dead-letter/` con `FALLBACK_LOCAL_SIGNATURE` si agota reintentos.

**Criterio de salida:** digest IOTA verificable en Testnet (o simulado con `SDDIA_LAB_SIMULATE_IOTA=1` en CI).

---

## 3. Hito 3 — Materialización idempotente en bus (H3)

- [ ] Tras confirmación digest, escribir `.events/pending/<digest>.json`.
- [ ] `event_id` = `transaction_digest`; incluir `dlt_anchor_address` en payload.
- [ ] `delivery_state`: `{ "argos": "pending", "cumulo": "success" }`.
- [ ] Guard idempotente: skip si archivo digest ya existe.
- [ ] Opcional: extraer lógica a `dlt_bus_materializer.py` si watcher > 200 LOC.

**Criterio de salida:** segundo ciclo sobre mismo PR no crea duplicado en pending.

---

## 4. Hito 4 — Smoke E2E desacoplado (H4)

- [ ] Crear `SddIA/scripts/qa/simulate_remote_pr.py` (sin acceso wallet).
- [ ] Manifest smoke: `docs/features/pull-request-automation-dlt/_smoke-remote-pr-dlt.json`.
- [ ] Secuencia reproducible:
  1. `SDDIA_LAB_SIMULATE_REMOTE_PR=1`
  2. `simulate_remote_pr.py`
  3. `github_bridge_watcher.py --once`
  4. `event-watcher.py --once`
  5. Verificar invocación `pull-request-review` en logs / `execution_report`
- [ ] Documentar comandos en `validacion.md`.

**Criterio de salida:** exit code 0 end-to-end; evento en `processed/` o pending consumido con aduana ejecutada.

---

## 5. Hito 5 — Evolución genómica y guard IOTA

- [ ] Evolucionar `SddIA/events/pull-request-presented.md` → v1.2.0 (payload ampliado).
- [ ] Actualizar `SddIA/events/index.md` y recalcular hash.
- [ ] Añadir guard en `route_domain_event_core.py`: skip IOTA si `dlt_anchor_address` presente.
- [ ] Entrada evolución `SddIA/evolution/pull-request-automation-dlt-oraculo-20260523.md`.
- [ ] Verificar `event-subscriptions.json` sin doble anclaje en ruta remota.

**Criterio de salida:** smoke remoto no dispara segunda publicación IOTA; ruta local Cursor intacta.

---

## 6. Hito 6 — Validación y cierre documental

- [ ] Tekton: `implementation.md` + `execution.md`.
- [ ] Argos: `validacion.md` con `global: APTO`, checks CA-1…CA-7.
- [ ] Mover PBI a `docs/todos/done/` + `pbi_archived: true` en rama PR.
- [ ] `delivery-close-cycle` → PR único → merge vía `accept-pr`.

---

## 7. Orden de commits atómicos

| # | Contenido |
|---|-----------|
| 1 | docs: PBI markdown + clarify + objectives + spec + plan |
| 2 | feat(daemon): github_bridge_watcher H1 |
| 3 | feat(dlt): puente firma H2 + materializer H3 |
| 4 | feat(lab): simulate_remote_pr + smoke H4 |
| 5 | feat(eda): ECST v1.2 + route guard + validacion |

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
