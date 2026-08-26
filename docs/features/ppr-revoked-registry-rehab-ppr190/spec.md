---
feature_name: ppr-revoked-registry-rehab-ppr190
created: "2026-08-26"
process: refactorization
phase: design
agents: dedalo
base: main
scope: rehab-ppr-cerbero + survival-hollow-detached-child
branch_name: refactor/ppr-revoked-registry-rehab-ppr190
persist_ref: docs/features/ppr-revoked-registry-rehab-ppr190
pbi_ref: docs/todos/pending/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #190).md
document_id: PBI-PPR-190-REVOKED-REGISTRY
uuid: e2b9a4f1-7c83-4d5e-9a16-0f8b3c5d7e21
version_spec: "1.0.0"
status: dedalo_locked
olas:
  - A1
  - A2
source_correlation_id: "5a4683c0-db46-4e8e-b5f4-b865ba417e0d"
source_pr_url: https://github.com/racso80es/SddIA/pull/190
parent_pbi: docs/todos/done/[ARQUITECTURA] umbrales Radamanto process — rehabilitación revoked_entities (PPR #174+#177).md
---

# Spec — ppr-revoked-registry-rehab-ppr190

## 1. Misión

Rehabilitar `pull-request-review` (buckets `permanent` + `revoked`) y cortar re-muerte por KO de hijos foreground post-CLI-detach (DA-5 Fire-and-Forget).

## 2. Laudos

| Ref | Decisión |
|-----|----------|
| **L-REHAB-INST** | A1 instancia; evidencia `execution.md`; `.SddIA/` fuera del diff |
| **L-CERBERO** | Borrar `permanent.pull-request-review` y `revoked.pull-request-review` |
| **L-STATS** | Reset bucket raíz: `healthy`, `recovery_attempts: 0`, poda KO, laudo `PBI-PPR-190-REVOKED-REGISTRY` |
| **L-PPR-CYCLE** | Incluir `pull-request-review` en emisión `cycle_phase` REF (paridad lifecycle) |
| **L-PPR-DETACH** | REF hijo con `SDDIA_DETACHED_EXECUTION_ID` → `detached_child: true` |
| **L-HOLLOW-DETACH** | `is_survival_hollow`: `detached_child` + `exit_code≠0` → skip; `detach: true` → skip |
| **L-THRESH** | Umbrales 1.1.0 intactos |
| **L-AGGREGATOR** | `phase_terminal.rs` intacto |

## 3. Touchpoints

| Artefacto | Mutación |
|-----------|----------|
| `thermodynamic.rs` | PPR en `LIFECYCLE_PROCESSES`; `detached_child` en payload REF |
| `radamanto_batch_core.rs` | Extender `is_survival_hollow`; tests |
| `.SddIA/cerbero/revoked_entities.json` | A1 (no PR) |
| `.SddIA/radamanto/stats.json` | A1 (no PR) |
| `SddIA/evolution/e2b9a4f1-7c83-4d5e-9a16-0f8b3c5d7e21.md` | Registro UUID |

## 4. AC

| AC | Verificación |
|----|--------------|
| AC-A1 | PPR ∉ revoked/permanent; stats healthy; evidencia execution |
| AC-A2 | Tests `ppr_detached_child_failure_is_hollow`, `derive_ppr_simulated_is_initialized` |
| AC-DOC | PBI done + validacion APTO |
