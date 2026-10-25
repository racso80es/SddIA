---
feature_name: kaizen-linear-backlog-intake-guard
created: "2026-10-25"
process: feature
persist_ref: docs/features/kaizen-linear-backlog-intake-guard
branch_name: feat/kaizen-linear-backlog-intake-guard
document_id: PBI-KAIZEN-LINEAR-BACKLOG-INTAKE-GUARD
uuid: "4e81d3a2-7c90-4b15-9f23-6a18c40e5b91"
---

# Objetivos — kaizen-linear-backlog-intake-guard

Linear: **AUT-14** (hijo de AUT-5). `tracker_ref` en frontmatter solo en `validacion.md` hasta cierre (pre-push `done_gate`).

## Misión

Evitar que operadores IA materialicen backlog en `docs/todos/{pending,done}/` cuando `.SddIA/project.md` declara `tracker.provider: linear`. Tres capas: norma IDE, hook Cursor y `pre-commit`.

## Criterios de aceptación (Linear AUT-14)

| ID | Criterio |
|----|----------|
| AC-1 | Reglas en `.cursor/rules/task-closure-documental.mdc` y `.cursorrules` §8 |
| AC-2 | `pre_commit_gate.sh` rechaza ficheros nuevos bajo cola `docs/todos` |
| AC-3 | Sin `_refine_lab_*` en el árbol; `.gitignore` ignora `_*_lab_*` |
| AC-4 | Hook Cursor `preToolUse` bloquea `Write`/`StrReplace` en cola todos |

## Ley aplicada

- `features-documentation-pattern` v1.3.1
- Proceso `feature` v1.3.2
- Issue Linear [AUT-14](https://linear.app/oscar-perez/issue/AUT-14)
