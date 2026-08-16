---
feature_name: process-creator-full-contract-forge
created: "2026-08-16"
process: bug-fix
base: main
branch: fix/process-creator-full-contract-forge
scope: EV-AUD-003
pbi_ref: docs/todos/pending/[FIX] process-creator — materialización contractual completa (EV-AUD-003).md
---

# Spec — process-creator materialización contractual (EV-AUD-003)

## Causa raíz

`run_process_forge` (CREATE) calculaba `hash_signature` sobre `process_phases` recibidas y escribía un stub YAML con `Fase inicial`. El hash no describía el artefacto. `workspace_template`, `inputs` y `outputs` no se persistían.

Precisión: no es «fases dentro de `workspace_template`». Son campos independientes omitidos por la forja.

## Fuera de alcance

- Circuito Daemon_Heartbeat / ola 20260812 (PR #177).
- Jurisdicción `process_domain_roots` (D7, ya cerrado).

## Cambios

1. CREATE serializa las `process_phases` efectivas (serde_yaml), no un literal `Fase inicial`.
2. Persistir `workspace_template`, `inputs`/`process_inputs`, `outputs`/`process_outputs`, `aliases`, `phase_invocations` si vienen.
3. Abortar y borrar el artefacto si las `phases` leídas ≠ las solicitadas.
4. Hash sigue `sha256_canon` sobre `process_phases` (paridad crypto-broker / lab SHA256).

## Criterios

| ID | Criterio |
|----|----------|
| CA1 | CREATE con `process_phases` escribe esas fases; cero `Fase inicial` |
| CA2 | `workspace_template`, inputs y outputs solicitados aparecen en el YAML |
| CA3 | `hash_signature` se calcula sobre las fases usadas en el write |
| CA4 | Jurisdicción Core/domain y L-UNIQ-MULTI intactos |
| CA5 | Test `ev_aud_003_create_persists_requested_phases_not_stub` |
