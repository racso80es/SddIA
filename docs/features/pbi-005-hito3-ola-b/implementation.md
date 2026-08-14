---
feature_name: pbi-005-hito3-ola-b
created: "2026-05-20"
process: feature
ola: B
---

# Implementación — Ola B (hooks ciclo PR)

## Entregables

| Artefacto | Ruta | Rol |
|-----------|------|-----|
| Utilidades | `hook_common.py` | O1–O2, bus EDA, invoke execute-process |
| Gate pre-push | `pre_push_gate.py` | O3 guarda, O1 idempotencia, delivery-close-cycle |
| Gate post-merge | `post_merge_gate.py` | accept-pr con `merge_already_done` |
| Wrappers | `pre-push`, `post-merge` | Entrada Git → Python |
| Instalador Win | `install-hooks.ps1` | O5 copia dinámica |
| Instalador Unix | `install-hooks.sh` | O5 symlink/copia dinámica |
| Cápsula accept-pr | `execute_process_capsules.py` | O4 merge huérfano |
| Sello anomalía | `execute-action.py` | `traceability_anomaly` en Merged |
| Contrato H3.1 | `SddIA/evolution/c032d392-a586-4b8c-baaf-6cb831ebb943.md` | Norma táctica |

## Cadena `pre-push`

1. `SDDIA_SKIP_HOOKS=1` → exit 0.
2. **O3** — rechazar push si rama local es `main`.
3. **O1** — skip silencioso si PR OPEN (`gh pr view`) o Presented en bus.
4. **O2** — `persist_ref` desde slug de rama si carpeta existe.
5. `execute-process.py --process delivery-close-cycle`.

## Cadena `post-merge`

1. Solo si `HEAD` es `main` tras merge.
2. Inferir `source_branch` desde mensaje de merge / `HEAD^2`.
3. `execute-process.py --process accept-pr` con `merge_already_done: true`.
4. Cápsula detecta **Merge Huérfano** (O4) y marca payload.

## Instalación (O5)

```powershell
powershell -ExecutionPolicy Bypass -File SddIA/scripts/qa/git-hooks/install-hooks.ps1
```

```bash
sh SddIA/scripts/qa/git-hooks/install-hooks.sh
```

Instala dinámicamente `pre-commit`, `pre-push`, `post-merge` (y futuros hooks sin extensión).

## Prueba sin push real

```powershell
# Guarda main (O3) — simular stdin pre-push
echo "refs/heads/main abc refs/heads/main def" | python SddIA/scripts/qa/git-hooks/pre_push_gate.py
echo $LASTEXITCODE   # debe ser 1

# Idempotencia / heurística — importar módulo
python -c "from pathlib import Path; import sys; sys.path.insert(0, 'SddIA/scripts/qa/git-hooks'); import hook_common as h; print(h.resolve_persist_ref('feat/pbi-005-hito3-ola-b'))"
```

## Variables laboratorio

| Variable | Efecto |
|----------|--------|
| `SDDIA_SKIP_HOOKS=1` | Bypass hooks (operador humano) |
| `SDDIA_LAB_SKIP_GIT_PUSH=1` | accept-pr / delivery-close-cycle omiten push |
| `SDDIA_LAB_SIMULATE_GH_PR=1` | PR simulado en delivery-close-cycle |

## Intocable (Ola A)

- `pre_commit_gate.py` — lógica pre-commit PR #12.
