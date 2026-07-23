---
feature_name: plumb-cid
created: "2026-07-23"
updated: "2026-07-23"
process: feature
purpose: Estabilización Mayeuta — lab plumb correlation_id en cascada documental feature (kalma2-agent-runtime-cursor)
branch_name: feat/plumb-cid
persist_ref: docs/features/plumb-cid
pbi_ref: docs/todos/pending/[FEATURE] plumb-cid.md
document_id: LAB-PLUMB-CID
execution_id: a1b2c3d4-e5f6-4789-a012-3456789abcde
correlation_id: a1b2c3d4-e5f6-4789-a012-3456789abcde
phase: mayeuta-stabilization
agents: mayeuta
---

# Clarificación — plumb-cid

Transcript Mayeuta (2026-07-23). Semilla operador: «inicia feature docs/todos/pending/[FEATURE] plumb-cid.md» + orden Raw Kernel fase Estabilización (`correlation_id` a1b2c3d4-…).

`persist_ref` vacío en inyección runtime → resuelto vía stub `workspace-init` + `paths.featurePath` (`docs/features`) + `branch_name`/`feature_name` → `docs/features/plumb-cid`.

---

## D0 — Apertura formal

| Pregunta | Decisión |
|----------|----------|
| Proceso | `feature` (fase Estabilización → handoff Dedalo) |
| `feature_name` | `plumb-cid` |
| Rama | `feat/plumb-cid` |
| `persist_ref` | `docs/features/plumb-cid` |
| `document_id` | `LAB-PLUMB-CID` |
| PBI físico | **Ausente** en `docs/todos/pending/` (path referenciado no materializado) |
| Naturaleza ciclo | **Lab / humo de tubería** — plumb de `correlation_id` en artefactos Mayeuta; no producto de dominio nuevo |
| Fase | Estabilización Mayeuta (esta sesión) → Dedalo blueprint lab de evidencia CID |

---

## D1 — Semilla vs realidad

| Afirmación | Hecho | Laudo |
|------------|-------|-------|
| Intención = iniciar feature `plumb-cid` | Stub `objectives.md` post-`workspace-init` existe | Fuente `raw_user_intent` válida |
| PBI en `docs/todos/pending/[FEATURE] plumb-cid.md` | **No existe** (pending solo: PBI-045, F3 git-manager residual, delivery-close revoked) | **Hueco KM** — Mayeuta **no** forja PBI (solo Cumulo / `Kaizen_Alert_Required`) |
| `persist_ref` inyectado | Vacío → resuelto a `docs/features/plumb-cid` | Precedente lab OK |
| `correlation_id` inyectado | `a1b2c3d4-e5f6-4789-a012-3456789abcde` | Debe quedar **auditable** en frontmatter clarify/objectives |
| Alcance producto amplio | Semilla no aporta qué de dominio más allá del nombre | Alcance = **lab plumb CID** (meta-tubería runtime), no inventar feature de negocio |

---

## D2 — Reutilización vs invención (entropía rechazada)

| Tentación | Laudo |
|-----------|-------|
| Inventar PBI bajo `docs/todos/` desde Mayeuta | **Veto** — Cumulo / Kaizen_Alert |
| Absorber residual F3 `git-manager` KM (PPR #136) como alcance de este ciclo | **Fuera** salvo laudo Racso (soft-dep operativo ortogonal) |
| Reabrir diseño pasarela Kalma2 / PBI-044 | **Fuera** |
| Declarar evidencia git sin stdout `git-manager` | **Prohibido** |
| Ampliar a DI / delivery-close / GesFer | **Fuera** |

---

## D3 — Vectores soberanos estabilizados (lab)

| ID | Qué (requisito estable) | Piso Done lab |
|----|-------------------------|---------------|
| **L-CID-FM** | Frontmatter de `clarify.md` y `objectives.md` declara el mismo `correlation_id` inyectado | Sí |
| **L-PERSIST** | Artefactos bajo `persist_ref` resuelto (`docs/features/plumb-cid`) con frontmatter `features-documentation-pattern` | Sí |
| **L-HANDOFF** | Cuerpo `objectives.md` apto como `refined_requirements` para Dedalo (qué lab, no cómo) | Sí |
| **L-PBI-GAP** | Hueco PBI documentado; no bloquear estabilización del **qué lab**; materialización PBI = Cumulo/operador | Documentado |
| **L-GIT** | Evidencia git solo vía `skill:git-manager` / `./sddia-run.sh --tool git-manager` | Sí (si runtime permite) |
| **L-NO-FAKE** | Ausencia de stdout/artefacto = blocked/NO_APTO en fases posteriores; no inventar éxito | Sí |

---

## D4 — Preguntas abiertas (laudos / handoff Dedalo)

| # | Pregunta | Laudo / default |
|---|----------|-----------------|
| **Q1** | ¿Materializar PBI `[FEATURE] plumb-cid.md` en este ciclo? | **No desde Mayeuta/Tekton/Argos.** Default: Cumulo/operador; Dedalo puede exigir path PBI como precondición de cierre documental (**L-PBI-LOC-LAB**) |
| **Q2** | ¿Alcance más allá del plumb documental CID? | **No** sin laudo Racso; este ciclo = tubería + trazabilidad cid |
| **Q3** | ¿Git evidencia en estabilización? | Intentar `git-manager` status; si Rejected → declarar sin evidencia (no inventar) |
| **Q4** | ¿Blueprint Dedalo? | Plan mínimo: AC de presencia cid en cascada + gates Argos de no-fake; sin forja genoma |

---

## D5 — Criterios de aceptación (mapeo AC lab)

| AC lab | Liga | Nota |
|--------|------|------|
| AC-L-CID | L-CID-FM | `correlation_id` idéntico en clarify + objectives |
| AC-L-DOC | L-PERSIST + L-HANDOFF | Patrón documental + handoff Dedalo |
| AC-L-PBI | L-PBI-GAP | Gap explícito; cierre PBI solo vía Cumulo |
| AC-L-GIT | L-GIT | Evidencia física o declaración honesta de no materializado |
| AC-DONE-LAB | L-NO-FAKE | `validacion.md` APTO solo con evidencia; sin inventar |

---

## D6 — Invariantes innegociables (handoff Dedalo)

1. Paths solo vía `SddIA/core/cumulo.paths.json` (`directories.documentation` / `featurePath`).
2. Git solo `skill:git-manager`; KM/TODOs solo Cumulo / `Kaizen_Alert_Required`.
3. Evidencia = artefacto físico / stdout; ausencia ≠ narrativa de éxito.
4. No mutar genoma Core en este lab salvo fallo demonstrable fuera de alcance actual.
5. `correlation_id` de sesión es SSOT de trazabilidad de este ciclo.

---

## D7 — Fuera de alcance

Forja PBI en `docs/todos/` · residual F3 PPR #136 como producto · pasarela async PBI-044 · DI PBI-042/043/045 · delivery-close revoked · GesFer · mutación allowlist/EDA · bypass Shell destructivo · inventar APTO.

---

## D8 — Veredicto Mayeuta

**ok** — requisitos lab termodinámicamente estables (L-CID-FM…L-NO-FAKE). Hueco PBI documentado (no bloquea el **qué** lab). Handoff a Dedalo: blueprint mínimo de evidencia CID + gates no-fake; sin inventar producto de dominio.

**Git esta fase:** pendiente intento `./sddia-run.sh --tool git-manager` — ver registro en sesión; no inventar stdout.
