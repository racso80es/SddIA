---
feature_name: fix-bucle-fantasma-sistema-nervioso
created: "2026-05-29"
process: bug-fix
base: main
scope: eda-bus-windows-io-resilience
version_spec: "1.0.0"
index_prefix: "[ARQUITECTURA]"
---

# Especificación — [ARQUITECTURA] Bucle fantasma y resiliencia E/S del bus

## Hito 1 — Idempotencia en caliente (`event-watcher.py`)

### 1.1 Estado en memoria

| Símbolo | Tipo | Semántica |
|---------|------|-----------|
| `processing_uuids` | `set[str]` | UUID (`path.stem`) en vuelo hacia `execute-process` |
| `routed_ok_pending_absent` | `set[str]` | *(D3)* UUID ya enrutado con exit 0; archivo aún en disco — no re-despachar |

> **Migración:** `in_flight` por `domain/file.json` se sustituye o complementa por sets basados en **UUID**, independientes de la carpeta fractal.

### 1.2 Algoritmo de detección (por iteración del poll)

```text
PARA cada (watch_dir, process_name) en _watch_targets:
  PARA cada path en sorted(watch_dir.glob("*.json")):
    event_uuid ← path.stem
    SI event_uuid ∈ processing_uuids: CONTINUE  # skip idempotente
    SI event_uuid ∈ routed_ok_pending_absent Y path.is_file(): CONTINUE  # D3
    SI route-domain-event Y testigos dead-letter: CONTINUE  # preservar Kaizen
    SI attempts[key] >= MAX_ROUTE_ATTEMPTS: CONTINUE
    processing_uuids.add(event_uuid)
    proc ← _invoke_route_process(...)
    processing_uuids.discard(event_uuid)  # solo tras retorno subprocess
    SI proc.returncode == 0 Y path.is_file():
      routed_ok_pending_absent.add(event_uuid)
    SI proc.returncode == 0 Y NOT path.is_file():
      routed_ok_pending_absent.discard(event_uuid)
      attempts.pop(key, None)
    _log_route_outcome(...)
```

### 1.3 Logs obligatorios

| Condición | Mensaje (prefijo `[WATCHER]`) |
|-----------|-------------------------------|
| Skip `processing_uuids` | `skip in-flight uuid={event_uuid}` |
| Skip D3 | `skip routed-ok pending file uuid={event_uuid}` |
| Nuevo despacho | `Detectado nuevo evento: {key} → {process_name}` (existente) |

### 1.4 Constantes (sin cambiar contrato externo)

| Constante | Valor actual | Notas |
|-----------|--------------|-------|
| `POLL_SECONDS` | 2 | Sin cambio salvo perf lab documentado |
| `MAX_ROUTE_ATTEMPTS` | 3 | Aplica a fallos de route; coordinar con D3 |

---

## Hito 2 — Absorción de latencia (`eda_bus_utils.py`)

### 2.1 `safe_remove_path(path: Path, *, retries: int = 3, delay_s: float = 0.05) -> bool`

| Paso | Acción |
|------|--------|
| 1 | Si `not path.is_file()` → `True` |
| 2 | Bucle `retries`: `path.unlink()` |
| 3 | Capturar `PermissionError`, `OSError` (incl. WinError 32) |
| 4 | `time.sleep(delay_s)` entre intentos |
| 5 | Retornar `False` si persiste tras agotar reintentos |

**Prohibido** usar `missing_ok=True` en call sites que deban reportar fallo de purga al caller.

### 2.2 Call sites (mínimo)

| Archivo | Función / línea lógica |
|---------|------------------------|
| `route_fractal_event_core.py` | Purga post-suscriptores (`purge_after`, telemetry terminal) |
| `eda_bus_utils.py` | `maybe_purge_fractal_telemetry_when_terminal`, `archive_event_after_sweep` (unlink padre), otros `unlink` de cabeceras si aplica |

### 2.3 Contrato de retorno en route fractal

| Campo `data` | Regla |
|--------------|-------|
| `purged` | `True` solo si `safe_remove_path` retornó `True` |
| `purge_failed` | *(nuevo opcional)* `True` si consenso OK pero archivo permanece |

---

## Hito 3 — Purga zona cero (`purge_stale_events.py`)

### 3.1 Ubicación

`SddIA/scripts/qa/purge_stale_events.py`

### 3.2 CLI

```text
purge_stale_events.py [--dry-run|--apply] [--json] [--max-age-hours N]
```

| Flag | Default | Efecto |
|------|---------|--------|
| `--dry-run` | **sí** (implícito si no `--apply`) | Solo reporta candidatos |
| `--apply` | no | Ejecuta purga según política |
| `--json` | no | Salida machine-readable |
| `--max-age-hours` | opcional | Candidato por antigüedad de `timestamp` |

### 3.3 Criterios de candidato (OR)

1. **Consenso fractal:** `delivery_state` completo con todos los suscriptores requeridos en estado terminal OK (`delivery_stamp_terminal_ok`).
2. **Huérfano con testigos:** testigos en `processed/subscribers/` para UUID pero padre aún en cola activa fractal.
3. **Antigüedad:** `timestamp` > `--max-age-hours` y sin suscriptores in-flight en `processing/subscribers/`.

### 3.4 Exclusiones

| Ruta | Regla |
|------|-------|
| `dead-letter/` | No tocar sin `--include-dead-letter` (fuera de v1.0) |
| `pending/` V3+ con DL Kaizen | Excluir si testigo `dead-letter/subscribers/` |

### 3.5 Salida `--json` (esquema)

```json
{
  "dry_run": true,
  "scanned": 46,
  "candidates": [{"event_id": "…", "queue": "domain", "reason": "delivery_complete"}],
  "purged": 0
}
```

---

## Hito 4 — Pruebas

| ID | Tipo | Descripción |
|----|------|-------------|
| T1 | Unit | Watcher: segundo poll con mismo UUID en `processing_uuids` no invoca subprocess |
| T2 | Unit | Watcher D3: tras mock exit 0 y archivo presente, tercer poll skip |
| T3 | Unit | `safe_remove_path`: fallo 2× + éxito en 3.er intento |
| T4 | Unit | `purge_stale_events --dry-run` sobre fixture con JSON stale |
| T5 | Manual lab | Stress `core-full-stress` + watcher 60s: sin líneas repetidas de route para mismo UUID |

Ubicación sugerida: `SddIA/scripts/qa/test_bucle_fantasma_bus.py`

---

## Hito 5 — Documentación post-código

| Artefacto | Cambio |
|-----------|--------|
| `README.md` | Nota breve: resiliencia Windows + `purge_stale_events` (solo lab) |
| `events-contract.md` | § runtime: purga fractal con reintentos; idempotencia watcher |
| `implementation.md` / `execution.md` | Rellenar tras Tekton |
| `validacion.md` | `global: APTO` + checks CA* |

---

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| **CA1** | Watcher en bucle: mismo UUID no genera >1 `execute-process` concurrente |
| **CA2** | Tras route exit 0 y archivo persistente (simulado): watcher no re-despacha (D3) |
| **CA3** | `safe_remove_path` supera fixture con `PermissionError` intermitente |
| **CA4** | `purge_stale_events --dry-run` detecta instancias del incidente 2026-05-29 sin borrar |
| **CA5** | `event-sweeper --once` y `try_sweep_event` sin regresión (smoke existente) |
| **CA6** | Sin cambio en semántica Kaizen dead-letter en `route-domain-event` |
