---
feature_name: pbi-005-hito3-git-hooks
created: "2026-05-20"
process: feature
purpose: Aduana pre-commit (Argos) y roadmap hooks CA-3 PBI-005
---

# Clarificación — PBI-005 Hito 3 (Git Hooks / Aduana de Fricción)

Transcript de decisiones (2026-05-20). Complementa `objectives.md` y alimenta `spec.md`.

---

## D1 — Inicio formal

| Pregunta | Decisión |
|----------|----------|
| ¿Proceso? | **`feature`** v1.2.0 |
| Rama | `feat/pbi-005-hito3-git-hooks` |
| `persist_ref` | `docs/features/pbi-005-hito3-git-hooks` |
| Manifiestos | Backlog post-PR11 § P1; PBI-005 CA-3; `TODO-BLINDAJE-IA-OBRERA.md` Fase C |
| Fases 2–6 en lab | `simulated` en arranque; esta entrega = **documentación** (clarify + spec) |

---

## D2 — Partición del Hito 3 (dos oleadas)

| Ola | Alcance | Hooks | Criterio |
|-----|---------|-------|----------|
| **Ola A (Fase 1)** | Aduana de fricción — blindaje IA obrera | `pre-commit` | Objetivos **A** y **B** |
| **Ola B (Fase 2+)** | Enlace orgánico ciclo PR (CA-3 completo) | `pre-push`, `post-merge` | **H3.1–H3.3**, **H3.5** |

**Motivo:** El backlog mezcla «pre-commit Argos» (blindaje) con «pre-push Presented / post-merge Merged» (automatización PR). Separar evita un hook monolítico que viole SRP y `pull-request-orchestration.md`.

---

## D3 — ¿Acción o proceso monolítico para hooks?

| Opción descartada | Motivo |
|-------------------|--------|
| Acción `git-pre-commit-gate` que combine Git + EDA + PR | Caja negra; duplica Argos y procesos existentes |

| Opción adoptada | Motivo |
|-----------------|--------|
| Scripts versionados en `SddIA/scripts/qa/git-hooks/` + instalador documentado | SSOT en repo; `.git/hooks/` es despliegue local, no genoma |
| Ola B delega en **`execute-process`** (`delivery-close-cycle`, `accept-pr`) | Misma fractalidad que `pr-presented-orchestration` |

---

## D4 — Contenido del `pre-commit` (Ola A)

Orden fijo, **fail-fast**:

| Paso | Invocación | Aborta si |
|------|------------|-----------|
| 1 | `verify-process-integrity.py` | `hash_signature` / frontmatter de `SddIA/process/*.md` inválido |
| 2 | `audit-entity-eda-coverage.py --scan --json` | `orphan_count > 0` |

**No** se invoca `git-manager` desde el hook: el hook es un script shell/Python del lado del host que llama QA. Los commits de genoma en runtime oficial siguen vía `skill:git-manager` dentro de procesos.

**Bypass de emergencia (solo operador humano):** variable `SDDIA_SKIP_HOOKS=1` documentada en `spec.md` § 4.4 — no expuesta a IAs en normas.

---

## D5 — Correlación EDA: Existencia en Bus (aceptada)

| Pregunta | Decisión | Notas |
|----------|----------|-------|
| Criterio **Fase 1** (operatividad S+) | **Scan completo del bus** — `pending`, `processing`, `processed`, `dead_letter` | Si el evento existe en **cualquier** estado, Argos da visto bueno |
| Implementación hook | `audit-entity-eda-coverage.py --scan` (sin `--require-pending-for-staged`) | Materializado en `pre_commit_gate.py` |
| Huérfanas | Artefacto indexado sin `Domain_Entity_Created` en bus | Integridad histórica sin bloquear flujo post-watcher |
| **Fase 1b** (opcional) | `--require-pending-for-staged` | **Solo diagnóstico** de mantenimiento Argos; **no** gatekeeper de commit |

---

## D6 — Alcance del escaneo EDA en `pre-commit`

| Pregunta | Decisión |
|----------|----------|
| ¿Solo archivos staged? | **Fase 1: no** — scan de repositorio completo (`--scan` actual), igual que aduana en `delivery-close-cycle` |
| ¿Exenciones? | `docs/features/**`, `docs/todos/**`, `docs/events/**`, `tmp/**` no están en `ENTITY_DIRS` — no disparan gate EDA |
| ¿`SddIA/process/`? | Cubierto por paso 1 (`verify-process-integrity`), no por audit de entidades |

**Mejora futura:** `--staged-only` en `audit-entity-eda-coverage` para reducir latencia en repos grandes.

---

## D7 — Prerrequisito `verify-process-integrity`

| Hallazgo | Decisión |
|----------|----------|
| Backlog E.3: posible drift de `hash_signature` en procesos | Antes de **imponer** el hook en CI/local, ejecutar `verify-process-integrity.py` en `main` y recalcular firmas en procesos tocados |
| Documentación | `spec.md` § 5 — gate de activación |

---

## D8 — Instalación de hooks

| Pregunta | Decisión |
|----------|----------|
| ¿Quién instala? | Operador humano tras laudo Fase 1 (copia/symlink desde `SddIA/scripts/qa/git-hooks/` → `.git/hooks/`) |
| ¿Versionado? | Scripts bajo `SddIA/scripts/qa/git-hooks/` en git; `.git/hooks/` en `.gitignore` implícito (local) |
| ¿Windows? | Script `pre-commit` invoca `python` con rutas relativas a raíz del repo (`git rev-parse --show-toplevel`) |

---

## D9 — Ola B: `pre-push` y `post-merge`

| Hook | Delegación | Prohibido |
|------|------------|-----------|
| `pre-push` | `execute-process.py --process delivery-close-cycle` (inputs mínimos: `branch_name`, `source_process`, `persist_ref` si aplica) o sub-secuencia documentada en H3.2 | `gh pr create` suelto; `emit-pr-presented-event` suelto |
| `post-merge` (en `main`) | `execute-process.py --process accept-pr` con JSON en `docs/events/pending/` según contrato | `gh pr merge`; `git merge` manual |

Contrato formal → **H3.1** (`SddIA/evolution/` o norma táctica). Detalle en `spec.md` § 6.

---

## D10 — Blindaje normativo IA (paralelo)

| Artefacto | Estado |
|-----------|--------|
| `SddIA/norms/external-ai-constraints.md` | **Fuera de alcance** de esta feature (TODO-BLINDAJE Fase A) |
| Cláusula en `objectives.md` | **Incluida** — precede implementación física del hook |

---

## D11 — Backlog y cierre PBI-005

| Pregunta | Decisión |
|----------|----------|
| ¿Marcar Hito 3 cumplido tras clarify/spec? | **No** — solo **en progreso**; enlace en manifiestos en Fase de ejecución (objetivo C) |
| ¿CA-3 cerrado con Ola A? | **No** — CA-3 requiere Ola B (H3.2–H3.3) + `validacion.md` |

---

## D12 — Git y commits de esta feature

Commits atómicos: (1) `objectives` + inicio feature, (2) `clarify` + `spec`, (3) implementación hooks Ola A, (4) norma/evolution Ola B si aplica, (5) `validacion.md`. Merge vía **`accept-pr`** cuando Argos emita **APTO**.
