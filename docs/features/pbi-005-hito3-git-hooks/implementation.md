---
feature_name: pbi-005-hito3-git-hooks
created: "2026-05-20"
process: feature
ola: A
---

# Implementación — Ola A (pre-commit Argos)

## Entregables

| Artefacto | Ruta | Rol |
|-----------|------|-----|
| Wrapper hook | `SddIA/scripts/qa/git-hooks/pre-commit` | Entrada Git → Python |
| Puerta lógica | `SddIA/scripts/qa/git-hooks/pre_commit_gate.py` | Fail-fast VPI + audit EDA |
| Instalador Windows | `SddIA/scripts/qa/git-hooks/install-hooks.ps1` | Copia a `.git/hooks/` |
| Crypto fix | `scripts/skills/cryptography-manager.py` | `surrogatepass` en SHA256 STRING |
| Procesos | `SddIA/process/*.md` | `hash_signature` recalculados (gate E.3) |

## Cadena `pre-commit`

1. `SDDIA_SKIP_HOOKS=1` → exit 0 (solo operador humano).
2. `verify-process-integrity.py` — integridad `SddIA/process/`.
3. `audit-entity-eda-coverage.py --scan --json` — **Existencia en Bus** (`orphan_count`).

**Nota:** El gate no fija `PYTHONIOENCODING` al invocar VPI (evita deriva de hash en Windows).

## Instalación local

### PowerShell (repo raíz)

```powershell
powershell -ExecutionPolicy Bypass -File SddIA/scripts/qa/git-hooks/install-hooks.ps1
```

### Git Bash

```bash
cp SddIA/scripts/qa/git-hooks/pre-commit .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```

### Equipo (opcional)

```bash
git config core.hooksPath SddIA/scripts/qa/git-hooks
```

En Windows con `core.hooksPath`, Git ejecuta hooks del directorio versionado; el wrapper `pre-commit` debe ser invocable (Git Bash recomendado).

## Prueba sin instalar

```powershell
python SddIA/scripts/qa/git-hooks/pre_commit_gate.py
echo $LASTEXITCODE   # 0 = APTO
```

## Fuera de alcance (Ola A)

- `pre-push` / `post-merge` (Ola B).
- `--require-pending-for-staged` (Fase 1b, solo diagnóstico Argos).
