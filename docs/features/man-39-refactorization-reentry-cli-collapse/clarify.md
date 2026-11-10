---
feature_name: man-39-refactorization-reentry-cli-collapse
created: "2026-11-10"
process: feature
purpose: Estabilización — reentrada del ciclo lifecycle no reejecuta el tren de forja; timeout, aborto, peaje y labels quedan verificables
branch_name: feat/man-39-refactorization-reentry-cli-collapse
persist_ref: docs/features/man-39-refactorization-reentry-cli-collapse
pbi_ref: MAN-39
historia_ref: MAN-36
document_id: PBI-70fb886c
phase: mayeuta-stabilization
mayeuta_verdict: ok
norm_ref: features-documentation-pattern@1.3.1
forge_key: "2e7baef71d4c29dc25c07db567099259c855c06271d36cd0ae854ba35cb5211b"
execution_id: "bd841ef3-fd25-43bd-a038-a6fe2f5efc41"
---
# Clarificación — man-39-refactorization-reentry-cli-collapse

Transcript Mayeuta (2026-11-10). Fase **Estabilización de Requisitos** del proceso `feature`.
Canal **MAN-39** (historia **MAN-36**). Documento **PBI-70fb886c**. Provider Linear: sin copia nueva bajo `docs/todos/`.

## D0 — Semilla y acuse de esta pasada

| Hecho | Laudo |
|-------|-------|
| Tracker | **MAN-39**. `refine-item` previo: `refined`, tipo `kaizen`, `execution_id` de refino `1acd1942-4924-481a-856c-94b402811b6a`. |
| Semilla | Frontmatter del ítem declara `process: refactorization` y `proposed_persist_ref: docs/refactors/refactorization-reentry-cli-collapse`. |
| Ciclo runtime | El operador invocó `feature`. Rama `feat/man-39-refactorization-reentry-cli-collapse`. `persist_ref` bajo `docs/features/`. **L-PROCESS**. |
| Init | `workspace-init` executed. `lifecycle_forge_started` → MAN-39. `Work_Initiated` `2ad9624c-515a-4229-8289-548bca449a12`. |
| Tren agente | Mayeuta `simulated` (`agentes IDE; sin SDDIA_AGENT_RUNTIME_COMMAND`). Dedalo…DCC `skipped` / `prior_agent_phase_not_executed`. |
| Halt pedido | `SDDIA_TQM_STOP_AFTER=design` no cortó en Dedalo: la barrera salta antes porque el diseño no llega a `executed`. |
| Peaje vivo | `thermodynamic_toll.llm_process_peaje` escribió `docs/audits/MAN-39.md` fuera de `persist_ref`. |
| Worktree | `SDDIA_LAB_ALLOW_DIRTY=1`. Viajan sucios ajenos (genoma de tools/daemon y docs de MAN-40). No son alcance de este ciclo. |

## D1 — Matriz PBI × código vivo

