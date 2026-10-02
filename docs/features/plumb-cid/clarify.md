---
feature_name: plumb-cid
created: "2026-07-23"
updated: "2026-10-02"
process: feature
purpose: Estabilización Mayeuta — lab plumb correlation_id en cascada documental feature (kalma2-agent-runtime-cursor)
branch_name: feat/plumb-cid
persist_ref: docs/features/plumb-cid
pbi_ref: docs/todos/pending/[FEATURE] plumb-cid.md
document_id: LAB-PLUMB-CID
correlation_id: a1b2c3d4-e5f6-4789-a012-3456789abcde
phase: mayeuta-stabilization
agents: mayeuta
execution_id: "8d69c53d-42dc-4462-a95a-8a869f9d0726"
---
# Clarificación — plumb-cid

Transcript Mayeuta (2026-10-02). Semilla operador: «inicia feature docs/todos/pending/[FEATURE] plumb-cid.md» + orden Raw Kernel fase Estabilización (`correlation_id` a1b2c3d4-…, `execution_id` 8d69c53d-…).

Reafirmación sobre estabilizaciones previas: cascada documental ya materializada bajo `persist_ref`; esta sesión **no inventa** producto ni PBI; consolida el **qué** lab como `refined_requirements` para Dedalo.

`persist_ref` resoluble vía `paths.featurePath` (`docs/features`) + `feature_name` → `docs/features/plumb-cid` (SSOT `SddIA/core/cumulo.paths.json`).

---

## D0 — Apertura formal

| Pregunta | Decisión |
|----------|----------|
| Proceso | `feature` (fase Estabilización → handoff Dedalo) |
| `feature_name` | `plumb-cid` |
| Rama | `feat/plumb-cid` |
| `persist_ref` | `docs/features/plumb-cid` |
| `document_id` | `LAB-PLUMB-CID` |
| PBI físico | **Ausente** en `docs/todos/pending/` y sin match `*plumb*` en `docs/todos/` (reconfirmado 2026-10-02 / exec 8d69c53d; pending = deudas Tracker / Paciente 0) |
| Naturaleza ciclo | **Lab / humo de tubería** — plumb de `correlation_id` en artefactos Mayeuta; no producto de dominio nuevo |
| Fase | Estabilización Mayeuta (esta sesión) → Dedalo consume este cuerpo |

---

## D1 — Semilla vs realidad

| Afirmación | Hecho | Laudo |
|------------|-------|-------|
| Intención = iniciar feature `plumb-cid` | Artefactos bajo `docs/features/plumb-cid/` presentes (clarify/objectives + cascada posterior) | Fuente `raw_user_intent` válida |
| PBI en `docs/todos/pending/[FEATURE] plumb-cid.md` | **No existe** (0 hits plumb-cid en `docs/todos/`) | **Hueco KM** — Mayeuta **no** forja PBI (solo Cumulo / `Kaizen_Alert_Required`) |
| `correlation_id` inyectado | `a1b2c3d4-e5f6-4789-a012-3456789abcde` | Debe quedar **auditable** en frontmatter clarify/objectives |
| `execution_id` | `8d69c53d-42dc-4462-a95a-8a869f9d0726` | Trazabilidad de sesión; no sustituye CID |
| Alcance producto amplio | Semilla no aporta dominio más allá del nombre | Alcance = **lab plumb CID** (meta-tubería runtime) |

---

## D2 — Reutilización vs invención (entropía rechazada)

| Tentación | Laudo |
|-----------|-------|
| Inventar PBI bajo `docs/todos/` desde Mayeuta | **Veto** — Cumulo / Kaizen_Alert |
| Absorber residual F3 `git-manager` KM / deudas Tracker como alcance | **Fuera** salvo laudo Racso |
| Reabrir diseño pasarela Kalma2 / DI / GesFer | **Fuera** |
| Declarar evidencia git sin stdout `git-manager` | **Prohibido** |
| Ampliar a mutación genoma Core | **Fuera** |

---

## D3 — Vectores soberanos estabilizados (lab)

| ID | Qué (requisito estable) | Piso Done lab |
|----|-------------------------|---------------|
| **L-CID-FM** | Frontmatter de `clarify.md` y `objectives.md` declara el mismo `correlation_id` inyectado | Sí |
| **L-PERSIST** | Artefactos bajo `persist_ref` resuelto con frontmatter `features-documentation-pattern` | Sí |
| **L-HANDOFF** | Cuerpo `objectives.md` apto como `refined_requirements` para Dedalo (qué lab, no cómo) | Sí |
| **L-PBI-GAP** | Hueco PBI documentado; no bloquear estabilización del **qué** lab; materialización PBI = Cumulo/operador | Documentado |
| **L-GIT** | Evidencia git solo vía `skill:git-manager` / `./sddia-run.sh --tool git-manager` | Sí (si runtime permite) |
| **L-NO-FAKE** | Ausencia de stdout/artefacto = blocked/NO_APTO en fases posteriores; no inventar éxito | Sí |

---

## D4 — Preguntas abiertas (laudos / handoff Dedalo)

| # | Pregunta | Laudo / default |
|---|----------|-----------------|
| **Q1** | ¿Materializar PBI `[FEATURE] plumb-cid.md` en este ciclo? | **No desde Mayeuta/Tekton/Argos.** Default: Cumulo/operador; Done de proceso exige PBI físico + archive |
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

Forja PBI en `docs/todos/` · residuales Tracker / F3 git-manager como producto · pasarela async · DI · GesFer · mutación allowlist/EDA · bypass Shell destructivo · inventar APTO.

---

## D8 — Evidencia git (esta sesión Mayeuta)

| Intento | Resultado |
|---------|-----------|
| `./sddia-run.sh --tool git-manager` (JSON stdin `status`) | **Rejected** — sin stdout físico materializado en esta sesión IDE |
| Bypass `git` raw / Shell IDE como evidencia | **Prohibido** — no usado como prueba de éxito |
| Conclusión L-GIT / AC-L-GIT (fase Mayeuta) | **No materializado** — declarado explícito; no inventado |

---

## D9 — Veredicto Mayeuta

**ok** — requisitos lab termodinámicamente estables (L-CID-FM…L-NO-FAKE). Hueco PBI reconfirmado (no bloquea el **qué** lab). Handoff a Dedalo: este `clarify.md` + `objectives.md` como `refined_requirements`; blueprint mínimo de evidencia CID + gates no-fake; sin inventar producto de dominio.

**Git esta fase:** no materializado (Rejected). Done de proceso feature permanece condicionado a PBI vía Cumulo (fuera de esta fase).
