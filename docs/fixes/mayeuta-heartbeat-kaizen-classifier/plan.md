---
feature_name: mayeuta-heartbeat-kaizen-classifier
created: "2026-08-30"
process: bug-fix
phases:
  - cube-heartbeat-starvation
  - trap-tests
  - entity-manager-bump
  - verify-unit
  - document-and-stop-for-laudo
branch_name: fix/mayeuta-heartbeat-kaizen-classifier
persist_ref: docs/fixes/mayeuta-heartbeat-kaizen-classifier
pbi_ref: docs/todos/pending/[FIX] Mayeuta — clasificador Kaizen ciego a latido de centinelas.md
document_id: PBI-FIX-MAYEUTA-HB-KAIZEN-CLASSIFIER
execution_id: "507e8ff0-388a-4040-8c52-c23b87af1dfd"
---

# Plan — Mayeuta cubo latido

Corte de **esta** entrega: **Diseño (spec + plan) + commit**. Sin implementación de código. **No** `delivery-close-cycle`. **No** `entity-manager`.

La implementación (fases 1–4) queda para el siguiente estímulo Tekton sobre esta rama.

## Fase 1 — Cubo `heartbeat_starvation` (F0, CA-1/CA-2)

`enrich_fracture_pbi_kaizen.rs` `analyze_fracture_kaizen`:

```text
fn is_heartbeat_starvation_trace(error_trace: &str) -> bool
```

Match **solo** `error_trace`. Anclas literales del `format!` de `emit_system_fracture` (`daemon_heartbeat.rs`): `Centinela `, `omitió`, `ciclos consecutivos de Daemon_Heartbeat`, `umbral=`, `last_heartbeat=`.

Si true:

- `root_causes`: inanición de `Daemon_Heartbeat` con proceso vivo; no es muerte del centinela; `process_name` es `daemon_id`, no proceso de `directories.process`.
- `proposals`: `refactor_tool` — emitir latido en worker / no bloquear el hilo de heartbeat (paridad keepalive de centinelas hermanos). Prohibido texto «Auditar proceso `{process_name}`».

Evaluar **antes** del cubo catch-all `timeout|block|abort|failed|colaps`. No tocar cubos hook / bypass / huérfano. No añadir `heartbeat` al array `has_any` del blob concatenado.

## Fase 2 — Tests (F1, CA-1…CA-6)

`cargo test -p execute-process -- analyze_fracture_kaizen`

| Test | Inputs | Aserción |
|------|--------|----------|
| `analyze_fracture_kaizen_heartbeat_starvation` | traza canónica email-watcher 3 ciclos; acción `daemon-heartbeat-audit`; emisor `argos` | `refactor_tool`; sección contiene inanición / Daemon_Heartbeat; **sin** «no clasificada»; **sin** «Auditar proceso» |
| `analyze_fracture_kaizen_heartbeat_not_from_action_name` | traza `timeout in worker` (sin patrón Argos); acción `daemon-heartbeat-audit` | **sin** texto de cubo latido (catch-all o fallback) |
| existentes DNS / hook | sin cambio de fixtures | verdes |

## Fase 3 — Genoma (CA-7)

```text
./sddia-run.sh --process entity-manager --inputs '{
  "entity_class": "action",
  "entity_name": "enrich-fracture-pbi-kaizen",
  "lifecycle_operation": "update",
  "semantic_seed": { … cubo + prohibición F1, version 1.2.0 }
}'
```

DA-2. Tras acuse: DA-5. No mutar `{name}.md` a mano.

## Fase 4 — Verificación

```text
cd SddIA && cargo test -p execute-process -- analyze_fracture_kaizen
```

No `sddia-qa` extra salvo que el bump de acción toque evolution. Evolution se registra en el ciclo de implementación, no en este corte.

## Fase 5 — Este corte (hecho al commit)

1. `objectives.md` / `spec.md` / `plan.md` bajo `persist_ref`.
2. Commit de esos tres paths.
3. Detener. Sin `implementation.md`, sin tests aplicados, sin `entity-manager`, sin DCC, sin push.

## Fuera de este corte

Código motor, tests, bump de acción, evolution, validacion, archivo PBI, PR, keepalive `email-watcher`.
