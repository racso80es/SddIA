---
fix_name: event-bus-audit-kaizen-redb-emit
created: "2026-10-30"
process: bug-fix
branch_name: Kaicen_Auditoria_Eventos_261030
persist_ref: docs/fixes/event-bus-audit-kaizen-redb-emit
scope: "Emisión Kaizen_Alert_Required alineada a SDDIA_EVENT_STORE"
status: IMPLEMENTED
related:
  - docs/audits/kaizen-auditoria-eventos-261030.md
  - docs/fixes/kaizen-event-bus-audit-sensor-tuning/spec.md
---

# Plan — `event-bus-audit`: emisión Kaizen vía gate (redb/dual/filesystem)

## 1. Problema

Con `SDDIA_EVENT_STORE=redb` (config actual de instancia):

| Capa | Comportamiento |
|------|----------------|
| **Censo** (`run_audit`) | Correcto: `audit_state_from_store` si `writes_store`. |
| **Emisión** (`write_pending_kaizen`) | **Incorrecto:** siempre `fs::write` en `.events/pending/`. |

Efecto: bypass del gate, JSON residual, watcher → `F-EDA-RESIDUAL-FILESYSTEM`, desacople respecto a `event-watcher` / `route-domain-event` en store.

`emit_kaizen_alert` **no es el bug**: es interruptor de invocación (`emit_kaizen_alert && needs_kaizen`). El bug es **no usar `store_mode()` al persistir** el ECST Kaizen.

## 2. Objetivo

Unificar `write_pending_kaizen` con el mismo contrato que `actions::write_pending_event` / `fractal::write_fractal_event`:

- **`redb`:** `stamp_published_event` + `record_ecst(repo, "domain", &event)`; **sin** fichero bajo `.events/pending/`.
- **`dual`:** store + JSON en `paths.pending` (espejo transitorio).
- **`filesystem`:** solo JSON en `paths.pending` (comportamiento actual).

Conservar:

- Predicado `needs_kaizen_actionable` (sin cambios).
- Flag `emit_kaizen_alert` (sin duplicar flags; laudo L-EMIT-FLAG).
- Forma del JSON de salida (`kaizen_event_id`, `kaizen_target_path` lógico).

## 3. Fuera de alcance

- Cambiar semántica de `needs_kaizen` o umbrales stale.
- Purga de `.events/` histórico.
- Refactor de `materialize-kaizen-alert-doc` (sigue suscrito vía store + route).
- Mutar genoma vía IDE: implementación en `SddIA/tools/event-bus-audit/` vía **`entity-manager`** si el diff toca ficha `event-bus-audit.md` / versión tool.

## 4. Diseño técnico

### 4.1 Refactor de `write_pending_kaizen` (`main.rs`)

Pseudoflujo:

```text
1. Construir event JSON (igual que hoy + alert_kind: event-bus-audit ya en payload).
2. sddia_eda_catalog::stamp_published_event(repo, &mut event).
3. mode = store_mode()
4. Si writes_store(mode): record_ecst(repo, "domain", &event)?
5. Si writes_filesystem(mode): fs write paths.pending/{event_id}.json
6. Retornar seal:
   - redb: target_path sintético ".events/pending/{event_id}.json" (paridad actions)
   - dual/filesystem: path real relativo al repo
```

Dependencias crate (ya presentes vía workspace): `sddia_event_store::{record_ecst, store_mode, writes_store, writes_filesystem}`, `sddia_eda_catalog::stamp_published_event`.

Errores: propagar `StoreError` como string en envelope de error del tool (patrón existente).

### 4.2 Invariantes

| Modo | Fichero `.events/pending/{id}.json` | Fila en `events` + `subscriber_delivery` |
|------|-------------------------------------|--------------------------------------------|
| redb | **No** | **Sí** (Pending → fan-out Cúmulo) |
| dual | Sí | Sí |
| filesystem | Sí | No |

