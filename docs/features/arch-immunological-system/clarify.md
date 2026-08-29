---
feature_name: arch-immunological-system
created: "2026-08-29"
process: feature
purpose: Estabilización Mayeuta — PBI-ARCH-IMMUNOLOGICAL-SYSTEM v1.1.0
branch_name: feat/arch-immunological-system
persist_ref: docs/features/arch-immunological-system
pbi_ref: docs/todos/pending/PBI-ARCH-IMMUNOLOGICAL-SYSTEM.md
document_id: PBI-ARCH-IMMUNOLOGICAL-SYSTEM
uuid: "056ac6a1-02fc-4988-a704-1f5b648d0e40"
execution_id: "987e1747-bd08-4c80-ad41-648f09cc4b12"
mayeuta_verdict: ok
laudo: suspend-skew-plus-phagocyte
---

# Clarificación — arch-immunological-system

Transcript Mayeuta. Semilla: PBI `PBI-ARCH-IMMUNOLOGICAL-SYSTEM` v1.1.0 (refinado 2026-08-29). Init lab `execution_id` `987e1747-bd08-4c80-ad41-648f09cc4b12`. Filtro A contra genoma vigente.

---

## D0 — Apertura formal

| Pregunta | Decisión |
|----------|----------|
| Proceso | `feature` v1.3.2 |
| `feature_name` | `arch-immunological-system` |
| Rama | `feat/arch-immunological-system` |
| `persist_ref` | `docs/features/arch-immunological-system` |
| `document_id` | `PBI-ARCH-IMMUNOLOGICAL-SYSTEM` |
| Init lab | `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery |
| `execution_id` | `987e1747-bd08-4c80-ad41-648f09cc4b12` |
| Stop planning | esta sesión: clarify + objectives + spec + plan + commit; **no** T0–Tn |

**Toll:** un `persist_ref`, un PR.

---

## D1 — «El sistema no tiene triaje»

| Semilla cruda (v1.0.0) | Filtro A | Laudo |
|------------------------|----------|-------|
| Reactividad ciega; caída → `System_Fracture_Detected` mecánico | **Falso.** `daemon-heartbeat-audit` (Argos, CEN-05) ya exige lock+PID vivo y `missed_cycles >= 3`. Baseline `max(last_heartbeat_at, lock.started_at)` (`effective_heartbeat_baseline`, evolución `c6931c73`). Telemetría base = `Daemon_Heartbeat`. | El gap no es «añadir cooldown». Es (a) **suspend/resume de host** (mismo PID, wall-clock salta, missed explota) y (b) **PBI residual** tras recuperación (`fracture_event_id` se limpia al ingest HB; el PBI git queda). |

---

## D2 — Evento `Anomaly_Detected`

| Semilla | Filtro A | Laudo |
|---------|----------|-------|
| Nuevo evento de telemetría intermedio | **Innecesario.** No existe clase ECST. El bus ya transporta `Daemon_Heartbeat`; el estado vive en `daemons_instance.state` → `heartbeat-audit.json`. | **Fuera.** Reutilizar `Daemon_Heartbeat` + campos de estado (`classification`, `last_audit_wall_at`, `last_audit_mono_ms`). Nueva clase solo si el diseño T2 demuestra que el fan-out actual bloquea (AC anti-bloqueo). Predicción: no bloquea — la discriminación es síncrona en el sweep, sin retener eventos en `eda_bus.pending`. |

---

## D3 — Macrófago = Radamanto vs Argos

| Semilla | Filtro A | Laudo |
|---------|----------|-------|
| ¿Radamanto barre telemetría o Argos ejecución dirigida? | Radamanto: inputs solo `Raw_Execution_Finished`; **prohibida** medición directa / `skill:shell-executor`. Argos ya es suscriptor `Daemon_Heartbeat` y emisor de fractura. | **Argos** vía `daemon-heartbeat-audit`. Radamanto **excluido** del sondeo PID. Sin sello `System_Immunity_Certified` en este PBI (jurisdicción caos/suite, no latido). |

---

## D4 — Umbral «2 ciclos» vs `MISSED_CYCLES_THRESHOLD`

| Semilla | Filtro A | Laudo |
|---------|----------|-------|
| Cooldown de 2 ciclos | **Inútil** contra gaps de horas (auditoría 237–1532 ciclos). Umbral actual **cableado** `const MISSED_CYCLES_THRESHOLD: i64 = 3` en `daemon_heartbeat.rs`. | Mantener semántica `>= N` para micro-latencia. Parametrizar N y el **skew de suspend** en SSOT JSON (no en `radamanto.thresholds.json`). Instancia puede overlay. |

---

## D5 — Qué detecta hoy el auditor (ontología)

`audit_running_daemon`: si **no** hay lock o PID muerto → **no** emite fractura. Fractura vigente = proceso **vivo** sin latido fresco.

| Escenario | Comportamiento hoy | Hueco |
|-----------|-------------------|-------|
| Crash + restart (lock nuevo) | Baseline = `started_at` nuevo | Cubierto |
| Lock huérfano, PID muerto | `Ok(None)` | No PBI (ceguera funcional distinta) |
| Suspend/resume, **mismo** PID | `now - last_hb` enorme → fractura | **Hueco de este PBI** |
| HB se restablece tras fractura | `record_heartbeat_at` borra `fracture_event_id` | PBI en `paths.todos.pending` permanece |

---

## D6 — Auto-poda vs cierre documental (un PR)

Mover PBI a `done/` sin `validacion.md` + PR viola `features-documentation-pattern` v1.2.x **si** el cierre se pretende Done de forja. Laudo:

- **Fagocitosis runtime (obligatoria):** ledger instancia bajo `daemons_instance.state` (fuera de git). El TQM / listados no deben tratar como «abierto operativo» un `PBI-FIX-FRACTURE-*` cuya traza `last_heartbeat` sea anterior a `lock.started_at` **o** cuyo daemon tenga `missed_cycles=0` y clasificación `phagocytosed`.
- **Fagocitosis documental (obligatoria en forja):** proceso que mueve candidatos `paths.todos.pending` → `paths.todos.done`, escribe `docs/fixes/centinelas-fracture-ola-{YYYYMMDD}/` + evolution. **No** `delivery-close-cycle` automático (evita PR ciego). El humano no audita cada PBI: el proceso aplica laudo B automático con evidencia de sweep.

Predicado candidato (Kaizen auditoría §10, endurecido):

```text
document_id ~= PBI-FIX-FRACTURE-*
AND fracture_process resuelto
AND (last_heartbeat_traza < lock.started_at vigente
     OR (missed_cycles==0 AND last_heartbeat_traza < lock.started_at))
```

**Prohibido:** mutar umbrales o reescribir `heartbeat-audit.json` para «limpiar» trazas; `SDDIA_SKIP_HOOKS`.

---

## D7 — Reloj: wall vs monotónico

Evidencia de suspend: `Δwall >> Δmonotonic` (o `CLOCK_BOOTTIME` vs wall) entre sweeps, con PID aún vivo y `lock.started_at` anterior al gap.

No inferir `/sys` ni D-Bus logind como dependencia dura (agnosticismo Core). Señal portable: timestamps persistidos en el estado de auditoría + `std::time::Instant` / equivalente en el proceso del sweep. Overlay Linux opcional (`/proc/uptime`) **fail-soft**.

Si el skew supera `suspend_skew_seconds` (SSOT): `classification=host_suspend`, reanclar `last_heartbeat_at` al `now` del sweep (equivalente cold-start), **no** emitir `System_Fracture_Detected`.

---

## D8 — Fuera de alcance

- Ceguera IMAP / watermark (familia B de la auditoría).
- Mutar emisores de `Daemon_Heartbeat` o intervalos de centinela.
- `governance-daemon-manager` / kill-switch.
- Certificación DLT / Radamanto.
- Ritual humano de olas históricas ya archivadas (20260812, 20260819).
