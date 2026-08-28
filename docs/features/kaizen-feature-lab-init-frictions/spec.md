---
feature_name: kaizen-feature-lab-init-frictions
created: "2026-08-28"
process: feature
base: main
scope: lab-init-relay-runtime-trace-daemon-hygiene
version_spec: "1.0.0"
document_id: PBI-KAIZEN-FEATURE-LAB-INIT-FRICTIONS
uuid: "58e3c9f7-0e90-4e51-8b87-a9054a9b30fe"
persist_ref: docs/features/kaizen-feature-lab-init-frictions
branch_name: feat/kaizen-feature-lab-init-frictions
execution_id: "80a3ca0d-80c5-4662-ab12-2afe757478c8"
dedalo_verdict: ok
laudo: flag-relevo-paridad-boveda-timeout-pgid-execid-motor
laudos:
  - L-RELAY-FLAG
  - L-VAULT-PARITY
  - L-TIMEOUT-MOTOR
  - L-PGID-BURIAL
  - L-REENTRY
  - L-EXEC-ID
  - L-CONFLICT
  - L-DAEMON-CLASS
  - L-DAEMON-NOISE
  - L-DAEMON-SCOPE
  - L-INDEX-CENSUS
  - L-DIRTY-INIT
  - L-TODOS-PRESERVE
---

# Spec — kaizen-feature-lab-init-frictions

## 1. Decisiones Dedalo

