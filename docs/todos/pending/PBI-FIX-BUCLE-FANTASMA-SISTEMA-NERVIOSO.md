---
document_id: PBI-FIX-BUCLE-FANTASMA-SISTEMA-NERVIOSO
created: "2026-05-29"
process: bug-fix
index_prefix: "[ARQUITECTURA]"
branch_name: fix/bucle-fantasma-sistema-nervioso
persist_ref: docs/fixes/fix-bucle-fantasma-sistema-nervioso
status: implementado
implementation: completado
---

# [ARQUITECTURA] PBI — Bucle fantasma Sistema Nervioso (Windows E/S)

## Resumen

Corregir el **bucle fantasma** del `event-watcher` cuando instancias ECST permanecen en colas fractal (`.events/domain|telemetry|orchestration/`) tras enrutamiento exitoso, por latencia/bloqueo de E/S en Windows.

## Artefactos de diseño (Tekton)

| Artefacto | Ruta |
|-----------|------|
| Clarificación | [clarify.md](../../fixes/fix-bucle-fantasma-sistema-nervioso/clarify.md) |
| Objetivos | [objectives.md](../../fixes/fix-bucle-fantasma-sistema-nervioso/objectives.md) |
| Especificación | [spec.md](../../fixes/fix-bucle-fantasma-sistema-nervioso/spec.md) |
| Plan | [plan.md](../../fixes/fix-bucle-fantasma-sistema-nervioso/plan.md) |
| Implementación | [implementation.md](../../fixes/fix-bucle-fantasma-sistema-nervioso/implementation.md) (stub) |
| Ejecución | [execution.md](../../fixes/fix-bucle-fantasma-sistema-nervioso/execution.md) (stub) |
| Validación | [validacion.md](../../fixes/fix-bucle-fantasma-sistema-nervioso/validacion.md) (PENDIENTE) |

## Plan de ataque (3 fases)

### Fase 1: Idempotencia en caliente (memoria volátil)

El daemon `event-watcher.py` ya no confiará ciegamente en la desaparición física del archivo en el disco. Se implementará un registro en memoria (`processing_uuids = set()`). En el momento en que un JSON sea detectado, su UUID se añade al set. Si el archivo sigue presente en la carpeta en las siguientes iteraciones debido a un bloqueo de E/S, el watcher lo ignorará si su UUID ya está registrado. La liberación del UUID solo ocurrirá cuando el subproceso del orquestador retorne su código de salida oficial.

**Complemento documentado (D3):** set `routed_ok_pending_absent` para no re-despachar tras route exit 0 mientras el archivo persista. Ver `spec.md` §1.

### Fase 2: Absorción de latencia (micro-sleeps)

La rutina física de archivado en `eda_bus_utils.py` dejará de ser un intento único. Se envolverá en reintentos pasivos (hasta 3 intentos, 50 ms) antes de declarar fallo de purga. Ver `safe_remove_path` en `spec.md` §2.

### Fase 3: Purga de la zona cero

Herramienta QA `purge_stale_events.py` para triaje en laboratorio de JSON estancados en colas activas. Default `--dry-run`. Ver `spec.md` §3.

## Protocolo de indexación

Commits y PR con prefijo **`[ARQUITECTURA]`** — alteración de la fisiología central del bus EDA y garantías transaccionales.

## Siguiente paso

- PR `[ARQUITECTURA] fix: bucle fantasma bus EDA` con rama `fix/bucle-fantasma-sistema-nervioso`
- Pre-merge: `validacion.md` APTO, PBI → `docs/todos/done/`
- Lab: `python SddIA/scripts/qa/purge_stale_events.py --dry-run` y, si procede, `--apply`
