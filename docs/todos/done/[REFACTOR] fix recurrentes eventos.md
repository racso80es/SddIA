---
document_id: PBI-REFACTOR-HEARTBEAT-CIRCUIT-20260811
uuid: 83bbfdeb-4715-4915-88be-751532dc268a
title: "[REFACTOR] Circuito Daemon_Heartbeat — erradicar fracturas recurrentes de telemetría"
format: markdown
version: "0.3.0"
created: "2026-08-11"
refined: "2026-08-11"
status: cerrado
closed: "2026-08-11"
priority: alta
process: refactorization
persist_ref: docs/features/heartbeat-circuit-regimen-20260811
fix_ref: "docs/features/heartbeat-circuit-regimen-20260811"
laudo_c3: "A+B+C (+D)"
implementation_status: "cerrado 2026-08-11 — A+B+C+D + ignición S+ Grade"
related_fix_pbis:
  - PBI-FIX-FRACTURE-b8e3c0e97eb4
  - PBI-FIX-FRACTURE-d47d7767e23b
  - PBI-FIX-FRACTURE-23c58000e252
  - PBI-FIX-FRACTURE-63c439de23d0
related:
  - SddIA/process/daemon-heartbeat-audit.md
  - SddIA/sddia-daemon-runtime/src/lib.rs
  - docs/fixes/daemon-heartbeat-ingest-ignition
  - docs/fixes/centinelas-fracture-ola-20260723
---

# [REFACTOR] Circuito Daemon_Heartbeat — fracturas recurrentes

## 1. Spec

### Conflicto (síntoma)

`daemon-heartbeat-audit` (Argos) emite `System_Fracture_Detected` y materializa PBI FIX satélites (`event-sweeper`, `event-watcher`, `github-bridge-watcher`, `telegram-watcher`) cuando `missed_cycles ≥ 3` en `heartbeat-audit.json`, **aunque los binarios sigan vivos** (mismo PID, lógica de dominio operativa).

Esto **no** es un falso positivo del umbral: Argos mide correctamente el **registro de auditoría**. El fallo está en el **circuito de prueba de vida** entre emisión y registro.

### Cadena real (SSOT operativo)

```text
Centinela (keepalive)
  → escribe Daemon_Heartbeat en .events/telemetry/
  → event-watcher / route-telemetry (fan-out)
  → daemon-heartbeat-audit (record_heartbeat)
  → .SddIA/daemons/state/heartbeat-audit.json
  → sweep: si lock vivo ∧ missed_cycles ≥ 3 → System_Fracture_Detected
```