| ID | Decisión | Rationale |
|----|----------|-----------|
| **L-RELAY-FLAG** | `SDDIA_AGENT_RELAY_IDE` truthy (`1`/`true`/`yes`/`on`) fuerza `is_configured() == false` **antes** de leer `SDDIA_AGENT_RUNTIME_COMMAND`. Log stderr: `agent-runtime: lab-relay activo (bóveda ignorada)`. Documentar en `external-ai-constraints.md` (vía `norm-creator` / update) y comentario de `agent_runtime.rs`; prohibido dejar el contrato solo en `clarify.md`. | R-RELAY; LAB-CA1 |
| **L-VAULT-PARITY** | `_sddia_load_vault` adopta setdefault: `export` solo si `${!key-}` no está definida. Lista de precedencia = misma que Rust (`VAULT_PRECEDENCE_KEYS`: hoy IOTA). Extraer la lista a constante documentada o comentario de paridad en ambos lados; no añadir `COMMAND` a precedencia. | R-PARITY; LAB-CA2/CA3 |
| **L-TIMEOUT-MOTOR** | Tras spawn, el motor no usa `wait_with_output()` desnudo. Techo = `SDDIA_AGENT_RUNTIME_TIMEOUT_SECS` parseado; **default motor 660**. Prótesis permanece en default 600. Superado: kill grupo, `status: failed`, `error: agent-runtime-timeout`, acuse JSON. Override por fase `SDDIA_AGENT_RUNTIME_TIMEOUT_SECS_EJECUCION` se respeta si el motor invoca esa fase (paridad prótesis). | R-CEILING; LAB-CA4 |
| **L-PGID-BURIAL** | Unix: `pre_exec` `setpgid(0,0)` (o equivalente portable). Timeout o drop del invocador: `kill(-pgid, SIGTERM)` + gracia corta + `SIGKILL`. Windows: `CREATE_NEW_PROCESS_GROUP` / `GenerateConsoleCtrlEvent` best-effort; test unitario gateado `cfg(unix)`. | R-BURIAL; LAB-CA5 |
| **L-REENTRY** | Motor setea `SDDIA_AGENT_RUNTIME_DEPTH=1` en el env del hijo. Si el proceso actual ya tiene profundidad ≥1, **no** re-spawnea fases `agent:`; marca `simulated` + nota `reentry-guard`. | Anidamiento CLI→runtime→CLI |
| **L-EXEC-ID** | Payload de `invoke_agent_phase` incluye `execution_id` en raíz (no solo dentro de `inputs`). `build_prompt` añade `- execution_id: {id}`. Frontmatter de `_agent_handoff.md` declara `execution_id`. Workspace stub `objectives.md` de `workspace_init` también lo escribe. | R-TRACE; LAB-CA6 |
| **L-CONFLICT** | Antes de spawn (motor) y antes de escribir artefactos de fase (prótesis): si algún `.md` bajo `persist_ref` (excepto `_agent_handoff.md`) declara `execution_id` distinto del vivo y no vacío → `status: failed`, `error: persist-execution-id-conflict`, lista de paths. No sobrescribir. | LAB-CA7 |
| **L-DAEMON-CLASS** | Añadir `daemon` a `entity-manager.md` (enum + fase piloto) vía `process-creator` **update**; espejo en `PILOT_CLASSES`, `creator_name` → `daemon-creator`, `dir_by_class` → `directories.daemons` (`SddIA/daemons`). Semillas futuras usan EM; lab directo `daemon-creator` sigue válido. | F-DAEMON-FORGE-PORTE |
| **L-DAEMON-NOISE** | Borrar el `Err` especial de `residual_runner.rs:746-748`. Fallo de forja daemon = `failed` como el resto. | LAB-CA8 |
| **L-DAEMON-SCOPE** | Contrato de `daemon-creator` (update vía `process-creator`): forja **solo** `{name}.md` + fila de índice + censo. Launcher `SddIA/daemons/{name}.sh`, wrapper `SddIA/scripts/daemons/{name}.sh`, crate y `SYSTEMD_FACTORY_DAEMONS` = **delivery post-forja** del ciclo que crea el daemon (Tekton, no forja). `iota-publish-relay`: vía canónica de arranque = wrapper `scripts/daemons/` (`_run_daemon.sh` + lock); `SddIA/daemons/*.sh` = entrypoint ELF. No borrar el ELF en este ciclo; documentar dualidad en `daemons-contract` (update menor). | Evita mutación furtiva; no reabre Aduana |
| **L-INDEX-CENSUS** | Tras `append_row` en `run_daemon_forge`, reescribir el bloque «Integridad» con `N = filas de tabla`. Helper compartido preferible a regex frágil: contar líneas `\| \`` bajo `## Catálogo`. Pie deja de ser texto libre desincronizable. | LAB-CA9; hoy 6 vs «cinco» |
| **L-DIRTY-INIT** | En `workspace_init`, **antes** de fetch: `git status --porcelain` vía `skill:git-manager`. Paths dirty fuera de `persist_ref/` y `pbi_ref` → abort `error: dirty-worktree` + lista. Escape: `SDDIA_LAB_ALLOW_DIRTY=1`. `SDDIA_LAB_SKIP_GIT` sigue saltando toda la secuencia git (y por tanto este gate). | LAB-CA10 |
| **L-TODOS-PRESERVE** | `delivery-snapshot-final`: de `files` porcelain, **excluir** `??` (untracked) cuyo path empiece por `docs/todos/` y no sea el `pbi_ref` del ciclo ni el destino `done/` de ese mismo `document_id`. Esos paths **no** se commitean ni se `rm`. Si tras el commit de snapshot siguen `??` bajo `docs/todos/`, el handler **no** falla por dirty residual de esos paths (sí falla por dirty de otros). Prohibido revert/rm de untracked todos ajenos. | LAB-CA11 |

## 2. Circuito

```
init (workspace_init)
  [L-DIRTY-INIT] porcelain → abort | allow
  fetch → checkout base → pull → checkout -b
  objectives.md stub + execution_id

agent phases (si !L-RELAY-FLAG && COMMAND no vacío && DEPTH<1)
  payload + execution_id
  [L-CONFLICT] persist_ref
  spawn PGID
  wait ≤ timeout motor
    OK → JSON fase
    timeout → killpg + agent-runtime-timeout
  DEPTH=1 en hijo → no re-spawn

cierre (ciclo futuro)
  snapshot: [L-TODOS-PRESERVE]
```

