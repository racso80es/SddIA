---
feature_name: kaizen-disk-thermodynamics-syslog
created: "2026-09-13"
process: feature
branch_name: feat/kaizen-disk-thermodynamics-syslog
persist_ref: docs/features/kaizen-disk-thermodynamics-syslog
pbi_ref: docs/todos/pending/[KAIZEN] Termodinámica de disco — inundación syslog del event-watcher y techos de log de instancia.md
execution_id: "e3bbb4f8-b27c-4833-9750-d855b504da8b"
document_id: PBI-KAIZEN-DISK-THERMODYNAMICS-SYSLOG
pbi_uuid: "9454e7eb-f397-4e36-abaf-91fd91cba802"
pbi_version: "1.2.0"
status: in-progress
---

# Objetivos — kaizen-disk-thermodynamics-syslog

## Misión

Erradicar la fuga térmica de stdout del centinela `event-watcher` en el hot path de sondeo (`POLL_SECONDS = 2`) que replica líneas `[WATCHER] skip …` hacia journald/syslog y satura el disco del host. Defensa en profundidad: `LogRateLimit*` en la fábrica systemd y en la unidad de instancia versionada. Veto absoluto a usar el purger de sandbox contra `/var`.

## Alcance (manifiesto)

- Ciclo `feature` inicializado (`execution_id` `e3bbb4f8-b27c-4833-9750-d855b504da8b`).
- Slice A: silencio de skips en ambos bucles de `run_watcher`; tests nativos `-p event-watcher`.
- Slice B: `LogRateLimitIntervalSec`/`LogRateLimitBurst` en `sddia-daemon@.service.template` y `.SddIA/systemd/sddia-event-watcher@.service`.
- Runbook de host en persist_ref (sin paths de host en genoma `SddIA/`).
- Cierre documental en rama + DCC + PR. `accept-pr` solo con CI verde (`run_id`).

## Fuera

- Mutar `ephemeral-cache-purger` / `purge_sandbox_cache.rs` / `VETO_PREFIXES`.
- Dependencia `tracing`. Unidad `sddia-daemon@event-watcher`.
- Silenciar `log_route_outcome` / println one-shot de despacho.
- Plantilla `sddia-email-watcher@.service.template`.
- Aplicar journald/logrotate en el host (operador).

## Ley aplicada

- Git vía `skill:git-manager`. Troncal `main`.
- DA-2/DA-4: topología `objectives.md` en rama antes de mutar código. `directories.daemons` no está en la tabla DA-2; A-NO-TRACING-DEPENDENCY rige el crate.
- `features-documentation-pattern` v1.2.1: un PR; `validacion.md` APTO solo con CA-CI verde.
- `CONSTITUTION_CORE` Filtro A: no declarar Done sobre diffs locales.

## Criterios (PBI v1.2.0)

KA-DISK-1…KA-DISK-8. KA-DISK-8 = cierre documental + CI `run_id`.
