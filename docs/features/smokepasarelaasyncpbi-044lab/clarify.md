---
feature_name: smokepasarelaasyncpbi-044lab
created: "2026-07-23"
updated: "2026-07-23"
process: feature
purpose: Estabilización Mayeuta — lab smoke físico pasarela async PBI-044 (evidencia S1–S3 / U1–U2); reingreso ciclico post Argos NO_APTO FAIL_EVIDENCE_GAP
branch_name: feat/smokepasarelaasyncpbi-044lab
persist_ref: docs/features/smokepasarelaasyncpbi-044lab
pbi_ref: docs/todos/done/[ARQUITECTURA] PBI-044 — Pasarela asíncrona Kalma2 y desacople por bus de eventos.md
document_id: PBI-044-SMOKE-PASARELA-ASYNC-LAB
pbi_uuid: 8c71b50f-7067-472a-a149-40041920b054
execution_id: e92ee44d-9992-4d1b-9384-b5aba5de1acc
correlation_id: e92ee44d-9992-4d1b-9384-b5aba5de1acc
prior_correlation_ids:
  - 33f4a9ee-290c-40af-8634-ae69c1445642
  - 54e86a6b-2bec-4010-8da8-ea50f2e86973
  - 6178f1d1-e1d7-4446-bc9b-fca16d79b872
  - 978397b0-c509-4678-a69c-3c69a4acaef7
  - 97af9687-41d5-4d6a-b094-bf2d4b678da8
  - ae3bba9e-ccd7-4d9a-a106-401c9897828f
  - e6bf6120-fb76-49c5-982d-b8e914e26174
phase: mayeuta-stabilization
agents: mayeuta
depends_on:
  - docs/features/kalma2-pasarela-asincrona-eda
---

# Clarificación — smokepasarelaasyncpbi-044lab

Transcript Mayeuta (2026-07-23). Semilla operador: «smoke pasarela async PBI-044 lab» + orden Raw Kernel fase Estabilización (`correlation_id` e92ee44d-…).

`persist_ref` / `pbi_ref` vacíos en inyección runtime → resueltos vía Cúmulo (`directories.documentation` / `featurePath`) y PBI-044 ya en `done/`.

---

## D0 — Apertura formal

