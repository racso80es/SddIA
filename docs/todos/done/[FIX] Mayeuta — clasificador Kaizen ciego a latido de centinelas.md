---
document_id: PBI-FIX-MAYEUTA-HB-KAIZEN-CLASSIFIER
uuid: "e3e0f05f-59bf-48c2-864a-0275049f4f1d"
title: "[FIX] Mayeuta — clasificador Kaizen ciego a latido de centinelas"
format: markdown
version: "1.0.0"
created: "2026-08-30"
updated: "2026-08-30"
closed: "2026-08-30"
status: "cerrado"
priority: alta
process: bug-fix
type: bug-fix
dispatch: true
suggested_branch: fix/mayeuta-heartbeat-kaizen-classifier
persist_ref: docs/fixes/mayeuta-heartbeat-kaizen-classifier
resolution_ref: docs/fixes/mayeuta-heartbeat-kaizen-classifier/
friction_id: F-MAYEUTA-HB-BLIND
friction_ids:
  - F-MAYEUTA-HB-BLIND
  - F-MAYEUTA-HB-TOKEN-TRAP
source_audit: "Auditoría Tekton 2026-08-30 — analyze_fracture_kaizen vs traza canónica emit_system_fracture; 24 PBIs ledger con fallback genérico; especimen 6c0db1296181; precedente F-MAYEUTA-DCC-TOKEN-COLLISION (d0cfd5b66ff1)"
review_notes: "Alcance acotado a F0–F1 (cubo latido + tests de no-regresión). F2–F5 (LLM Kintsugi, payload missed_cycles, plantilla Cúmulo bug-fix, dedup) quedan residuales explícitos."
related_pbis:
  - id: PBI-FIX-FRACTURE-6c0db1296181
    rol: "Especimen vivo: traza canónica Argos + fallback Mayeuta. Laudo A de ese PBI es keepalive email-watcher (cápsula daemon). Este PBI es el clasificador, no el keepalive."
  - id: PBI-FIX-FRACTURE-d0cfd5b66ff1
    rol: "Precedente de clase: token delivery-close contaminaba blob vía process_name. Parcheó el cubo hook; no añadió cubo latido."
  - id: PBI-FIX-FRACTURE-63c439de23d0
    rol: "Cierre humano contradice Mayeuta: inanición de circuito, no muerte. Fallback process_fix era falso."
architectural_constraints:
  - A-MATCH-SOLO-ERROR-TRACE
  - A-NO-TOKEN-HEARTBEAT-EN-BLOB-GENERAL
  - A-NO-COLAPS-COMO-LATIDO
  - A-PARIDAD-REGEX-FAGOCITO
  - A-NO-MUTAR-UMBRALES-ARGOS
related:
  - SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs
  - SddIA/engine/execute-process/src/engine/handlers/daemon_heartbeat.rs
  - SddIA/actions/enrich-fracture-pbi-kaizen.md
  - SddIA/agents/mayeuta.md
  - SddIA/process/daemon-heartbeat-audit.md
  - SddIA/process/phagocyte-recovered-fracture-pbis.md
  - SddIA/engine/execute-process/src/engine/handlers/phagocyte_recovered_fracture_pbis.rs
  - docs/todos/pending/[FIX] email-watcher — fractura sistémica (6c0db1296181).md
  - docs/todos/done/[FIX] delivery-close-cycle — fractura sistémica (d0cfd5b66ff1).md
  - docs/audits/centinelas-fracturas-eventos-pending-20260826.md
---

# [FIX] Mayeuta — clasificador Kaizen ciego a latido de centinelas

## 1. Defecto

`analyze_fracture_kaizen` (`enrich_fracture_pbi_kaizen.rs`) es un matcher de 4 cubos léxicos + fallback. La traza canónica que emite Argos ante omisión de `Daemon_Heartbeat` **no intersecta ningún cubo**. Toda fractura de centinela recibe el mismo párrafo genérico y el veredicto `process_fix`.

Traza canónica (`emit_system_fracture`):

```text
Centinela {daemon_id} omitió {n} ciclos consecutivos de Daemon_Heartbeat (umbral={t}). last_heartbeat={ISO}
```

Especimen (`6c0db1296181`):

- Diagnóstico: *«Causa raíz no clasificada automáticamente para `email-watcher`; requiere laudo humano.»*
- Veredicto: `process_fix`
- Propuesta: *«Auditar proceso `email-watcher`…»* — alucinación ontológica: `email-watcher` es daemon, no proceso.

Ledger: **24 PBIs** (`pending`+`done`) con el literal `Causa raíz no clasificada`. Cierre humano típico (`63c439de23d0`): inanición de circuito, no muerte del órgano. Mayeuta no aportó esa distinción.

## 2. Cadena de fricciones (alcance de este PBI)

