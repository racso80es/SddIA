---
feature_name: ia-obrera-blindaje
created: "2026-05-22"
process: feature
purpose: Estabilización de requisitos — blindaje IA obrera (Fases A y B del TODO)
---

# Clarificación — Blindaje Ontológico IA Obrera

Transcript de decisiones (2026-05-22). Resuelve ambigüedades del TODO antes de implementación Tekton.

---

## D1 — Inicio formal

| Pregunta | Decisión |
|----------|----------|
| ¿Proceso? | **`feature`** v1.2.0 |
| Rama | `feat/ia-obrera-blindaje` |
| `persist_ref` | `docs/features/ia-obrera-blindaje` |
| Manifiesto | `docs/todos/TODO-BLINDAJE-IA-OBRERA.md` |
| Alcance | **Fases A + B** únicamente |

---

## D2 — Fase C (Aduana física) — fuera de alcance

| Entrega Fase C | Estado | Decisión |
|----------------|--------|----------|
| `pre-commit` VPI + bus | ✅ PR #12 | No reabrir |
| `pre-push` / `post-merge` | ✅ PR #13 Ola B | No reabrir |
| Norma `external-ai-constraints.md` | ⏳ | **Esta feature** (Fase A) |

**Motivo:** Fase C cerrada en features hermanas `pbi-005-hito3-git-hooks` y `pbi-005-hito3-ola-b`. Esta feature completa el vacío normativo pendiente en la tabla del TODO.

---

## D3 — Ubicación de la norma (motor vs library)

| Opción descartada | Motivo |
|-------------------|--------|
| `SddIA/library/norms/` vía `norm-creator` | `norm-creator` materializa **normas tácticas** de librería; el blindaje es **comportamiento motor** de IA, no patrón de producto |

| Opción adoptada | Motivo |
|-----------------|--------|
| **`SddIA/norms/external-ai-constraints.md`** | Coherente con `touchpoints-ia.md`, `obediencia-procesos.md` y el ejemplo del TODO; referenciada en `cumulo.paths.json` → `directories.norms` |

---

## D4 — Estrategia touchpoint (.cursorrules vs .cursor/rules)

| Pregunta | Decisión |
|----------|----------|
| ¿SSOT del contenido normativo? | **`SddIA/norms/external-ai-constraints.md`** — touchpoints solo difunden |
| ¿Dónde inyectar? | **`.cursorrules`** — añadir §8 «Blindaje IA Obrera» con referencia explícita a la norma y resumen de las tres Directrices de Acero |
| ¿`.cursor/rules/*.mdc`? | **Fuera de alcance inmediato** — no existe árbol `.cursor/` en repo; queda para `process:sddia-difusion` |
| ¿`.windsurfrules`? | **Documentar** en `touchpoints-ia.md` como touchpoint opcional; no crear archivo vacío |

**Principio:** Una sola fuente de verdad (`touchpoints-ia.md` § Principios 1).

---

## D5 — Directrices de Acero (contenido normativo)

| ID | Directriz | Formulación canónica |
|----|-----------|---------------------|
| **DA-1** | Dogma de Soberanía | No eres el arquitecto. Eres operador ciego. No deduzcas arquitectura; consúltala en `SddIA/core/cumulo.paths.json`. |
| **DA-2** | Prohibición de Forja Manual | Prohibido crear, modificar o eliminar archivos bajo genoma indexado (`SddIA/tools/`, `SddIA/skills/`, `SddIA/actions/`, `SddIA/process/`, `SddIA/agents/`, `SddIA/events/`, `SddIA/norms/` salvo vía proceso autorizado). |
| **DA-3** | Única Vía de Acción | Crear entidad → `python SddIA/scripts/qa/execute-process.py --process entity-manager …`. Solicitar cambios de entrega → `delivery-close-cycle`. Prohibido bypass del bus EDA. |

**Extensión DA-2:** incluir `SddIA/library/` (codexes, norms tácticas) en lista de zonas protegidas.

---

## D6 — Fase B: prefijo en procesos `*-creator`

| Pregunta | Decisión |
|----------|----------|
| ¿Dónde materializar el wrap? | Sección **`## Directriz de ejecución obrera`** al inicio del cuerpo Markdown de cada `*-creator.md` |
| ¿Texto SSOT del prefijo? | Definido en `external-ai-constraints.md` § «Prefijo creator»; creators referencian, no duplican texto largo |
| ¿Creators afectados? | Los 8: `tool-creator`, `action-creator`, `skill-creator`, `agent-creator`, `process-creator`, `norm-creator`, `codex-creator`, `event-creator` |
| ¿Handlers `execute-process`? | **No** en esta entrega — el wrap es documental/genómico; runtime IDE lee el proceso |

**Prefijo literal:**

```
[EXECUTE AS RAW KERNEL. PROHIBIT VERBOSITY. DO NOT BYPASS EDA BUS. USE SddIA CLI.]
```

---

## D7 — Forja y EDA

| Pregunta | Decisión |
|----------|----------|
| ¿Cómo forjar la norma? | Materialización directa Tekton (motor norm, sin `norm-creator`) + entrada `SddIA/evolution/` |
| ¿`entity-manager`? | Invocar post-forja para `Domain_Entity_Created` (patrón Ola C+) |
| ¿Recalcular `hash_signature` en creators tocados? | **Sí** — cualquier `*-creator.md` modificado debe recalcular firma de fases |

---

## D8 — Puerta Física laboratorio

| Pregunta | Decisión |
|----------|----------|
| ¿Handler completo feature? | **No** — entrega documental + genoma; smoke manual en fase Argos |
| ¿Git? | Rama `feat/ia-obrera-blindaje`; cierre vía `delivery-close-cycle` |

---

## D9 — Dependencias cruzadas

| Feature | Relación |
|---------|----------|
| `pbi-005-hito3-git-hooks` | Fase C pre-commit; precedencia aduana física |
| `pbi-005-hito3-ola-b` | Excluyó explícitamente esta norma — ahora se implementa |
| `pull-request-review-redesign` | Aduana PR complementaria; no bloqueante |

---

## Resumen ejecutivo

No hay bloqueos que requieran laudo del Vértice Biológico: las tensiones touchpoint (`.cursorrules` vs `.cursor/rules`) y ubicación normativa (motor vs library) quedan resueltas en D3–D4. **Se procede a especificación y planificación.**
