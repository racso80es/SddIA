---
feature_name: kaizen-tqm-single-flight-pbi
created: "2026-08-28"
process: bug-fix
branch_name: fix/kaizen-tqm-single-flight-pbi
persist_ref: docs/fixes/kaizen-tqm-single-flight-pbi
pbi_ref: docs/todos/pending/[KAIZEN] TQM sin single-flight por PBI — cadenas bug-fix duplicadas y agentes en carrera.md
document_id: PBI-KAIZEN-TQM-SINGLE-FLIGHT-PBI
uuid: a559c127-fd21-4fec-954e-116840a9a875
scope: tqm-single-flight-pbi
base: main
execution_id: "25e1072f-3ba1-4b64-8e24-b9513ab702e3"
---

# Spec — TQM single-flight por PBI

## Problema

Dos eventos de dominio con `correlation_id` distintos sobre el mismo `pbi_ref` producen dos cadenas `task-queue-manager` → `bug-fix` → `cursor-agent` concurrentes. Incidente verificado 2026-08-28 07:31 (PIDs 752860/753456) sobre `docs/todos/pending/[FIX] x.md`.

## Causa raíz (D1) y latentes (D2–D3)

| ID | Hecho | Trabajo |
|----|-------|---------|
| D1 | Clave del lock = `{correlation_id}.lock` | Cambiar clave a identidad de PBI |
| D1b | Guard solo si hay `correlation_id` | Adquirir siempre que `pbi_ref` resuelva |
| D2 | `lock_pid_alive` + PID en fichero **ya existen** | No reimplementar; endurecer |
| D2a | `create_new` + `writeln` deja ventana de fichero vacío; parse fallo ⇒ purga de vivo | Fail-closed: ilegible ≠ muerto |
| D2b | `/proc/{pid}` no prueba identidad (PID recycle) | Sellar `starttime` (o equivalente) |
| D2c | Sin `/proc`, liveness = `false` silencioso | Rama por plataforma + registro explícito |
| D3 | Hit solo en envelope efímero | Proof durable + evento `orchestration` |
| R1 | `Drop` libera al retornar; `cli_detach` evaporaría el cerrojo | Invariante + fail ruidoso si hijo detached |

`fs::canonicalize` queda descartado (host-absoluto, falla si el path no existe, rompe worktrees).

## Solución

### L1 — Clave de exclusión = identidad de PBI

Módulo: `task_queue_manager.rs` (sin quinta copia de `normalize_rel`; extraer helper local o reutilizar el de `eda_bus.rs` del mismo crate).

**Resolución de clave (orden):**

1. Rechazar `pbi_ref` con `..`.
2. Si `load_pbi_body` + `extract_fm_string(..., "document_id")` (o `uuid` si no hay `document_id`) → `lock_id = "id:{document_id}"` (TQM-CA3: sobrevive `pending/` → `done/`).
3. Si no hay cuerpo/frontmatter → `lock_id = "path:{sha256(normalize_rel(pbi_ref))}"` con `normalize_rel` = trim + `\`→`/` + strip `./`.
4. Nombre de fichero: `hex(sha256(lock_id)).lock` bajo `.SddIA/daemons/state/tqm-single-flight` (ruta ya usada; no inventar topología).
5. Sin `pbi_ref` resoluble: **no** caer al `correlation_id` como clave de exclusión de PBI (eso reabre D1). Despacho sin PBI permanece fuera de este guard (fuera de alcance del incidente).

Adquisición **antes** de `build_child_inputs` / `invoke_process_full`, con o sin `correlation_id` (TQM-CA2).

### L2 — Liveness (endurecer, no reescribir)

Contenido del lock: JSON de una línea, p. ej. `{"pid":N,"starttime":"…"}`, escrito con `write_all` + `sync_all` inmediatamente tras `create_new`.

| Caso | Decisión |
|------|----------|
| Lock existe, JSON parseable, PID+starttime vivos | Hit (ocupado) |
| Lock existe, JSON parseable, proceso muerto o starttime no coincide | Purgar y reintentar (≤3) |
| Lock existe, vacío / no parseable, mtime reciente | **Ocupado** (TQM-CA4) |
| Lock existe, vacío / no parseable, mtime > gracia (2s) | Tratar como crash-en-adquisición: purgar |
| Plataforma Unix sin `/proc` | `kill(pid, 0)` vía `libc` (ya dependencia) + log stderr `[TQM-SF-LIVENESS] backend=kill0` |
| Plataforma sin señal equivalente | **No** adquirir en silencio: `Err` explícito en `try_acquire` (TQM-CA6) |

`starttime`: en Linux, campo 22 de `/proc/{pid}/stat`. Fuera de Linux, el backend `kill0` no cubre D2b; documentar residual en `execution.md` (falso positivo de recycle es raro en ventana de ciclo).

### L3 — Descarte auditable (TQM-CA7/CA8)

**No** reutilizar `Process_Execution_Completed` como vehículo del hit: suscriptores actuales (`persist-pec-correlation-proof`, `send-telegram-notification`) interpretan PEC como cierre de ciclo. Un hit no es un ciclo completado.

**Durable (síncrono, antes del return):** escribir proof bajo `eda_instance.proofs` (`cumulo.paths.json` → `.SddIA/proofs`) en `tqm-single-flight/{lock_hex}.json` (último hit gana el fichero; el JSON incluye histórico acotado o al menos el hit actual + `holder_correlation_id`). Sobrevive al `purge_after` de `route_orchestration_event`.

**Bus (familia `orchestration`):** emitir instancia ECST con `event_family: orchestration`, `emitter_agent: task-queue-manager`, payload:

- `pbi_ref` normalizado
- `lock_key`
- `holder_correlation_id` (vigente; `null` si el holder no tenía cid)
- `discarded_correlation_id`
- `reason: single_flight_pbi`

Clase nueva `{name}.md` vía `entity-manager` (genoma `directories.events/orchestration/`). **Prohibido** forja manual del `.md`. Si el ciclo de ejecución no puede forjar la clase en el mismo PR, el proof síncrono sigue siendo CA8; la emisión se aplaza con residual explícito — CA7 no se marca cumplido hasta que exista clase catalogada.

Envelope de hit: `success: true`, `single_flight_hit: true`, `reason` explícito, ambos correlation ids. No invocar hijo.

### L4 — Alcance temporal (TQM-CA11)

Hoy `DISPATCHABLE` (`bug-fix`/`feature`/`refactorization`) no está en `DEFAULT_ALLOWLIST` de `cli_detach`. Test de invariante: intersección vacía. Si `invoke_process_full` devolviera `detached: true`, TQM **falla ruidoso** (no Drop silencioso del lock mientras el agente vive). No implementar lease-hasta-PEC en este fix.

## Fuera de alcance

- Unificar las cuatro copias de `normalize_rel` entre crates.
- Extraer `document_id` si el PBI aún no existe en disco (fallback path-hash).
- Sustituir `persist-execution-id-conflict` de `agent_runtime` (segunda barrera).
- Single-flight entre worktrees distintos (locks son por instancia `.SddIA/`).

## Criterios (mapeo)

| CA | Capa |
|----|------|
| TQM-CA1, CA2, CA3, CA9 | L1 |
| TQM-CA4, CA5, CA6, CA10 | L2 |
| TQM-CA7, CA8 | L3 |
| TQM-CA11 | L4 |
| TQM-CA12 | Smoke post-implementación (dos domain events, un `cursor-agent`) |
