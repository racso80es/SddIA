---






feature_name: aut-111-sigterm-orphan-lock
created: "2026-11-23"
updated: "2026-11-23"
process: feature
phase: Diseño de Blueprint
agents: dedalo
branch_name: feat/aut-111-sigterm-orphan-lock
persist_ref: docs/features/aut-111-sigterm-orphan-lock
pbi_ref: AUT-111
document_id: PBI-FIX-CONSOLIDATED-ORPHAN-LOCK-SIGTERM
status: plan_ready
dor_profile: bug-fix
ola: "0 + 1 + 2 + cierre"
phases:
  - ola0-oracle-red-baseline
  - runtime-sigterm-shared
  - wire-six-sentinels
  - audit-orphan-grace
  - corpus-validacion-cierre
execution_id: "a5f95b61-ab09-4659-8554-403502c4a239"
---
# Plan — aut-111-sigterm-orphan-lock

Blueprint ejecutable para Tekton. Oráculos = CA de `spec.md` / `objectives.md`.

```
Ola-0  baseline hermético + oracle-red.log
Ola-1  runtime SIGTERM compartido + wire 6 binarios
Ola-2  gracia orphan-lock en daemon-heartbeat-audit + umbrales
Cierre corpus / validacion.md / Linear AUT-111
```

---

## Fase P0 — Ola 0: oráculo en rojo + baseline hermético

**Intent:** Capturar fallo actual antes de mutar producto (KZ-PRM-01 / KZ-PRM-03).

**delegates_to:** `skill:git-manager`, `action:execute-process` (tests / audit sweep), shell-executor whitelisted solo vía runtime autorizado.

**Pasos:**

1. Registrar en `execution.md`: `git status --porcelain` (vía git-manager); unidades `sddia-*` user; ausencia de ruido raíz (`.events`, etc.).
2. Ejecutar tests actuales que **demuestran** emisión orphan inmediata (`orphan_lock_dead_pid_emits_fracture_once` debe seguir pasando en rojo respecto al nuevo contrato — documentar como baseline).
3. Si lab disponible: un `systemctl --user restart` de un centinela en alcance; capturar log `lock huérfano` y/o fractura audit → `oracle-red.log` bajo `persist_ref`.
4. Sin lab: declarar bloqueo lab explícito; Ola 1 puede avanzar con oráculos unitarios; CA-1 lab queda `PENDIENTE` hasta evidencia.

**Gate:** `oracle-red.log` materializado **o** sección explícita en `execution.md` con causa de no-lab + evidencia test baseline.

---

## Fase P1 — Runtime compartido SIGTERM

**Intent:** API única de terminación limpia en `sddia-daemon-runtime` (D-RT / D-SH).

**delegates_to:** implementación Tekton sobre crate `sddia-daemon-runtime`; verificación `cargo test -p sddia-daemon-runtime`.

**Pasos:**

1. Añadir flag + `install_termination_handlers` + `shutdown_requested`.
2. Añadir `graceful_shutdown` (heartbeat `shutting_down` best-effort + `shutdown()`).
3. Tests: lock eliminado; flag tras simulación de request (sin depender de señal OS en CI si es frágil).

**Gate:** tests del crate en verde; API usable desde binarios.

---

## Fase P2 — Cablear los 6 binarios

**Intent:** Homogeneizar alcance L-SCOPE / CA-1.

| Binario | Crate path |
|---------|------------|
| telegram-watcher | `SddIA/daemons/telegram-watcher` |
| iota-publish-relay | `SddIA/daemons/iota-publish-relay` |
| github-bridge-watcher | `SddIA/daemons/github-bridge-watcher` |
| email-watcher | `SddIA/daemons/email-watcher` |
| event-watcher | `SddIA/daemons/event-watcher` |
| kalma2-bridge | `SddIA/interfaces/kalma2-bridge` |

**Pasos por binario:**

1. Instalar handlers en modo continuo.
2. Salir del bucle cuando `shutdown_requested()`.
3. Llamar `graceful_shutdown`; workers keepalive respetan la flag.
4. Test mínimo o prueba de integración de shutdown donde el crate ya tenga harness.

