---
contrato_version: "1.0.0"
id_cambio: "f3a8c2d1-4e5b-6a7c-8d9e-0f1a2b3c4d5e"
fecha: "2026-05-20T00:00:00+00:00"
autor: "pbi-005-hito3-ola-b"
proyecto_origen_cambio: "SddIA"
contexto: "Hito 3 Ola B — hooks pre-push y post-merge (CA-3 PBI-005)."
descripcion_breve: "Contrato normativo de hooks Git ciclo PR: idempotencia Presented, guarda main, heurística persist_ref, merge huérfano, instalador dinámico."
tipo_operacion: "feature"
impacto: "Medio — automatización Git sin CLI suelta; bloquea push directo a main."
relacionado:
  - "docs/features/pbi-005-hito3-ola-b/spec.md"
  - "SddIA/norms/pull-request-orchestration.md"
  - "SddIA/process/delivery-close-cycle.md"
  - "SddIA/process/accept-pr.md"
---

# Evolution — Contrato hooks CA-3 Ola B

## Tabla canónica

| Hook | Trigger | Proceso | Evento | Resoluciones |
|------|---------|---------|--------|--------------|
| `pre-commit` | `git commit` | QA Argos (Ola A) | — | Heredado PR #12 |
| `pre-push` | `git push` (rama ≠ `main`) | `delivery-close-cycle` | `PullRequest_Presented` | O1, O2, O3 |
| `post-merge` | Merge local → `main` | `accept-pr` (`merge_already_done`) | `PullRequest_Merged` | O4 |

## O1 — Idempotencia `pre-push`

Si `gh pr view` reporta PR **OPEN** o existe `PullRequest_Presented` en bus para la rama → **no** invocar proceso; exit 0.

## O2 — `persist_ref`

`feat/{slug}` → `docs/features/{slug}/` si existe; si no, `null`.

## O3 — Guarda `main`

Push a `main` → Hard Fail: *Violación de Soberanía…*

## O4 — Merge huérfano

`accept-pr` emite `PullRequest_Merged` con `traceability_anomaly: merge_huérfano` si falta Presented previo.

## O5 — Instalador

`install-hooks.ps1` / `install-hooks.sh` — iteración dinámica de `git-hooks/` sin extensiones auxiliares.

## Prohibiciones

- `gh pr merge`, `gh pr create`, acciones EDA sueltas en hooks.
- `git push` / `git merge` directos desde hooks (solo lectura `git rev-parse`).

## Referencias

- `SddIA/scripts/qa/git-hooks/`
- `docs/features/pbi-005-hito3-ola-b/`
