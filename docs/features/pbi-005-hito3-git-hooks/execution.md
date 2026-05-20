---
feature_name: pbi-005-hito3-git-hooks
created: "2026-05-20"
process: feature
branch_name: feat/pbi-005-hito3-git-hooks
---

# Ejecución — Ola A pre-commit

## Rama

`feat/pbi-005-hito3-git-hooks`

## Registro de cambios

| Bloque | Archivos |
|--------|----------|
| Git hooks SSOT | `SddIA/scripts/qa/git-hooks/pre-commit`, `pre_commit_gate.py`, `install-hooks.ps1` |
| QA crypto | `scripts/skills/cryptography-manager.py` (`surrogatepass`) |
| Genoma procesos | `SddIA/process/*.md` — recálculo `hash_signature` (`tmp/recalc-process-hashes.py`) |
| Documentación | `clarify.md`, `spec.md` v1.1.0, `plan.md`, `implementation.md`, `validacion.md` |

## Comandos de verificación (laboratorio)

```powershell
python SddIA/scripts/qa/verify-process-integrity.py
python SddIA/scripts/qa/audit-entity-eda-coverage.py --scan --json
python SddIA/scripts/qa/git-hooks/pre_commit_gate.py
```

## Instalación hook (operador)

```powershell
powershell -ExecutionPolicy Bypass -File SddIA/scripts/qa/git-hooks/install-hooks.ps1
```

## Cierre producción (PR #12)

| Paso | Resultado |
|------|-----------|
| Push | `origin/feat/pbi-005-hito3-git-hooks` |
| PR | https://github.com/racso80es/SddIA/pull/12 |
| `delivery-close-cycle` | `PullRequest_Presented` `0c9a8a63-…` |
| Watcher DLT | `cumulo: success` (sin `SDDIA_LAB_SIMULATE_IOTA`) |
| `accept-pr` (git-manager + sello) | merge `12119f73168b78713fde861f6a26aa7754ca873c` |
| `PullRequest_Merged` | `34cfbad5-009e-4ace-b597-571de282f280` |
| Push `main` | `c6a8620..12119f7` |

### Comandos

```powershell
python SddIA/scripts/qa/execute-process.py --process delivery-close-cycle --inputs-file docs/features/pbi-005-hito3-git-hooks/_delivery-close-hito3.json
Remove-Item Env:SDDIA_LAB_SIMULATE_IOTA -ErrorAction SilentlyContinue
python SddIA/scripts/daemons/event-watcher.py --once

Get-Content tmp/git-checkout-main-hito3.json -Raw | python scripts/skills/git-manager.py
Get-Content tmp/git-merge-hito3.json -Raw | python scripts/skills/git-manager.py
python SddIA/scripts/qa/execute-action.py --action emit-pr-merged-event --input-file tmp/emit-pr-merged-hito3.json
Get-Content tmp/git-push-main-hito3.json -Raw | python scripts/skills/git-manager.py
python SddIA/scripts/daemons/event-watcher.py --once
```

## Pendiente Ola B

H3.1–H3.3 (`pre-push`, `post-merge`) según `spec.md` § 7.
