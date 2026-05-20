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

## Pendiente Ola B

H3.1–H3.3 (`pre-push`, `post-merge`) según `spec.md` § 7.
