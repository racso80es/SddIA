---
feature_name: diff-digest-ci
created: "2026-10-30"
process: feature
phase: Verificación
branch: feat/diff-digest-ci
global: NO_APTO
tracker_ref: AUT-40
pbi_archived: false
execution_id: ""
approval_status: integrity_failure
checks:
  AC-VERIFY-DIFF-DIGEST: NO_EVIDENCIA_CAPSULA
  AC-CI-JOB-SDDIA-DIFF-DIGEST: PENDIENTE-CI
  cascade-documental: INCOMPLETA
  capsules-qa: BLOCKED
  git-evidence: BLOCKED
git_changes:
  - docs/features/diff-digest-ci/
  - SddIA/engine/execute-process/src/engine/diff_seal.rs
  - SddIA/tools/sddia-qa/src/verify_diff_digest.rs
  - SddIA/tools/sddia-qa/src/main.rs
  - .github/workflows/sddia-index-qa.yml
  - SddIA/evolution/a7c4e2b1-9f3d-4a8e-b6c5-d1e0f9a8b7c6.md
---

# Validación — AUT-40 (Argos)

## approval_status: integrity_failure

Inyección kalma2 con `process: ?` / `phase: ?` / `persist_ref` vacío / `execution_id` vacío.
Contrato **inferido** (no inyectado): `feature` / `Verificación` / `docs/features/diff-digest-ci` / `feat/diff-digest-ci` / AUT-40 (`objectives.md`).

Peaje Shell **Rejected** (×2 en esta sesión). GetDynamicTools: sin namespace sddia/kalma. Sin stdout de `git-manager` ni `sddia-qa verify-diff-digest`.

## Evidencia de cápsulas

| Cápsula | Resultado |
|---------|-----------|
| `./sddia-run.sh --tool git-manager` | Shell Rejected — sin stdout |
| `sddia-qa verify-diff-digest` | No invocado — peaje Shell ausente |
| CI job `sddia-diff-digest` | Sin `run_id`/URL verde — CA CI no APTO |

**L-ARGOS-SYNTHESIS:** lectura estática de artefactos ≠ evidencia de cápsula. `global: NO_APTO`.

## Hallazgos estáticos (descubrimiento, no veredicto APTO)

1. Cascada bajo `persist_ref`: existen `objectives.md`, `spec.md`, `validacion.md`; **faltan** `clarify.md`, `plan.md`, `implementation.md`, `execution.md` (patrón documental).
2. Workflow: job `sddia-diff-digest` presente en `.github/workflows/sddia-index-qa.yml` (líneas ~399–423); existencia de YAML ≠ run verde.
3. Evolution `a7c4e2b1-9f3d-4a8e-b6c5-d1e0f9a8b7c6` declara alcance AUT-40.
4. Entorno: `docs/todos/pending/[FIX] delivery-close-cycle — fractura sistémica (c499eec7fb3f).md` (fractura DCC / lock redb). Con `tracker.provider: linear`, Argos **no** muta esa cola. Protocolo Kintsugi aplicable al colapso DCC; no sustituye peaje de verificación.

## acceptance_criteria (objectives + spec) — estado

1. `sddia-qa verify-diff-digest` (paridad sello AUT-39) — **sin stdout cápsula**.
2. Job CI `sddia-diff-digest` en PR — **PENDIENTE-CI** (norma: sin run verde no contribuye a `global: APTO`).
3. Gate `verify_diff_seal_ci` / cuatro campos de sello — **sin verificación física**; `validacion.md` actual sin `validated_*` / `diff_digest`.

## correction_blueprint_md

```yaml
name: reentrada-verificacion-aut-40
intent: Reinyectar Argos con peaje Shell/MCP y campos explícitos; no inventar APTO.
delegates_to:
  - tool:git-manager
  - action:execute-process
  - agent:argos
phases:
  - name: Evidencia-git
    intent: status/diff/log vía skill:git-manager (JSON stdin sddia-run)
    delegates_to:
      - tool:git-manager
  - name: Verify-diff-digest
    intent: sddia-qa verify-diff-digest --json contra HEAD y target main
    delegates_to:
      - action:execute-process
  - name: Completar-cascada
    intent: Si faltan plan/implementation/execution, Tekton/Dedalo antes de re-auditoría
    delegates_to:
      - agent:tekton
  - name: Sello-validacion
    intent: Argos reescribe validacion.md solo tras stdout físico + CA CI con run_id si gate
    delegates_to:
      - agent:argos
```

Desbloqueo: habilitar Shell; reinyectar con `process: feature`, `phase: Verificación`, `persist_ref: docs/features/diff-digest-ci`, `branch_name: feat/diff-digest-ci`, `pbi_ref: AUT-40`, `execution_id` real.
