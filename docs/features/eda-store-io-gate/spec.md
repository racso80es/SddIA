---
feature_name: eda-store-io-gate
created: "2026-10-30"
base: SddIA/sddia-event-store/src/gate.rs
scope: "Lectura y escritura de ECST condicionadas a StoreMode"
---

# Spec — Fachada de I/O y residuos

## Contrato único

| Modo | Escritura padre | Lectura padre | Testigos suscriptor |
|------|-----------------|---------------|---------------------|
| `redb` | `record_ecst` | `load_dispatched_event` / `list_*` | `subscriber_delivery` |
| `filesystem` | JSON bajo ruta de `cumulo` (`eda_bus` / `eda_fractal`) | `read` del JSON | testigos FS |
| `dual` | store **y** JSON | store si la clave existe; si no, JSON | ambos |

Ruta lógica `.events/pending/{event_id}.json` puede devolverse como `target_path` en `redb` **sin** crear el fichero (paridad `actions::write_pending_event`).

Prohibido en código de producción: `fs::write` / `read_dir` sobre `.events/**` de un ECST sin rama `writes_filesystem` (escritura) o sin fallback explícito tras store (lectura). Excepción documentada: detección de residual (`F-EDA-RESIDUAL-FILESYSTEM`) que **no** reinyecta el JSON al router.

## Fachada (crate `sddia-event-store` o módulo `eda_io` en `execute-process`)

Funciones mínimas (nombres orientativos):

1. `publish_ecst(repo, family, event) -> Seal` — stamp dominio si aplica, `record_ecst` si `writes_store`, JSON si `writes_filesystem`.
2. `load_ecst(repo, event_id, logical_path) -> Value` — delega en `load_dispatched_event` (ya existe; los call sites fractales deben usarla).
3. `find_domain_by_payload(repo, pred)` — store si `writes_store`; si `writes_filesystem` (o `dual`), también barrido FS.

Los call sites dejan de duplicar el `if writes_*`.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-W1 | `daemon-heartbeat-audit` en `redb` emite `System_Fracture_Detected` solo en store; cero `*.json` nuevos en `.events/pending/`. |
| AC-W2 | `kalma2-bridge` `emit_system_fracture` igual que AC-W1. |
| AC-W3 | `capability_di_reactor::emit_di_requested` y `lancedb_thought_repo` Thought_Persisted respetan el gate. |
| AC-W4 | `sddia-installer.sh` no escribe pending si el entorno efectivo es `redb` (delega en binario que llama `publish_ecst`, o no-op documentado + evento vía CLI). |
| AC-R1 | `find_existing_domain_event` encuentra el ECST en store con `redb` y directorio pending vacío. |
| AC-R2 | `tracker_ref_from_presented_for_branch` / `scan_presented_for_branch` no dependen de JSON cuando el Presented está solo en store (el scan FS queda detrás de `writes_filesystem` o como residual explícito). |
| AC-F1 | Con `redb`, un `Daemon_Heartbeat` o evento fractal insertado solo en store es reclamado y enrutado sin fichero bajo `.events/telemetry|domain|orchestration`. |
| AC-F2 | `route_fractal_event` carga el ECST vía `load_ecst` si el path no es fichero y `writes_store`. |
| AC-N1 | `filesystem`: suites que cuentan JSON en `.events/` siguen verdes (`SDDIA_EVENT_STORE=filesystem`). |
| AC-N2 | `dual`: store y JSON coexisten; sin regresión de `F-EDA-RESIDUAL` (residual solo en `redb` puro). |

## No regresión

- Side-channel heartbeat no genera fila `source=side-channel` (test `heartbeat_sidechannel_does_not_become_an_event_row`).
- `event-bus-audit` censo sigue en store si `writes_store`.
- Watcher en `redb` sigue fracturando JSON **nuevo** en pending (`F-EDA-RESIDUAL-FILESYSTEM`) en lugar de enrutarlo.
