---
feature_name: pull-request-review-redesign
created: "2026-05-22"
process: feature
purpose: Aduana reactiva PullRequest_Presented — triaje, bloqueo, Kaizen, handoff accept-pr
---

# Clarificación — Rediseño `pull-request-review`

Transcript de decisiones (2026-05-22), incluyendo triaje del genoma existente y laudos de arquitectura.

---

## D1 — Inicio formal

| Pregunta | Decisión |
|----------|----------|
| ¿Proceso de inicio? | **`feature`** v1.2.0 |
| Rama | `feat/pull-request-review-redesign` |
| `persist_ref` | `docs/features/pull-request-review-redesign` |
| Manifiesto operativo | TODO `docs/todos/ARQUITECTURA_Rediseno_Proceso_pull-request-review.md` |

---

## D2 — Triaje del genoma existente (¿hay que limpiar?)

| Artefacto | Estado actual | Acción |
|-----------|---------------|--------|
| `SddIA/process/pull-request-review.md` v1.0.0 | **Declarativo only** — 5 fases (git-manager, **Dedalo**, Argos×2, filesystem) | **Reescribir** v2.0.0 alineado al TODO; no patch incremental |
| Handler `execute_process_capsules.py` | **Ausente** — cero referencias a `pull-request-review` | **Forjar** handlers lab Fase 1–2 mínimos |
| `event-subscriptions.json` → `PullRequest_Presented` | Solo **Cúmulo** + IOTA | **Añadir** suscriptor proceso/aduana (sin eliminar IOTA) |
| `pull-request-presented.md` | Nota «no-op hasta auditoría Argos» | **Actualizar** suscripciones al cablear aduana |
| Legacy `validate-pull-requests` | Purgado del Core; referencias en `SddIA_1`…`SddIA_4/process/README.md` y `SddIA_4/linter/acceptable_pr.md` | **Documentar purge** en plan; no tocar labs en esta feature salvo checklist |
| Solapamiento con `accept-pr` fase «Auditoría Genómica» | Ambos usan Argos pre-fusión | **Diferenciar jurisdicción** (véase D4) |

**Conclusión:** el proceso **existe en nombre** pero **no está operativo** como aduana EDA. Requiere limpieza semántica del genoma v1.0.0 y cableado bus, no borrado del archivo.

---

## D3 — Retirada de Dedalo del escrutinio PR

| Hallazgo | Decisión |
|----------|----------|
| Genoma v1.0.0 delega «Escrutinio de arquitectura» a **agent:dedalo** | **Eliminar** fase Dedalo en v2 |
| TODO arquitectónico cita **Argos** (juez diff ↔ normas) y **Cerbero** (RBAC) | Escrutinio estructural absorbido en fases Argos + validación documental Tekton/Argos |
| Evolución `ebdc4cb8` ya absorbió architect en pull-request-review vía Dedalo+Argos | v2 **consolida** en Argos + Cerbero + Cúmulo Kaizen, coherente con Gobernanza S+ Grade del TODO |

---

## D4 — Jurisdicción vs `accept-pr` (resolución Fase 4 TODO)

| Pregunta | Decisión |
|----------|----------|
| TODO Fase 4 menciona «Fusión Física» vía `git-manager` | **Reinterpretado:** la aduana **no fusiona**; autoriza handoff |
| ¿Duplicar merge en aduana? | **No** — viola `pull-request-orchestration.md` §4 |
| Flujo canónico | `PullRequest_Presented` → **`pull-request-review`** (aduana) → si `verdict: aprobado` → **`accept-pr`** (merge + `PullRequest_Merged`) |
| Rol de Argos en ambos procesos | **pull-request-review:** triaje multidimensional post-presentación (docs, tests, RBAC, Kaizen). **accept-pr:** auditoría genómica final inmediata pre-merge físico (puede reutilizar evidencia cacheada) |

---

## D5 — Estímulo y enrutador

| Pregunta | Decisión |
|----------|----------|
| Bus runtime SSOT | `docs/events/{pending,processing,processed,dead-letter}` |
| Trigger | Instancia ECST `PullRequest_Presented` promovida por `event-watcher.py` |
| Correlación | `payload.branch`, `payload.pr_url` (v1.1), `event_id` como `correlation_id` hacia `accept-pr` |
| Scripts locales | **Prohibido** decidir veredicto fuera de agentes; scripts solo orquestan cápsulas (`execute-process`) |

---

## D6 — Fases del TODO → fases del proceso v2

| TODO | Fase proceso v2 (propuesta) |
|------|----------------------------|
| Fase 1 — Triaje multidimensión | F1 Preparación · F2 Documental · F3 Técnica · F4 Cerbero RBAC |
| Fase 2 — Bloqueos | F5 Veredicto Argos (abort + comentarios) |
| Fase 3 — Kaizen | F6 Cúmulo — cosecha TODO |
| Fase 4 — Materialización | F7 Handoff `accept-pr` *(condicional éxito)* |

---

## D7 — Entropía Kaizen (Cúmulo)

| Pregunta | Decisión |
|----------|----------|
| ¿Interrumpir al programador en caliente? | **No** — deuda no bloqueante va a `docs/todos/` |
| Prefijo archivo | `[ARQUITECTURA]` si afecta genoma/normas; `[OPERATIVO]` si es deuda táctica |
| Agente | **Cúmulo** (no Argos) persiste semillas; Argos solo clasifica severidad |

---

## D8 — Referencias cruzadas

| Artefacto | Relación |
|-----------|----------|
| `pr-presented-orchestration` | Emisor upstream (`delivery-close-cycle` → `PullRequest_Presented`) |
| `accept-pr` | Consumidor downstream de aduana aprobada |
| `knowledge-contract.md` | Principios `blocking_for_pr` enrutan a Argos en aduana |
| `pbi-005-hito3-ola-b` | Hooks disparan presentación; aduana es **post**-presentación, no sustituto de hook |
