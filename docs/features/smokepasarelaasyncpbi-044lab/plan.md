---
feature_name: smokepasarelaasyncpbi-044lab
created: "2026-07-23"
updated: "2026-07-23"
process: feature
phases: [T-GATE, T0, T1, T2, T3, T4, T5]
document_id: PBI-044-SMOKE-PASARELA-ASYNC-LAB
pbi_uuid: 8c71b50f-7067-472a-a149-40041920b054
branch_name: feat/smokepasarelaasyncpbi-044lab
persist_ref: docs/features/smokepasarelaasyncpbi-044lab
correlation_id: e92ee44d-9992-4d1b-9384-b5aba5de1acc
prior_correlation_ids:
  - 33f4a9ee-290c-40af-8634-ae69c1445642
  - 54e86a6b-2bec-4010-8da8-ea50f2e86973
  - 6178f1d1-e1d7-4446-bc9b-fca16d79b872
  - 978397b0-c509-4678-a69c-3c69a4acaef7
  - 97af9687-41d5-4d6a-b094-bf2d4b678da8
  - ae3bba9e-ccd7-4d9a-a106-401c9897828f
  - e6bf6120-fb76-49c5-982d-b8e914e26174
phase: Diseño de Blueprint
agents: dedalo
version_plan: "1.1.0"
reentry: "Reafirmación D14 post FAIL_EVIDENCE_GAP — T-GATE+T1–T5 intactos; sin reinventar (Q6)"
---

# Plan — smokepasarelaasyncpbi-044lab

Blueprint de **ejecución y captura de evidencia** lab (smokes/units/auditorías). No reabre pasarela salvo fallo demonstrable (spec L7).

**Q6 / D14:** plan T-GATE+T1–T5 ya cubría remediación de evidencia; Dedalo **reafirma** sin reinventar vectores L-S*/L-U*/BLIND/REG. Unlock RBAC sigue siendo precondición dura.

## Fases

### T-GATE — Unlock runtime (preflight)
- name: Unlock runtime RBAC
- intent: Verificar que el ejecutor puede invocar `skill:shell-executor` y `skill:git-manager` sin Rejected antes de materializar evidencia.
- delegates_to:
  - skill:shell-executor
  - skill:git-manager
- checklist:
  - [ ] `target_executor_rbac.allowed_policies` incluye `system-operations`, `filesystem-ops`, `source-control`
  - [ ] Smoke cápsula: `./sddia-run.sh --tool git-manager` JSON `{"action":"status",...}` → stdout físico
  - [ ] Smoke shell: comando trivial vía `shell-executor` (p. ej. `true` / `pwd`) → no Rejected
  - [ ] Si alguno falla → **stop**; `execution.md` `verdict: blocked` + `block_reason`; **no** intentar T1–T4 narrando éxito

### T0 — Documentación Dedalo
- [x] Consumir `objectives.md` / `clarify.md` (D0–D14, Q1–Q6)
- [x] `spec.md` v1.1.0 laudos L1–L12 + contratos lab (reafirmado cid e92ee44d)
- [x] este `plan.md` v1.1.0 (+ T-GATE; reafirmado D14)

### T1 — Prep entorno lab
- name: Prep entorno lab
- intent: Compilar bridge (+ motor si L-U2) y fijar env `SDDIA_CLIENT_PORT=18765`, `SDDIA_LAB_SKIP_GIT=1`.
- delegates_to:
  - skill:shell-executor
  - skill:filesystem-manager
- checklist:
  - [ ] `cargo build -p kalma2-bridge` (y `execute-process` si hace falta)
  - [ ] Verificar binario `SddIA/target/debug/kalma2-bridge`
  - [ ] Fixture `_smoke-timing-execute.json` bajo `persist_ref` (si ausente/incoherente L4)
  - [ ] Liberar/elegir puerto efímero; documentar valor

### T2 — Smokes HTTP L-S1 / L-S2 / L-S3
- name: Smokes pasarela async
- intent: Materializar evidencia física timing, correlación y status sin await del ciclo en el POST.
- delegates_to:
  - skill:shell-executor
  - skill:filesystem-manager
- checklist:
  - [ ] Arrancar bridge en `$SDDIA_CLIENT_PORT`
  - [ ] **L-S1:** N=12 `POST /api/execute` → 202/`accepted`/cid; p99 RTT &lt; 50 ms; capturar métricas
  - [ ] **L-S2:** localizar `Kalma2_Process_Requested` en `eda_fractal.domain` con `event_id≡cid`
  - [ ] **L-S3:** `GET /api/status?event_id=<cid>` proyección viva; documentar techo si sin PEC
  - [ ] Parar bridge; no dejar proceso zombie

### T3 — Units baseline L-U1 / L-U2
- name: Units bridge y handler
- intent: Re-ejecutar baseline padre y capturar stdout.
- delegates_to:
  - skill:shell-executor
- checklist:
  - [ ] **L-U1:** `cargo test -p kalma2-bridge` verde
  - [ ] **L-U2:** `cargo test -p execute-process kalma2` verde
  - [ ] Persistir resumen en `execution.md`