| Afirmación | Laudo |
|------------|-------|
| Relanzar el ciclo con forja ya en rama reentra Mayeuta → Dedalo → Tekton | **L-REENTRY**. `persist_forge_artifacts_ready` solo mira `implementation.md` + `execution.md` para cerrar forja al entrar en Verificación. No hay salto que omita las fases agente de forja. `skip_agent_phase_entry` solo aplica a `pull-request-review` / `docs-only`. |
| Timeout de Ejecución lo fija la bóveda y debe ser ≥ 2400 s | **L-TIMEOUT**. `resolve_timeout_secs`: fase que empieza por `ejecuc` puede leer `SDDIA_AGENT_RUNTIME_TIMEOUT_SECS_EJECUCION`; si no, cae a `SDDIA_AGENT_RUNTIME_TIMEOUT_SECS` o al compilado **660**. Un export de sesión no es SSOT: `apply_env` no pisa una clave ya presente. |
| Abortar el invocador deja hijos y no hay envelope | **L-ABORT**. `kill_process_group` solo corre en timeout del `wait`. No hay limpieza de `sddia-workspace-server` ni envelope si el padre muere por señal. |
| El peaje `docs/audits/{tracker_ref}.md` tumba el siguiente `workspace-init` | **L-DIRTY**. `path_in_scope` admite `persist_ref` y el path exacto de `pbi_ref`. No admite `paths.auditsPath`. Esta pasada ya materializó el espécimen MAN-39. |
| `SDDIA_TQM_STOP_AFTER=design` no abre puerta a Argos/DCC | **L-RESUME**. La barrera `stop_after` exige fase Dedalo con status `executed`. No existe entrada que, con forja ya en rama, arranque en Verificación. |
| El equipo del `parent_ref` debe tener las labels del mapa antes de crear el hijo | **L-LABEL**. `run_create_issue` resuelve el equipo del padre y `resolve_label_ids` falla `LINEAR_LABEL_UNKNOWN` antes del `issueCreate`. El catálogo lab de MAN sigue siendo `Hu`, `Pbi`, `Fix` (sin `Kaizen`). Prohibido alias tipográfico. |

## D2 — Laudos cerrados

| ID | Decisión |
|----|----------|
| **L-PROCESS** | Vehículo de este ciclo: `feature` / `branch_pr` / `docs/features/man-39-refactorization-reentry-cli-collapse`. El `process: refactorization` del PBI queda como metadato de forja. El defecto vive en el ejecutor compartido: el arreglo cubre `feature`, `bug-fix` y `refactorization`. |
| **L-REENTRY** | Si `persist_ref` ya tiene `implementation.md` y `execution.md`, el relanzamiento no invoca Mayeuta, Dedalo ni Tekton de forja. El acuse marca esas fases `skipped` con razón de forja presente y sigue hacia Verificación. |
| **L-SAME-EXECUTION** | Mismo `execution_id` o la misma huella de artefactos en `persist_ref`. No se exige reabrir el cuerpo de MAN-36. |
| **L-TIMEOUT** | Para Ejecución, el valor efectivo sale de la bóveda y es ≥ 2400 s. El compilado 660 no puede ganar si la bóveda trae un valor mayor. Una clave heredada vacía cuenta como ausente y no bloquea la bóveda. Un export de la sesión del IDE no sustituye el fichero de bóveda. |
| **L-ABORT** | Si el invocador muere o aborta durante la fase agente, el runtime hijo y el `sddia-workspace-server` de ese `execution_id` terminan, y el proceso emite envelope `success: false`. |
| **L-DIRTY** | El peaje en `{paths.auditsPath}/{tracker_ref}.md` entra en el scope del dirty-gate del mismo flujo. No se muda el SSOT del peaje. |
| **L-RESUME** | Con forja ya en rama, hay entrada explícita a Argos y, si la validación queda APTO, a `delivery-close-cycle`, sin repetir diseño ni Ejecución. `STOP_AFTER=design` sigue cortando cuando Dedalo sí ejecutó y aún no hay forja. |
| **L-LABEL** | `forge-pbi` compara las labels pedidas con el catálogo del equipo del `parent_ref`. Falta de label → `LINEAR_LABEL_UNKNOWN` y cero issue creado. `Kaicen` no satisface `Kaizen`. |
| **L-NON-GOALS** | No reabrir el DoR de MAN-36. No cambiar producto de event-manager, bóveda de store ni contrato ECST. No commitear los sucios ajenos que viajaron con `ALLOW_DIRTY`. |
| **L-GIT** | Git solo vía `skill:git-manager`. |
| **L-GENOME** | Contratos `.md` bajo directorios de genoma solo vía `entity-manager`. |
| **L-TRACKER** | Provider `linear`. Sin ficheros nuevos en `docs/todos/pending/` ni `done/`. |

## Veredicto

`mayeuta_verdict: ok`. Sin preguntas abiertas. Dedalo puede diseñar sobre estos laudos.
