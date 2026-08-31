---
document_id: AUDIT-KALMA2-FORGE-EMAIL-WATCHER-KEEPALIVE-HALT-20260830
uuid: "6f668e7c-bd33-4e7c-bda2-d9cb39fbe343"
title: "Auditoría — forja Kalma2 bug-fix email-watcher-heartbeat-keepalive: detención post-plan"
created: "2026-08-31"
process: bug-fix
branch_name: fix/email-watcher-heartbeat-keepalive
persist_ref: docs/fixes/email-watcher-heartbeat-keepalive
pbi_ref: docs/todos/pending/[FIX] email-watcher — fractura sistémica (6c0db1296181).md
document_id_pbi: PBI-FIX-FRACTURE-6c0db1296181
execution_id: "9dbcfea6-4df8-47ac-873a-cf9bce846929"
correlation_id: "17546079-3b13-4c21-9e9a-486ee3fec1a3"
pec_event_id: "6776b2b7-2cee-4a3a-80a3-b01502b628ba"
dedalo_transcript: "1fbbe079-75f9-4e91-889b-514bd9c2859b"
verdict: CYCLE_FAILED_AFTER_DESIGN_NO_COMMIT_TEKTON_DNS
kaizen_pbi: docs/todos/pending/[KAIZEN] Kalma2 agent-runtime — veredicto blocked, DNS y halt-after-phase (a9fe100f).md
kaizen_pbi_uuid: "a9fe100f-f0e3-4871-83b2-295862650f5c"
related:
  - SddIA/library/codexes/codex-software-engineering/process/bug-fix.md
  - SddIA/engine/execute-process/src/engine/handlers/task_queue_manager.rs
  - SddIA/scripts/tools/kalma2-agent-runtime-cursor.py
  - docs/todos/done/[FEATURE] kalma2-full-cycle — runtime de agentes y semántica de cierre (527007fa).md
  - docs/features/kalma2-process-dispatch/spec.md
---

# Auditoría — forja Kalma2 `email-watcher-heartbeat-keepalive`

Estímulo (Vértice Biológico): petición de forja Kalma2 con texto dual; el ciclo **parece** detenerse tras forjar la planificación. Evidencia: rama actual `fix/email-watcher-heartbeat-keepalive` @ `5e40ed9` (= `main`), `persist_ref` untracked, cero parche Rust, cero PR.

## 0. Veredicto

El ciclo **sí arrancó** (TQM → `bug-fix` v1.4.2, workspace-init + Dedalo). **No se detuvo por el mandato «detente».** Dedalo forjó `spec.md`+`plan.md`, declaró `blocked` (Shell castrado → sin commit) y el runtime **mintió `executed`**. El orquestador lanzó Tekton **1 s después**. Tekton **no arrancó** (`getaddrinfo ENOTFOUND api2.cursor.sh`). Barrera de fase saltó el resto. PEC `cycle_phase: failed`.

El «después… PR con tests en verde» **no es ejecutable** bajo el laudo L2 de TQM: con `correlation_id` y sin `SDDIA_TQM_FULL_CYCLE=1` el hijo recibe `SDDIA_LAB_SKIP_DELIVERY_CLOSE=1`. Aunque Tekton hubiera vivido, no habría PR.

## 1. Identidad del ciclo

| Campo | Valor |
|-------|--------|
| Proceso hijo | `bug-fix` |
| `execution_id` | `9dbcfea6-4df8-47ac-873a-cf9bce846929` |
| `correlation_id` | `17546079-3b13-4c21-9e9a-486ee3fec1a3` |
| PEC | `.SddIA/proofs/pec-correlation/17546079-3b13-4c21-9e9a-486ee3fec1a3.json` — `cycle_phase: failed` @ `2026-08-30T18:11:09Z` |
| Workspace | `.SddIA/workspaces/bug-fix/9dbcfea6-4df8-47ac-873a-cf9bce846929/` — **vacío** (mkdir 20:09 CEST) |
| Rama | `fix/email-watcher-heartbeat-keepalive` local, **sin tracking remoto**, HEAD = `main` (`5e40ed9`) |
| PBI | `PBI-FIX-FRACTURE-6c0db1296181` sigue en `docs/todos/pending/` |
| Código | `SddIA/daemons/email-watcher/src/main.rs` **sin** `spawn_heartbeat_worker` / `HEARTBEAT_TICK` |

