---
document_id: PBI-ARCH-IMMUNOLOGICAL-SYSTEM
uuid: "056ac6a1-02fc-4988-a704-1f5b648d0e40"
title: "[ARQUITECTURA] Sistema Inmunológico Autónomo: Discriminación Suspend/Crash y Auto-Poda de Ruido Sistémico"
format: markdown
version: "1.1.0"
created: "2026-08-26"
refined: "2026-08-29"
status: "pending"
priority: "alta"
process: feature
related:
  - docs/audits/centinelas-fracturas-eventos-pending-20260826.md
  - SddIA/events/events-contract.md
  - SddIA/events/domain/system-fracture-detected.md
  - SddIA/events/telemetry/daemon-heartbeat.md
  - SddIA/process/daemon-heartbeat-audit.md
  - SddIA/agents/argos.md
  - SddIA/agents/radamanto.md
  - SddIA/engine/execute-process/src/engine/handlers/daemon_heartbeat.rs
  - SddIA/evolution/83bbfdeb-4715-4915-88be-751532dc268a.md
  - SddIA/evolution/c6931c73-4cfc-4a11-b082-54099d420f59.md
---

# [ARQUITECTURA] Sistema Inmunológico Autónomo: Discriminación Suspend/Crash y Auto-Poda de Ruido Sistémico

## 0. Nota de refinamiento (v1.1.0)

Refinamiento con verificación empírica contra el genoma vigente. Correcciones frente a v1.0.0:

- **Referencias corregidas:** la ruta `docs/audits/AUDIT-CENTINELAS-...` no existe (nombre real en minúsculas sin prefijo `AUDIT-`); `SddIA/norms/events-contract.md` no existe (el contrato vive en `SddIA/events/events-contract.md`).
- **Premisa corregida (§1):** es **inexacto** afirmar que el sistema opera "sin capa de triaje" o que una caída "dispara mecánicamente `System_Fracture_Detected`". Ya existe triaje: `daemon-heartbeat-audit` (Argos, CEN-05) exige `missed_cycles >= 3` sobre centinela con lock/PID vivo, con baseline anti-cold-start. La telemetría base es `Daemon_Heartbeat`, no una fractura directa.
- **Gap real reencuadrado:** el umbral de 3 ciclos (~90 s) no protege contra suspensiones largas de host (p.ej. ~12 h ⇒ ~1500 ciclos omitidos ⇒ fractura real emitida). El problema no es "añadir cooldown", sino (a) discriminar *suspend/resume del host* de *crash del proceso* y (b) auto-podar/consolidar los PBIs de fractura ya recuperados.
- **Rol Macrófago acotado:** Radamanto tiene **prohibida** contractualmente la medición directa y el uso de cronómetros (solo consume acumulado de `Raw_Execution_Finished`). No puede sondear PID vivo. El auditor empírico de latido es **Argos**.

## 1. Contexto y Fricción Evolutiva

El sistema EDA de SddIA **ya dispone** de una capa de triaje de vitalidad: los Centinelas emiten telemetría `Daemon_Heartbeat` y el proceso `daemon-heartbeat-audit` (Argos, CEN-05) calcula `missed_cycles = floor((now - last_heartbeat_at) / heartbeat_interval_seconds)` y solo emite `System_Fracture_Detected` cuando `missed_cycles >= 3` sobre un centinela con **lock y PID vivos**. Existe además baseline anti-falso-positivo en cold-start (`max(last_heartbeat_at, lock.started_at)`, evolución `c6931c73`).

La fricción real es otra: ese umbral (~90 s con intervalo 30 s) está diseñado para tolerar micro-latencias, **no** para suspensiones prolongadas del host. Cuando el host se suspende o se apaga con el proceso aún registrado (lock vivo, PID persistente pero congelado), al despertar el sweep acumula cientos/miles de ciclos omitidos y emite una `System_Fracture_Detected` legítima según el umbral, pero **espuria** en la ontología: no hubo muerte del proceso, hubo letargo físico del host. Cúmulo entonces materializa un PBI en `pending/` (patrón Kintsugi correcto pero mal alimentado).

La auditoría `docs/audits/centinelas-fracturas-eventos-pending-20260826.md` evidencia el síntoma: 5 PBIs `System_Fracture_Detected` con `missed_cycles` de 237–1532 (downtime ~2–13 h), todos con runtime **sano** al auditar (`missed_cycles=0`, `fractures_emitted: []`). Su Kaizen §10 lo nombra: "reincidencia de PBIs fractura sin consolidación periódica" — el circuito detecta y materializa bien, pero **falta el ritual de archivo** tras recuperación, dejando `pending/` con señales históricas que parecen incidentes abiertos.

Esta carga de auditar ruido transitorio recae hoy sobre el Vértice Biológico, violando el Filtro C (Eficiencia).

## 2. Objetivos Estratégicos (S+ Grade)

