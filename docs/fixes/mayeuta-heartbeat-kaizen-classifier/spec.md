---
feature_name: mayeuta-heartbeat-kaizen-classifier
created: "2026-08-30"
process: bug-fix
base: main
scope: mayeuta-heartbeat-starvation-cube
branch_name: fix/mayeuta-heartbeat-kaizen-classifier
persist_ref: docs/fixes/mayeuta-heartbeat-kaizen-classifier
pbi_ref: docs/todos/pending/[FIX] Mayeuta — clasificador Kaizen ciego a latido de centinelas.md
document_id: PBI-FIX-MAYEUTA-HB-KAIZEN-CLASSIFIER
execution_id: "507e8ff0-388a-4040-8c52-c23b87af1dfd"
---

# Especificación — clasificador Kaizen ciego a latido

## Problema

`analyze_fracture_kaizen` (`enrich_fracture_pbi_kaizen.rs`) es un matcher de 4 cubos léxicos + fallback. La traza canónica que emite Argos (`emit_system_fracture`) **no intersecta ningún cubo**. Toda fractura de centinela recibe `process_fix` genérico.

Traza canónica:

```text
Centinela {daemon_id} omitió {n} ciclos consecutivos de Daemon_Heartbeat (umbral={t}). last_heartbeat={ISO|never}
```

Especimen `6c0db1296181`: *«Causa raíz no clasificada… Auditar proceso `email-watcher`»* — `email-watcher` es daemon, no proceso. Ledger: 24 PBIs con el mismo literal.

## Defectos (alcance)

| ID | Defecto |
|----|---------|
| F0 `F-MAYEUTA-HB-BLIND` | Cero tokens para `omitió` / `Daemon_Heartbeat` / `last_heartbeat` / `umbral`. Caída al fallback. |
| F1 `F-MAYEUTA-HB-TOKEN-TRAP` | Meter `heartbeat`/`daemon`/`audit` en el blob concatenado clasifica por `attempted_action=daemon-heartbeat-audit`, no por la traza. Catch-all `colaps`/`failed`/`block` maldiagnostica si Argos cambia el wording. |

Precedente de clase: `F-MAYEUTA-DCC-TOKEN-COLLISION` (`d0cfd5b66ff1`). El cubo hook ya evalúa `error_trace+attempted_action` sin `process_name`. **No se añadió cubo de latido.**

## Cambio requerido

### F0 — Cubo `heartbeat_starvation` (motor, no DA-2)

`SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs` → `analyze_fracture_kaizen`.

Predicado **exclusivo** sobre `error_trace` (prohibido `attempted_action` y `process_name`):

| Ancla | Fuente |
|-------|--------|
| `Centinela ` | prefijo `emit_system_fracture` |
| `omitió` | mismo `format!` |
| `ciclos consecutivos de Daemon_Heartbeat` | mismo `format!` |
| `umbral=` | mismo `format!` |
| `last_heartbeat=` | mismo `format!`; parser fagocito `last_heartbeat=([0-9T:\-+Z]+)` para ISO; `never` también clasifica |

No exigir parseo ISO exitoso para el cubo (`never` es starvation). No tokens unarios `heartbeat` / `daemon` / `audit` / `colaps` sobre el blob concatenado.

**Veredicto:** `refactor_tool`. Texto: inanición de latido con proceso vivo (el auditor no emite si el PID está muerto); no es muerte del centinela; **prohibido** «Auditar proceso `{daemon_id}`».

**Orden:** evaluar el cubo latido **antes** del catch-all `timeout|block|abort|failed|colaps`. Cubos hook / bypass / huérfano EDA intactos. Prioridad de veredicto existente (`new_norm` > `refactor_tool` > `prompt_adjustment` > `process_fix`) no se reordena.

### F1 — Tests de trampa

Una traza **sin** el patrón Argos + `attempted_action=daemon-heartbeat-audit` **no** entra al cubo latido (cae al cubo real o al fallback).

### Genoma (fase de implementación, no este corte)

Bump `enrich-fracture-pbi-kaizen.md` 1.1.0 → 1.2.0 vía `entity-manager` (`lifecycle_operation: update`, `entity_class: action`): documentar cubo + prohibición F1. Handler nativo no es genoma.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA-1 | Traza canónica Argos (`email-watcher`, 3 ciclos, `last_heartbeat=2026-08-30T07:51:47Z`) → `refactor_tool`; sección **sin** «no clasificada» ni «Auditar proceso» |
| CA-2 | Misma traza con `attempted_action=daemon-heartbeat-audit` **sí** cubo latido (el match es la traza, no la acción) |
| CA-3 | Traza **sin** patrón Argos + `attempted_action=daemon-heartbeat-audit` → **no** cubo latido |
| CA-4 | DNS DCC (`Could not resolve host`) **no** cubo latido; test `analyze_fracture_kaizen_dns_not_hook_recursion` verde |
| CA-5 | `pre-push hook blocked` sigue `refactor_tool` + «Recursión o re-entrada» |
| CA-6 | `cargo test -p execute-process -- analyze_fracture_kaizen` verde |
| CA-7 | `enrich-fracture-pbi-kaizen.md` bump via `entity-manager` (corte de implementación) |
| CA-8 | Cascada documental completa + PBI en `done/` en el PR (cierre posterior a este corte) |

## Fuera de alcance (este PR de implementación; este corte aún más estrecho)

- F2 `mayeuta-llm` en enrich.
- F3 inputs `missed_cycles` / `fracture_kind`.
- F4 plantilla Cúmulo `bug-fix` / título «colapso».
- F5 reescritura por dedup.
- Keepalive `email-watcher` (`PBI-FIX-FRACTURE-6c0db1296181`).
- Umbrales Argos. Discriminar `host_suspend` en Mayeuta.
- Fagocitar PBIs históricos con fallback.
- Código y tests en **este** corte (mandato: spec + plan + commit).
