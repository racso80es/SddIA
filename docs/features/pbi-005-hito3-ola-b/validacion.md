---
feature_name: pbi-005-hito3-ola-b
created: "2026-05-20"
process: feature
branch: feat/pbi-005-hito3-ola-b
global: apto
checks:
  V-B0: pass
  V-B1: pass
  V-B2: pass
  V-B3: pass
  V-B4: pass
  V-B5: pass
  V-B6: pass
  V-B7: pass
  V-B8: pass
git_changes:
  - SddIA/scripts/qa/git-hooks/
  - SddIA/scripts/qa/execute_process_capsules.py
  - SddIA/scripts/qa/execute-action.py
  - SddIA/evolution/git-hooks-ca3-ola-b-contract.md
---

# Validación — Ola B (hooks ciclo PR / CA-3)

Perfil **laboratorio** (`SDDIA_LAB_SKIP_GIT_PUSH`, `SDDIA_LAB_SIMULATE_GH_PR`, `SDDIA_LAB_SIMULATE_IOTA=1` en watcher). Hooks físicos verificados; push/merge remoto real pendiente de PR.

## Checks spec § 9

| ID | Check | Estado | Evidencia |
|----|-------|--------|-----------|
| V-B0 | Push a `main` bloqueado (O3) | ✅ | `pre_push_gate.py` exit 1 + mensaje soberanía |
| V-B1 | Primer ciclo → `PullRequest_Presented` | ✅ | `e71a367b-33f4-4c6e-9066-c42a43c4d550` |
| V-B2 | Segundo `pre-push` → exit 0, sin nuevo Presented | ✅ | `presented_before=1` `presented_after=1` |
| V-B3 | `persist_ref` inyectado (`feat/pbi-005-hito3-ola-b`) | ✅ | `docs/features/pbi-005-hito3-ola-b` |
| V-B4 | Rama sin carpeta → `null` | ✅ | `resolve_persist_ref('feat/no-such-feature-folder')` → `None` |
| V-B5 | `accept-pr` → `PullRequest_Merged` | ✅ | `e7812b3a-d320-4f14-991c-5fa75b80a683` |
| V-B6 | Merge huérfano → `traceability_anomaly` | ✅ | `890b6a55-…` payload `merge_huérfano` |
| V-B7 | Instalador dinámico O5 | ✅ | `install-hooks.ps1` → pre-commit, pre-push, post-merge |
| V-B8 | Sin `gh pr merge` en `git-hooks/` | ✅ | grep vacío |

## Aduanas previas (herencia Ola A)

| Check | Estado |
|-------|--------|
| `verify-process-integrity.py` | ✅ OK |
| `audit-entity-eda-coverage --scan` | ✅ `orphan_count: 0` |

## Comandos reproducibles

```powershell
cd C:\Proyectos\SddIA

# O3
echo "refs/heads/main dead refs/heads/main beef" | python SddIA/scripts/qa/git-hooks/pre_push_gate.py

# V-B1 Presented (lab)
$env:SDDIA_LAB_SKIP_GIT_PUSH='1'
$env:SDDIA_LAB_SIMULATE_GH_PR='1'
$env:SDDIA_LAB_SKIP_SNAPSHOT='1'
python SddIA/scripts/qa/execute-process.py --process delivery-close-cycle --inputs-file docs/features/pbi-005-hito3-ola-b/_smoke-delivery-close-presented.json

# V-B2 idempotencia
echo "refs/heads/feat/pbi-005-hito3-ola-b dead refs/heads/feat/pbi-005-hito3-ola-b beef" | python SddIA/scripts/qa/git-hooks/pre_push_gate.py

# V-B5 Merged canónico
$env:SDDIA_LAB_SKIP_GIT_PUSH='1'
python SddIA/scripts/qa/execute-process.py --process accept-pr --inputs-file docs/features/pbi-005-hito3-ola-b/_smoke-accept-pr-merged.json

# V-B6 merge huérfano
python SddIA/scripts/qa/execute-process.py --process accept-pr --inputs-file docs/features/pbi-005-hito3-ola-b/_smoke-accept-pr-orphan.json

# Watcher DLT
$env:SDDIA_LAB_SIMULATE_IOTA='1'
python SddIA/scripts/daemons/event-watcher.py --once

# O5 instalador
powershell -ExecutionPolicy Bypass -File SddIA/scripts/qa/git-hooks/install-hooks.ps1
```

## Eventos EDA (smoke laboratorio)

| event_id | event_type | Destino | DLT (`cumulo`) | Notas |
|----------|------------|---------|----------------|-------|
| `e71a367b-33f4-4c6e-9066-c42a43c4d550` | `PullRequest_Presented` | `processed/` | success | V-B1; rama `feat/pbi-005-hito3-ola-b` |
| `e7812b3a-d320-4f14-991c-5fa75b80a683` | `PullRequest_Merged` | `processed/` | success | V-B5; `orphan_merge: false` |
| `890b6a55-82f4-460e-b9d4-3ee2881f1666` | `PullRequest_Merged` | `processed/` | success | V-B6; `traceability_anomaly: merge_huérfano` |

## Veredicto Ola B

**APTO** (laboratorio) — hooks Ola B operativos; cápsula `accept-pr` resiliente; instalador O5 verificado. **Cierre PBI-005 al 100 %** condicionado a merge de esta rama en `main` vía `accept-pr` + sello PR real (`PullRequest_Presented` / `Merged` sin flags lab).

## Pendiente post-APTO

- [ ] `delivery-close-cycle` → PR GitHub desde `feat/pbi-005-hito3-ola-b`
- [ ] `accept-pr` post-merge en `main`
- [ ] PBI operativo v1.5.0 estado «completado» en `main`
- [ ] `finalize-process.md` con `pr_url`
