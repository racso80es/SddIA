---
feature_name: refactor-execute-process-engine
created: "2026-05-20"
process: feature
branch_name: feat/refactor-execute-process-engine
persist_ref: docs/features/refactor-execute-process-engine
pbi_ref: PBI-005
status: cerrado
merged_at: "2026-05-20"
pr_url: https://github.com/racso80es/SddIA/pull/9
merge_commit: d2c9559aee9c5bf1bcff0e08e5a671ab9368d175
---

# Objetivos — refactor-execute-process-engine

## Misión

Convertir `SddIA/scripts/qa/execute-process.py` de script piloto con ramas hardcodeadas por proceso en un **intérprete dinámico agnóstico** que lee contratos MD (`SddIA/process/{name}.md`), valida inputs y enruta fases vía cápsulas físicas — alineado con el modelo exitoso de `execute-action.py`.

## Alcance entregado

| Ítem | Estado |
|------|--------|
| Módulos `execute_process_core.py` + `execute_process_capsules.py` | ✅ |
| CLI canónico `--process` / `--inputs` (+ `--inputs-file`) | ✅ |
| Compatibilidad Ola C: `--input-file` y `--action` (shim + warning stderr) | ✅ |
| Handler genérico `workspace-init` (git-manager + `objectives.md` por `feature_name`) | ✅ |
| Validación semántica de inputs frente a contrato YAML | ✅ |
| `CAPSULE_ACTION_REGISTRY` → `execute-action.py` | ✅ |
| Acciones físicas: `emit-domain-mutation`, `emit-pr-presented-event`, `emit-pr-merged-event` | ✅ |
| PR #9 merge a `main` + cierre EDA IOTA físico | ✅ |
| Fix genoma `event-subscriptions.json` (`PullRequest_Presented` + IOTA en `Domain_Entity_*`) | ✅ (`18d80ea`) |
| TODO deuda Ola C (retirada futura de shims CLI) | ✅ documentado |

## Fuera de alcance (deuda explícita)

- Intérprete completo de `stdin_spec` / todas las `phase_invocations` de creators.
- Retirada de shims `--input-file` y `--action` (ver TODO Ola C).
- Perfil laboratorio vs IDE en `SddIA/process/feature.md` (pendiente del TODO laboratorio histórico).
- Orquestación PR presentado: `delivery-close-cycle` v1.1 + `emit-pr-presented-event` (ver `docs/features/pr-presented-orchestration/`).

## Ley aplicada

- `process-contract v1.3.0`: resolución canónica, `phases`, `delegates_to`.
- Git exclusivamente vía `skill:git-manager` en fase de inicialización de contexto.
- Jerarquía: Acción → Agente → Skill → Tools; EDA vía `execute-action.py` + `event-watcher.py`.
