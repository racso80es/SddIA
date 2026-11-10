---
feature_name: man-39-refactorization-reentry-cli-collapse
created: "2026-11-10"
process: feature
purpose: El ciclo lifecycle no reentra el tren de forja si la rama ya tiene la entrega documental; timeout, aborto, peaje y labels quedan alineados
branch_name: feat/man-39-refactorization-reentry-cli-collapse
persist_ref: docs/features/man-39-refactorization-reentry-cli-collapse
pbi_ref: MAN-39
historia_ref: MAN-36
document_id: PBI-70fb886c
status: requisitos-estabilizados
mayeuta_verdict: ok
phase: mayeuta-stabilization
norm_ref: features-documentation-pattern@1.3.1
forge_key: "2e7baef71d4c29dc25c07db567099259c855c06271d36cd0ae854ba35cb5211b"
execution_id: "bd841ef3-fd25-43bd-a038-a6fe2f5efc41"
---
# Objetivos — man-39-refactorization-reentry-cli-collapse

## Misión

Cerrar la fricción de tren y CLI descrita en **MAN-39** (documento **PBI-70fb886c**, historia **MAN-36**): un ciclo `feature` / `bug-fix` / `refactorization` con forja ya materializada no vuelve a recorrer Mayeuta → Dedalo → Tekton; el timeout de Ejecución obedece a la bóveda; abortar el invocador no deja hijos; el peaje termodinámico no ensucia el dirty-gate; existe reanudación hacia Argos/DCC; `forge-pbi` exige las labels reales del equipo padre.

**Por qué:** el relanzamiento de `refactorization` tras la forja de MAN-36 reejecutó el tren, el CLI no acusó hasta agotar el timeout, y el peaje en `docs/audits/` bloquea el `workspace-init` siguiente. Esta pasada lo reprodujo en init: peaje escrito en `docs/audits/MAN-39.md` y diseño nunca alcanzado.

## Contexto estabilizado

| Hecho | Impacto |
|-------|---------|
| Tracker | **MAN-39**. Canal Linear. Sin cola local de backlog. |
| Semilla | **PBI-70fb886c**. Metadato `process: refactorization`. Ciclo runtime **`feature`**. |
| Historia | **MAN-36**. No se reabre. |
| Ejecutor | `executor.rs`: barrera `stop_after` solo si Dedalo queda `executed`. |
| Forja detectada | `persist_forge_artifacts_ready` = `implementation.md` y `execution.md` en `persist_ref`. Hoy no salta las fases de forja. |
| Timeout | Default compilado 660 s. Bóveda vía `apply_env` solo si la clave no existe ya en el entorno. |
| Aborto | Muerte del grupo solo en timeout del wait del agente. |
| Peaje | `maybe_record_process_peaje` → `{auditsPath}/{tracker_ref}.md`. Fuera de `path_in_scope`. |
| Labels | `run_create_issue` usa el equipo del `parent_ref` y falla antes de crear si la label no está. |
| Clarificaciones | `clarify.md` (laudos L-* , veredicto **ok**). |
| `execution_id` | `bd841ef3-fd25-43bd-a038-a6fe2f5efc41` |

## Alcance

| Tajo | Objetivo |
|------|----------|
| **T0** | Salto de reentrada cuando la forja documental ya está en `persist_ref`. |
| **T1** | Timeout de Ejecución leído de bóveda, suelo 2400 s, clave vacía = ausente. |
| **T2** | Aborto del invocador: envelope de fallo y fin del runtime hijo y del workspace-server de ese `execution_id`. |
| **T3** | Dirty-gate admite el peaje `docs/audits/{tracker_ref}.md` del mismo tracker. |
| **T4** | Entrada explícita a Verificación/DCC con forja presente, sin repetir diseño. |
| **T5** | Contrato de labels del padre: nombre exacto, fallo antes de crear, catálogo lab MAN con `Kaizen`. |

## Objetivos medibles

| ID | Criterio de cierre |
|----|--------------------|
| **KA-REENTRY-1** | Relanzar el ciclo con `implementation.md` y `execution.md` ya en `persist_ref` no invoca Mayeuta ni Dedalo. El acuse indica salto a Verificación. |
| **KA-REENTRY-2** | Con bóveda `SDDIA_AGENT_RUNTIME_TIMEOUT_SECS` ≥ 2400, una Ejecución > 660 s y < timeout de bóveda no termina en `agent-runtime-timeout` por el default compilado. |
| **KA-REENTRY-3** | Abortar el invocador deja envelope `success: false` y cero procesos `kalma2-agent-runtime-cursor` / `sddia-workspace-server` de ese `execution_id`. |
| **KA-REENTRY-4** | Tras el peaje que escribe `docs/audits/{tracker_ref}.md`, el siguiente `workspace-init` del mismo flujo no aborta por `dirty-worktree` atribuible a ese fichero. |
| **KA-REENTRY-5** | Con forja ya en rama, hay camino a Argos/DCC que no reejecuta diseño ni Ejecución. `STOP_AFTER=design` sigue cortando si Dedalo ejecutó y aún no hay forja. |
| **KA-REENTRY-6** | `forge-pbi` cuyo equipo padre no tiene la label pedida falla con `LINEAR_LABEL_UNKNOWN` y no crea issue. Una grafía distinta no cuenta como la label del mapa. |

## Ley aplicada

- Git solo vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tool.
- Genoma `.md` solo vía `entity-manager`.
- Provider Linear: sin alta en `docs/todos/`.
- Fuera de alcance: cuerpo DoR de MAN-36 y producto event-manager.

## Parada de esta pasada

Planificación forjada (`clarify.md`, `objectives.md`, `spec.md`, `plan.md`). Ejecución Tekton no iniciada.
