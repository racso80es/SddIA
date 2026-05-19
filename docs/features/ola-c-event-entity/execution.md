---
feature_name: ola-c-event-entity
branch_name: feat/ola-c-event-entity
executed_at: "2026-05-19"
---

# Ejecución — Ola C: Evento como entidad de dominio

Registro de commits y fases materializadas en la rama `feat/ola-c-event-entity`.

## Commits de entrega

| Commit | Mensaje | Fase |
|--------|---------|------|
| `291aa25` | Topología Ola C (README, consumidores, `.gitignore`) | Pre-Hito 1 |
| `430c0a1` | Corrección ruta runtime `docs/events/` | Pre-Hito 1 |
| `b35f9fb` | Planificación v2 forense ECST | Plan |
| `9ebd37d` | Hito 1: Constitución §3.1, contrato, índice | Hito 1 |
| `9492d84` | Fase 2: `event-creator` | Fase 2 |
| `c530a83` | Fase 3: `entity-manager` piloto `event` | Fase 3 |
| `5e8c0ad` | Fase 4: 5 Clases ECST forense | Fase 4 |
| `ad78616` | Fase 5: validación cruzada ECST | Fase 5 |

## Comandos de verificación reproducibles

### Integridad procesos

```powershell
python SddIA/scripts/qa/verify-process-integrity.py
```

### Forja vía entity-manager (piloto event)

```powershell
'{"entity_class":"event","entity_name":"test-event","lifecycle_operation":"create","semantic_seed":{"event_name":"test-event","event_type":"Test_Event","payload_required":[],"payload_optional":[],"payload_forbidden":[]}}' | python SddIA/scripts/qa/execute-process.py
```

*(Requiere `event_type` único; usar solo en laboratorio.)*

### E2E bus (PullRequest_Presented → processed)

```powershell
python SddIA/scripts/qa/execute-process.py --action emit-pr-presented-event --inputs '{"branch":"feat/smoke","status":"presented"}'
python SddIA/scripts/daemons/event-watcher.py --once
```

Comprobar que el JSON aparece en `docs/events/processed/`.

### Validación ECST (instancia inválida → dead-letter)

Emitir JSON con `event_type` no catalogado en `docs/events/pending/` y enrutar con `--event-file-path`; debe terminar en `docs/events/dead-letter/` con `delivery_state.ecst_validation = failed`.

## Handoff

Entrega lista para `delivery-close-cycle` con `source_process: feature` y rama `feat/ola-c-event-entity`.