### T4 — Auditorías L-BLIND / L-REG
- name: Ceguera y regresión nervio
- intent: Unit/audit no-write-bus con stdout + diff suscripciones vs `main` = 0 injustificado.
- delegates_to:
  - skill:shell-executor
  - skill:git-manager
  - skill:filesystem-manager
- checklist:
  - [ ] **L-BLIND:** ejecutar unit `bridge_execute_path_has_no_eda_write_helpers` (o audit crate con stdout); **no** sustituir por Grep IDE
  - [ ] **L-REG:** diff `event-domain-subscriptions.json` + `event-orchestration-subscriptions.json` vs `main` vía git-manager
  - [ ] Evidencia git preferente vía `./sddia-run.sh --tool git-manager` (status/diff); si Rejected → blocked honesto

### T5 — Cascada documental + handoff Argos
- name: Persistencia evidencia
- intent: Cerrar `implementation.md` + `execution.md` con evidencia real; no inventar APTO.
- delegates_to:
  - skill:filesystem-manager
  - skill:git-manager
- checklist:
  - [ ] `implementation.md` (forja=0 → baseline intacto; o bugfix mínimo L7)
  - [ ] `execution.md` tabla L-S1..L-REG + comandos + cid smoke (+ resultado T-GATE)
  - [ ] **No** mover PBI-044; **no** sembrar `docs/todos/`
  - [ ] Handoff Argos → `validacion.md` (AC-L-* / AC-DONE-LAB)

### T-FIX (condicional) — Bugfix mínimo
- name: Bugfix acotado a fallo smoke
- intent: Solo si T2/T3 fallan con causa demostrable; diff mínimo; no H3/ECST/allowlist/F4.
- delegates_to:
  - skill:filesystem-manager
  - skill:shell-executor
- checklist:
  - [ ] Reproducir fallo + causa en `execution.md`
  - [ ] Parche mínimo → re-correr vector fallido
  - [ ] Si no hay causa/código: **no forjar**; verdict blocked

## Orden de ejecución

```text
T-GATE (Unlock RBAC)
  → ok → T1 (build/env) → T2 (smokes HTTP) → T3 (units) → T4 (audits) → T5 (docs)
  → fail → execution.md blocked + stop (sin narrar AC-L-*)
                 ↘ T-FIX solo si fallo smoke demonstrable (no por Rejected RBAC)
```

T3 puede paralelizarse tras T1 si el lab lo permite; T2 requiere bridge up. T5 es gate documental antes de Argos.

## Delegación / RBAC (ejecutor Tekton)

| Fase | Cápsulas | Políticas mínimas | Notas |
|------|----------|-------------------|-------|
| T-GATE | `skill:shell-executor`, `skill:git-manager` | `system-operations`, `source-control` | Preflight; abort si Rejected |
| T1–T3 | `skill:shell-executor`, `skill:filesystem-manager` | `system-operations`, `filesystem-ops` | Build/test/HTTP lab |
| T4 | + `skill:git-manager` | `source-control` | Diff suscripciones / status |
| T5 | `skill:filesystem-manager`, `skill:git-manager` | `filesystem-ops`, `source-control` | Docs + evidencia git |
| Genoma indexado | — | — | Sin `entity-manager` (no previsto) |
| KM / `docs/todos/` | Solo Cumulo / `Kaizen_Alert_Required` | — | Tekton no siembra TODOs |

Si `target_executor_rbac` del runtime **no** incluye `system-operations`, T-GATE falla: abortar con causa — **prohibido** bypass raw destructivo inventando éxito.

Cruce mecánico Dedalo (contextos cápsula vs políticas):

| Cápsula | `context` YAML | Requiere en ejecutor |
|---------|----------------|----------------------|
| `skill:shell-executor` | `system-operations` | sí |
| `skill:filesystem-manager` | `filesystem-ops` | sí |
| `skill:git-manager` | `source-control` | sí |

## Criterios de salida por fase

| Fase | Done local |
|------|------------|
| T-GATE | stdout físico shell + git-manager; o blocked honesto |
| T1 | Binario bridge listo; env documentado |
| T2 | Evidencia física S1+S2+S3 en stdout/artefacto capturable |
| T3 | Units verdes con captura |
| T4 | BLIND (unit stdout) + REG materializados; git-manager o blocked explícito |
| T5 | `execution.md` + `implementation.md` listos para Argos |

## Riesgos operativos

| Riesgo | Mitigación |
|--------|------------|
| IDE Rejected en shell/git-manager | T-GATE → blocked; no APTO narrativo |
| Puerto ocupado | Rotar `SDDIA_CLIENT_PORT`; anotar |
| p99 ruidoso | Warm-up; N=12; excluir cold-start |
| PEC ausente | S3 APTO con techo documentado (spec L6) |
| Re-uso de Grep como BLIND | Veto (spec L12); exige unit |

## Explicitamente no planificado

Re-diseño pasarela · H3 Telegram · F4/Cerbero PR #146 · DI PBI-043 · PEC e2e como gate · mutación subscriptions · re-archivo PBI-044 · semillas `docs/todos/` desde Tekton · bajar AC-L-* por Rejected.
