---
feature_name: kaizen-runtime-binary-parity-gate
created: "2026-10-29"
updated: "2026-10-29T19:35:00Z"
process: feature
phase: Verificación
agent: argos
agents: argos
branch: feat/kaizen-runtime-binary-parity-gate
branch_name: feat/kaizen-runtime-binary-parity-gate
persist_ref: docs/features/kaizen-runtime-binary-parity-gate
pbi_ref: MAN-25
tracker_ref: MAN-25
execution_id: "b4bc0b99-532a-4faa-8e57-dcd957d39b45"
global: NO_APTO
pbi_archived: false
approval_status: rechazado
verdict: rechazado
git_manager_invoked: false
git_manager_error: "Shell IDE Rejected sobre ./sddia-run.sh --tool git-manager; sin stdout físico esta sesión"
shell_session: "Rejected — sin reejecución de cargo/sddia-run; síntesis solo sobre terminales IDE ya materializados"
checks:
  KZ-CA1: PENDIENTE
  KZ-CA2: PENDIENTE
  KZ-CA3: NO_APTO
  KZ-CA4: PENDIENTE
  KZ-CA5: PENDIENTE
  runtime_binary_parity: NO_APTO
  sddia-runtime-parity-lib: PENDIENTE
  sddia-ecosystem-health-lib: NO_APTO
  kalma2-bridge-runtime-tests: PENDIENTE
  execute-process-lib: PENDIENTE
  prior_apto_claim: NO_APTO
git_changes:
  - SddIA/sddia-runtime-parity/
  - SddIA/ecosystem-health/
  - SddIA/engine/execute-process/
  - SddIA/interfaces/kalma2-bridge/
  - start-sddia.sh
  - docs/features/kaizen-runtime-binary-parity-gate/
blocking_findings:
  - KZ-CA3
  - sddia-ecosystem-health-lib
  - prior_apto_claim
---

# Validación Argos — MAN-25

## Contexto

- `process: feature` · fase `Verificación` · `agent:argos`
- `persist_ref: docs/features/kaizen-runtime-binary-parity-gate`
- `execution_id: b4bc0b99-532a-4faa-8e57-dcd957d39b45`
- Inyección kalma2 llegó con `process/phase/?` vacíos; ancla documental del `persist_ref` + workspace feature.

## Evidencia física (terminales IDE)

| Fuente | Comando | Resultado |
|--------|---------|-----------|
| `288331` | `cargo test -p sddia-runtime-parity -p sddia-ecosystem-health -p execute-process -p kalma2-bridge` | Compile fail `execute-process`: `unresolved import crate::capsule_seal` en `runtime_recycle.rs` (posteriormente `cargo check` en `288333`/`288334` OK tras `use super::capsule_seal`) |
| `288332` | `cargo test -p sddia-runtime-parity -p sddia-ecosystem-health --lib` | **FAILED** `ecosystem_health_map_absent_still_ok`: `map_status` left=`DEGRADED` right=`absent` |
| `288333`/`288334` | `cargo check -p execute-process` | Finished OK (warnings dead_code) |
| `288335` | release build + `--seal-capsules` | **running** al sello Argos; sin stdout de éxito |

Nota: `exit_code: 0` en terminales con `| tail` **no** acredita éxito de cargo; el cuerpo muestra `FAILED` / `error: could not compile`.

## Hallazgo bloqueante (KZ-CA3)

En tempdir sin map-snapshot ni testigos, `fuse_ecosystem_health` inserta `runtime_binary_parity` y, si `ok != true`, fuerza `map_status: DEGRADED` también cuando el mapa estaba `absent` (`ecosystem-health/src/lib.rs` ~691–696). El test canónico `ecosystem_health_map_absent_still_ok` exige `map_status == "absent"` + `success: true` → **regresión verde→rojo**.

## APTO previo

El `validacion.md` con `global: APTO` y checks verdes **no** se sostiene en la evidencia anterior. Argos lo **anula** (L-ARGOS-SYNTHESIS: prohibido inventar éxito).

## Cápsulas no reejecutables esta sesión

Shell IDE **Rejected** → no `cargo test` fresco, no `git-manager`, no `sddia-run`. Checks KZ-CA1/2/4/5 y baterías bridge/execute quedan `PENDIENTE` (no APTO).

## approval_status

`rechazado` · `global: NO_APTO` · `pbi_archived: false`

## correction_blueprint_md

```yaml
name: corregir-kz-ca3-map-absent-parity
intent: Alinear degradación runtime_binary_parity con contrato de map_status absent; re-verificar batería y re-inyectar Argos
delegates_to:
  - agent:tekton
  - agent:argos
phases:
  - name: Ejecución
    intent: >
      Corregir fuse_ecosystem_health para no pisar map_status absent→DEGRADED
      (o actualizar test + spec si el laudo es degradar siempre que parity.ok=false);
      asegurar fixtures de test con parity determinista; re-correr
      cargo test -p sddia-runtime-parity -p sddia-ecosystem-health -p execute-process -p kalma2-bridge --lib
      y dejar evidencia stdout sin pipe que enmascare exit code.
    delegates_to:
      - agent:tekton
  - name: Verificación
    intent: Re-auditar con Shell/cápsulas operativas; sellar validacion.md solo si batería verde.
    delegates_to:
      - agent:argos
requires_runtime:
  - Shell IDE operativo (o stdout de cápsulas materializado)
  - ./sddia-run.sh --tool git-manager para git_changes
```

## Límites Argos

- Sin escritura bajo `docs/todos/`.
- Sin mutación de genoma/engine en esta fase.
- Sin inventar APTO ni Linear Done.
