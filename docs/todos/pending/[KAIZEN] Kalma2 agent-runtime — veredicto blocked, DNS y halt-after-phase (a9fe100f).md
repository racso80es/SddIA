---
document_id: PBI-KAIZEN-KALMA2-AGENT-VERDICT-BARRIER
uuid: "a9fe100f-f0e3-4871-83b2-295862650f5c"
title: "[KAIZEN] Kalma2 agent-runtime — veredicto blocked, DNS y halt-after-phase"
format: markdown
version: "1.1.0"
created: "2026-08-31"
updated: "2026-08-31"
status: "abierto"
refined: true
refinement_status: "refinado"
priority: alta
process: feature
type: kaizen
dispatch: true
source_audit: docs/audits/kalma2-forge-email-watcher-keepalive-halt-20260830.md
incident_ref: "bug-fix 9dbcfea6-4df8-47ac-873a-cf9bce846929 / correlation 17546079-3b13-4c21-9e9a-486ee3fec1a3"
suggested_branch: feat/kalma2-agent-verdict-barrier
persist_ref_suggested: docs/features/kalma2-agent-verdict-barrier
review_notes: "v1.1.0 — el v1.0.0 omitía el normalizador de agent_runtime.rs (blocked→executed), confundía taxonomía F4c DCC con tokens Node ENOTFOUND, atribía phase_reports a workspace_init, y proponía TQM_FULL_CYCLE por heurística NL en contradicción con el laudo 527007fa."
architectural_constraints:
  - A-NO-MENTIR-EXECUTED
  - A-DNS-NO-ES-COLAPSO
  - A-L2-HONESTO-SIN-FULL-CYCLE
  - A-NO-MUTAR-GENOMA-SIN-ENTITY-MANAGER
  - A-NO-DOGFOOD-KALMA2-HASTA-OLA-A
friction_ids:
  - F2-VERDICT-IGNORED
  - F5-DNS-NOT-SOFT
  - F1-DUAL-MANDATE
  - F10-HALT-NOT-CONTRACT
  - F6-L2-UNDECLARED
  - F7-OBJECTIVES-PBI-DUMP
  - F8-PROMPT-DUPLICATE-CHANNEL
  - F9-NO-PHASE-REPORTS-ON-DISK
related_pbis:
  - id: PBI-KALMA2-FULL-CYCLE-RUNTIME
    uuid: 527007fa-7200-41ee-84bb-202737f4f983
    path: docs/todos/done/[FEATURE] kalma2-full-cycle — runtime de agentes y semántica de cierre (527007fa).md
    rol: "Antecesor slice B (done). Runtime existe; no parsea veredicto. Deuda L2 (arranque ≠ auto-PR) sigue abierta."
  - id: PBI-FIX-FRACTURE-6c0db1296181
    path: docs/todos/done/[FIX] email-watcher — fractura sistémica (6c0db1296181).md
    rol: "Ciclo víctima histórico (spec+plan del 9dbcfea6). Keepalive ya cerrado en done/. Fuera: no retocar daemon ni re-forjar ese persist_ref."
  - id: PBI-FIX-FRACTURE-d0cfd5b66ff1
    path: docs/todos/done/[FIX] delivery-close-cycle — fractura sistémica (d0cfd5b66ff1).md
    rol: "Política hermana: red transitoria ≠ Kintsugi. Tokens F4c son git/glibc, no Node ENOTFOUND. No mutar DCC."
related:
  - docs/audits/kalma2-forge-email-watcher-keepalive-halt-20260830.md
  - SddIA/scripts/tools/kalma2-agent-runtime-cursor.py
  - SddIA/scripts/tools/test_kalma2_runtime_timeout.py
  - SddIA/engine/execute-process/src/engine/agent_runtime.rs
  - SddIA/engine/execute-process/src/engine/executor.rs
  - SddIA/engine/execute-process/src/engine/thermodynamic.rs
  - SddIA/engine/execute-process/src/engine/phase_terminal.rs
  - SddIA/engine/execute-process/src/engine/handlers/task_queue_manager.rs
  - SddIA/engine/execute-process/src/engine/workspace_init.rs
  - SddIA/engine/execute-process/src/engine/delivery_close.rs
  - docs/features/kalma2-process-dispatch/spec.md
  - docs/todos/done/[FEATURE] kalma2-full-cycle — runtime de agentes y semántica de cierre (527007fa).md
---

# [KAIZEN] Kalma2 agent-runtime — veredicto blocked, DNS y halt-after-phase

