---
feature_name: pr-presented-orchestration
created: "2026-05-20"
process: feature
purpose: Simetría fractal presentación PR ↔ fusión accept-pr
---

# Clarificación — Orquestación PR presentado

Transcript de decisiones (2026-05-20), incluyendo síntesis S+ del Arquitecto.

---

## D1 — Inicio formal

| Pregunta | Decisión |
|----------|----------|
| ¿Proceso de inicio? | **`feature`** v1.2.0 |
| Rama propuesta | `feat/pr-presented-orchestration` |
| `persist_ref` | `docs/features/pr-presented-orchestration` |
| Manifiesto operativo | TODO renombrado conceptualmente: orquestación en proceso, no acción monolítica |

---

## D2 — Síntesis S+ (impase resuelto)

| Opción descartada | Motivo |
|-------------------|--------|
| Acción **`request-change-incorporation`** (PR + bus) | Viola SRP; caja negra; **Entropía Táctica** frente al patrón ya validado en merge |

| Opción adoptada | Motivo |
|-----------------|--------|
| Orquestación en **`delivery-close-cycle`** | Misma física que **`accept-pr`**: proceso secuencial, acciones atómicas al final de cada tramo físico |

**Analogía fractal (extremos del ciclo):**

| Extremo | Proceso orquestador | Hacer físico | Sello EDA (acción pura) | Evento |
|---------|---------------------|--------------|-------------------------|--------|
| Presentación | `delivery-close-cycle` | `git-manager` push + `shell-executor` + `gh pr create` | `emit-pr-presented-event` | `PullRequest_Presented` |
| Fusión | `accept-pr` | `git-manager` merge + push + higiene | `emit-pr-merged-event` | `PullRequest_Merged` |

---

## D3 — Destino de `emit-pr-presented-event`

| Pregunta | Decisión |
|----------|----------|
| ¿Absorber en acción combinada? | **No** |
| ¿Deprecar? | **No** — acción **sobrevive intacta** |
| Jurisdicción | Única: escribir `PullRequest_Presented` en `eda_bus.pending` y preparar anclaje vía watcher |
| Emisor en payload | `emitter_agent: delivery-close-cycle` (proceso invocante), no el nombre de la acción |

---

## D4 — Pasos atómicos en `delivery-close-cycle` (fase «Sync remoto y PR» + sello)

| Paso | Delegado | Responsabilidad |
|------|----------|-----------------|
| **A** | `skill:git-manager` | `push` de `branch_name` a `origin` (precondición Cerbero/Argos según norma) |
| **B** | `skill:shell-executor` | `gh pr create` (o `gh pr view` si PR ya existe) — **`gh` prohibido en git-manager** |
| **C** | `action:emit-pr-presented-event` | Inputs: `branch`, `status`, `emitter_agent`; propagar `pr_url` al output del **proceso** (no obligatorio en payload ECST v1.0) |

Orden: **A → B → C**. Higiene local permanece **después** del sello (no antes), para no cerrar ramas antes de tener `pr_url`.

---

## D5 — Corrección de deuda en genoma actual

| Hallazgo | Decisión |
|----------|----------|
| `delivery-close-cycle.md` declara fase final `emit-pr-merged-event` / `PullRequest_Merged` | **Error de cableado** — merge pertenece solo a `accept-pr`. Esta feature **elimina** esa fase y la sustituye por sello **Presented** |

---

## D6 — `pr_url` en payload ECST (abierto → propuesta)

| Pregunta | Propuesta para implementación | Estado |
|----------|------------------------------|--------|
| ¿Añadir `pr_url` opcional a `PullRequest_Presented`? | **Sí** — implementado en evento/acción v1.1 y handlers lab | ✅ 2026-05-20 |
| Si se rechaza ampliación ECST | `pr_url` solo en **outputs del proceso** y en `validacion.md`; correlación manual en laboratorio | Fallback documentado |

**Recomendación S+:** incluir `pr_url` opcional en payload — cierra el síntoma «PR #7 sin evento correlacionado».

---

## D7 — Inputs del proceso para paso B

| Campo | Obligatorio | Origen |
|-------|-------------|--------|
| `branch_name` | Sí | Ya existe en contrato |
| `pr_title` | Sí* | Derivado de `feature_name` / frontmatter `objectives.md` si el orquestador no lo inyecta |
| `pr_body` | No | Ruta `persist_ref/validacion.md` o cuerpo mínimo generado |

\* En laboratorio, el handler puede leer título desde inputs-file; en runtime IDE, el orquestador padre (`feature`) inyecta.

---

## D8 — Normas y catálogo

| Artefacto | Cambio |
|-----------|--------|
| `pull-request-orchestration.md` | Presentación = secuencia del **proceso** `delivery-close-cycle`; prohibido promover acción monolítica PR+bus |
| `pull-request-presented.md` | Emisor único autorizado: `emit-pr-presented-event` (sin `request-change-incorporation`) |
| `actions/index.md` | Sin fila `request-change-incorporation` |
| PBI-005 operativo | CA-3: proceso + acción pura, no acción combinada |

---

## D9 — Git y commits

Commits atómicos por sub-entrega: (1) docs spec/clarify + TODO, (2) genoma proceso/norma/evento, (3) handler laboratorio, (4) validación smoke.

Merge hacia `main` vía **`accept-pr`** cuando Argos emita APTO (no vía `delivery-close-cycle`).
