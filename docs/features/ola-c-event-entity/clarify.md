---
feature_name: ola-c-event-entity
created: "2026-05-19"
process: feature
purpose: Estabilización de requisitos Ola C — Evento, bus runtime e instancia
---

# Clarificación — Ola C (Event Entity)

Transcript de decisiones del Arquitecto (2026-05-19) para cerrar ambigüedades detectadas en el triaje inicial.

---

## D1 — Inicio de feature

| Pregunta | Decisión |
|----------|----------|
| ¿Proceso de inicio? | **`feature`** (`SddIA/process/feature.md`, v1.2.0). |
| ¿Puerta Física con handler completo? | **No.** Opción **B**: Git vía cápsula **`git-manager`** (`scripts/skills/git-manager.py`). |
| Rama | `feat/ola-c-event-entity` |
| `persist_ref` | `docs/features/ola-c-event-entity` |

---

## D2 — Doctrina vs ontología operativa

| Pregunta | Decisión |
|----------|----------|
| ¿Enmienda a `CONSTITUTION_CORE.md`? | **Sí** (laudo Vértice Biológico + Tormentosa, Hito 1). §3.1 Genoma operativo con **Event**. Revoca decisión inicial “solo README”. |
| ¿README? | **Sí** — mapa visual tres rutas (complemento operativo, no sustituto de Constitución). |
| Definición de Event | *El contrato inmutable de comunicación asíncrona. Representa una señal con propósito (finalidad) que blinda la soberanía de las entidades conscientes, operando bajo el paradigma de coreografía pura y evitando el acoplamiento físico entre procesos.* |

---

## D3 — Resegmentación del bus (decisión estructural)

### Antes (Ola A)

| Rol | Ruta |
|-----|------|
| Bus runtime (colas volátiles) | `.SddIA/events/{pending,processed,dead-letter}` |

### Después (Ola C)

| Rol | Ruta | Naturaleza |
|-----|------|------------|
| **Genoma — Clases de Evento** | `SddIA/events/` | Definiciones versionadas (`{name}.md`, contrato, índice). Versionado en repo. |
| **Runtime — Instancias volátiles** | `docs/events/{pending,processing,processed,dead-letter}` | Bus local de tránsito; **no** versionado (`.gitignore`). |
| **Instancia — Personalización** | `.SddIA/events/` | Overrides y configuración táctica por proyecto (Vía C); **no** cola del bus federal. |

### Cola `processing`

- **Nueva** cola intermedia entre `pending` y destino final.
- Semántica: el watcher **promueve** el JSON de `pending/` → `processing/` antes de invocar `route-domain-event`; el enrutador consume desde `processing/` y mueve a `processed/` o `dead_letter/`.
- Motivo: evitar doble consumo concurrente y dejar trazabilidad de eventos en vuelo.

---

## D4 — Git y commits

| Pregunta | Decisión |
|----------|----------|
| Canal Git | **`git-manager`** exclusivamente. |
| Commit constitucional (README + cumulo + consumidores) | `docs(core): legalización ontológica de Evento y resegmentación del bus para Ola C` |

---

## D5 — `.gitignore`

| Ruta | Acción |
|------|--------|
| `.SddIA/events/` | Mantener ignorado (personalización instancia). |
| `docs/events/` | **Añadir** a `.gitignore` (runtime volátil). |

---

## D6 — Cúmulo — claves propuestas

```json
"directories": {
  "events": "SddIA/events"
},
"eda_bus": {
  "pending": "docs/events/pending",
  "processing": "docs/events/processing",
  "processed": "docs/events/processed",
  "dead_letter": "docs/events/dead-letter",
  "subscriptions": "SddIA/core/event-subscriptions.json"
},
"eda_instance": {
  "customization": ".SddIA/events"
}
```

---

## D7 — Consumidores a actualizar (post-spec)

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/scripts/qa/execute-process.py` | Fallback `eda_bus.pending` → `docs/events/pending` |
| `SddIA/scripts/daemons/event-watcher.py` | Rutas SSOT + promoción `pending` → `processing` |
| `SddIA/actions/emit-domain-mutation.md` | Literales y fallback |
| `SddIA/actions/emit-pr-merged-event.md` | Ejemplo `target_path` |
| `SddIA/actions/route-domain-event.md` | Input desde `pending` o `processing`; destinos vía SSOT |
| `SddIA/norms/execution-contexts.md` | Autorización RBAC sobre `docs/events/` |
| `SddIA/process/delivery-close-cycle.md` | Referencia al bus |
| `SddIA/process/accept-pr.md` | `target_path` |
| `.gitignore` | `docs/events/` |

Los documentos bajo `SddIA/evolution/*-temp.md` **no** se mutan (borradores históricos no normativos).

---

## D8 — Secuencia de ejecución acordada

1. ✅ Rama `feat/ola-c-event-entity` (git-manager).
2. ✅ Paquete documental inicial (`objectives`, `clarify`, `spec`).
3. ✅ Implementación topológica (README, cumulo, consumidores, `.gitignore`) — commits `291aa25`, `430c0a1`.
4. ✅ Planificación Dedalo v2 (`plan.md` + laudo forense).
5. ✅ **Hito 1** — Constitución + `events-contract` + índice + `contracts.events` en Cúmulo.
6. ⏳ Tekton — Fases 2–6 (`event-creator`, `entity-manager`, clases ECST, Argos).

---

## D9 — Aseguramiento forense ECST (laudo Dedalo)

| Evento | Campo | Estatus |
|--------|-------|---------|
| `PullRequest_Merged` | `merge_commit_hash` | **REQUIRED** (40 hex, ancla DLT IOTA) |
| `PullRequest_Merged` | `hash_signature` en payload | **PROHIBIDO** (Opción A — sin contaminación semántica) |
| `Domain_Entity_Created` | `hash_signature_new` | **REQUIRED** |
| `Domain_Entity_Created` | `payload_schema_hash` | **OPTIONAL** (transición; no romper emisores Ola A) |

---

## D10 — Ruta runtime (confirmación final)

- **Canónica:** `docs/events/{pending,processing,processed,dead-letter}`.
- **Descartada:** `.docs/events/` (no alinear Cúmulo ni consumidores a esta variante).
- **Referencia SSOT:** `clarify.md` D3/D6 = `cumulo.paths.json` = README = consumidores activos.

---

## Preguntas cerradas (sin pendiente)

- ~~`processing/` vs `processed/`~~ → Se adopta **ambas**: `processing` es cola de vuelo; `processed` es terminal de éxito.
- ~~¿Constitución o README?~~ → **Ambos** (Hito 1); laudo posterior revoca “solo README”.
- ~~¿Ruta runtime?~~ → **`docs/events/`** (instancias generadas; no confundir con `SddIA/events/` ni `.SddIA/events/`).
- ~~¿`hash_signature` en payload Git?~~ → **No**; usar solo `merge_commit_hash`.