| Pregunta | Decisión |
|----------|----------|
| Proceso | `feature` (fase Estabilización → handoff Dedalo) |
| `feature_name` | `smokepasarelaasyncpbi-044lab` |
| Rama | `feat/smokepasarelaasyncpbi-044lab` |
| `persist_ref` | `docs/features/smokepasarelaasyncpbi-044lab` |
| `document_id` | `PBI-044-SMOKE-PASARELA-ASYNC-LAB` |
| PBI padre | **PBI-044** ya en `docs/todos/done/` (`PBI-044-KALMA2-PASARELA-ASINCRONA-EDA`) |
| Feature padre | `docs/features/kalma2-pasarela-asincrona-eda` (H1+H2 código + docs; PR #146 peaje F4 RBAC NO_APTO) |
| Naturaleza ciclo | **Lab smoke / evidencia física** — no reabrir diseño ni forja de pasarela |
| Fase | Estabilización Mayeuta (esta sesión) → Dedalo blueprint de smokes lab |

---

## D1 — Diagnóstico (qué falta / qué no)

| Afirmación | Veredicto Mayeuta |
|------------|-------------------|
| Contrato H1+H2 (202/`accepted`/spawn; ceguera; ECST `Kalma2_Process_Requested`; UI poll) | **Ya especificado e implementado** en feature padre — **no reabrir** |
| Smokes S1–S3 / U1–U2 reportados en `execution.md` padre | **Históricos** — este ciclo exige **re-materializar evidencia física** en lab bajo este `persist_ref` |
| Cascada lab previa (spec/plan/impl/exec/validacion) | **Existe**; Tekton `blocked` + Argos `NO_APTO` / `FAIL_EVIDENCE_GAP` — **no invalida el qué** |
| Terminal PEC e2e (watcher+TQM completo) | **Fuera del smoke lab mínimo** (heredado nota padre) salvo laudo Racso |
| Rehabilitación F4 / Cerbero / `accept-pr` PR #146 | **Ortogonal** — no es el qué de este lab |
| waiting-for-shell / Shell async Cursor Agent | **Fuera** (Q5 padre intacto) |

**Toll:** el dolor no es «inventar pasarela»; es **cerrar el hueco de evidencia lab reproducible** (timing, correlación, status, units, ceguera estática) que Argos/PPR marcaron como frágil o no auditable cuando Shell/`git-manager`/`shell-executor` quedan Rejected.

---

## D2 — Reutilización vs invención (entropía rechazada)

| Tentación | Laudo |
|-----------|-------|
| Re-diseñar spawn/202 / segundo evento de intención | **Veto** — consumir contrato padre R1–R5 |
| Mutar bridge/handler «por si acaso» sin fallo demonstrable de smoke | **Veto** — lab = medir/auditar; forja solo si smoke falla con causa |
| Escribir PBI/TODO bajo `docs/todos/` desde Mayeuta/Tekton/Argos | **Veto** — solo Cumulo / `Kaizen_Alert_Required` |
| Absorber H3 Telegram / chat SSE / PBI-043 DI / PPR#136 F3 | **Fuera** |
| Declarar éxito sin stdout/artefacto físico de smokes | **Prohibido** — no inventar APTO |
| Relajar AC-L-* porque runtime IDE bloqueó Shell | **Veto** — el gate sigue; remediación = capacidad de ejecución, no bajar el piso |

---

## D3 — Vectores soberanos estabilizados (lab)

| ID | Qué (requisito estable) | Piso Done lab |
|----|-------------------------|---------------|
| **L-S1** | Smoke timing: N≥10 `POST /api/execute` → HTTP **202**, cuerpo `success`+`status:accepted`+`correlation_id`; p99 RTT &lt; **50 ms** | Sí |
| **L-S2** | Correlación: `event_id ≡ correlation_id` en rastro `Kalma2_Process_Requested` (o evidencia spawn correlacionado equivalente) | Sí |
| **L-S3** | `GET /api/status?event_id=<cid>` proyecta estado vivo (`pending`/`completed`/`failed` según lab); documentar si orchestration/PEC ausente | Sí |
| **L-U1** | `cargo test -p kalma2-bridge` verde (baseline padre) | Sí |
| **L-U2** | `cargo test -p execute-process` filtros kalma2 verdes (baseline padre) | Sí |
| **L-BLIND** | Audit estático: cero writes EDA nuevas desde crate `kalma2-bridge` (camino execute) | Sí |
| **L-REG** | Diff suscripciones/allowlist vs baseline: **0** cambios no justificados (AC-R3 lab) | Sí |
| **L-DOC** | Evidencia en `execution.md` + `validacion.md` de **este** `persist_ref`; fixtures `_smoke-*.json` si aplica | Sí |
| **L-PEC-E2E** | Terminal PEC completo watcher+TQM | **Fuera** mínimo (heredado) |

---

## D4 — Preguntas abiertas (laudos / handoff Dedalo)

| # | Pregunta | Laudo / default |
|---|----------|-----------------|
| **Q1** | ¿Re-archivar PBI-044? | **No** — ya en `done/`. Este lab no mueve el PBI padre. Done documental lab = `validacion.md` APTO + evidencia; PBI operativo propio solo vía Cumulo/Kaizen si Racso lo exige (**L-PBI-LOC-LAB**) |
| **Q2** | ¿N mínimo disparos timing? | Default **N=12** (precedente padre / plan lab); Dedalo puede fijar N∈[10,30] |
| **Q3** | ¿Puerto / skip-git lab? | Preferir `SDDIA_CLIENT_PORT` efímero (lab: `18765`) + `SDDIA_LAB_SKIP_GIT=1` (o equivalente documentado); Dedalo fija contrato de entorno |
| **Q4** | ¿Forja código si smoke falla? | Solo bugfix mínimo acotado al fallo; **no** reabrir H3 ni ECST |
| **Q5** | ¿Git evidencia? | Obligatorio vía `skill:git-manager` / `./sddia-run.sh --tool git-manager`; sin bypass Shell destructivo |
| **Q6** | ¿Reescribir blueprint Dedalo? | **Solo si** el plan vigente no cubre remediación de evidencia; si plan T-GATE+T1–T5 ya define smokes/units, Dedalo **reafirma** y no reinventar |

---

## D5 — Criterios de aceptación producto (mapeo AC lab)

| AC lab | Liga | Nota |
|--------|------|------|
| AC-L-S1 | L-S1 / AC-R1 padre | Timing + 202/`accepted` |
| AC-L-S2 | L-S2 / AC-R2 padre | Correlación cid≡event_id |
| AC-L-S3 | L-S3 / AC-R4 padre (proyección) | Status vivo; PEC e2e fuera mínimo |
| AC-L-U | L-U1+L-U2 | Units bridge + handler |
| AC-L-BLIND | L-BLIND / AC-R2 audit | Grep/audit no-write-bus (stdout test) |
| AC-L-REG | L-REG / AC-R3 | Sin regresión suscripciones |
| AC-L-DOC | L-DOC | Cascada lab + no inventar éxito |
| AC-DONE-LAB | cierre | `validacion.md` APTO en rama; sin re-archivo PBI-044 |

---

## D6 — Invariantes innegociables (handoff Dedalo)

1. Paths solo vía `SddIA/core/cumulo.paths.json`.
2. Bridge = aduana inerte; emisión = genoma; `correlation_id ≡ event_id`.
3. No inventar segundo evento de intención.
4. Git solo `skill:git-manager`; KM/TODOs solo Cumulo / `Kaizen_Alert_Required`.
5. Evidencia = artefacto físico en `persist_ref` / stdout capturado; ausencia = **blocked/NO_APTO**, no narrativa.
6. Lectura estática de código **no** sustituye AC-L-BLIND/AC-L-U sin captura de test.

---

## D7 — Fuera de alcance (exclusión)

Re-diseño H1+H2 · H3 Telegram · chat SSE · waiting-for-shell Cursor · rehabilitación F4 PR #146 · PBI-043 DI · PPR#136 F3 · IOTA/systemd · mutación allowlist/subscriptions · escritura `docs/todos/` por Tekton/Argos/Mayeuta · PEC e2e completo como gate mínimo · bajar AC-L-* por bloqueo RBAC IDE.

Precedentes a preservar: `kalma2-pasarela-asincrona-eda`, `kalma2-event-bus-integration`, `kalma2-process-dispatch`, `kalma2-full-cycle`.

---

## D9 — Reingreso post Argos (ciclo 6178f1d1)

| Hecho | Laudo Mayeuta |
|-------|---------------|
| `execution.md` lab: `verdict: blocked` (Shell/`shell-executor`/`git-manager` Rejected) | Toll confirmado = **capacidad de materializar**, no requisito inestable |
| `validacion.md`: `global: NO_APTO`, `resolution: FAIL_EVIDENCE_GAP` | Correcto — **no** reclasificar a APTO narrativo |
| Vectores L-S1…L-DOC | **Reafirmados** sin cambio de piso |
| Remediación | Upstream: RBAC `system-operations` + re-inyectar Ejecución/Verificación con stdout físico; Mayeuta **no** diseña el cómo |

---

## D10 — Reingreso ciclico (cid 978397b0)

| Hecho | Laudo Mayeuta |
|-------|---------------|
| Semilla idéntica: «smoke pasarela async PBI-044 lab» | Sin cambio de intención; **qué** ya termodinámicamente estable |
| Argos previo: `NO_APTO` / `FAIL_EVIDENCE_GAP` / T-GATE fail | Toll intacto — evidencia física ausente; no se baja AC-L-* |
| Plan Dedalo v1.1.0 (+T-GATE) | **Vigente** (Q6): Dedalo reafirma; no reinventar T1–T5 |
| `./sddia-run.sh --tool git-manager` esa sesión | **Rejected** (sin stdout) — evidencia git **no materializada** |
| Handoff | Dedalo: reafirmar blueprint; Tekton no arranca T1–T4 sin T-GATE Unlock |

---

## D11 — Reingreso ciclico (cid 97af9687)

| Hecho | Laudo Mayeuta |
|-------|---------------|
| Semilla idéntica: «smoke pasarela async PBI-044 lab» | Sin cambio de intención; **qué** intacto (L-S1…L-DOC) |
| Argos cid 978397b0: `NO_APTO` / `FAIL_EVIDENCE_GAP` / `T_GATE_UNLOCK: NO_APTO` | Confirmado — **no** reclasificar |
| Plan Dedalo v1.1.0 (+T-GATE) | **Sigue vigente** (Q6): Dedalo **reafirma**; no reinventar |
| `./sddia-run.sh --tool git-manager` esa sesión | **Rejected** (sin stdout) — evidencia git **no materializada**; docs sí en disco |
| `persist_ref` / `pbi_ref` inyección | Vacíos → resueltos: featurePath + PBI-044 en `done/` |
| Handoff | Dedalo: reafirmar T-GATE+T1–T5; Tekton no arranca T1–T4 sin Unlock |

---

## D12 — Reingreso ciclico (cid ae3bba9e)

| Hecho | Laudo Mayeuta |
|-------|---------------|
| Semilla idéntica: «smoke pasarela async PBI-044 lab» | Sin cambio de intención; **qué** intacto (L-S1…L-DOC) |
| Argos cid 97af9687: `NO_APTO` / `FAIL_EVIDENCE_GAP` / T-GATE+AC-L-* sin evidencia | Confirmado — **no** reclasificar |
| Plan Dedalo v1.1.0 (+T-GATE) | **Sigue vigente** (Q6): Dedalo **reafirma**; no reinventar |
| `./sddia-run.sh --tool git-manager` esa sesión | **Rejected** (sin stdout) — evidencia git **no materializada**; docs sí en disco |
| `persist_ref` / `pbi_ref` inyección | Vacíos → resueltos: `docs/features/smokepasarelaasyncpbi-044lab` + PBI-044 en `done/` |
| Handoff | Dedalo: reafirmar T-GATE+T1–T5; Tekton no arranca T1–T4 sin Unlock |

---

## D13 — Reingreso ciclico (cid e6bf6120)

| Hecho | Laudo Mayeuta |
|-------|---------------|
| Semilla idéntica: «smoke pasarela async PBI-044 lab» | Sin cambio de intención; **qué** intacto (L-S1…L-DOC) |
| Argos cid ae3bba9e: `NO_APTO` / `FAIL_EVIDENCE_GAP` / `T_GATE_UNLOCK: NO_APTO` | Confirmado — **no** reclasificar |
| Tekton cid ae3bba9e: `verdict: blocked` (`t_gate: fail`; forge=0; T1–T4 not_started) | Toll = **capacidad de materializar**, no requisito inestable |
| Plan Dedalo v1.1.0 (+T-GATE) | **Sigue vigente** (Q6): Dedalo **reafirma**; no reinventar |
| `./sddia-run.sh --tool git-manager` esa sesión | **Rejected** (sin stdout) — evidencia git **no materializada**; docs sí en disco |
| `persist_ref` / `pbi_ref` inyección | Vacíos → resueltos: `docs/features/smokepasarelaasyncpbi-044lab` + PBI-044 en `done/` |
| Handoff | Dedalo: reafirmar T-GATE+T1–T5; Tekton no arranca T1–T4 sin Unlock |

---

## D14 — Reingreso ciclico (esta sesión, cid e92ee44d)

| Hecho | Laudo Mayeuta |
|-------|---------------|
| Semilla idéntica: «smoke pasarela async PBI-044 lab» | Sin cambio de intención; **qué** intacto (L-S1…L-DOC) |
| Argos cid e6bf6120: `NO_APTO` / `FAIL_EVIDENCE_GAP` / `T_GATE_UNLOCK: NO_APTO` | Confirmado en `validacion.md` vigente — **no** reclasificar |
| Tekton cid e6bf6120: `verdict: blocked` (`t_gate: fail`; forge=0; T1–T4 not_started) | Toll = **capacidad de materializar**, no requisito inestable |
| Plan Dedalo v1.1.0 (+T-GATE) | **Sigue vigente** (Q6): Dedalo **reafirma**; no reinventar |
| `./sddia-run.sh --tool git-manager` esta sesión | **Rejected** (sin stdout) — evidencia git **no materializada**; docs sí en disco; MCP git = [] |
| `persist_ref` / `pbi_ref` inyección | Vacíos → resueltos: `docs/features/smokepasarelaasyncpbi-044lab` + PBI-044 en `done/` (lectura FS) |
| Handoff | Dedalo: reafirmar T-GATE+T1–T5; Tekton no arranca T1–T4 sin Unlock |

---

## D8 — Veredicto Mayeuta

**ok** — requisitos lab termodinámicamente estables (L-S1…L-DOC) y **reafirmados** tras NO_APTO por hueco de evidencia (reingreso cid e92ee44d). Handoff a Dedalo: **reafirmar** blueprint T-GATE+T1–T5 de captura de evidencia; sin reabrir pasarela salvo fallo demonstrable (Q4); sin relajar AC-L-*.

Pendiente Dedalo (cómo, no qué): secuencia lab ejecutable bajo RBAC habilitado; N exacto; puerto; captura p99; paths artefactos `_smoke-*` / logs; gates Argos = evidencia física.

**Git esta fase:** `./sddia-run.sh --tool git-manager` → **Rejected** — sin stdout; no inventar evidencia git.