### 4.3 Compatibilidad EDA

- `Kaizen_Alert_Required` es **domain** (`kaizen-alert-required.md`); suscriptor Cúmulo en `event-subscriptions.json`.
- Tras parche, `event-watcher` reclama por `list_pending_deliveries` y lanza route con path lógico (sin residual).

## 5. Tareas (orden)

| ID | Tarea | Entregable |
|----|--------|------------|
| T1 | Extraer lógica de persistencia a función testeable `emit_kaizen_domain_event(repo, paths, event) -> Result<Value, String>` | `main.rs` |
| T2 | Implementar ramas redb / dual / filesystem según §4 | Código |
| T3 | Tests unitarios en `event-bus-audit` (temp repo + `override_store_mode`) | `#[cfg(test)]` en `main.rs` o `tests/` |
| T4 | `cargo test -p event-bus-audit` + `cargo check -p event-bus-audit` | Verde |
| T5 | Actualizar `SddIA/tools/event-bus-audit.md` (v1.2.1): emisión vía gate | Genoma tool vía **entity-manager** |
| T6 | Actualizar `SddIA/process/event-bus-audit.md`: «pending» → store en redb | Genoma process vía entity-manager |
| T7 | `spec.md` + `validacion.md` en este `persist_ref`; `global: APTO` en rama PR | Documentación fix |
| T8 | Registro `SddIA/evolution/{uuid}.md` + fila Evolution_log | Evolución |

## 6. Criterios de aceptación (tests)

| ID | Criterio |
|----|----------|
| AC-1 | `StoreMode::Redb`: tras `write_pending_kaizen`, **no** existe `.events/pending/{id}.json`; `get_event(id)` en store devuelve ECST con `event_type=Kaizen_Alert_Required`. |
| AC-2 | `StoreMode::Filesystem`: existe JSON pending; store no requerido. |
| AC-3 | `StoreMode::Dual`: existen **ambos** (store + fichero). |
| AC-4 | Con `emit_kaizen_alert: false`, **ningún** side effect de emisión (sin regresión). |
| AC-5 | `needs_kaizen_actionable` tests existentes siguen verdes. |
| AC-6 | Manual (instancia redb): ejecutar audit con `needs_kaizen` forzado o fixture; log watcher **sin** `F-EDA-RESIDUAL-FILESYSTEM` para ese `event_id`. |

## 7. Verificación manual sugerida

```bash
# Solo informe (sin Kaizen)
echo '{"emit_kaizen_alert":false}' | SddIA/target/release/event-bus-audit

# Con emisión (lab: bajar umbral o fixture con circuit_alert en repo test)
echo '{"emit_kaizen_alert":true}' | SddIA/target/release/event-bus-audit
# Comprobar: sin nuevo JSON en .events/pending/; fila pending en store
```

Recompilar cápsula tras parche; supervisor `start-sddia.sh` o `_run_daemon.sh event-watcher`.

## 8. Riesgos y mitigaciones

| Riesgo | Mitigación |
|--------|------------|
| `stamp_published_event` falla (catálogo) | Propagar error; no escribir FS parcial en redb |
| Lock `events.redb` | Mismo retry que resto del bus (`OPEN_BUDGET_MS`); audit es batch corto |
| Tests flake por temp dir | Patrón `store_audit_counts` (temp + `override_store_mode` + cleanup) |
| ELF fósil en launcher | `cargo build -p event-bus-audit --release` en `SddIA/target/` (no sandbox target) |

## 9. Definición de Done (rama kaizen)

- Un PR desde `Kaicen_Auditoria_Eventos_261030` (o rama `fix/event-bus-audit-kaizen-redb-emit` rebased).
- Código + tests + docs fix + validacion APTO en el mismo PR.
- Sin segundo PR solo documental.

## 10. Estimación

- Implementación + tests: **~1–2 h**.
- Documentación + entity-manager + evolution: **~30 min**.
