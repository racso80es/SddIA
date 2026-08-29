---
feature_name: arch-immunological-system
created: "2026-08-29"
process: feature
phase: planning
agents: dedalo
phases:
  - T0-thresholds-ssot
  - T1-suspend-discriminate
  - T2-handler-tests
  - T3-phagocyte
  - T4-process-em-update
  - T5-evolution
  - T6-doc-execution
  - T7-delivery-close
branch_name: feat/arch-immunological-system
persist_ref: docs/features/arch-immunological-system
pbi_ref: docs/todos/pending/PBI-ARCH-IMMUNOLOGICAL-SYSTEM.md
document_id: PBI-ARCH-IMMUNOLOGICAL-SYSTEM
uuid: 056ac6a1-02fc-4988-a704-1f5b648d0e40
runtime_execution_id: "987e1747-bd08-4c80-ad41-648f09cc4b12"
---

# Plan — arch-immunological-system

Blueprint Tekton. Contratos: `spec.md`. **Stop planning:** no ejecutar T0–T7 en esta sesión.

Init lab: `execution_id` `987e1747-bd08-4c80-ad41-648f09cc4b12` · vehículo `feature` · relevo IDE.

## T0 — SSOT umbrales

1. Crear `SddIA/daemons/heartbeat-audit.thresholds.json` (`missed_cycles_threshold: 3`, `suspend_skew_seconds: 120`).
2. Clave Cúmulo `argos.heartbeat_audit_thresholds` en `SddIA/core/cumulo.paths.json` apuntando a esa ruta. Lectura siempre vía Cúmulo; overlay `{daemons_instance.state}/heartbeat-audit.thresholds.json`.
3. Loader en motor (`execute-process`): merge overlay sobre default; fail-soft a default si JSON inválido + log.
4. **Prohibido:** escribir umbrales en `radamanto.thresholds.json`.

## T1 — Discriminación suspend/crash

Locus: `SddIA/engine/execute-process/src/engine/handlers/daemon_heartbeat.rs`.

1. Persistir `last_audit_wall_at` + `last_audit_mono_ms` en `heartbeat-audit.json`.
2. Calcular `skew`; si `>= suspend_skew_seconds` y mono comparable → reanclar daemons del sweep, `suspend_reanchored`, **cero** `emit_system_fracture`.
3. Sustituir `MISSED_CYCLES_THRESHOLD` por valor SSOT.
4. `classification` por entrada daemon: `healthy` \| `host_suspend` \| `stale` (pre-fractura) — sin nuevo evento ECST.
5. **Prohibido:** `sleep`/hold EDA; D-Bus logind como dependencia dura; Radamanto.

## T2 — Tests handler

`cargo test -p execute-process daemon_heartbeat`. Casos spec §8. No romper tests de baseline `c6931c73`. Evidencia stdout → `execution.md` (fase ejecución, no ahora).

## T3 — Fagocitosis

1. Ledger `{daemons_instance.state}/phagocytosed-fractures.json` desde el sweep sano (siempre).
2. Predicado spec §5.2 sobre `paths.todos.pending`.
3. Handler + proceso `phagocyte-recovered-fracture-pbis` (`entity-manager` create; **no** Write sobre `SddIA/process/`).
4. `apply` default false; `SDDIA_PHAGOCYTE_APPLY=1` para mover pending→done + ola `docs/fixes/centinelas-fracture-ola-{YYYYMMDD}/`.
5. Tests predicado en tmp. **Prohibido:** `delivery-close-cycle` desde el fagocito; `SDDIA_SKIP_HOOKS`.

## T4 — Update `daemon-heartbeat-audit`

`./sddia-run.sh --process entity-manager` `lifecycle_operation: update` sobre `daemon-heartbeat-audit`. Documentar SSOT, skew, fagocito, Argos-only. Bump SemVer. EDA coverage si el creator lo exige.

## T5 — Evolution

`{uuid}.md` v1.1.2 + `sddia-qa evolution-rehash`. `relacionado`: PBI `056ac6a1-…`, evoluciones régimen A+B+C+D y cold-start. Gate `gate-evolution --range` antes de push si el diff toca `directories.evolution`.

## T6 — Documental de ejecución

`implementation.md` + `execution.md` (patrón v1.2.1). No `validacion.md` hasta T7.

## T7 — Cierre (sesión posterior)

Cierre documental en rama: PBI → `docs/todos/done/`, `validacion.md` `global: APTO`, `pbi_archived: true`. Luego `delivery-close-cycle`. Un PR.

## Orden y dependencias

```text
T0 → T1 → T2
T3 paralelo a T1 tras T0 (predicado no depende del skew)
T4 tras T1 (texto alineado al código)
T5–T7 al final
```

## Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Falso `host_suspend` por NTP | `suspend_skew_seconds=120`; NTP típico << 120 |
| Auditor reiniciado (mono reset) | No clasificar suspend; missed_cycles sigue gobernando |
| Apply documental en CI | Default `apply=false` |
| Parse traza frágil | Fail-soft: no archivar si ISO ausente |

## Fuera de esta sesión

Código, entity-manager, tests, evolution, PR.
