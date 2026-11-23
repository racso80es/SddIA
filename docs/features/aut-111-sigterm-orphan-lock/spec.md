---
feature_name: aut-111-sigterm-orphan-lock
created: "2026-11-23"
updated: "2026-11-23"
process: feature
phase: Diseño de Blueprint
agents: dedalo
base: main
scope: sddia-daemon-runtime, daemons (6), daemon-heartbeat-audit, kalma2-bridge
branch_name: feat/aut-111-sigterm-orphan-lock
persist_ref: docs/features/aut-111-sigterm-orphan-lock
pbi_ref: AUT-111
document_id: PBI-FIX-CONSOLIDATED-ORPHAN-LOCK-SIGTERM
status: spec_ready
dor_profile: bug-fix
execution_id: "a4111dfb-8fd3-4792-b2a8-b43f89fe369a"
dedalo_verdict: ok
consolidated_from:
  - AUT-100
  - AUT-101
  - AUT-102
  - AUT-103
  - AUT-104
  - AUT-105
  - AUT-106
  - AUT-107
  - AUT-108
---

# Spec — aut-111-sigterm-orphan-lock

## Resumen

Causa raíz **dual** (laudo L-ROOT): (1) bucles de centinela sin handler SIGTERM efectivo → proceso muere sin `DaemonRuntime::shutdown()` → `.lock` con PID muerto y mensaje `lock huérfano pid=…; recuperando` al rearrancar; (2) `audit_running_daemon` emite `F-ARGOS-ORPHAN-LOCK` / `System_Fracture_Detected` en el **primer** muestreo con `!pid_alive(pid)` (tras descartar `lock_predates_host_boot`), sin gracia ante reinicio controlado post-`btime`.

Esta feature cierra **AUT-111** (consolidación `AUT-100`…`AUT-108`) cumpliendo `daemons-contract.md` §5 / §6.1 / §6.3.

## Decisión Dedalo (L-HOW)

| Ref | Decisión |
|-----|----------|
| **D-RT** | Preferir **runtime compartido** en `sddia-daemon-runtime`: flag atómico de shutdown + API de instalación de handlers SIGTERM/SIGINT + helper `request_shutdown` / `shutdown_requested`. Los binarios solo cablean el bucle a esa flag (sin lógica de señal duplicada). |
| **D-SH** | Secuencia canónica al señalar: (opcional) `tick_with_status(…, "shutting_down")` best-effort → `DaemonRuntime::shutdown()` (borra `.lock`) → salir `0`. `Drop` sigue siendo red de seguridad, no el camino primario bajo SIGTERM. |
| **D-GR** | Gracia en auditoría: **confirmación diferida** — no emitir orphan-lock en el primer muestreo muerto; exigir **≥2 barridos consecutivos** con PID muerto y lock aún presente, o superar `orphan_lock_grace_seconds` desde el primer sospechoso (lo que ocurra primero según umbral SSOT). Si el lock desaparece o el PID revive → limpiar sospecha sin fractura. |
| **D-SD** | Si el último heartbeat conocido es `status: "shutting_down"` y el PID ya no vive, **no** emitir orphan-lock; depurar lock muerto silenciosamente (alineado a shutdown controlado). |

## Alcance de artefactos

| Área | Paths lógicos (topología) | Mutación |
|------|---------------------------|----------|
| Runtime | `SddIA/sddia-daemon-runtime` | signal flag + API shutdown + tests |
| Centinelas | `telegram-watcher`, `iota-publish-relay`, `github-bridge-watcher`, `email-watcher`, `event-watcher` bajo `SddIA/daemons/<name>/` | cablear bucle a flag; salida limpia |
| Bridge | `SddIA/interfaces/kalma2-bridge` | idem SIGTERM → lock |
| Auditoría | `SddIA/engine/execute-process/…/daemon_heartbeat.rs` + `heartbeat_audit_thresholds.rs` | gracia/confirmación + tests |
| Umbrales SSOT | `SddIA/daemons/heartbeat-audit.thresholds.json` | claves de gracia/confirmación |
| Corpus | fracture-corpus / evidencia Argos si aplica | caso anti-FP reinicio controlado |
| Units | `.SddIA/systemd/sddia-*.service` / templates | **solo lectura** salvo drift `KillSignal=SIGTERM` |

**Fuera de alcance:** rediseño Argos ajeno a orphan-lock; discriminar por `btime` adicional; centinelas no listados; reabrir `AUT-100`…`AUT-108`.

---

## S-1 — Shutdown SIGTERM en runtime compartido

### Comportamiento

1. `sddia-daemon-runtime` expone:
   - flag global/proceso `AtomicBool` (o equivalente) `shutdown_requested`;
   - `install_termination_handlers()` que intercepta SIGTERM y SIGINT y activa la flag (sin panics en el handler);
   - `shutdown_requested() -> bool` para que los bucles salgan.
2. Extender `DaemonRuntime` (o helper asociado) con `graceful_shutdown(&mut self, top: &BusTopology)`:
   - best-effort `tick_with_status(top, "shutting_down")`;
   - luego `shutdown()` (elimina `{daemons_instance.status}/{name}.lock`).