| ID | Naturaleza | En alcance |
|----|------------|:----------:|
| **F-MAYEUTA-HB-BLIND** (F0) | Cero tokens para `omitió` / `Daemon_Heartbeat` / `last_heartbeat` / `umbral` | Sí |
| **F-MAYEUTA-HB-TOKEN-TRAP** (F1) | Añadir `heartbeat`/`daemon`/`audit` al blob general clasificaría por `attempted_action=daemon-heartbeat-audit`, no por la traza. Catch-all `colaps`/`failed`/`block` maldiagnostica si Argos cambia el wording | Sí (prohibición + test negativo) |
| F2 LLM Kintsugi desconectado | `mayeuta-llm` no entra en enrich | No — residual |
| F3 Payload `missed_cycles` desechado | Contrato enrich no consume campos estructurados de Argos | No — residual |
| F4 Plantilla Cúmulo fuerza `bug-fix` | Título «colapso» / mandato de entrega sobre deuda a menudo laudo B | No — residual |
| F5 Dedup por proceso × enrich ciego | Reescritura del mismo fallback | No — residual |

Precedente de clase: `F-MAYEUTA-DCC-TOKEN-COLLISION` (`d0cfd5b66ff1`). Se aisló el blob de hook a `error_trace+attempted_action`. **No se añadió cubo de latido.** Los tests de heartbeat cubren dedup/path, no clasificación.

## 3. Remediación (F0–F1)

### Dentro

Cubo `heartbeat_starvation` en `analyze_fracture_kaizen`:

1. Match **exclusivo** sobre `error_trace` (no `attempted_action`, no `process_name`).
2. Regex alineada al parser del fagocito (`last_heartbeat=ISO` en `phagocyte_recovered_fracture_pbis.rs`) y al `format!` de `emit_system_fracture`.
3. Veredicto: `refactor_tool` — inanición de latido con proceso vivo (el auditor no emite si el PID está muerto). Texto: no es muerte del centinela; no es «auditar proceso `{daemon_id}`».
4. No usar tokens `heartbeat` / `daemon` / `audit` / `colaps` en el blob concatenado para este cubo.

Tests (`cargo test -p execute-process -- analyze_fracture_kaizen`):

| Test | Traza | No debe | Debe |
|------|-------|---------|------|
| Canónica Argos | `Centinela email-watcher omitió 3 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-08-30T07:51:47Z` | fallback «no clasificada»; `process_fix`; «Recursión» | cubo latido; `refactor_tool` |
| Negativo DCC DNS | `fatal: Could not resolve host: github.com` + acción `Publicación remota` | cubo latido | (regresión `analyze_fracture_kaizen_dns_not_hook_recursion`) |
| Negativo hook | `pre-push hook blocked` | cubo latido pisa hook | `refactor_tool` hook (test existente) |
| Trampa `attempted_action` | traza **sin** patrón Argos + `attempted_action=daemon-heartbeat-audit` | cubo latido | fallback u otro cubo real |

Bump documental de `enrich-fracture-pbi-kaizen.md` vía `entity-manager` (DA-2): describir el cubo y la prohibición de tokens en blob general. Handler nativo en `SddIA/engine/` no es genoma; el `{name}.md` de la acción sí.

Cascada `bug-fix`: `persist_ref` = `docs/fixes/mayeuta-heartbeat-kaizen-classifier`. Rama sugerida `fix/mayeuta-heartbeat-kaizen-classifier`.

### Fuera

- Invocar `mayeuta-llm` desde enrich (F2).
- Extender inputs con `missed_cycles` / `fracture_kind` (F3).
- Cambiar plantilla `build_pbi_body` / dejar de forzar `process: bug-fix` (F4).
- Keepalive de `email-watcher` — jurisdicción `PBI-FIX-FRACTURE-6c0db1296181`.
- Mutar umbrales Argos (`missed_cycles_threshold`, `suspend_skew_seconds`).
- Discriminar `host_suspend` / cold-start en Mayeuta: el auditor ya no emite esas; no reabrir el debate en el clasificador.
- Fagocitar PBIs históricos con fallback: archivo documental aparte, no este fix.

## 4. Afirmaciones descartadas

| Tentación | Por qué no |
|-----------|------------|
| `blob.contains("heartbeat")` | `attempted_action` siempre contiene `daemon-heartbeat-audit` → 100 % falso positivo de clase DCC |
| Catch-all `colaps` como latido | Argos dice `omitió`. Si cambiara a «colapsó», el cubo actual `prompt_adjustment` («bloqueo sin Kintsugi») seguiría siendo falso |
| Veredicto `process_fix` | `process_name` del payload es `daemon_id`, no un proceso de `directories.process` |
| LLM como camino principal | Viola paridad determinista del handler nativo; LLM es residual F2 |

## 5. Criterio de cierre

- [x] Cubo latido match solo `error_trace` + formato canónico Argos
- [x] Tests de la tabla §3 verdes; tests hook/DNS existentes siguen verdes
- [x] `enrich-fracture-pbi-kaizen.md` bump via `entity-manager` (cubo + prohibición F1)
- [x] `validacion.md` global `APTO`, `pbi_archived: true`
- [x] Este TODO en `docs/todos/done/` en la rama del PR
