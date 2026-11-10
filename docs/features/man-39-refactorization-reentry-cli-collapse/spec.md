---

feature_name: man-39-refactorization-reentry-cli-collapse
created: "2026-11-10"
process: feature
base: main
scope: "Reentrada del ciclo lifecycle sin repetir forja; timeout de bóveda; aborto con envelope; peaje dentro del dirty-gate; labels exactas del equipo padre"
branch_name: feat/man-39-refactorization-reentry-cli-collapse
persist_ref: docs/features/man-39-refactorization-reentry-cli-collapse
pbi_ref: MAN-39
historia_ref: MAN-36
document_id: PBI-70fb886c
agent: dedalo
phase: diseno-blueprint
version_spec: "1.0.0"
status: blueprint-ready
norm_ref: features-documentation-pattern@1.3.1
forge_key: "2e7baef71d4c29dc25c07db567099259c855c06271d36cd0ae854ba35cb5211b"
execution_id: "454bcd99-8bbb-4cb3-8038-31cbe046ac55"
---
# Especificación técnica — man-39-refactorization-reentry-cli-collapse

Entrada: `objectives.md` + `clarify.md` (laudos L-*) + semilla **PBI-70fb886c** / **MAN-39** (historia **MAN-36**).
Diseño de Blueprint **v1.0.0**. `execution_id` **bd841ef3-fd25-43bd-a038-a6fe2f5efc41**.

## 0. Evidencia viva (2026-11-10)

| Artefacto | Estado | Implicación |
|-----------|--------|-------------|
| `engine/execute-process/src/engine/executor.rs` | `stop_after_design` solo si el nombre es fase Dedalo y `status == executed`. `simulated_relay_blocks_close` convierte un `simulated` en barrera `awaiting_agents` y salta el resto. | **T0** + **T4** |
| `lifecycle_phase_action.rs` `persist_forge_artifacts_ready` | True si existen `implementation.md` y `execution.md`. Se usa al entrar en Verificación (`forge_closed`), no para omitir el tren. | **T0** |
| `qa_profile.rs` `skip_agent_phase_entry` | Solo `pull-request-review` + perfil `docs-only`. | No reutilizar como atajo; el salto es otro handler. |
| `agent_runtime.rs` `resolve_timeout_secs` | `ejecuc*` → `SDDIA_AGENT_RUNTIME_TIMEOUT_SECS_EJECUCION`, si no `SDDIA_AGENT_RUNTIME_TIMEOUT_SECS`, si no **660**. `kill_process_group` solo tras `RecvTimeoutError::Timeout`. | **T1** + **T2** |
| `core/env.rs` `apply_env` | `set_var` solo si `env::var(key)` es error. Cadena vacía ya presente bloquea la bóveda. Precedencia incondicional: solo `SDDIA_LAB_SIMULATE_IOTA` y `SDDIA_IOTA_TIMEOUT_SECONDS`. | **T1** |
| `workspace_init.rs` `path_in_scope` | `persist_ref` o path igual a `pbi_ref`. | **T3** |
| `llm_interaction_audit.rs` `maybe_record_process_peaje` | Escribe bajo `paths.auditsPath`. Acuse de esta pasada: `docs/audits/MAN-39.md`. | **T3** espécimen |
| `linear-tracker-adapter` `run_create_issue` | Equipo efectivo = equipo del `parent_ref`. `resolve_label_ids` devuelve `LINEAR_LABEL_UNKNOWN` antes de la mutación. Lab MAN: `Hu`, `Pbi`, `Fix`. | **T5** |

## 1. Decisión

El ejecutor de los tres procesos lifecycle comparte tren. La forja presente en disco es la señal de salto. El halt post-diseño se conserva para cuando Dedalo acaba de ejecutar y aún no hay `implementation.md`. Timeout, aborto, peaje y labels son contratos del mismo ciclo, no features aparte.

| Vector | Laudo |
|--------|-------|
| Proceso | **L-PROCESS** |
| Reentrada | **L-REENTRY** / **L-SAME-EXECUTION** |
| Timeout | **L-TIMEOUT** |
| Aborto | **L-ABORT** |
| Peaje | **L-DIRTY** |
| Reanudación | **L-RESUME** |
| Labels | **L-LABEL** |

## 2. Contratos

### 2.1 Salto de forja (T0)

Condición: `persist_forge_artifacts_ready == true` al empezar una fase cuyo `delegates_to` es solo agente de forja (`mayeuta`, `dedalo`, `tekton`) en `feature`, `bug-fix` o `refactorization`.

Efecto: la fase queda `skipped`, `reason: forge_already_present`, sin invocar `SDDIA_AGENT_RUNTIME_COMMAND`. Verificación no se salta por esta regla. Si faltan los dos markdown, el tren corre igual que hoy.

### 2.2 Timeout (T1)

Para fase de Ejecución el entero efectivo es:

1. `SDDIA_AGENT_RUNTIME_TIMEOUT_SECS_EJECUCION` si es un entero > 0.
2. Si no, `SDDIA_AGENT_RUNTIME_TIMEOUT_SECS`.
3. Si la variable existe y está vacía o no es entero, se trata como ausente.
4. Suelo: si el resuelto es < 2400, el efectivo de Ejecución es 2400 y el acuse lo declara (`timeout_floor_applied: true`).
5. El default compilado solo aplica cuando no hay entero en bóveda ni en entorno, y para Ejecución también queda en 2400.

El export de una sesión ya arrancada no cambia un proceso en vuelo. No hace falta una segunda fuente de verdad.

### 2.3 Aborto (T2)

Además del timeout actual: al recibir muerte del padre o al fallar el wait por señal, se señala el grupo del hijo agente y se termina el `sddia-workspace-server` asociado al `execution_id` de esa invocación. Stdout final: envelope JSON `success: false`, `exitCode != 0`, código estable `agent-runtime-aborted`. Prohibido dejar el CLI sin línea JSON de cierre cuando el runtime hijo ya arrancó.

### 2.4 Dirty-gate (T3)

`path_in_scope` admite también `{auditsPath}/{tracker_ref}.md` cuando el input trae ese `tracker_ref` (o el resuelto por el peaje, misma clave). El resto de sucios sigue abortando `workspace-init`. El peaje no se reubica.

### 2.5 Reanudación (T4)

- Con forja presente, T0 deja pasar hasta Verificación. Argos y el cierre documental no se marcan `skipped` por `prior_agent_phase_not_executed` de una fase de forja ya saltada por `forge_already_present`.
- `SDDIA_TQM_STOP_AFTER=design` permanece: si Dedalo `executed` y la forja no está, Ejecución y posteriores quedan `skipped` / `stop_after`.
- No se añade un segundo proceso. La entrada es el mismo `./sddia-run.sh --process feature|bug-fix|refactorization` con el `persist_ref` que ya contiene la forja.

### 2.6 Labels (T5)

Sin cambio de semántica live si el fallo ya es previo al `issueCreate`. Se fija con test: label ausente en el equipo del padre → `LINEAR_LABEL_UNKNOWN` y ningún issue; `Kaizen` no coincide con `Kaicen`. El catálogo lab de MAN incluye `Kaizen` para que el mock no contradiga el mapa `tracker.labels` del manifiesto Core.

## 3. Fuera de alcance

Cuerpo de MAN-36, store Redb, consumidores EDA, commits de los ficheros sucios ajenos observados en el checkout de esta pasada.

## 4. Aceptación

Los seis **KA-REENTRY-*** de `objectives.md`, cubiertos por tests del crate `execute-process` y del adapter Linear. `validacion.md` queda para la fase Argos, no para esta parada.
