---
feature_name: kaizen-feature-lab-init-frictions
created: "2026-08-28"
process: feature
purpose: Estabilización Mayeuta — PBI-KAIZEN-FEATURE-LAB-INIT-FRICTIONS
branch_name: feat/kaizen-feature-lab-init-frictions
persist_ref: docs/features/kaizen-feature-lab-init-frictions
pbi_ref: docs/todos/pending/[KAIZEN] Init lab feature — bóveda reinyecta AGENT_RUNTIME y carrera de agentes.md
document_id: PBI-KAIZEN-FEATURE-LAB-INIT-FRICTIONS
uuid: "58e3c9f7-0e90-4e51-8b87-a9054a9b30fe"
execution_id: "80a3ca0d-80c5-4662-ab12-2afe757478c8"
mayeuta_session_id: "80a3ca0d-80c5-4662-ab12-2afe757478c8"
correlation_id: ""
incident_ref: "Sesión Tekton 2026-08-27 19:59→20:14 — hang init + runtime huérfano + execution_id fabricado"
mayeuta_verdict: ok
laudo: flag-relevo-paridad-boveda-timeout-pgid-execid-motor
---

# Clarificación — kaizen-feature-lab-init-frictions

Transcript Mayeuta (2026-08-28). Semilla PBI v1.1.0. Filtro A contra genoma vigente en `main` (`882cc0e`) + init lab de este ciclo. Relé IDE: Dedalo forja `spec.md`/`plan.md` en el mismo ciclo; **Tekton no arranca**.

---

## D0 — Apertura formal

| Pregunta | Decisión |
|----------|----------|
| Proceso | `feature` (semilla `type: kaizen`, `process: feature`) |
| `feature_name` | `kaizen-feature-lab-init-frictions` |
| Rama | `feat/kaizen-feature-lab-init-frictions` |
| `persist_ref` | `docs/features/kaizen-feature-lab-init-frictions` |
| `document_id` | `PBI-KAIZEN-FEATURE-LAB-INIT-FRICTIONS` |
| `uuid` PBI | `58e3c9f7-0e90-4e51-8b87-a9054a9b30fe` |
| Init lab | `./sddia-run.sh --process feature` + `SDDIA_LAB_SKIP_PBI_ARCHIVE=1` + `SDDIA_LAB_SKIP_DELIVERY_CLOSE=1` + `SDDIA_AGENT_RUNTIME_COMMAND=""` (vacío, **no** `unset`) |
| `execution_id` ciclo | `80a3ca0d-80c5-4662-ab12-2afe757478c8` — workspace `.SddIA/workspaces/feature/80a3ca0d-80c5-4662-ab12-2afe757478c8/` |
| Fase | Mayeuta + Dedalo (plan). **Tekton no arranca.** |
| Antecesor | `kaizen-aduana-dlt-relay-supervisado` (merge `ecd8438`) — porte `run_daemon_forge` aterrizado; corpus con `execution_id` no resolubles |

**Toll:** un `persist_ref`, un PR. Cierre documental en rama (PBI → `docs/todos/done/` + `validacion.md` APTO) en el mismo PR.

**Fricciones semilla:** `F-VAULT-UNSET-REINJECT`, `F-VAULT-DUAL-POLICY`, `F-AGENT-RUNTIME-NO-TIMEOUT`, `F-AGENT-RUNTIME-ORPHAN`, `F-EXECUTION-ID-NO-PROPAGADO`, `F-DAEMON-FORGE-PORTE`, `F-DAEMON-INDEX-DESYNC`, `F-DIRTY-WT-CROSS-CHECKOUT`, `F-PBI-UNTRACKED-BARRIDO`.

---

## D1 — Filtro A: semilla vs territorio (2026-08-28)

