---
feature_name: arch-immunological-system
created: "2026-08-29"
process: feature
base: main
scope: arch-immunological-system
version_spec: "1.0.0"
document_id: PBI-ARCH-IMMUNOLOGICAL-SYSTEM
uuid: "056ac6a1-02fc-4988-a704-1f5b648d0e40"
---

# Especificación — arch-immunological-system

## 1. Contratos vigentes (no reinventar)

| Pieza | Hecho |
|-------|--------|
| `SddIA/process/daemon-heartbeat-audit.md` v1.0.1 | `missed_cycles = floor((now − baseline) / interval)`; fractura si `>= 3` |
| `daemon_heartbeat.rs` | `MISSED_CYCLES_THRESHOLD` **cableado** `3`; `effective_heartbeat_baseline`; ingest A+C; emit a `eda_bus.pending` |
| `System_Fracture_Detected` | Fan-out Cúmulo `materialize-fracture-pbi` → Mayeuta enrich |
| Dedup materialize | Un PBI abierto por `process_name` (`fracture_pbi.rs`) |
| Auditoría 2026-08-26 | 5 PBIs con 237–1532 ciclos; runtime sano al auditar |

## 2. SSOT de umbrales

**No** reutilizar `radamanto.thresholds`.

| Capa | Ruta |
|------|------|
| Default Core | `SddIA/daemons/heartbeat-audit.thresholds.json` |
| Overlay instancia | `{daemons_instance.state}/heartbeat-audit.thresholds.json` (gana overlay si existe y parsea) |
| Cúmulo | clave nueva `argos.heartbeat_audit_thresholds` → default Core (lectura; no inferir path) |

Esquema (nombres estables):

```json
{
  "missed_cycles_threshold": 3,
  "suspend_skew_seconds": 120
}
```

- `missed_cycles_threshold`: reemplaza el `const` (misma semántica).
- `suspend_skew_seconds`: umbral de `max(0, Δwall − Δmono)` para clasificar `host_suspend`. 120 s por defecto (cubre NTP menor + resume; no cubre micro-latencia de 3 ciclos).

Prohibido literales de umbral en tests de producto salvo lectura del fixture SSOT o inyección explícita en test.

## 3. Discriminación suspend/crash (handler)

Locus: `SddIA/engine/execute-process/src/engine/handlers/daemon_heartbeat.rs` (fuera de DA-2; mutación directa de motor permitida).

### 3.1 Reloj de auditoría (global al state)

En `heartbeat-audit.json` (raíz, no por daemon):

| Campo | Tipo | Rol |
|-------|------|-----|
| `last_audit_wall_at` | ISO-8601 | Wall clock del sweep anterior |
| `last_audit_mono_ms` | u64 | `Instant` persistido como ms desde un ancla de proceso **o** ms de `CLOCK_MONOTONIC`/`boottime` fail-soft |

Primer sweep de un proceso auditor nuevo: persistir relojes, **no** clasificar suspend (sin Δ).

Sweep N≥2:

```text
wall_delta = now_wall − last_audit_wall_at
mono_delta = now_mono − last_audit_mono_ms   # si mono no comparable (auditor reiniciado): tratar como unknown
skew = wall_delta − mono_delta
host_suspend = mono_comparable AND skew >= suspend_skew_seconds
```

Si el auditor **reinició** (mono no comparable): no afirmar suspend; aplicar solo la regla de missed_cycles vigente (cold-start del auditor ≠ letargo de host). Overlay Linux `/proc` opcional para recuperar comparabilidad; fail-soft.

### 3.2 Efecto de `host_suspend`

Antes de `audit_running_daemon` por cada id:

1. `classification` (por daemon o global del sweep) = `host_suspend`.
2. Reanclar: `last_heartbeat_at = now` (o baseline = now), `missed_cycles = 0`, **no** tocar `fracture_event_id` existente salvo política §3.4.
3. **No** llamar `emit_system_fracture`.
4. Log estructurado en stdout del proceso (JSON ya usado) + campo en `audit_result` (`suspend_reanchored: true`, `skew_seconds`).

PID vivo + lock antiguo + skew alto = letargo físico, no muerte térmica.

### 3.3 Quarantine / micro-latencia

Sin cambios ontológicos: `missed < threshold` → no fractura. No hay cola de eventos en `eda_bus.pending`. **Inmunidad anti-bloqueo:** la discriminación es CPU-local al handler; no `sleep`, no hold de envelope EDA.

### 3.4 HB restablecido

`record_heartbeat_at` sigue poniendo `missed_cycles=0` y quitando `fracture_event_id`. Añadir `classification=healthy` (o `recovered` si había sello).

## 4. Eventos

**No** crear `Anomaly_Detected`. Justificación: el síntoma ya es staleness derivada de `Daemon_Heartbeat` + A+C; el veredicto es estado de auditoría. Emitir un segundo evento por timeout duplicaría fan-out y no aporta suscriptor que no sea el propio auditor.

`System_Fracture_Detected` solo tras: no-`host_suspend` ∧ `missed >= threshold` ∧ lock+PID vivos ∧ sin `fracture_event_id`.

