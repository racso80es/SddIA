---
feature_name: ppr-revoked-registry-rehab-kaizen-aduana-evolution
created: "2026-08-29"
process: refactorization
phase: planning
agents: dedalo
phases:
  - T0-hollow-a2
  - T1-instance-rehab
  - T2-evolution
  - T3-argos
  - T4-doc-archive
  - T5-delivery-close
branch_name: refactor/ppr-revoked-registry-rehab-kaizen-aduana-evolution
persist_ref: docs/features/ppr-revoked-registry-rehab-kaizen-aduana-evolution
pbi_ref: docs/todos/pending/PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL-PPR-REVOKED-REGISTRY.md
document_id: PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL-PPR-REVOKED-REGISTRY
uuid: c4e8f1a2-9b3d-4f7e-a6c1-2d8e5f0b3a71
ola: A1
olas:
  - A1
  - A2
runtime_execution_id: "aa0d1244-043a-421f-9b60-efb76c4985ca"
---

# Plan — ppr-revoked-registry-rehab-kaizen-aduana-evolution

Blueprint Tekton. Contratos: `spec.md`. **Stop planning:** no ejecutar T0–T5 en esta sesión.

Init lab: `execution_id` `aa0d1244-043a-421f-9b60-efb76c4985ca` · vehículo `feature` · `process_label: refactorization` · relevo IDE.

## T0 — A2 motor (AC-A2-HOLLOW / AC-A2-TESTS) — **si laudo**

1. `thermodynamic.rs`: asegurar `failed_phase_code` en payload KO (ya existe; no reabrir semántica).
2. `radamanto_batch_core.rs`: extender `is_survival_hollow` per **L-A2-HOLLOW**.
3. Tests `t_a2_hollow_*`. Assert podas `lab_hollow` / `detach` / `detached_child` / `cycle_phase` intactas.
4. **Prohibido:** `phase_terminal.rs`, umbrales, YAML `pull-request-review.md`.
5. Sin laudo: omitir T0; abrir PBI hijo con este diagnóstico.

## T1 — A1 instancia (AC-A1-* / AC-GIT-CLEAN)

Locus Cúmulo: `radamanto.revoked_entities` = `.SddIA/cerbero/revoked_entities.json`; `radamanto.stats` = `.SddIA/radamanto/stats.json`. **Fuera del diff git.**

1. DELETE `revoked.pull-request-review`. Assert `permanent.pull-request-review` ausente.
2. Reset absoluto bucket raíz `pull-request-review` (**L-RESET-ABS** + **L-SAMPLES** + laudo este `document_id`).
3. Assert laterales `revoked.{bug-fix,refactorization}` intactos.
4. Smoke PPR (o handoff) sin re-revocación inmediata; `execution_id` en `execution.md`.
5. Evidencia (campos/timestamp, no secretos) en `execution.md`.

## T2 — Documental + evolution

1. `implementation.md` + `execution.md` (frontmatter patrón; `items` / `items_applied`).
2. Entrada `directories.evolution` UUID `c4e8f1a2-9b3d-4f7e-a6c1-2d8e5f0b3a71` (una por ciclo).
3. Assert diff: **no** `.SddIA/cerbero/` ni `.SddIA/radamanto/` ni umbrales.

## T3 — Argos

`validacion.md`: `global`, checks AC-*, `git_changes`, `pbi_archived: true`, `branch: refactor/ppr-revoked-registry-rehab-kaizen-aduana-evolution`.

## T4 — Archive PBI

Mover canónico `docs/todos/pending/PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL-PPR-REVOKED-REGISTRY.md` → `docs/todos/done/`. **L-DEDUP:** un solo path.

## T5 — DCC

`delivery-close-cycle` · `source_process: feature` (vehículo) / `process_label: refactorization` · `persist_ref` · `branch_name`.

Git: `skill:git-manager`.

## Orden

```text
[T0 si laudo A2] → T1 → T2 → T3 → T4 → T5
```

A1 no espera T0.

## Delegaciones

| Fase | Cápsula |
|------|---------|
| A2 engine | Tekton crate `execute-process` (no genoma DA-2) |
| A1 FS | Tekton `filesystem-ops` |
| Git | `skill:git-manager` |
| PR | `action:execute-process` → `delivery-close-cycle` |

## Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Receta #190 (sin poda) | **L-SAMPLES** bloqueante |
| `structure_valid` false | **L-RESET-ABS** |
| Vehículo revocado | **L-VEHICLE** |
| Instancia en PR | AC-GIT-CLEAN |
| A2 sin evidencia F4 | **L-A2-SPLIT** |

## Fuera de este plan

Rehab laterales; umbrales; ejecución T0–T5 esta sesión.