## 2. Texto de petición (semilla)

Literal inyectado (vía TQM `build_child_inputs` → `bug_summary`):

```text
docs/todos/pending/[FIX] email-watcher — fractura sistémica (6c0db1296181).md
implementa el pbi adjunto y detente una vez forjado plan y realizado commit de los cambios.
despues
Ejecuta planificación hasta haber forjado PR con test en verde.
```

Dos mandatos en un solo estímulo. Kalma2 no tiene operador de **halt-after-phase**. Concatena prompt + `pbi_body` y despacha **un** `bug-fix` completo (fases 1→6 del contrato).

## 3. Cronología (CEST = UTC+2)

| Reloj | Actor | Hecho |
|-------|--------|--------|
| 20:09:04 | `workspace_init` | Crea `objectives.md` (dump del PBI como «Misión»). Checkout rama. Workspace vacío. |
| 20:09 | `kalma2-agent-runtime-cursor` / Dedalo | Sesión [`1fbbe079…`](1fbbe079-75f9-4e91-889b-514bd9c2859b). `role_brief`: spec+plan. Semilla incluye el texto dual. |
| 20:10:19 | Dedalo | Escribe `spec.md` (laudo A, CA1–CA6, paridad telegram-watcher). |
| 20:10:39 | Dedalo | Escribe `plan.md` (fases Tekton 1–6). |
| 20:10–20:11 | Dedalo | Shell rechaza `git-manager`, `git status`, incluso `true`. Write sí funciona. |
| 20:11:08Z | Runtime | Handoff Dedalo `status: executed` (CLI exit 0). Transcript: **Veredicto: blocked**. |
| 20:11:09Z | Runtime | Inyecta Tekton. Falla spawn: `Error: [unavailable] getaddrinfo ENOTFOUND api2.cursor.sh`. Handoff `status: failed`. **Sin transcript de Tekton.** |
| 20:11:09Z | `execute-process` | `agent_phase_blocks_downstream("failed")` → Argos / cierre documental / DCC = `skipped` (`phase-barrier`). PEC failed. |

Duración Dedalo ≈ 2 min. Tekton = 0 s de trabajo de dominio.

## 4. Cadena causal

```text
POST/estímulo Kalma2
  → Kalma2_Process_Requested
  → task-queue-manager (process=bug-fix, pbi_ref extraído del path)
  → build_child_inputs: seed = "## Prompt operador\n{dual}\n## PBI adjunto\n{body}"
  → child_env_for_kalma2: SKIP_PBI_ARCHIVE + SKIP_DELIVERY_CLOSE  (L2; sin TQM_FULL_CYCLE)
  → bug-fix
      1. workspace-init     executed (rama + objectives)
      2. Dedalo             CLI ok → runtime "executed"  ≠ agente "blocked"
      3. Tekton             failed DNS api2.cursor.sh
      4–6. Argos / archive / DCC   skipped (barrier)
  → PEC cycle_phase=failed
```

## 5. Estado residual (rama actual)

| Artefacto | Estado |
|-----------|--------|
| `objectives.md` | Presente. Misión = PBI completo (no destilado). |
| `spec.md` | Presente. Diseño correcto (keepalive 10 s, `--once` sin hilo). **Untracked.** |
| `plan.md` | Presente. Blueprint Tekton 1–6. **Untracked.** |
| `_agent_handoff.md` | Dedalo `executed` + Tekton `failed`. **Untracked.** |
| `implementation.md` / `execution.md` / `validacion.md` | **Ausentes.** |
| Commit de diseño | **No existe.** |
| PR | **No existe.** |
| Keepalive en cápsula | **Ausente.** |

## 6. Fricciones

