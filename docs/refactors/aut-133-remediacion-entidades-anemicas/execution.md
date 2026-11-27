---
feature_name: aut-133-remediacion-entidades-anemicas
created: "2026-11-27"
process: refactorization
branch_name: refactor/aut-133-remediacion-entidades-anemicas
persist_ref: docs/refactors/aut-133-remediacion-entidades-anemicas
tracker_ref: AUT-133
historia_ref: AUT-125
pbi_ref: PBI-81e91c67
execution_id: "5ea79c56-a610-4223-b2b3-7498a510d239"
closed_phases:
  - T1-rama
  - T2-ola0-inventario
  - T3-uuid-politica-prep
pending_phases:
  - T4-remediacion-olas
  - T5-promocion-ci
  - T6-evolution
  - T7-validacion
  - T8-cierre
verdict: blocked
---
# Ejecución — AUT-133

## Baseline hermético Ola 0

| Campo | Valor |
| --- | --- |
| Rama | `refactor/aut-133-remediacion-entidades-anemicas` |
| Tip SHA (rama/`main`) | `3aaaa2668bfb040bf6cce37a9b145b7ca8e4dbcd` |
| Tip semilla inventario | `fd7ef31b51e795d1aaea2aba53b7cae693a019db` |
| `tip_match_seed` | `false` |
| `aggregates_match_seed` | `true` |
| Worktree (MCP `git_status`) | dirty untracked: `docs/audits/AUT-133.md` + `docs/refactors/aut-133-remediacion-entidades-anemicas/` |
| `violation_count` | **322** (142 entidades / 207 escaneadas) |
| `by_code` | CONTRACT_SECTION_MISSING 218 · SECTIONS_INSUFFICIENT 79 · UUID_NOT_V4 19 · UUID_INVALID 6 |
| `by_family` (desc) | process 146 → action 47 → tool 46 → skill 29 → daemon 21 → norm 10 → event 7 → suite 7 → codex 5 → agent 4 |
| AUT-132 | merged (#482) |
| AUT-131 | merged (#481) |
| `oracle-red.log` | **presente** (`mode: all`, `violation_count: 322`) |
| `live_gate_capture` | `captured` (metadatos en inventario; sha256 `3467f906…03561`) |

## Fases

| Fase | Estado | Nota |
| --- | --- | --- |
| T1-rama | hecho | HEAD en rama objetivo; tip alineado con `main` local |
| T2-ola0-inventario | **hecho** | `pre-remediation-inventory.json` + `oracle-red.log` con `violation_count: 322` (KA-DOC-1) |
| T3-uuid-politica | prep | `uuid-migration-register.md` 25 filas `migrate`; 0 rotaciones |
| T4-remediacion-olas | **bloqueado** | `entity-manager` / `execute-process` / `sddia-qa` no invocables vía MCP `run_check` |
| T5-promocion-ci | **bloqueado** | Requiere `violation_count: 0` sin plantilla |
| T6-evolution | pendiente | Tras remediación |
| T7-validacion | pendiente | Argos |
| T8-cierre | pendiente | PR `branch_pr` |

## Bloqueo (causa raíz)

En esta sesión Tekton, `run_check` denegó: `sddia-run.sh`, `./sddia-run.sh`, `cargo`, `sddia-qa`, `execute-process`, `SddIA/target/debug/sddia-qa`, `python`/`python3`/`node`/`npm`/`gh`. `git` raw → «route via git-manager».

Sin canal `action:execute-process` → `process:entity-manager`, **AC-2/AC-3 no pueden materializarse** sin violar DA-2 (edición directa del genoma prohibida). Stop antes de T4; genoma/CI intactos.

## Estado AC

| AC | Estado | Evidencia |
| --- | --- | --- |
| AC-1 | **ok** | Inventario versionado + `oracle-red.log` con `violation_count: 322` |
| AC-2 | **bloqueado** | 0 entidades remediadas vía `entity-manager` |
| AC-3 | prep / **bloqueado** | Registro UUID; 0 migraciones aplicadas |
| AC-4 | no iniciado | CI sigue en `--if-touched` |
| AC-5 | abierto | Evolution no sellada |

## Prohibiciones respetadas

- No plantilla compartida / no `remediate_bulk.py`.
- No rotación silenciosa de UUID.
- No promoción CI con deuda abierta.
- No escritura en `docs/todos/` desde Tekton.
- No reescritura de `_agent_handoff.md`.
- No mutación de genoma fuera de `entity-manager`.

## Veredicto

**blocked** — Ola 0 (AC-1) y prep UUID (T3) materializados; remediación masiva gobernada (T4+) pendiente de allowlist/`execute-process` nativo.
