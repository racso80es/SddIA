---






feature_name: aut-111-sigterm-orphan-lock
created: "2026-11-23"
updated: "2026-11-23"
process: feature
purpose: Objetivos estabilizados — SIGTERM limpio en centinelas y anti-falso-positivo F-ARGOS-ORPHAN-LOCK
branch_name: feat/aut-111-sigterm-orphan-lock
persist_ref: docs/features/aut-111-sigterm-orphan-lock
pbi_ref: AUT-111
document_id: PBI-FIX-CONSOLIDATED-ORPHAN-LOCK-SIGTERM
status: requisitos-estabilizados
dor_profile: bug-fix
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
execution_id: "a5f95b61-ab09-4659-8554-403502c4a239"
---
# Objetivos — aut-111-sigterm-orphan-lock

## Misión

Garantizar que un reinicio controlado de la flota de centinelas (`systemctl --user restart` / `KillSignal=SIGTERM`) **no** deje `.SddIA/daemons/status/<daemon>.lock` huérfanos ni dispare falsos positivos `F-ARGOS-ORPHAN-LOCK` / `System_Fracture_Detected` desde `daemon-heartbeat-audit`, cumpliendo `daemons-contract.md` §5 / §6.3.

Tracker **AUT-111** (`document_id: PBI-FIX-CONSOLIDATED-ORPHAN-LOCK-SIGTERM`; consolidación de `AUT-100`…`AUT-108`).

## Alcance

1. **Shutdown limpio SIGTERM** en: `telegram-watcher`, `iota-publish-relay`, `github-bridge-watcher`, `email-watcher`, `event-watcher`, `kalma2-bridge` — handler efectivo → shutdown → eliminación de `.lock` (y heartbeat `shutting_down` cuando el contrato lo permita).
2. **Ventana de gracia / confirmación** en `daemon-heartbeat-audit` ante `!pid_alive(pid)` para no emitir falso positivo en reinicios controlados.
3. **Pruebas** de shutdown por SIGTERM y de la gracia/confirmación en auditoría.
4. **Corpus / cierre:** caso en `fracture-corpus` si aplica; `validacion.md` Argos APTO; cierre Linear de `AUT-111`.

## Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| **CA-1** | Tras restart de cada centinela en alcance: sin `.lock` con PID muerto y sin mensaje `lock huérfano pid=…; recuperando` en arranque inmediato. | Lab `systemctl --user restart` + logs/status lock. |
| **CA-2** | Reinicio controlado de flota: no emitir `System_Fracture_Detected` / `F-ARGOS-ORPHAN-LOCK` por PID muerto transitorio en el primer muestreo. | Observación bus/audit + test de gracia. |
| **CA-3** | Tests unitarios/integración: (a) SIGTERM → borrado de lock; (b) gracia/confirmación en `daemon_heartbeat.rs`. | `cargo test` en crates afectados. |
| **CA-4** | Caso en `fracture-corpus` si aplica; `validacion.md` APTO; `AUT-111` en `state_map.done`. | Argos + Linear. |

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
- `features-documentation-pattern` v1.3.3 — un `.md` por fase; frontmatter YAML obligatorio.
- Norma: `SddIA/daemons/daemons-contract.md` (§5 lock, §6.1 `shutting_down`, §6.3 SIGTERM).
- Materialización piloto solo vía MCP `sddia-workspace-server`.

## Dependencias

- Módulos: `SddIA/sddia-daemon-runtime`, centinelas listados, `SddIA/engine/execute-process/src/engine/handlers/daemon_heartbeat.rs`, unidades `.SddIA/systemd/sddia-*.service`.
- Sin PBI bloqueante externo; linaje `AUT-100`…`AUT-108` = evidencia consolidada (no reabrir).