Auditoría SSOT: `docs/audits/kalma2-forge-email-watcher-keepalive-halt-20260830.md`. Ciclo `9dbcfea6-4df8-47ac-873a-cf9bce846929` (`bug-fix` v1.4.2). Dedalo forjó spec+plan, transcript `Veredicto: blocked`, prótesis mintió `executed`, Tekton 1 s después con `getaddrinfo ENOTFOUND api2.cursor.sh`, PEC `cycle_phase: failed`.

Este PBI repara **orquestación/runtime**. No es retoma del keepalive.

**Vehículo de forja:** `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` (o CLI local) hasta que Ola A esté en `main`. Prohibido despachar *este* ciclo vía Kalma2 live (la prótesis bajo arreglo). `dispatch: true` = elegible *después* de A.

## 0. Afirmaciones descartadas (v1.0.0)

| Afirmación v1.0.0 | Verdad en código (2026-08-31) |
|-------------------|-------------------------------|
| Basta parchear el `.py` para que `blocked` dispare la barrera | `agent_runtime.rs` normaliza solo `executed` \| `awaiting_agents` \| `failed` (\+ alias `awaiting`). `blocked` + `success:true` → **`executed`**. Sin tocar el normalizador, CA-A1 es inerte. |
| «Paridad F4c» = copiar `dcc_transient_network_trace` | F4c tokens: `could not resolve host`, `temporary failure in name resolution`, `name or service not known`, `network is unreachable`, `connection timed out`. **No** incluyen `ENOTFOUND` ni `getaddrinfo`. La traza del incidente es Node. Política hermana (red ≠ colapso), predicado **distinto**. |
| DNS → `blocked` si `REQUIRE_CLI` | Env real: `SDDIA_AGENT_RUNTIME_REQUIRE_CLI`. Hoy convierte *cualquier* soft en `failed` (CLI ausente). No aplica a DNS. DNS debe seguir `awaiting_agents` **aunque** el flag esté on. |
| `phase_reports.json` lo escribe `workspace_init.rs` | Init solo crea el dir + `objectives.md` bajo `persist_ref`. `phase_reports` viven en memoria → envelope/PEC. Writer = `executor.rs` al cerrar el bucle. Ruta = template `bug-fix.md`: `.SddIA/workspaces/{process_name}/{execution_id}/`. |
| Heurística NL «pide PR» → `SDDIA_TQM_FULL_CYCLE=1` | Contradice Fuera + laudo 527007fa (full-cycle sobre ciclo vacío). El estímulo víctima contenía halt **y** PR. Default = acuse `skipped_l2`, no flip del flag. |
| `PBI-FIX-FRACTURE-6c0db1296181` pendiente / keepalive por hacer | Archivado en `docs/todos/done/`; `spawn_heartbeat_worker` ya está en `email-watcher`. Fuera de alcance se mantiene. |
| F6 «DCC skipped por L2» en `9dbcfea6` | En ese PEC, Argos/DCC = `skipped` handler `phase-barrier` (`prior_agent_phase_not_executed`) porque Tekton fue `failed`. L2 es **estructural** (`child_env_for_kalma2`); no se observó como reason de skip en ese ciclo. |
| `pbi_ref` ausente en `objectives.md` | `workspace_init.rs` ya escribe `pbi_ref` en frontmatter si viene en inputs. El defecto F7 es **Misión = dump de `pbi_body`**. |
| `stop_after=commit` con oráculo git-manager existente | No hay sonda post-fase de commit en `executor.rs`. `role_brief` Dedalo pide spec+plan, **no** commit. `commit` no es MVP. |

## 1. Síntoma

| Expectativa operador | Resultado `9dbcfea6` |
|----------------------|----------------------|
| Dedalo `blocked` (Shell castrado, sin commit) corta Tekton | Prótesis `executed` (CLI 0) → Tekton inyectado |
| DNS `api2.cursor.sh` = red transitoria | `failed` duro → PEC `failed`; barrera salta Argos/DCC |
| Halt tras plan (y, en el texto, commit) | Un solo `bug-fix` 1→6; no existe `stop_after` |
| «después, PR en verde» | Inalcanzable: L2 inyecta `SDDIA_LAB_SKIP_DELIVERY_CLOSE=1` (código TQM; no evidenciado como reason en este PEC) |

Persist_ref del ciclo víctima: `docs/fixes/email-watcher-heartbeat-keepalive/` (spec/plan/handoff). Workspace `.SddIA/workspaces/bug-fix/9dbcfea6-…/` = mkdir vacío. No confundir ambos.

## 2. Causa (código verificado)

### F2 — Veredicto ignorado + normalizador ciego

Cadena:

