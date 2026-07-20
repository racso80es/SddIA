---
feature_name: kalma2-full-cycle
created: "2026-07-20"
process: feature
base: main
scope: kalma2-full-cycle
version_spec: "1.0.0"
uuid: 527007fa-7200-41ee-84bb-202737f4f983
status: dedalo_locked
laudo: A-cycle-phase-first
---

# Especificación — kalma2-full-cycle

## 1. Topología

```text
Kalma2 → kalma2-interact → Kalma2_Process_Requested
  → TQM → hijo (bug-fix|feature|refactorization)
       → workspace-init (físico)
       → agentes (hoy simulated / mañana runtime B)
       → PEC con cycle_phase          ← Slice A
  → GET /api/status → project_status(cycle_phase)
  → UI poll terminal honesto
```

## 2. Laudos Dedalo

| Ref | Pregunta | Laudo | Justificación |
|-----|----------|-------|---------------|
| **L1** | Estados nuevos | `initialized` · `awaiting_agents` | Distinguen arranque vs espera de agentes vs cierre |
| **L2** | `completed` | Solo si PEC `cycle_phase=completed` | Evita engaño e022814f |
| **L3** | Legacy sin `cycle_phase` | Proyectar `completed` (compat) | No romper status históricos |
| **L4** | Derogar L2 process-dispatch | **No** en A | Honestidad ≠ full-cycle |
| **L5** | Derivación `cycle_phase` | Desde `phase_reports` en peaje | Fuente única en state del orquestador |
| **L6** | Runtime B (esta entrega) | Contrato + puntos de extensión documentados; forja mínima opcional | Evitar PR monstruo; A entrega valor ya |

### Regla de derivación (L5)

Para procesos de ciclo `bug-fix` \| `feature` \| `refactorization`:

| Condición en `phase_reports` | `cycle_phase` |
|------------------------------|---------------|
| Alguna fase `awaiting` / `awaiting_agents` | `awaiting_agents` |
| Alguna fase `simulated` | `initialized` |
| Resto (executed/skipped sin simulated) | `completed` |

Otros procesos: omitir `cycle_phase` o `completed` (status bridge legacy).

### Mensajes `project_status` (L1/L2)

| `cycle_phase` | status UI | Mensaje (patrón) |
|---------------|-----------|------------------|
| `initialized` | `initialized` | Ciclo «{p}» arrancado (init OK). Fases de agentes pendientes o simuladas en lab. |
| `awaiting_agents` | `awaiting_agents` | Ciclo «{p}» en espera de agentes IDE. |
| `completed` / ausente+success | `completed` | Proceso «{p}» completado (PEC correlacionado). |
| failure | `failed` | (sin cambio) |

UI: poll termina también en `initialized` y `awaiting_agents` (estados terminales mientras no exista runtime B que avance el ciclo).

## 3. Slice B — runtime de agentes

| Elemento | Contrato |
|----------|----------|
| Trigger | Fase con `delegates_to` solo `agent:*` tras init |
| Env | `SDDIA_AGENT_RUNTIME_COMMAND` (bóveda `.dev/.env`) |
| I/O | JSON stdin `operation=AGENT_PHASE` → última línea JSON stdout |
| Status fase | `executed` \| `awaiting_agents` \| `failed` |
| Sin env | `simulated` → `cycle_phase=initialized` (slice A) |
| Fallo fase | Envelope hijo `success=false` (no PEC success engañoso) |

### Payload stdin (AGENT_PHASE)

```json
{
  "operation": "AGENT_PHASE",
  "process_name": "bug-fix",
  "phase_name": "Diseño del fix",
  "agents": ["dedalo"],
  "persist_ref": "docs/fixes/…",
  "branch_name": "fix/…",
  "correlation_id": "<uuid>",
  "pbi_ref": "docs/todos/pending/….md",
  "inputs": {},
  "workspace_path": null,
  "repo_root": "/abs/path"
}
```

### Respuesta stdout

```json
{"success": true, "data": {"status": "executed", "message": "spec.md materializado"}}
```

## 4. Slice C — contrato

| Elemento | Contrato |
|----------|----------|
| Input | `pbi_ref` resuelto (ya A′ en process-dispatch) |
| Enriquecimiento | Leer FS del PBI → `pbi_body` / semilla de `objectives.md` |
| Momento | Preferible en TQM/`workspace_init` al despachar hijo |

## 5. Touchpoints código (Slice A)

| Artefacto | Cambio |
|-----------|--------|
| `thermodynamic.rs` | Emitir `cycle_phase` (+ resumen fases opcional) en payload PEC |
| `kalma2-bridge` `project_status` | Mapear `cycle_phase` → status/message |
| `interfaces/kalma2/app.js` | Terminal poll: `initialized` \| `awaiting_agents` |
| `interfaces/kalma2/style.css` | Colores status nuevos |

## 6. Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-A1 | PEC de `bug-fix` lab con fases `simulated` incluye `cycle_phase=initialized` |
| AC-A2 | `/api/status` proyecta `initialized` (no `completed`) para ese PEC |
| AC-A3 | UI deja de pollar y muestra mensaje de arranque, no «completado» |
| AC-A4 | PEC legacy sin `cycle_phase` → `completed` (compat) |
| AC-A5 | Tests unitarios bridge + derivación thermodynamic |
| AC-B0 | `spec`/`plan` documentan contrato runtime B (esta entrega) |
| AC-B1 | Con `SDDIA_AGENT_RUNTIME_COMMAND`, fase `agent:` → `handler=agent-runtime` (no `simulated`) |
| AC-B2 | CLI mock `status=executed` → fase `executed`; `awaiting_agents` → fase `awaiting_agents` |
| AC-B3 | Sin env → sigue `simulated` (compat A) |
| AC-B4 | Fase `failed` → envelope hijo `success=false` |
| AC-C0 | `spec`/`plan` documentan consumo `pbi_body` (forja C diferible) |