## 3. Contratos de entorno

| Variable | Semántica |
|----------|-----------|
| `SDDIA_AGENT_RELAY_IDE` | Relé IDE; ignora bóveda de `COMMAND` |
| `SDDIA_AGENT_RUNTIME_COMMAND` | CLI agente; vacío = no configurado; `unset` ya no reinyecta si el flag de relevo está, y la puerta shell no pisa vars definidas |
| `SDDIA_AGENT_RUNTIME_TIMEOUT_SECS` | Techo motor (default 660) y prótesis (default 600) |
| `SDDIA_AGENT_RUNTIME_TIMEOUT_SECS_EJECUCION` | Override fase Ejecución |
| `SDDIA_AGENT_RUNTIME_DEPTH` | Guarda de reentrada (inyectada; no contrato operador) |
| `SDDIA_LAB_ALLOW_DIRTY` | Permite init con WT sucio ajeno |
| `SDDIA_LAB_SKIP_GIT` / `SDDIA_LAB_SKIP_*` | Intactos |

## 4. Touchpoints (Tekton)

| Área | Paths lógicos |
|------|----------------|
| Motor runtime | `SddIA/engine/execute-process/src/engine/agent_runtime.rs` |
| Bóveda Rust | `SddIA/engine/execute-process/src/core/env.rs` |
| Bóveda shell | `SddIA/scripts/common/sddia_shell_lib.sh` |
| Init | `SddIA/engine/execute-process/src/engine/workspace_init.rs` |
| Snapshot | `SddIA/engine/execute-process/src/engine/phase_capsules.rs` |
| EM Rust | `SddIA/engine/execute-process/src/engine/entity_manager.rs` |
| Fail-soft | `SddIA/engine/execute-process/src/engine/residual_runner.rs` |
| Forja daemon | `SddIA/engine/execute-process/src/forges/factory.rs` |
| Prompt/handoff | `SddIA/scripts/tools/kalma2-agent-runtime-cursor.py` |
| Genoma (cadena) | `entity-manager` update; `daemon-creator` update; `external-ai-constraints` / `daemons-contract` vía creators |

## 5. Tests / smokes

| ID | Diseño |
|----|--------|
| T-RELAY | Bóveda con COMMAND + `SDDIA_AGENT_RELAY_IDE=1` → fases `simulated`, acuse <15s, stderr contiene log |
| T-UNSET | `env -u COMMAND` + flag relevo → no hang; sin flag, setdefault no debe colgar si COMMAND vacío no se reinyecta como “configurado” — `is_configured` exige no-vacío |
| T-PARITY | Mismo fixture dotenv: Rust `apply_env` vs función testable de `_sddia_load_vault` (extraer o smoke bash) |
| T-TIMEOUT | COMMAND=`sleep infinity` + timeout lab corto (env 2s) → `agent-runtime-timeout` + exit ≠ 0 de fase |
| T-PGID | Tras timeout, `ps` del pgid vacío (unix) |
| T-REENTRY | Hijo con DEPTH=1 no spawnea |
| T-EXECID | Prompt/payload/handoff/objectives contienen UUID del workspace |
| T-CONFLICT | Segundo writer con UUID distinto → `persist-execution-id-conflict` |
| T-DAEMON | `daemon-creator` lab: exit 0 real o `failed`; nunca nota `forja pendiente porte` |
| T-CENSUS | Tras forja de daemon de prueba (o fixture de índice), pie = N filas |
| T-DIRTY | Archivo dirty ajeno → abort; `ALLOW_DIRTY=1` pasa |
| T-TODOS | `??` PBI ajeno + snapshot → archivo sigue en disco y no entra al commit |

## 6. Genoma

Mutaciones de `{name}.md` de process/norm/contract **solo** vía `./sddia-run.sh --process entity-manager` / creator aplicable (DA-2). Código Rust y prótesis Python no son genoma indexado; van por Tekton bajo topología DA-4 de este `persist_ref`.
