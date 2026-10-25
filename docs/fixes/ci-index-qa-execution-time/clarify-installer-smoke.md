---
feature_name: ci-index-qa-execution-time
created: "2026-10-25"
process: bug-fix
tracker_ref: AUT-13
---
# Laudo — comportamiento `sddia-installer-smoke` (post-merge)

## SSOT workflow

Archivo: `.github/workflows/sddia-installer-qa.yml`. Job: `sddia-installer-smoke`.

## Disparadores

| Evento | Condición |
|--------|-----------|
| `pull_request` → `main` | Solo si el diff toca `paths` |
| `push` → `main` | **Mismos** `paths` (no smoke en cada push a `main`) |

Rutas `paths`:

- `sddia-installer.sh`
- `SddIA/scripts/**`
- `SddIA/engine/execute-process/**`
- `.github/workflows/sddia-installer-qa.yml`

## Fuera de `sddia-index-qa`

El job ya no vive en `sddia-index-qa.yml`. **No** cuenta entre los cinco `KNOWN_SDDIA_QA_JOBS` de `github_bridge.rs`.

## Prep CI (AC-4 lab)

`SddIA/scripts/qa/prep-installer-lab-release.sh` compila en release los paquetes mínimos del lab (sin `--workspace` completo).

## Merge

- PR [#386](https://github.com/racso80es/SddIA/pull/386) → `15b2945b`
- Ajuste `paths` en push `main`: `6943b166` (incluido en #386)
