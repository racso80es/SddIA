---
feature_name: laboratorio-handlers-l2-l3
created: "2026-05-24"
process: feature
purpose: Handlers mínimos lab para delivery-close-cycle (L.2) y feature (L.3)
---

# Clarificación — Laboratorio handlers L.2 + L.3

Transcript de decisiones (2026-05-24).

---

## D1 — Inicio formal

| Pregunta | Decisión |
|----------|----------|
| ¿Proceso de inicio? | **`feature`** v1.3.0 |
| Rama | `feat/laboratorio-handlers-l2-l3` |
| `persist_ref` | `docs/features/laboratorio-handlers-l2-l3` |
| Manifiesto operativo | `docs/todos/pending/[OPERATIVO] Backlog pendiente post-PR11 — Hito 3, Ola C y laboratorio.md` § Prioridad 2 |
| Upstream | `docs/features/pr-presented-orchestration/` (L.2 fases 4–7) |

---

## D2 — Triaje L.2: estado real `delivery-close-cycle`

| Fase | Handler actual | Estado lab | Acción |
|------|----------------|------------|--------|
| 1 Snapshot final | `capsule_delivery_snapshot_final` | `executed` | **Mantener** |
| 2 Impacto SddIA condicional | — (solo `agent:argos`) | `simulated` | **Forjar** gate mínimo |
| 3 Aduana EDA genómica | `capsule_eda_genomic_audit_gate` | `executed` / `blocked` | **Mantener** |
| 4 Publicación remota | `capsule_delivery_remote_push` | `executed` | **Mantener** |
| 5 Apertura en forja | `capsule_delivery_gh_pr` | `executed` | **Mantener** |
| 6 Sello Presentación ECST | `capsule_delivery_emit_presented` | `executed` | **Mantener** |
| 7 Higiene local | `capsule_delivery_local_hygiene` | `executed` | **Mantener** |

**Corrección PBI:** el gap L.2 **no** incluye fases 1 ni 3 (ya físicas); el backlog decía «Fases 1–3» por desfase documental post-PR #11.

---

## D3 — Semántica gate «Impacto SddIA condicional» (L.2 fase 2)

| Pregunta | Decisión |
|----------|----------|
| ¿Reemplazar Argos completo en lab? | **No** — gate determinista mínimo |
| ¿Cuándo evaluar? | Solo si `source_process == feature` |
| Señal de impacto | `git diff --name-only origin/<target_branch>...HEAD` contiene prefijo `SddIA/` |
| Si no hay mutaciones SddIA | `impact: none`, `status: executed`, no bloquea ciclo |
| Si hay mutaciones | `impact: core_mutation`, registrar paths en `execution_report`; no bloquea (Argos IDE sigue fuera de lab) |
| `source_process` bug-fix / refactorization | `skipped: true`, `reason: source_process != feature` |

---

## D4 — Triaje L.3: estado real `feature`

| Fase | delegates_to | Estado lab | Acción |
|------|--------------|------------|--------|
| 1 Inicialización | `skill:git-manager` | `executed` (`workspace-init`) | **Mantener** |
| 2 Estabilización | `agent:mayeuta` | `simulated` | **Mantener** (honesto) |
| 3 Diseño Blueprint | `agent:dedalo` | `simulated` | **Mantener** |
| 4 Ejecución | `agent:tekton` | `simulated` | **Mantener** |
| 5 Verificación | `agent:argos` | `simulated` | **Mantener** |
| 6 Cierre documental | `skill:filesystem-manager` | `simulated` | **Forjar** handler |
| 7 Cierre de entrega | `action:execute-process` | `simulated` | **Forjar** subproceso `delivery-close-cycle` |

---

## D5 — Contrato fase 6 «Cierre documental en rama»

| Pregunta | Decisión |
|----------|----------|
| ¿Mover PBI automáticamente? | **Sí**, si `validacion.md` existe con `global: APTO` y `pbi_archived: true` |
| Input PBI | `related_todo` en inputs del proceso o frontmatter de `objectives.md` |
| Operación | `filesystem-manager` move `pending/` → `done/` (mismo `document_id`) |
| Si `validacion.md` ausente o `NO_APTO` | `status: skipped`, `reason: validacion pendiente`; no abortar proceso completo |
| Variable skip lab | `SDDIA_LAB_SKIP_PBI_ARCHIVE=1` omite move (smoke sin side-effect) |

---

## D6 — Contrato fase 7 «Cierre de entrega»

| Pregunta | Decisión |
|----------|----------|
| Mecanismo | `invoke_subprocess_process(repo, "delivery-close-cycle", child_inputs)` |
| `child_inputs` mínimos | `source_process: feature`, `persist_ref`, `branch_name`, `pr_title`, `pr_body?`, `target_branch?` |
| Propagación | `state["pr_url"]`, `state["event_id"]`, `state["target_path"]` desde salida subproceso |
| Fallo subproceso | Propagar excepción; fase `status: failed` |
| Skip lab | `SDDIA_LAB_SKIP_DELIVERY_CLOSE=1` → `skipped` |

---

## D7 — Orquestación tracks

| Pregunta | Decisión |
|----------|----------|
| ¿Paralelo o secuencial? | **Paralelo** — L.2 y L.3 independientes |
| ¿Un PR o dos? | **Un PR** — revisión unificada |
| Orden plan | Hito 1 (L.2 fase 2) ∥ Hito 2 (L.3 fases 6–7) → Hito 3 smoke |

---

## D8 — Smoke y lab

| Escenario | Track | Perfil |
|-----------|-------|--------|
| `delivery-close-cycle` sin mutación SddIA | L.2 | Fase 2 `impact: none` |
| `delivery-close-cycle` con diff `SddIA/` | L.2 | Fase 2 `impact: core_mutation` + paths |
| `feature` fases 2–5 | L.3 | Permanecen `simulated` |
| `feature` fase 6 con validacion APTO | L.3 | PBI en `done/` |
| `feature` fase 7 | L.3 | Subproceso DC + `pr_url` |

Variables lab existentes sin cambio semántico: `SDDIA_LAB_SKIP_*`, `SDDIA_LAB_SIMULATE_GH_PR`.
