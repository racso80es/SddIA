---
document_id: PBI-KAIZEN-KALMA2-AGENT-VERDICT-BARRIER
uuid: "a9fe100f-f0e3-4871-83b2-295862650f5c"
title: "[KAIZEN] Kalma2 agent-runtime — veredicto blocked, DNS y halt-after-phase"
format: markdown
version: "1.0.0"
created: "2026-08-31"
updated: "2026-08-31"
status: "abierto"
priority: alta
process: feature
type: kaizen
dispatch: true
source_audit: docs/audits/kalma2-forge-email-watcher-keepalive-halt-20260830.md
incident_ref: "bug-fix 9dbcfea6-4df8-47ac-873a-cf9bce846929 / correlation 17546079-3b13-4c21-9e9a-486ee3fec1a3"
suggested_branch: feat/kalma2-agent-verdict-barrier
persist_ref_suggested: docs/features/kalma2-agent-verdict-barrier
architectural_constraints:
  - A-NO-MENTIR-EXECUTED
  - A-DNS-NO-ES-COLAPSO
  - A-L2-HONESTO-SIN-FULL-CYCLE
  - A-NO-MUTAR-GENOMA-SIN-ENTITY-MANAGER
related_pbis:
  - id: PBI-KALMA2-FULL-CYCLE-RUNTIME
    rol: "Antecesor slice B: runtime existe; no parsea veredicto ni DNS. Deuda L2 abierta."
  - id: PBI-FIX-FRACTURE-6c0db1296181
    rol: "Ciclo víctima (email-watcher keepalive). Fuera: no re-forjar spec ni parche daemon en este PBI."
  - id: PBI-FIX-FRACTURE-d0cfd5b66ff1
    rol: "Paridad F4c DCC (DNS → blocked). Replicar taxonomía en agent-runtime, no en DCC."
related:
  - docs/audits/kalma2-forge-email-watcher-keepalive-halt-20260830.md
  - SddIA/scripts/tools/kalma2-agent-runtime-cursor.py
  - SddIA/engine/execute-process/src/engine/executor.rs
  - SddIA/engine/execute-process/src/engine/handlers/task_queue_manager.rs
  - SddIA/engine/execute-process/src/engine/workspace_init.rs
  - SddIA/engine/execute-process/src/engine/delivery_close.rs
  - docs/features/kalma2-process-dispatch/spec.md
  - docs/todos/done/[FEATURE] kalma2-full-cycle — runtime de agentes y semántica de cierre (527007fa).md
---

# [KAIZEN] Kalma2 agent-runtime — veredicto blocked, DNS y halt-after-phase

Auditoría: `docs/audits/kalma2-forge-email-watcher-keepalive-halt-20260830.md`. Ciclo `9dbcfea6` forjó spec+plan, mintió `executed`, Tekton DNS, PEC `failed`. El PBI de keepalive (`6c0db1296181`) **no** se implementa aquí.

## 1. Síntoma

| Expectativa | Resultado `9dbcfea6` |
|-------------|------------------------|
| Dedalo `blocked` (sin commit) corta Tekton | Runtime `executed` → Tekton 1 s después |
| DNS `api2.cursor.sh` = red transitoria | `failed` duro; barrera mata Argos/DCC |
| «detente tras plan+commit» **luego** «PR en verde» | Un solo `bug-fix`; halt no es input; L2 impide PR |

## 2. Causa (código)

### F2 — Veredicto ignorado

`kalma2-agent-runtime-cursor.py` `run_agent_phase`: CLI exit 0 → `status = "executed"`. El prompt exige `veredicto (ok|blocked)`. No hay parser. `executor.rs` `agent_phase_blocks_downstream("blocked")` ya existe y **no se dispara**.

### F5 — DNS no es soft

`is_soft_config_error`: token `"not found"` (con espacio). `ENOTFOUND` / `getaddrinfo` no matchean. Paridad F4c (`dcc_transient_network_trace`) **ausente** en la prótesis.

### F1/F10 — Halt no es contrato

TQM concatena `task_text` dual en `bug_summary`. Cero `stop_after`. `role_brief` Dedalo gana; el proceso sigue.

