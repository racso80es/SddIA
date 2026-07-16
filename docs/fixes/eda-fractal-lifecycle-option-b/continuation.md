---
feature_name: eda-fractal-lifecycle-option-b
created: "2026-07-16"
process: bug-fix
status: residual-open
handoff: continue
---

# Continuación — residual empírico (2026-07-16)

## Estado del merge

| Campo | Valor |
|-------|--------|
| PR | https://github.com/racso80es/SddIA/pull/113 |
| Merge | `6b3fe69` en `main` |
| PBI opción B | `docs/todos/done/[FIX] EDA fractal — lifecycle opción B.md` |
| Persist | `docs/fixes/eda-fractal-lifecycle-option-b/` |

## Qué ya está en código (opción B)

1. `route_domain_fractal_event(..., purge_after=true)` — purga **solo si todos** los suscriptores quedan terminales OK.
2. Stamp `delivery_state[subscriber_id]` + `skipped-already-delivered`.
3. Sweeper fractal: unlink **solo** si todos los stamps son `success|skipped*` y **ninguno** es `failed`.
4. Telegram ACK-first + seen; watcher anti-eco.

## Hallazgo empírico post-prueba (repo local)

Snapshot 2026-07-16 ~18:04:

| Métrica | Valor |
|---------|--------|
| `.events/domain/*.json` | **145** |
| Con `delivery_state` no vacío | **145** |
| Fully terminal (purgables) | **0** |
| Con al menos un `failed` | **145** |

Suscriptores `failed` dominantes:

| Subscriber | Count |
|------------|-------|
| `cumulo.iota-immutable-publisher` | 113 |
| `radamanto.iota-immutable-publisher` | 32 |
| `tekton.task-queue-manager` | 15 |

Por `event_type` (top):

| event_type | Count |
|------------|-------|
| Kaizen_Idea_Captured | 43 |
| TelegramMessage_Received | 28 |
| Manual_Task_Requested | 27 |
| System_Immunity_Certified | 25 |
| Kalma2_Process_Requested | 15 |

Muestra Telegram (anti-eco OK, archivo permanece por IOTA):

```json
"delivery_state": {
  "cumulo.iota-immutable-publisher": "failed",
  "mayeuta.telegram-fallback-responder": "skipped-already-delivered"
}
```

## Diagnóstico estructural (no conjetura)

Los eventos **siguen en `.events/domain/` porque la opción B cumplió su contrato**:

- `purge_after` exige `all_ok`.
- Con stamp `failed` (IOTA / task-queue) → `all_ok=false` → **no unlink**.
- Sweeper fractal exige cero `failed` → **no unlink**.

La validación lab APTO usó `SDDIA_LAB_SIMULATE_IOTA=1` (happy path). El backlog real tiene IOTA en `failed` → comportamiento esperado bajo la política actual, **no** regresión de stamp/anti-eco.

Binarios locales al muestreo:

- `SddIA/target/debug/execute-process` → contiene `skipped-already-delivered` (opción B).
- `SddIA/target/release/execute-process` → **sin** opción B (no rebuild release).

## Cómo retomar

### 0. Prerrequisito runtime

```bash
cd /home/racso/Proyectos/SddIA
git checkout main && git pull
cd SddIA && cargo build -p execute-process -p event-watcher -p event-sweeper -p telegram-watcher
# Opcional alineado a SSOT release:
# cargo build --release -p execute-process -p event-watcher -p event-sweeper -p telegram-watcher
# Reiniciar centinelas (start-sddia.sh o governance-daemon-manager)
```

### 1. Verificar política vs síntoma

```bash
# ¿Hay algún domain purgable YA?
python3 - <<'PY'
import json, pathlib
root=pathlib.Path('.events/domain')
term=fail=0
for p in root.glob('*.json'):
    ds=json.loads(p.read_text()).get('delivery_state') or {}
    vals=list(ds.values())
    if not vals: continue
    if any(v=='failed' for v in vals): fail+=1
    elif all(v=='success' or str(v).startswith('skipped') for v in vals): term+=1
print({'terminal':term,'failed_partial':fail,'total':len(list(root.glob('*.json')))})
PY
```

Si `terminal=0` y `failed_partial≈total` → el síntoma es **política de purga**, no “opción B no arrancó”.

### 2. Decisiones (laudo)

| Opción | Efecto | Estado |
|--------|--------|--------|
| **C1** | Remediación IOTA → re-route → purge natural | abierto |
| **C2** | Terminal-with-failure → `eda_fractal.dead_letter` (`./.events/dead-letter`) | **elegido 2026-07-16** → `docs/fixes/eda-fractal-dlq-c2/` |
| **C3** | Purga one-shot backlog histórico | abierto / parcialmente absorbido por C2 en runtime |

### 3. PBI residual

`docs/todos/pending/[FIX] EDA domain — residual IOTA failed bloquea purga.md`  
Persist: `docs/fixes/eda-fractal-dlq-c2/` · rama `fix/eda-fractal-dlq-c2`

### 4. Archivos clave

| Path | Rol |
|------|-----|
| `SddIA/engine/execute-process/src/engine/route_fractal_core.rs` | purge_after + stamp |
| `SddIA/sddia-daemon-runtime/src/eda_sweep.rs` | sweep fractal |
| `SddIA/daemons/telegram-watcher/src/main.rs` | ACK/seen |
| `SddIA/daemons/event-watcher/src/main.rs` | anti-eco |
| `SddIA/core/event-domain-subscriptions.json` | suscriptores IOTA en casi todos los domain |

## Nota de honestidad documental

`validacion.md` APTO de opción B = **happy-path lab**. Residual empírico de producción/local con IOTA `failed` documentado aquí; no invalida anti-eco/stamp, sí limita la expectativa “domain vacío tras merge #113”.