| ID | Capa | Fricción | Efecto en este ciclo |
|----|------|----------|---------------------|
| **F1** | Prompt operador | Dos mandatos (halt-after-plan **y** continuar a PR) en un solo `task_text`. TQM no parsea fases ni «detente»/«después». | Dedalo obedece el halt local. El proceso no. El operador ve «se paró en el plan». |
| **F2** | `kalma2-agent-runtime-cursor.py` | Pide `veredicto (ok\|blocked)` pero mapea **solo** éxito CLI → `data.status=executed`. No lee el veredicto del agente. | `blocked` (sin commit) no activa `agent_phase_blocks_downstream`. Tekton se lanza igual. |
| **F3** | Runtime Cursor (sesión Dedalo) | Shell castrado (incluso `true`). Write de markdown OK. | Diseño materializado; **invariante de commit vía `skill:git-manager` imposible**. Dedalo no inventó éxito. |
| **F4** | Runtime Cursor (sesión Tekton) | `getaddrinfo ENOTFOUND api2.cursor.sh`. | Tekton no existe como sesión. Cero `implementation.md`. |
| **F5** | `is_soft_config_error` | Marcadores: `"not found"` (con espacio). `ENOTFOUND` **no** matchea. DNS → `failed` duro, no `awaiting_agents`. | Ciclo terminal. Sin reintento. Clase emparentada con discriminación DNS DCC (`d0cfd5b66ff1`), **no** aplicada al agent-runtime. |
| **F6** | TQM L2 | `correlation_id` + ausencia `SDDIA_TQM_FULL_CYCLE` → skip archive + DCC. Laudo documentado: *arranque de ciclo, no auto-PR*. | El segundo mandato («forja PR») es **estructuralmente inalcanzable** en este path. |
| **F7** | `workspace_init` | `objectives.md` «Misión» = `pbi_body` crudo. | Dedalo consume un dump, no un manifiesto. |
| **F8** | `build_prompt` | `seed[:8000]` + `pbi_body[:12000]`. El PBI viaja **duplicado**; la semilla se corta a mitad de §5 en el transcript Dedalo. | Ruido + truncación. El `pbi_body` posterior salvó el diseño. |
| **F9** | Workspace | Plantilla materializa directorio; **cero** `phase_reports` / state en disco. | Auditoría posterior solo vía PEC + handoff + transcript IDE. |
| **F10** | Contrato `bug-fix` vs semilla | Contrato: Dedalo → Tekton → Argos → DCC. Semilla: «detente tras plan+commit». | Colisión de autoridad: el `role_brief` gana sobre el halt; el CLI-ok gana sobre el blocked. |

## 7. Qué **no** ocurrió

- No hay fractura Kintsugi nueva de este ciclo en `docs/todos/pending/` (PEC failed ≠ `System_Fracture_Detected` de proceso colapsado en el sentido del protocolo).
- No hay bypass raw `git commit` / `gh pr create` (Dedalo no pudo ni `true`).
- El diseño Dedalo es coherente con el PBI v1.1.0 (laudo A, paridad centinelas, alcance prohibido respetado). El fallo es de **orquestación/runtime**, no de especificación.

## 8. Semillas Kaizen

Materializadas en `docs/todos/pending/[KAIZEN] Kalma2 agent-runtime — veredicto blocked, DNS y halt-after-phase (a9fe100f).md` (`PBI-KAIZEN-KALMA2-AGENT-VERDICT-BARRIER`).

| Semilla | Ola PBI |
|---------|---------|
| Parser `ok\|blocked` → barrera | A |
| DNS agent-runtime ≠ `failed` | A |
| `stop_after` design/commit | B |
| L2 vs mandato PR (acuse honesto o full-cycle) | B |
| `objectives` destilado + prompt un canal | C |
| `phase_reports.json` en workspace | C |

## 9. Retoma (fuera de esta auditoría)

El `persist_ref` está listo para Tekton (spec+plan). Falta: commit de diseño vía `git-manager`, parche keepalive, tests, cascada documental, DCC **con** `SDDIA_TQM_FULL_CYCLE` si el laudo es auto-PR. No re-forjar spec.
