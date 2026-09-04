---
feature_name: centinelas-fracture-ola-20260901
created: "2026-09-04"
process: bug-fix
items:
  - archive-five-fracture-pbis
  - laudo-b-no-regression-verify
  - no-genome-mutation
  - vitality-probe-segregated
---

# Implementation — centinelas-fracture-ola-20260901

## Laudo

**(B) deuda documental** — sin causa raíz residual en genoma. Gate empírico no tumbó el laudo; **genoma intacto**.

Mayeuta (`refactor_tool` / backfill EDA) **descartado**: el cubo `huérfan` colisiona con la traza canónica `Centinela {id} lock huérfano`. Cubo `orphan_lock` queda como Kaizen aparte; no bloquea este archivo.

## Touchpoints

| Artefacto | Cambio |
|-----------|--------|
| `docs/todos/done/[FIX] * (ace57b065f9b\|6cc3b954bad3\|4f209670a96f\|3d326490b80d\|19bfe7cf3371).md` | Archivados `status: cerrado`, `fix_ref: docs/fixes/centinelas-fracture-ola-20260901` |
| `docs/todos/pending/` (mismos document_id) | Eliminados tras archivo canónico en `done/` |
| `SddIA/evolution/70b29d72-b36e-4055-830b-e2809047f0b2.md` | Hito de no-regresión + cierre documental |
| Genoma (`start-sddia.sh`, daemons, `daemon_heartbeat.rs`, umbrales, Mayeuta) | **Sin mutación** |
| `PBI-FIX-FRACTURE-7bc20a6b4dd6` | **No tocado** — ciclo A separado (`sddia-qa` / sonda tools_index) |

## Evidencia runtime (CA1/CA2)

Fuente: `daemons_instance` → `.SddIA/daemons/{status,state}` @ 2026-09-04T09:33:54Z

| Centinela | Lock PID | Vivo | missed_cycles | last_heartbeat_at |
|-----------|----------|------|---------------|-------------------|
| event-sweeper (oblig.) | 67914 | sí | 0 | 2026-09-04T09:33:53Z |
| event-watcher (oblig.) | 67844 | sí | 0 | 2026-09-04T09:33:53Z |
| email-watcher | 75943 | sí | 0 | 2026-09-04T09:33:46Z |
| github-bridge-watcher | 75938 | sí | 0 | 2026-09-04T09:33:46Z |
| iota-publish-relay | 75932 | sí | 0 | 2026-09-04T09:33:50Z |
| telegram-watcher | 75937 | sí | 0 | 2026-09-04T09:33:25Z |
| kalma2-bridge | 67968 | sí | 0 | 2026-09-04T09:33:53Z |