3. Tests unitarios del crate: flag se activa; `graceful_shutdown` elimina el lock aunque el proceso no muera por señal real.

### Oráculo (CA-3a / CA-1)

- `cargo test -p sddia-daemon-runtime …` — lock borrado tras shutdown explícito / graceful.
- Lab: `systemctl --user restart sddia-<daemon>@…` → sin `.lock` con PID muerto; sin log `lock huérfano pid=…; recuperando` en arranque inmediato.

---

## S-2 — Homogeneizar los 6 binarios en alcance

### Comportamiento

En cada binario de alcance (`telegram-watcher`, `iota-publish-relay`, `github-bridge-watcher`, `email-watcher`, `event-watcher`, `kalma2-bridge`):

1. Tras bootstrap del modo continuo: `install_termination_handlers()`.
2. Condición de salida del bucle: `shutdown_requested()` (además de `--once`).
3. Al salir: `graceful_shutdown` sobre el `DaemonRuntime` compartido (tomar el `Mutex` si aplica) y `exit 0`.
4. Workers de keepalive deben respetar la flag (dejar de tickear / unirse o terminar) para no retener el proceso.

No se exige uniformidad cosmética del código; sí la **secuencia observable** S-1.

### Oráculo

- Tests por crate o integración mínima que simulen request de shutdown y verifiquen ausencia de lock.
- Lab CA-1 por cada unidad `systemd --user` del workspace.

---

## S-3 — Anti-falso-positivo en `daemon-heartbeat-audit`

### Comportamiento

En `audit_running_daemon`, tras descartar `lock_predates_host_boot`:

```text
si !pid_alive(pid):
  si último status conocido == "shutting_down" → remove_lock; classification=clean_shutdown; return None
  registrar / incrementar sospecha (orphan_suspect_since | orphan_dead_sweeps)
  si sospecha NO confirmada (sweeps < umbral Y elapsed < grace) → classification=orphan_suspect; return None
  si confirmada → emit_orphan_lock_fracture (idempotente como hoy)
si pid_alive:
  limpiar campos de sospecha
```

### Umbrales SSOT

Ampliar `SddIA/daemons/heartbeat-audit.thresholds.json` (y parser/overlay):

| Clave | Default propuesto | Rol |
|-------|-------------------|-----|
| `orphan_lock_confirm_sweeps` | `2` | Barridos consecutivos con PID muerto antes de emitir |
| `orphan_lock_grace_seconds` | `45` | Techo temporal desde primera sospecha (cubre hueco entre audit ~30s y restart) |

Emisión cuando se cumple **cualquiera** de los dos umbrales con lock aún presente y PID aún muerto (confirmación positiva de orphan persistente). Un solo muestreo nunca basta.

### Oráculo (CA-2 / CA-3b)

- Test: primer `audit_staleness` con PID muerto post-boot → `fractures` vacío + classification `orphan_suspect` (o equivalente).
- Test: segundo barrido (o clock avanzado ≥ grace) con mismo lock muerto → 1 fractura orphan; tercero → idempotente vacío.
- Test: primer sospechoso + lock eliminado antes del segundo barrido → sin fractura.
- Test: heartbeat previo `shutting_down` + PID muerto → sin fractura; lock depurado.

---

## S-4 — Corpus, validación y cierre

1. Si el corpus de fracturas documenta `F-ARGOS-ORPHAN-LOCK`, añadir/actualizar caso **anti-FP reinicio controlado** (no emitir en primer muestreo / sí tras confirmación).
2. `validacion.md` Argos APTO con checks CA-1…CA-4.
3. Cierre Linear `AUT-111` → `state_map.done` (provider linear; sin PBI file en `docs/todos/`).

### Oráculo (CA-4)

- `validacion.md` con `global: APTO` y evidencias; issue Linear en done.

---

## Criterios de aceptación ↔ oráculos

| ID | Criterio | Oráculo mecánico |
|----|----------|------------------|
| **CA-1** | Restart limpio por centinela: sin lock muerto; sin `lock huérfano… recuperando` | Lab `systemctl --user restart` + inspección status/logs; tests shutdown runtime |
| **CA-2** | Reinicio flota: no `System_Fracture_Detected` / orphan-lock en primer muestreo | Tests gracia `daemon_heartbeat.rs` + observación audit en lab |
| **CA-3** | (a) SIGTERM → borrado lock; (b) gracia/confirmación audit | `cargo test` crates afectados |
| **CA-4** | Corpus si aplica; `validacion.md` APTO; AUT-111 done | Argos + Linear |

## Ola 0 (KZ-PRM-01 / KZ-PRM-03)

Antes de mutar código productivo: baseline hermético en `execution.md` + captura `oracle-red.log` bajo este `persist_ref` demostrando fallo actual (orphan inmediato / lock huérfano en restart) para CA-2/CA-3b y, si reproducible en lab, CA-1.

## Restricciones

- Git solo vía `skill:git-manager`.
- Materialización piloto solo vía MCP `sddia-workspace-server`.
- Sin forja de genoma ajeno al alcance; KM/`docs/todos/` solo vía Cúmulo / Kaizen_Alert_Required.
- No inventar éxito de lab: sin evidencia, CA lab = `PENDIENTE` / no elevar `global: APTO`.
