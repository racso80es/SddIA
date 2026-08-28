---
feature_name: kaizen-feature-lab-init-frictions
created: "2026-08-28"
process: feature
phases: "T0-docs,T1-timeout-burial,T2-relay-vault,T3-exec-id,T4-daemon-circuit,T5-hygiene,T6-tests-smoke,T7-aduana-doc"
uuid: "58e3c9f7-0e90-4e51-8b87-a9054a9b30fe"
persist_ref: docs/features/kaizen-feature-lab-init-frictions
branch_name: feat/kaizen-feature-lab-init-frictions
execution_id: "80a3ca0d-80c5-4662-ab12-2afe757478c8"
dedalo_verdict: ok
pbi_ref: docs/todos/pending/[KAIZEN] Init lab feature — bóveda reinyecta AGENT_RUNTIME y carrera de agentes.md
document_id: PBI-KAIZEN-FEATURE-LAB-INIT-FRICTIONS
---

# Plan — kaizen-feature-lab-init-frictions

Orden no negociable: **techo+entierro (T1) antes de desbloquear relevo (T2)**. Un PR. Git solo `skill:git-manager`. **Tekton no arranca en T0.**

## T0 · Documentación (esta entrega Dedalo)

- `clarify.md`, `objectives.md`, `spec.md`, `plan.md` bajo `persist_ref`.
- `execution_id: 80a3ca0d-80c5-4662-ab12-2afe757478c8` = workspace vivo.
- Topología DA-4 activa. Estímulo cierra aquí.

## T1 · Timeout + entierro + reentrada (L-TIMEOUT-MOTOR, L-PGID-BURIAL, L-REENTRY)

1. `agent_runtime.rs`: wait acotado; default 660s; `error: agent-runtime-timeout`.
2. Spawn en PGID; kill grupo en timeout.
3. Inyectar `SDDIA_AGENT_RUNTIME_DEPTH`; skip spawn si ≥1.
4. Tests unitarios unix + caso timeout con comando `sleep`.
5. Criterio de salida: CLI no puede colgarse indefinidamente; huérfanos del grupo = 0.

## T2 · Relé + paridad bóveda (L-RELAY-FLAG, L-VAULT-PARITY)

1. `is_configured()` consulta flag de relevo primero.
2. Log stderr al activarse.
3. `_sddia_load_vault` setdefault + misma lista de precedencia que `env.rs`.
4. Norma obrera: update `external-ai-constraints.md` vía `norm-creator` (lab/CI: `./sddia-run.sh --process entity-manager`).
5. Criterio: LAB-CA1–CA3.

## T3 · Trazabilidad (L-EXEC-ID, L-CONFLICT)

1. Payload + prompt + handoff + stub `workspace_init` escriben `execution_id` del motor.
2. Guard de conflicto en persist_ref.
3. Criterio: LAB-CA6–CA7. Auditoría: `ls .SddIA/workspaces/feature/{id}/`.

## T4 · Circuito daemon (L-DAEMON-CLASS, L-DAEMON-NOISE, L-DAEMON-SCOPE, L-INDEX-CENSUS)

1. `process-creator` update `entity-manager` (enum + texto piloto 10 clases).
2. Espejo Rust `PILOT_CLASSES` / `creator_name` / `dir_by_class`.
3. Extirpar fail-soft `residual_runner`.
4. `run_daemon_forge`: actualizar censo del pie = N filas. Corregir pie actual (6, no cinco) en el mismo cambio de helper.
5. `daemon-creator` + `daemons-contract`: declarar alcance forja vs delivery; vía canónica wrapper `scripts/daemons/`.
6. Criterio: LAB-CA8–CA9. Si forja de genoma bloquea → **parar y escalar**; prohibido bisturí de `{name}.md`.

## T5 · Higiene WT y PBI (L-DIRTY-INIT, L-TODOS-PRESERVE)

1. Gate porcelain al inicio de `workspace_init`.
2. Snapshot: no add/rm de `??` `docs/todos/` ajenos.
3. Criterio: LAB-CA10–CA11.

## T6 · Smokes aceptación

Ejecutar la tabla §5 de `spec.md` (T-RELAY … T-TODOS). Fallo de T1/T2 es criterio de parada: no T4–T5 con runtime aún huérfano.

## T7 · Cierre documental (ciclo de implementación, no este estímulo)

- `implementation.md` / `execution.md` / `validacion.md` APTO.
- PBI → `docs/todos/done/` en la **misma** rama.
- `delivery-close-cycle` un PR.

## Criterio de parada de este estímulo

`plan.md` materializado con `execution_id` resoluble. Prohibido abrir fase Ejecución (Tekton) hasta nuevo mandato del Vértice Biológico.