| Afirmación semilla | Territorio | Veredicto |
|--------------------|------------|-----------|
| `apply_env` setdefault; `unset` reinyecta bóveda | `env.rs:68-78`; `VAULT_PRECEDENCE_KEYS` solo IOTA (`:9`) | **Confirmado** |
| `_sddia_load_vault` `export` incondicional | `sddia_shell_lib.sh:107` | **Confirmado** |
| `wait_with_output()` sin timeout | `agent_runtime.rs:240` | **Confirmado** |
| Timeout 600s solo en prótesis Python | `kalma2-agent-runtime-cursor.py:20,83` | **Confirmado** |
| Spawn sin PGID / sin reentrada | `agent_runtime.rs:215-246`; cero `SDDIA_AGENT_RUNTIME_DEPTH` | **Confirmado** |
| `build_prompt` no entrega `execution_id` | `kalma2-agent-runtime-cursor.py:488-498` | **Confirmado** |
| Handoff sin `execution_id` | `append_handoff` `:565-571` | **Confirmado** |
| `entity-manager` 9 clases, sin `daemon` | `entity-manager.md:6`; `PILOT_CLASSES` / `creator_name` / `dir_by_class` | **Confirmado** |
| Fail-soft `daemon-creator` → `simulated` | `residual_runner.rs:746-748` | **Confirmado** |
| `run_daemon_forge` + rama `"daemon"` en `main` | `factory.rs:1109+` | **Confirmado** (consolidación, no bloqueante) |
| Índice 6 filas / pie «cinco Centinelas» | `daemons/index.md:19-24` vs `:32` | **Confirmado** |
| `workspace_init` sin porcelain; `SDDIA_LAB_ALLOW_DIRTY` inexistente | `workspace_init.rs:201-242` | **Confirmado** |
| Snapshot DCC commitea todo porcelain | `phase_capsules.rs:316-366` | **Confirmado** (trampa: PBI `??` entra al snapshot o se barre en revert ajeno) |

**Este init:** acuse JSON 1.5s; fases agente `simulated`; git-steps fetch/checkout/pull/rama nueva. Mandato stop-at-plan **cumplido en el CLI**; el corpus documental lo escribe el relé IDE con el `execution_id` del acuse.

**Laudo causal:** tres huecos que se refuerzan — bóveda con dos políticas, runtime sin techo ni entierro, trazabilidad delegada al agente. No es carrera de dos ciclos legítimos.

---

## D2 — Decisiones de requisito (no diseño)

| ID | Decisión |
|----|----------|
| **R-RELAY** | Relé IDE = flag positivo `SDDIA_AGENT_RELAY_IDE=1`. Ausencia/vacío de `COMMAND` deja de ser el contrato de relevo. |
| **R-PARITY** | Una semántica de bóveda en Rust y shell: `export`/`set_var` solo si la var no está definida, salvo lista de precedencia explícita (hoy IOTA). |
| **R-CEILING** | Timeout en el motor; acuse `failed` + `error: agent-runtime-timeout`. Default motor **>** default prótesis (600s). Misma familia de env. |
| **R-BURIAL** | Grupo de proceso propio; kill del grupo al timeout o abort. Guarda de reentrada en el hijo. |
| **R-TRACE** | `execution_id` lo emite el motor; entra en payload, prompt y frontmatter de handoff. Artefacto con UUID sin workspace = inválido. Conflicto si `persist_ref` ya tiene otro `execution_id`. |
| **R-DAEMON** | Clase `daemon` en piloto EM **o** invocación `daemon-creator` canonizada. Fail-soft `simulated` extirpado. Alcance de forja declarado (definición+índice vs delivery post-forja). |
| **R-CENSUS** | Pie de `daemons/index.md` no puede contradecir el número de filas. |
| **R-DIRTY** | Porcelain **antes** de la secuencia git de init. Abort + lista de paths salvo `SDDIA_LAB_ALLOW_DIRTY=1`. Exención: `persist_ref` y `pbi_ref`. |
| **R-TODOS** | Snapshot/cierre no elimina ni barre `??` bajo `docs/todos/` ajenos al `pbi_ref` del ciclo. |

---

## D3 — Orden no negociable

1. Techo + entierro del runtime (hemorragia).
2. Flag de relevo + paridad de bóveda.
3. Propagación de `execution_id`.
4. Consolidación daemon (clase + fail-soft + censo).
5. Gate dirty WT + preservación `docs/todos/`.

---

## D4 — Fuera (sello)

- Sustituir Kalma2 agent runtime.
- Reabrir PBI Aduana DLT / saneamiento retroactivo de su corpus.
- Auto-review / Smart Mode Cursor.
- GC de `.SddIA/workspaces/feature/` (deuda anotada).

---

## No objetivos

- Inventar `execution_id` en frontmatter.
- Continuar a Tekton/Argos/DCC en este estímulo.
- Mutar genoma (`entity-manager.md`, `daemon-creator`, normas) a mano en T0.