## 5. Fagocitosis

### 5.1 Ledger instancia (siempre)

Archivo: `{daemons_instance.state}/phagocytosed-fractures.json`.

Entrada: `document_id`, `fracture_process`, `trace_last_heartbeat`, `lock_started_at`, `phagocytosed_at`, `reason` (`trace_before_lock` \| `host_suspend_reanchor`).

Escritura atómica (`write_json_atomic`). Fuera de git.

### 5.2 Predicado documental

Resolver `paths.todos.pending`. Candidato si:

- `document_id` / filename encaja `PBI-FIX-FRACTURE-*` (hash 12 hex, con o sin sufijo `-R{n}`).
- `fracture_process` presente (frontmatter) o parseable de título/traza.
- `last_heartbeat` extraído de bloque «Traza de error» (`last_heartbeat=ISO`) **o** campo frontmatter si se añade en materialize (preferir parse de traza existente; **no** exigir re-forja de PBIs viejos).
- Lock vigente del `fracture_process` con PID vivo **y** `started_at > last_heartbeat_traza`.

No candidato: PBI no-fractura; lock ausente/PID muerto (posible incidente real abierto); traza sin timestamp parseable (fail-soft, no archivar).

### 5.3 Proceso `phagocyte-recovered-fracture-pbis`

Forja: `entity-manager` `entity_class: process` (domain root software-engineering o Core según jurisdicción del creator). Handler nativo en `execute-process` (paridad `materialize-fracture-pbi`).

Inputs: `sweep` boolean (default true); `apply` boolean (default false = dry-run lista candidatos).

Outputs: `candidates[]`, `applied[]`, `skipped[]`.

Si `apply=true` (forja/operador o hook post-sweep sano):

1. Mover archivo `paths.todos.pending` → `paths.todos.done` (mismo filename).
2. Frontmatter: `status: cerrado`, `closed: YYYY-MM-DD`, `laudo: B-automatic-phagocyte`, `fix_ref: {paths.fixPath}/centinelas-fracture-ola-{YYYYMMDD}`.
3. Crear/actualizar `docs/fixes/centinelas-fracture-ola-{YYYYMMDD}/` con lista y evidencia de sweep (`missed_cycles` por daemon).
4. Evolution `{uuid}.md` contrato v1.1.2; `relacionado` incluye este `document_id` y el PBI fagocitado. Hash vía `sddia-qa evolution-rehash` / cápsula register (**no** placeholder).

**Prohibido:** `delivery-close-cycle` desde este proceso; push a `main`; mutar genoma de centinelas.

### 5.4 Enganche al audit

Tras `audit_staleness` si `fractures_emitted` vacío **y** todos los daemons indexados con lock vivo tienen `missed_cycles=0`: invocar fagocitosis **ledger** (5.1) siempre; `apply` documental solo si input/env `SDDIA_PHAGOCYTE_APPLY=1` (default off en CI para no mover todos ajenos). En forja del operador: apply explícito.

Garantía AC «ningún apagón deja PBI abierto»: la vía **preventiva** (§3) es el gate duro; la vía **documental** (§5.3) es residual + históricos.

## 6. Update de proceso `daemon-heartbeat-audit`

Vía `entity-manager` update: documentar umbrales SSOT, suspend-skew, campos de state, exclusión Radamanto, enganche fagocitosis. SemVer patch/minor. No cambiar `delegates_to` a Radamanto.

## 7. Criterios de aceptación (mapeo PBI)

| AC PBI | Verificación |
|--------|----------------|
| Macrófago Argos | Spec §3–5; cero llamadas Radamanto; tests no importan umbrales Radamanto |
| Discriminación + SSOT | Fixture JSON; test `skew>=threshold` → `fractures_emitted=[]` + baseline reanclado |
| Auto-poda | Test predicado + dry-run; apply en test tmp repo |
| Eventos | Ausencia de clase `anomaly-detected`; justificación §4 |
| Anti-bloqueo | Sin wait/hold en handler; sweep no escribe pending salvo fractura confirmada |
| No-ruido host | Test: gap wall 12h, mono ~0, PID vivo → 0 fracturas |

## 8. Tests (mínimo)

En `daemon_heartbeat.rs` (o módulo test del crate):

1. Baseline cold-start **intacta** (tests actuales).
2. `t_suspend_skew_reanchors_no_fracture`.
3. `t_no_skew_stale_hb_still_fractures` (PID vivo, skew 0, missed≥N).
4. `t_thresholds_from_ssot_not_const`.
5. `t_phagocyte_predicate_trace_before_lock` / `t_phagocyte_skip_unparseable`.

```text
cd SddIA && cargo test -p execute-process daemon_heartbeat -- --nocapture
```

## 9. Evolución

Registro nuevo bajo `directories.evolution` al **implementar** (no en stop-planning). Relacionar `056ac6a1-02fc-4988-a704-1f5b648d0e40`, `83bbfdeb-4715-4915-88be-751532dc268a`, `c6931c73-4cfc-4a11-b082-54099d420f59`.
