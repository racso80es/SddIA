---
feature_name: l1-o5-runbooks-paridad
created: "2026-05-25"
process: feature
purpose: Runbook único accept-pr — cerrar L1-O5 y archivar manifiesto post-PR11
---

# Clarificación — L1-O5 Runbooks paridad operativa

Transcript de decisiones (2026-05-25).

---

## D1 — Inicio formal

| Pregunta | Decisión |
|----------|----------|
| ¿Proceso de inicio? | **`feature`** v1.3.0 |
| Rama | `feat/l1-o5-runbooks-paridad` |
| `persist_ref` | `docs/features/l1-o5-runbooks-paridad` |
| Manifiesto operativo | `docs/todos/pending/[OPERATIVO] Backlog pendiente post-PR11 — Hito 3, Ola C y laboratorio.md` |
| Upstream | `docs/features/vanguardia-soberania-local/` (L1-O1–O4 entregados; O5 residual) |
| Alcance PBI | **Solo objetivos abiertos:** L1-O5 + cierre manifiesto + FIX D.2 |

---

## D2 — Triaje: qué es «runbook operativo» vs registro histórico

| Superficie | Tipo | Tratamiento |
|------------|------|-------------|
| `docs/features/*/execution.md` con `Get-Content … git-manager.py` (merge/push/delete) | **Registro histórico** de entregas 2026-05-20 | Banner «inmutable» + enlace runbook SSOT — **no** borrar comandos |
| Ausencia de guía única para operador post-vanguardia | **Brecha L1-O5** | Crear `runbook-accept-pr.md` |
| `pull-request-orchestration.md` §4 | **Norma SSOT** | Ya prohíbe merge suelto — añadir referencia explícita al runbook |
| `accept-pr.md` § Fase 4 | **Genoma proceso** | Ya alineado PR #37 — runbook **referencia** genoma, no lo duplica |
| Invocaciones `git-manager` **dentro** de procesos (`feature` fase 1, `delivery-close-cycle`) | **Delegación legítima** | **Fuera de alcance** — no son «invocación suelta» |
| `execute_process_capsules.py` | **Runtime** | Fuera de alcance salvo bugfix |

**Corrección PBI:** L1-O5 no exige retirar `git-manager` del runtime ni de genomas de proceso — exige **paridad operativa** en guías para humanos/agentes IDE.

---

## D3 — Inventario inicial de violaciones documentales

| Archivo | Patrón prohibido en runbook | Acción |
|---------|----------------------------|--------|
| `docs/features/pbi-005-hito2-action-engine/execution.md` | merge/push/delete vía `git-manager.py` | Banner + enlace SSOT |
| `docs/features/pbi-005-debt-liquidation/execution.md` | merge/push vía `git-manager.py` | Banner + enlace SSOT |
| `docs/features/pbi-005-hito3-git-hooks/execution.md` | checkout/merge/push vía `git-manager.py` | Banner + enlace SSOT |
| `docs/features/ola-c-event-entity/clarify.md` | Mención git-manager como opción B | Verificar contexto — clarify histórico, banner si aplica |
| `docs/todos/done/… Laboratorio — Handler físico feature.md` | git-manager fase 1 | **Legítimo** — workspace-init, no merge |

**Gate propuesto:** grep en CI pre-commit sobre `docs/` excluyendo bloques marcados `<!-- runbook-historical -->` o frontmatter `runbook_status: historical`.

---

## D4 — Contenido mínimo del runbook canónico

| Sección | Contenido |
|---------|-----------|
| Precondiciones | Rama feature revisada; `pull-request-review` aprobado si aplica cadena PR |
| Inputs JSON | Plantilla `_smoke-accept-pr-*.json` con `source_branch`, `author`, `correlation_id` |
| Comando | `python SddIA/scripts/qa/execute-process.py --process accept-pr --inputs-file …` |
| Post-sello | `event-watcher.py --once` (IOTA / route) |
| Salida esperada | `merge_commit_hash`, `event_id`, `closed_branch` o `hygiene_failure` |
| Hooks Ola B | Nota `SDDIA_SKIP_HOOKS` en cápsula sync (referencia `pbi-005-hito3-ola-b/execution.md`) |
| Anti-patrones | Tabla explícita: prohibido `git merge`, `git push main`, `git-manager merge` manual |
| Handoff upstream | Cadena `delivery-close-cycle` → review → `accept-pr` |

---

## D5 — Estrategia gate Argos / pre-commit

| Pregunta | Decisión |
|----------|----------|
| ¿Nuevo script o extender existente? | **`verify-runbook-paridad.py`** bajo `SddIA/scripts/qa/` — scope acotado, invocable desde `verify-process-integrity` o job CI ligero |
| Patrones a detectar | `git-manager.py` + (`merge` \| `delete_branch` \| `push`) en `docs/**/*.md` sin exención histórica |
| Exenciones | Bloque HTML `<!-- runbook-historical -->` … `<!-- /runbook-historical -->`; rutas `docs/todos/done/` inmutables |
| Fallo | exit 1 + lista de archivos/líneas para operador |

---

## D6 — FIX delete_branch y cierre PBI

| Pregunta | Decisión |
|----------|----------|
| ¿Reabrir bug-fix? | **No** — código cerrado en vanguardia |
| FIX en `pending/` | Mover a `done/` en **fase 6** de esta feature (mismo PR) |
| Manifiesto post-PR11 | Mover a `done/` cuando L1-O5 ✅ + `validacion.md` APTO |
| `pbi_archived: true` | Solo en `validacion.md` de esta feature, pre-merge |

---

## D7 — P5 fuera de alcance

| Ítem PBI | Decisión |
|----------|----------|
| D.3 PDF operativo | Kaizen posterior — no bloquea archivo manifiesto |
| D.5 TODO-BLINDAJE-IA-OBRERA Fase C | Feature `ia-obrera-blindaje` separada |
| norma-paridad-documental (pending) | No mezclar — posible dependencia futura |

---

## D8 — Criterios de aceptación Argos (preview)

| ID | Check |
|----|-------|
| V-L1O5-1 | `runbook-accept-pr.md` existe y enlazado desde norma |
| V-L1O5-2 | Inventario legacy con banners; cero violaciones en gate |
| V-L1O5-3 | Smoke `accept-pr` documentado en `execution.md` |
| V-L1O5-4 | FIX + PBI en `docs/todos/done/` en rama PR |
| V-L1O5-5 | `validacion.md` APTO, `pbi_archived: true` |
| V-L1O5-6 | Regresión: `verify-process-integrity` verde |

---

## D9 — Orquestación implementación (preview)

| Hito | Entrega |
|------|---------|
| H1 | `runbook-accept-pr.md` + enlace norma |
| H2 | Banners legacy + gate `verify-runbook-paridad.py` |
| H3 | Smoke + `execution.md` + `implementation.md` |
| H4 | Cierre documental: FIX, PBI, `validacion.md`, PR |