Los centinelas **no** escriben `heartbeat-audit.json`. Ese archivo lo actualiza Argos al ingerir eventos (o el gate `_ingest_telemetry_heartbeats` solo en **ignición** vía `start-sddia.sh`, PR #155).

### Causa raíz probable (evidencia 2026-08-11)

| Hecho | Lectura |
|-------|---------|
| PID vivos desde 2026-08-10T15:18Z durante las fracturas | No es muerte del proceso |
| Congelación del audit ~15,4 h (`last_heartbeat` ≈ 10 ago 15:48 → bang 11 ago 07:11; ~1845 ciclos @30s / ~923 @60s) | El **registro** dejó de actualizarse con locks vivos |
| Rebrotes 07:14 / ~07:32–07:51 durante carga PPR | Misma familia bajo saturación del bus |
| `_ingest_telemetry_heartbeats` solo en ignición | Régimen continuo depende del fan-out |
| Keepalive: `eprintln!(…)` ante error de `tick` y el main sigue | Errores de emisión **tragados** (cofactor; no explica solo la noche) |
| Dead-letter masivo histórico | Presión / fallos de entrega en el bus |

**Hipótesis primaria:** inanición del fan-out (`event-watcher` síncrono saturado por `pending/` u otras familias) → latidos emitidos (o purgados) **sin** `record_heartbeat` a tiempo. Misma familia que `daemon-heartbeat-ingest-ignition`, **sin cobertura en régimen**.

**Hipótesis descartada / inexacta en borradores previos:** contención de *file lock* entre centinelas sobre el monolito `heartbeat-audit.json`. Los centinelas no compiten por ese archivo.

### Objetivo S+ Grade

1. **Régimen:** la prueba de vida debe llegar al registro de auditoría sin depender solo de la ignición ni de la fairness accidental del fan-out de dominio.
2. **Fail-fast en emisión:** si el centinela **no puede materializar** `Daemon_Heartbeat` en el bus fractal (escritura a telemetría), no puede continuar como entidad indocumentable (“zombi de radar”). Proceso muerto + reinicio limpio > proceso vivo invisible a Argos.
3. **Consolidar** los 4 PBI FIX `related_fix_pbis` bajo este refactor (o una ola `bug-fix` hija) tras remediación + no-regresión empírica — no archivarlos como “deuda documental” mientras el agujero de régimen siga abierto.

### Fuera de alcance (este PBI)

- Mutar el umbral `missed_cycles ≥ 3` para “silenciar” fracturas.
- Tratar como Done el mero archivo de PBI satélite sin cambio de circuito (laudo B solo aplica a fracturas históricas ya mitigadas; ver ola 20260723).
- Exigir ack de Argos en el hilo keepalive como condición de vida (acoplaría centinela ↔ auditor; ver Clarify).

---

## 2. Clarify (Filtro A)

Resolver **antes** de codificar:

### C1 — Dos capas, dos remedios

| Capa | Fallo | Remedio legítimo |
|------|-------|------------------|
| **Emisión** | `write_fractal_event` / `tick` falla | Reintentos + **Crash-Only** tras N fallos (Veto al silencio) |
| **Ingesta / fan-out** | Evento en telemetry pero audit no actualiza | Bypass de inanición en régimen (ingest periódico, cola prioritaria heartbeat, o side-channel) |

Crash-Only **no** debe interpretarse como “si Argos no me auditó en T segundos, me inmoló”: eso castiga al emisor por congestión del consumidor y puede tumbar el ecosistema bajo carga legítima.

### C2 — Veto al silencio (Swallowed Errors) — válido en capa Emisión

Hoy el worker de keepalive traga el error (`eprintln` + loop). Hay que decidir mecanismo:

- Propagar fallo letal al proceso (canal/`JoinHandle`/flag + exit del main), **o**
- `panic!` / `process::exit` desde el worker tras agotar reintentos de **escritura**.

El colapso del hilo de telemetría **sin** derribar el main está prohibido si la emisión es irrecuperable.

### C3 — Cuello de botella real (decidir vía)

**No** es el lock del monolito `heartbeat-audit.json` entre centinelas. Decidir entre:

| Vía | Qué cambia | Efecto |
|-----|------------|--------|
| **A — Ingest de régimen** | Extender el patrón `_ingest_telemetry_heartbeats` (o equivalente nativo) fuera de ignición: cron/sweep ligero que reingiere el último HB por daemon desde telemetry sin pasar por la cola domain saturada | Cierra el agujero PR #155 en continuo |
| **B — Prioridad / fairness en fan-out** | `event-watcher` / `route-telemetry`: cola o chunk prioritario para `Daemon_Heartbeat` frente a `pending/` de dominio | Ataca la inanición en origen |
| **C — Side-channel de prueba de vida** | Cada centinela escribe `daemons_instance.state/heartbeats/<daemon_name>.json` (o mtime en lock) **además** del ECST; Argos auditá ese directorio | Desacopla vitalidad del bus; requiere actualizar `daemon-heartbeat-audit` |
| **D — Solo Crash-Only en emit** | Sin cambio de ingest | Insuficiente solo: no explica la noche con emisión presumiblemente viva |

**Laudo Vértice Biológico (2026-08-11):** **A + B + C** (+ **D** siempre).

| Vía | Decisión |
|-----|----------|
| A | Ingest de régimen nativo en `daemon-heartbeat-audit` (side-channel + último HB telemetry) antes de cada sweep; `event-sweeper` invoca sweep periódico |
| B | `event-watcher` prioriza `telemetry` / `Daemon_Heartbeat` antes de `pending` y resto fractal |
| C | Side-channel `.SddIA/daemons/state/heartbeats/<daemon>.json` en cada emit |
| D | Crash-Only tras N fallos consecutivos de emisión (side-channel obligatorio) |

### C4 — Relación con PBI FIX abiertos

Los cuatro FIX de fractura (b8e3c0e97eb4, d47d7767e23b, 23c58000e252, 63c439de23d0) son **síntomas del mismo circuito**. Este refactor es el vehículo de causa raíz; los FIX no deben ejecutarse como cuatro `bug-fix` aislados de keepalive.

---

## 3. Plan (post-laudo A+B+C+D)

1. ~~Decide vía~~ → **A+B+C+D**.
2. **Runtime** (`sddia-daemon-runtime`): side-channel + ECST; Crash-Only en workers.
3. **Auditor** (`daemon_heartbeat.rs`): `ingest_regime` (side-channel + latest telemetry) antes de `audit_staleness`; sweeper dispara sweep.
4. **Fairness** (`event-watcher`): orden de roots + prioridad HB en telemetry.
5. **Validación empírica** (smoke) + cierre de 4 FIX satélite en misma entrega.

### Criterio de éxito

Un centinela vivo **siempre** tiene rastro fresco en el registro que Argos usa para el umbral; si no puede dejar rastro de emisión, **muere**. Dejan de nacer olas recurrentes de FIX por el mismo mecanismo bajo carga normal del bus.

---

## 4. Notas de refinamiento (errata del borrador 0.1)

| Afirmación previa | Corrección |
|-------------------|------------|
| “Falsos positivos de Argos” | Argos acierta sobre el audit; falla el circuito de actualización |
| “Contención de locks sobre `heartbeat-audit.json`” | Los centinelas no escriben ese archivo |
| “Canal con Argos amputado” → panic | El canal es el **bus/telemetría** (y opcional side-channel); no hay IPC directo a Argos |
| Título `[OPERATIVO]` vs fichero `[REFACTOR]` | Unificado a `[REFACTOR]` + `process: refactorization` |
| Vía A/B solo sobre monolito audit | Reformuladas a ingest/fairness/side-channel |
| Smoke solo `chmod` del audit | Debe apuntar al **destino de emisión** real (telemetry o side-channel) |
