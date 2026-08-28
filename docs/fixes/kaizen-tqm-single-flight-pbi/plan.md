---
feature_name: kaizen-tqm-single-flight-pbi
created: "2026-08-28"
process: bug-fix
branch_name: fix/kaizen-tqm-single-flight-pbi
persist_ref: docs/fixes/kaizen-tqm-single-flight-pbi
pbi_ref: docs/todos/pending/[KAIZEN] TQM sin single-flight por PBI — cadenas bug-fix duplicadas y agentes en carrera.md
document_id: PBI-KAIZEN-TQM-SINGLE-FLIGHT-PBI
uuid: b9adc2b5-f759-4f17-ba9a-4cf01973ebb2
phases:
  - l1-lock-key-pbi
  - l2-liveness-hardening
  - l3-discard-proof-event
  - l4-detach-invariant
  - tests-unit
  - smoke-ca12
---

# Plan — TQM single-flight por PBI

Orden: L1 → L2 → L4 (invariante barata) → tests unitarios de lock → L3 (proof + clase ECST) → smoke CA12. Cierre documental y `delivery-close-cycle` **después** de ejecución/Argos; este commit solo sella Diseño.

## Fase L1 — Clave por PBI

Archivo único de lógica: `SddIA/engine/execute-process/src/engine/handlers/task_queue_manager.rs`.

1. Extraer `normalize_rel` del crate (`eda_bus.rs` ya lo tiene) o helper privado en el handler; no copiar una quinta variante.
2. `lock_identity(repo, pbi_ref) -> Option<String>`: `document_id`/`uuid` FM o `path:{sha256}`.
3. `try_acquire_single_flight(repo, lock_identity)` — dejar de usar `correlation_id` como nombre de fichero.
4. `dispatch_child`: adquirir si hay `pbi_ref`; envelope de hit con `pbi_ref` + cids.

## Fase L2 — Liveness

1. Payload JSON `{pid, starttime?}` + `sync_all`.
2. `lock_pid_alive` → `lock_holder_alive`: parse JSON; ilegible reciente = vivo; ilegible viejo = stale.
3. Linux: `/proc/{pid}` + `starttime`; resto Unix: `libc::kill(pid, 0)`; otro OS: `Err`.

## Fase L4 — Invariante detach

Test en `task_queue_manager` o `cli_detach`: `DISPATCHABLE ∩ DEFAULT_ALLOWLIST == ∅`. Si el envelope hijo trae `detached: true`, error explícito (no Drop del guard como éxito).

## Fase tests unitarios (antes de L3)

- CA9: dos acquires, mismo `pbi_ref`, cids distintos → segundo `None`.
- CA3: `./docs/todos/pending/X.md` vs `docs/todos/pending/X.md` → misma clave; `document_id` igual con path `done/`.
- CA10: lock vacío reciente no se purga.
- CA2: acquire sin `correlation_id` con `pbi_ref`.
- CA4/CA5: mock de contenido JSON con pid muerto vs starttime mismatch.

Comando: `cd SddIA && cargo test -p execute-process task_queue_manager`.

## Fase L3 — Proof + evento

1. Proof síncrono: `resolve_eda_proofs_dir` (ya en `persist_pec_correlation_proof.rs`) + namespace `tqm-single-flight`.
2. Clase ECST: `./sddia-run.sh --process entity-manager` (no Write sobre `SddIA/events/`). Catalogar en `orchestration/index.md` vía forja.
3. Emitir con `write_fractal_event` + `load_fractal_dirs` (mismo patrón que `thermodynamic::emit_initialized_pec`).
4. No añadir suscriptor Telegram. Subscriptions: vacías o solo persist no-op; CA8 no depende del bus.

## Fase smoke CA12

Tras binario recompilado: dos `route-domain` (o dos TQM) sobre el mismo PBI de lab; un solo proceso `cursor-agent`. No en este commit.

## Cierre (fuera de esta parada)

`implementation.md` + `execution.md` → Argos `validacion.md` → PBI a `done/` → `delivery-close-cycle`.