- **Discriminación Suspend/Crash:** dotar al auditor de la capacidad de reconocer suspensión/apagado del host (saltos de reloj monotónico/wall-clock, señales de resume) y **no** contabilizarlos como muerte térmica del proceso.
- **Triaje Empírico Reforzado:** cuando la anomalía sea ambigua, verificar el estado físico actual del proceso antes de escalar (ya lo hace Argos vía locks/PID; extender a confirmación de latido restablecido tras cuarentena).
- **Poda Ontológica Dinámica:** si el sistema confirma auto-recuperación (latido restablecido y/o `lock.started_at` posterior a la traza de la fractura), el síntoma se registra en la bitácora evolutiva y se **descarta/auto-archiva** su PBI sin intervención humana ("ruido fagocitado").

## 3. Especificación del Flujo de Defensa (Línea de Montaje)

Extensión del metabolismo del error actual (`daemon-heartbeat-audit` → `System_Fracture_Detected` → Cúmulo/Mayeuta):

### Fase 1 — Interceptación del síntoma (base ya existente)
`Daemon_Heartbeat` sigue siendo el substrato de telemetría. La staleness (`missed_cycles`) se calcula igual. **Nuevo:** ante staleness, en lugar de resolver de inmediato a fractura, se clasifica la naturaleza del gap (ver Fase 2).

### Fase 2 — Discriminación Suspend/Crash y cuarentena
Antes de emitir fractura, el auditor evalúa si el gap es atribuible a suspensión/apagado del host (evidencia: salto de reloj coherente con el gap, ausencia de actividad de PID durante el intervalo, resume reciente). Si es letargo físico, se **reancla el baseline** (equivalente a cold-start) y no se contabiliza como fractura. Para latencias ambiguas cortas se mantiene la ventana de cuarentena (umbral vigente `missed_cycles >= 3`, ajustable vía SSOT, no cableado).

### Fase 3 — Auditoría Macrófaga (verificación empírica)
Transcurrida la cuarentena, **Argos** (no Radamanto — ver §4) testea el estado físico actual: lock vivo (`kill -0`), side-channel `.SddIA/daemons/state/heartbeats/{daemon}.json` con timestamp fresco, PID coherente con la instancia esperada.

### Fase 4 — Resolución y Veredicto
- **Laudo B automático (ruido fagocitado):** si el proceso está vivo y el latido se restableció (o el gap fue suspend del host), el evento se clasifica como "latencia/letargo del host", se purga de la cola de alertas y se documenta en la bitácora evolutiva. Si ya existiese un PBI materializado cuya traza (`last_heartbeat`) sea **anterior** al `lock.started_at` vigente, se auto-archiva (candidato a ola B) sin laudo humano.
- **Laudo A (fractura confirmada):** solo si tras la cuarentena el proceso sigue sin latido, con lock ausente/PID muerto o error estructural, se emite `System_Fracture_Detected` al bus para que Cúmulo materialice el PBI.

## 4. Criterios de Aceptación (Protocolo de Acero)

- [ ] **Rol Macrófago resuelto:** el verificador empírico es **Argos** (vía `daemon-heartbeat-audit`). Radamanto queda excluido del sondeo directo por prohibición contractual (medición directa/cronómetros); su rol se limita, si aplica, a certificación estadística/DLT (precedente `System_Immunity_Certified`, D0.4).
- [ ] **Discriminación suspend/crash implementada** en el handler `daemon_heartbeat.rs` (o cápsula equivalente), con reancla de baseline ante resume de host; parametrizada vía SSOT (`heartbeat-audit`/thresholds), sin valores cableados.
- [ ] **Auto-poda/consolidación:** proceso o fase que, tras ignición con `missed_cycles=0`, identifique PBIs `PBI-FIX-FRACTURE-*` cuya traza sea anterior al `lock.started_at` vigente y los mueva a ola B / bitácora evolutiva sin intervención humana.
- [ ] **Contratos de eventos:** si se introduce un evento intermedio de anomalía (p.ej. `Anomaly_Detected`, aún inexistente), crearlo conforme a `SddIA/events/events-contract.md` bajo `SddIA/events/{event_family}/`, con payload ECST y suscripciones; en caso contrario, justificar la reutilización de `Daemon_Heartbeat` + estado de auditoría.
- [ ] **Inmunidad anti-bloqueo:** los eventos/anomalías en cuarentena no deben bloquear la orquestación principal del bus EDA.
- [ ] **Garantía de no-ruido:** ningún apagón/suspensión de host debe resultar en un PBI de fractura abierto pendiente de revisión humana (ni al detectar, por discriminación suspend; ni tras recuperar, por auto-poda).

## 5. Referencias técnicas

- Umbral y sweep: `SddIA/process/daemon-heartbeat-audit.md` (v1.0.1).
- Handler: `SddIA/engine/execute-process/src/engine/handlers/daemon_heartbeat.rs`.
- Régimen de vitalidad A+B+C+D y baseline cold-start: `SddIA/evolution/83bbfdeb-...` y `SddIA/evolution/c6931c73-...`.
- Contrato centinelas: `SddIA/daemons/daemons-contract.md` §6.1.
- Auditoría de evidencia: `docs/audits/centinelas-fracturas-eventos-pending-20260826.md` (§3.3 downtime, §10 Kaizen, §11 referencias).
