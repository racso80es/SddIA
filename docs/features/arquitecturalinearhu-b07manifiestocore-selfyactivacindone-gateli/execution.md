---
feature_name: arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli
created: "2026-10-14"
process: feature
document_id: PBI-LINEAR-B-07-CORE-SELF-GATE
pbi_uuid: "9b4e7c12-5a38-4f19-8d64-3e2f1a9c0b85"
persist_ref: docs/features/arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli
correlation_id: "fd2db080-dbf3-44ab-b70f-63b90d748455"
branch_name: feat/arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli
phase: execution
agents: tekton
verdict: blocked
execution_id: "f351761b-9b35-4e44-bd81-ae99fd259679"
---
# Ejecución — HU-B 07

## Veredicto

**blocked.** T1–T3 verificados en disco (paridad spec §4; UUID Core canónico intacto). T4 y T5 **sin evidencia**: Shell IDE rechazado de forma absoluta (probe, cargo, git-manager, subagente). HEAD actual: `docs/cerrar-pbi-kalma2-branch-before-mutate` — no la rama del spec. No se inventa éxito de suite ni `hash_integrity`.

## Tabla AC (estado)

| ID | Criterio | Estado |
|----|----------|--------|
| AC-24a | Índice + manifiesto Core-self `linear` + `project_root: .` | Manifiestos OK en disco; `cargo test project_binding` **no ejecutado** |
| AC-24b | Con `tracker_ref` → gate `Linear` + proof pre-push | Código/tests presentes; suite **no ejecutada** |
| AC-24c | Sin `tracker_ref` → `Git` (AC-17) | Código/tests presentes; suite **no ejecutada** |
| AC-24d | D-I otros proyectos / fixture both | Estático OK; e2e **no ejecutado** |

## Orden vs plan

| Fase | Estado |
|------|--------|
| T1 bind + prosa contrato | Verificado en disco |
| T2 degrade `resolve_done_gate` | Verificado en disco |
| T3 manifiestos Core-self | Verificado en disco (UUID canónico) |
| T4 cargo suite | **blocked** (Shell) |
| T5 evolution + cierre | **blocked** (Shell / rama incorrecta / Argos) |

## Comandos pendientes (desbloqueo)

```bash
./sddia-run.sh --tool git-manager   # checkout feat/arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli
cd SddIA && cargo test -p execute-process project_binding
cd SddIA && cargo test -p execute-process done_gate
cd SddIA && cargo test -p execute-process linear_done_gate_both_e2e
# evolution + sddia-qa evolution-rehash + gate-evolution --range
```

## Desbloqueo

Sesión con Shell operativo → checkout rama feat → T4 → T5 del `plan.md`.