1. `build_prompt` exige `veredicto (ok|blocked)`. Cero parser.
2. `run_agent_phase`: `if ok: status = "executed"` (exit 0).
3. `agent_runtime.rs` `normalized`: `blocked` ∉ allowlist → remap a `executed` si `success` else `failed`.
4. `executor.rs` `agent_phase_blocks_downstream("blocked")` **sí existe** y no se alcanza.

Tests actuales: `test_kalma2_runtime_timeout.py` cubre `is_soft_config_error` / timeout. No hay fixture de veredicto.

### F5 — DNS no es soft

`SOFT_CONFIG_MARKERS` incluye `"not found"` (con espacio). `"enotfound"` **no** lo contiene. `getaddrinfo ENOTFOUND api2.cursor.sh` → `failed`.

Soft actual → `awaiting_agents` + `emit(success=true)` salvo `SDDIA_AGENT_RUNTIME_REQUIRE_CLI` (entonces `failed`). Timeout ya es terminal (no tocar).

DCC F4c mapea red a status `blocked` en fases push/forja para **no** emitir Kintsugi. Aquí no hay Kintsugi de agent-runtime; el daño es PEC `failed`. Status objetivo DNS = `awaiting_agents` (reintento / no colapso), no `blocked` DCC.

### F1/F10 — Halt no es contrato

`build_child_inputs` concatena `## Prompt operador` + `task_text` + `## PBI adjunto` + `pbi_body` en `bug_summary`. No hay campo `stop_after`. El `role_brief` Dedalo (spec+plan) gana sobre el halt del operador. El proceso recorre todas las fases agente.

El texto dual («detente tras plan y commit» / «forja PR») **no** se mapea 1:1 a un enum. `design` = halt tras fase Dedalo (nombres exactos abajo). `commit` = deuda: sin oráculo.

### F6 — L2 no declarado en acuse TQM

`child_env_for_kalma2`: `correlation_id` no vacío ∧ ¬`SDDIA_TQM_FULL_CYCLE` → setea `SDDIA_LAB_SKIP_PBI_ARCHIVE` y `SDDIA_LAB_SKIP_DELIVERY_CLOSE` si no vienen ya. Envelope TQM (`dispatched_process`, `child`, `handler`) **no** expone esos skips. `derive_cycle_phase` ignora `skipped`; si las fases agente salen `executed`/`skipped` sin `awaiting*`, PEC puede ser `completed` (hueco 527007fa). Pedir «PR» en NL no activa full-cycle.

### F7/F8/F9 — prompt y disco

| ID | Hecho |
|----|--------|
| F7 | `workspace_init` Misión = `pbi_body` (o fallbacks) crudo. `pbi_ref` en FM ya existe. |
| F8 | `build_prompt`: `seed[:8000]` (`bug_summary` = prompt+body) **más** `pbi_body[:12000]`. Triple canal con objectives. |
| F9 | Workspace template materializa dir; cero `phase_reports` en disco. Auditoría solo vía PEC + `_agent_handoff.md` + transcript IDE. |

## 3. Alcance

### Dentro

| Ola | Target (motor / prótesis; no DA-2) | Cambio |
|-----|-------------------------------------|--------|
| **A** | `kalma2-agent-runtime-cursor.py` | Parser de veredicto en transcript (ok\|blocked, case-insensitive, última ocurrencia). CLI 0 + `blocked` → `data.status=blocked`, `success=true`, exit 0 (misma forma que `awaiting_agents`: el status porta la barrera; el exit no finge colapso). Ampliar `test_kalma2_runtime_timeout.py` (o sibling). |
| **A** | `agent_runtime.rs` | Allowlist: aceptar `blocked` sin remap. Tests del normalizador. |
| **A** | `kalma2-agent-runtime-cursor.py` `is_soft_config_error` | Añadir tokens Node/DNS: `enotfound`, `getaddrinfo`, `eai_again`. Opcional: `could not resolve host` (solape F4c, no sustituto). **No** copiar el predicado DCC entero (`connection timed out` colisionaría con timeout-terminal). DNS → `awaiting_agents` **incluso** con `SDDIA_AGENT_RUNTIME_REQUIRE_CLI=1` (ese flag = CLI ausente, no red). |
| **B** | TQM + `executor.rs` | Contrato `stop_after` vía **env** `SDDIA_TQM_STOP_AFTER` y/o input TQM (no YAML de proceso). Valores: `design` \| unset. Tras fase Dedalo **aunque** `executed`: barrera. Nombres exactos: `Diseño del fix` (`bug-fix`) · `Diseño de Blueprint` (`feature`) · `Diseño de refactor` (`refactorization`). |
| **B** | `thermodynamic.rs` / `phase_terminal.rs` | Halt `stop_after` ≠ `completed` de negocio y ≠ `failed` de colapso. Hoy: `blocked` es causal fail (`first_causal_failure`) → PEC `failed`; `skipped` es neutro → PEC `completed`. Hace falta reason/status de halt que `derive_cycle_phase` no clasifique `completed`. Laudo: p. ej. reason `stop_after` + `cycle_phase=awaiting_agents` (o valor nuevo documentado). No reutilizar `blocked` del agente para el halt-by-contrato. |
| **B** | TQM envelope | Campo estable `delivery_close: skipped_l2` (o equivalente en `data`) cuando L2 inyectó skip. **Prohibido** inferir `SDDIA_TQM_FULL_CYCLE=1` del texto «PR». Full-cycle = flag explícito del operador/bóveda. |
| **C** | `workspace_init.rs` | Misión destilada (título + 1–2 líneas / `document_id`); no volcar YAML+cuerpo del PBI. Conservar `pbi_ref` en FM. |
| **C** | `build_prompt` | Un canal de PBI: o `pbi_ref` + puntero a fichero, o un extracto; no `seed[:8000]` + `pbi_body` duplicado. |
| **C** | `executor.rs` | Al terminar (éxito, barrera o fail): escribir `phase_reports.json` en `{workspace_path}/phase_reports.json`. |