**Gate:** `cargo test` de los 6 crates (o workspace filtrado) OK; sin regresión `--once`.

---

## Fase P3 — Gracia / confirmación orphan-lock

**Intent:** CA-2 / CA-3b (D-GR / D-SD).

**delegates_to:** mutación `daemon_heartbeat.rs`, `heartbeat_audit_thresholds.rs`, SSOT `SddIA/daemons/heartbeat-audit.thresholds.json`.

**Pasos:**

1. Extender umbrales: `orphan_lock_confirm_sweeps` (default 2), `orphan_lock_grace_seconds` (default 45); parser + overlay + tests de carga.
2. Estado por daemon: sospecha (`orphan_suspect_since` / `orphan_dead_sweeps`); limpiar en PID vivo o lock ausente.
3. Rama `shutting_down` → depurar lock sin fractura.
4. Ajustar test legacy `orphan_lock_dead_pid_emits_fracture_once` a **confirmación** (dos barridos o grace forzada en test).
5. Añadir tests: no emitir en primer muestreo; emitir tras confirmación; idempotencia; shutdown limpio.

**Gate:** suite `daemon_heartbeat` / execute-process handlers en verde con nuevo contrato.

---

## Fase P4 — Corpus, validación, cierre

**Intent:** CA-4.

**Pasos:**

1. Caso anti-FP en fracture-corpus **si** el índice de firmas lo contempla; si no aplica, documentar N/A en `validacion.md`.
2. Lab flota (si entorno): restart controlado de los 6; verificar CA-1/CA-2 con evidencia en `execution.md`.
3. `validacion.md` Argos: checks CA-1…CA-4; `global` solo APTO con evidencia.
4. Cerrar `AUT-111` en Linear (`state_map.done`).
5. PR vía git-manager / delivery-close-cycle según proceso feature.

**Gate:** `validacion.md` APTO + Linear done + PR listo.

---

## Mapa de artefactos

| Artefacto | Acción | Fase |
|-----------|--------|------|
| `docs/features/aut-111-sigterm-orphan-lock/oracle-red.log` | crear | P0 |
| `docs/features/aut-111-sigterm-orphan-lock/execution.md` | crear/actualizar | P0–P4 |
| `SddIA/sddia-daemon-runtime/src/lib.rs` (+módulos señal) | mutar | P1 |
| `SddIA/daemons/*/src/main.rs` (5) | mutar | P2 |
| `SddIA/interfaces/kalma2-bridge` (entry continuo) | mutar | P2 |
| `…/daemon_heartbeat.rs` + thresholds | mutar | P3 |
| `SddIA/daemons/heartbeat-audit.thresholds.json` | mutar | P3 |
| `docs/features/aut-111-sigterm-orphan-lock/validacion.md` | crear | P4 |
| fracture-corpus (si aplica) | mutar | P4 |

---

## Orden y dependencias

| Orden | Depende de |
|-------|------------|
| P0 | — |
| P1 | P0 iniciado (oracle-red o waiver lab documentado) |
| P2 | P1 |
| P3 | P0 (puede paralelizarse con P1/P2 tras P0; ideal tras P1 para lab conjunto) |
| P4 | P1–P3 + evidencias |

P3 es **independiente en código** de P2 (defensa en profundidad): conviene aterrizar ambos; el orden recomendado es P1→P2→P3 para lab de flota coherente.

---

## Bloqueos conocidos

| ID | Descripción | Desbloqueador |
|----|-------------|---------------|
| B-LAB | Host sin flota `systemd --user` / tokens | Racso lab; CA lab PENDIENTE sin inventar APTO |
| B-RED | oracle-red no reproducible en CI | Waiver en execution + tests unitarios como oráculo primario |

---

## Veredicto Dedalo

**ok** — requisitos estabilizados (`clarify.md` / `objectives.md`); diseño D-RT/D-SH/D-GR/D-SD cerrado; `spec.md` + este `plan.md` listos para implementación Tekton.