### F6 — L2 vs mandato PR

`child_env_for_kalma2`: con `correlation_id` y sin `SDDIA_TQM_FULL_CYCLE` → `SDDIA_LAB_SKIP_DELIVERY_CLOSE=1`. Pedir PR en el prompt **no** activa full-cycle. Acuse no declara el skip.

## 3. Alcance

### Dentro

| Ola | Target (no DA-2) | Cambio |
|-----|------------------|--------|
| **A** | `kalma2-agent-runtime-cursor.py` | Parser de veredicto. CLI ok + `blocked` → `data.status=blocked`, `success` coherente con barrera. Tests/unit del parser. |
| **A** | mismo fichero | DNS/red: `enotfound`, `getaddrinfo`, `eai_again`, `could not resolve host` → `awaiting_agents` (o `blocked` si `REQUIRE_CLI`). No `failed`. Tests del predicado. |
| **B** | `task_queue_manager.rs` + `executor.rs` | Input/env `stop_after` ∈ {`design`, `commit`, unset}. Tras fase Dedalo (`Diseño del fix` / equivalente feature): si `design`, barrera **aunque** status `executed`. `commit` = halt solo si git-manager commit OK en esa fase. Sin mutar YAML de `bug-fix.md` (genoma). |
| **B** | TQM | Heurística o flag: texto pide PR **y** no hay `stop_after` que corte antes de DCC → o bien `SDDIA_TQM_FULL_CYCLE=1` en el hijo, o bien envelope `delivery_close: skipped_l2` explícito en acuse (nunca PEC de negocio `completed` fingiendo PR). |
| **C** | `workspace_init.rs` + `build_prompt` | `objectives.md` misión destilada (no dump `pbi_body`). Prompt: un canal PBI (no `seed[:8000]` + body duplicado). Persistir `phase_reports.json` en `workspaces/{process}/{execution_id}/`. |

### Fuera

- Parche keepalive `email-watcher` / retoma `6c0db1296181`.
- Castrar/restaurar Shell del host Cursor (F3): producto IDE, no Core.
- Mutar `SddIA/process/`, `library/codexes/**/process/bug-fix.md`, events, norms → `entity-manager`.
- Derogar L2 en silencio sin acuse. Activar full-cycle sobre ciclo vacío (laudo 527007fa).
- `iota-publish-relay`, umbrales Argos, fagoctio.

## 4. Criterios de aceptación

- [ ] **CA-A1** Fixture transcript con `Veredicto: blocked` + CLI 0 → JSON `data.status=blocked`. `executor` no invoca la fase agente siguiente.
- [ ] **CA-A2** Fixture `getaddrinfo ENOTFOUND api2.cursor.sh` → `awaiting_agents` o `blocked`; **no** `failed`. Argos/DCC skipped por barrera, no por colapso.
- [ ] **CA-B1** `stop_after=design` (env o input TQM): tras Dedalo `executed` con spec+plan, Tekton no arranca. PEC `cycle_phase` ≠ completed de negocio.
- [ ] **CA-B2** Prompt que pide PR sin `TQM_FULL_CYCLE`: o el hijo corre DCC, o el acuse declara `skipped_l2` (campo estable). Nunca UI/PEC que implique PR forjado.
- [ ] **CA-C1** `objectives.md` no incluye el cuerpo YAML completo del PBI; `pbi_ref` en frontmatter.
- [ ] **CA-C2** Workspace del `execution_id` contiene `phase_reports.json` al terminar (éxito o barrera).

## 5. Orden

```text
Ola A (veredicto + DNS)  →  desbloquea barrera real
  → Ola B (stop_after + honestidad L2)
    → Ola C (prompt/objectives/workspace)  — no bloquea A/B
```

## 6. Cierre

Un PR. `validacion.md` APTO, `pbi_archived: true`, este fichero en `docs/todos/done/` en la misma rama. Git vía `skill:git-manager`. Genoma solo vía `entity-manager` si Dedalo demuestra que B exige input en el contrato de proceso (preferir env/TQM).