`delivery_close.rs`: **solo lectura** (SSOT tokens F4c). No mutar.

### Fuera

- Keepalive `email-watcher` / retoma `6c0db1296181` (ya done).
- Castrar/restaurar Shell del host Cursor (F3): producto IDE.
- Mutar `SddIA/process/`, `library/codexes/**/process/*.md`, events, norms → `entity-manager`.
- Derogar L2 en silencio. Activar full-cycle sobre ciclo vacío (laudo 527007fa).
- Heurística NLP «detente»/«después»/«PR» como parser de fases.
- MVP `stop_after=commit` (sin oráculo git-manager post-fase; Dedalo no tiene mandato de commit).
- `iota-publish-relay`, umbrales Argos, fagoctio.
- Unificar status DNS agent-runtime (`awaiting_agents`) con status F4c DCC (`blocked`). Jurisdicciones distintas.

## 4. Criterios de aceptación

- [ ] **CA-A1** Fixture transcript `Veredicto: blocked` + CLI 0 → JSON prótesis `data.status=blocked` y `success=true`. Tras normalizador Rust, `entry.status=blocked`. `executor` no invoca la fase agente siguiente (`Ejecución` / Tekton).
- [ ] **CA-A2** Fixture `getaddrinfo ENOTFOUND api2.cursor.sh` → `awaiting_agents`; **no** `failed`. Mismo resultado con `SDDIA_AGENT_RUNTIME_REQUIRE_CLI=1`. PEC `cycle_phase=awaiting_agents` (no `failed`). Argos/DCC `skipped` por `phase-barrier`, no por colapso Kintsugi.
- [ ] **CA-A3** Test unitario Rust: stdout `{"success":true,"data":{"status":"blocked"}}` no se remapea a `executed`.
- [ ] **CA-B1** `SDDIA_TQM_STOP_AFTER=design` (o input TQM): tras Dedalo `executed` con spec+plan, Tekton no arranca. PEC `cycle_phase` ∉ {`completed`}. PEC ≠ `failed` salvo fallo real de fase.
- [ ] **CA-B2** Despacho Kalma2 con `correlation_id`, sin `SDDIA_TQM_FULL_CYCLE`: acuse TQM declara skip L2 (`skipped_l2` o campo estable documentado). Nunca UI/PEC `completed` que implique PR forjado. Un `task_text` que mencione «PR» **no** enciende full-cycle.
- [ ] **CA-C1** `objectives.md` Misión ≠ cuerpo completo del PBI; frontmatter conserva `pbi_ref` si el input lo trae.
- [ ] **CA-C2** `{workspace_path}/phase_reports.json` existe al terminar (éxito, `stop_after`, `blocked` o DNS `awaiting_agents`).

## 5. Orden

```text
Ola A (parser + normalizador Rust + DNS)  →  barrera real
  → Ola B (stop_after=design + cycle_phase honesto + skipped_l2)
    → Ola C (objectives / prompt / phase_reports.json)  — no bloquea A/B
```

Un PR puede cubrir A+B+C; no invertir el orden de dependencia (B asume que `blocked`/`awaiting_agents` sobreviven al normalizador).

## 6. Cierre

Un PR. `validacion.md` APTO, `pbi_archived: true`, este fichero en `docs/todos/done/` en la misma rama. Git vía `skill:git-manager`. Genoma solo vía `entity-manager` si Dedalo demuestra que B exige input en el contrato YAML de proceso (preferir env/TQM).
