---






feature_name: aut-111-sigterm-orphan-lock
created: "2026-11-23"
updated: "2026-11-23"
process: feature
purpose: Clarificaciones y laudos Mayeuta — AUT-111 SIGTERM/orphan-lock en centinelas y anti-falso-positivo F-ARGOS-ORPHAN-LOCK
branch_name: feat/aut-111-sigterm-orphan-lock
persist_ref: docs/features/aut-111-sigterm-orphan-lock
pbi_ref: AUT-111
document_id: PBI-FIX-CONSOLIDATED-ORPHAN-LOCK-SIGTERM
status: requisitos-estabilizados
phase: mayeuta-stabilization
agents: mayeuta
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
dor_profile: bug-fix
source_refine: .SddIA/workspaces/refine-item/6081e9da-416f-4ff8-a29d-e3a41d4e8bb5
execution_id: "a5f95b61-ab09-4659-8554-403502c4a239"
---
# Clarificación — aut-111-sigterm-orphan-lock

Transcript Mayeuta (Estabilización de Requisitos). Fuente: `item.refined.md` / `verdict.json` del refine `6081e9da-…` (`verdict: refined`, sin preguntas abiertas).

## D0 — Semilla

El `2026-11-22` (18:12Z–18:56Z) hubo **9** incidentes `F-ARGOS-ORPHAN-LOCK` (`AUT-100`…`AUT-108`) en **4 olas** de `systemctl --user restart` sobre centinelas con `KillSignal=SIGTERM`. Tras cada ola: mensaje `[<daemon>] lock huérfano pid=…; recuperando` y emisión de `System_Fracture_Detected`.

**AUT-111** consolida ese linaje (`document_id: PBI-FIX-CONSOLIDATED-ORPHAN-LOCK-SIGTERM`, labels Pbi+Bug, DoR `bug-fix`). Este ciclo se ejecuta vía proceso orquestado `feature` / rama `feat/aut-111-sigterm-orphan-lock` (contexto de entrega del runtime); el perfil DoR del ítem permanece `bug-fix`.

## D1 — Misión (qué)

Un reinicio controlado de la flota de centinelas **no** debe dejar `.SddIA/daemons/status/<daemon>.lock` con PID muerto ni disparar falsos positivos `F-ARGOS-ORPHAN-LOCK` / `System_Fracture_Detected` desde `daemon-heartbeat-audit`, cumpliendo `daemons-contract.md` §5 / §6.3.

## D2 — Laudos

| Ref | Pregunta | Laudo |
|-----|----------|-------|
| **L-ROOT** | ¿Causa raíz? | **Dual:** (1) shutdown SIGTERM incompleto → lock huérfano; (2) `audit_running_daemon` emite fractura en el primer muestreo con `!pid_alive(pid)` sin gracia ante reinicio en caliente. `lock_predates_host_boot` **no** cubre reinicios post-`btime`. |
| **L-SCOPE** | ¿Alcance? | **D-1/D-2:** cumplimiento shutdown SIGTERM+lock en `telegram-watcher`, `iota-publish-relay`, `github-bridge-watcher`, `email-watcher`, `event-watcher`, `kalma2-bridge` **y** anti-falso-positivo en `daemon-heartbeat-audit`. Homogeneizar aunque no todos aparezcan en las 9 fracturas. |
| **L-HOW** | ¿Runtime compartido vs `ctrlc`+`termination`? | **D-3:** decisión Dedalo/Tekton. El **qué** exige: interceptar SIGTERM (y SIGINT) → shutdown limpio → borrar `.lock` antes de salir; heartbeat `status: "shutting_down"` cuando el contrato lo permita. |
| **L-GRACE** | ¿Gracia en auditoría? | **D-4:** requisito de comportamiento — no emitir `F-ARGOS-ORPHAN-LOCK` en el primer muestreo instantáneo durante reinicio controlado. Duración/mecanismo = diseño; no bloquea DoR. |
| **L-OUT** | ¿Fuera de alcance? | **D-5:** discriminar por `btime` (ya existe); rediseño Argos fuera de orphan-lock; centinelas no listados. |
| **L-PROCESS** | ¿`feature` vs `bug-fix`? | Persistencia bajo `docs/features/…` y rama `feat/…` según manifiesto del ciclo en curso. DoR y tipología del PBI = `bug-fix` / fix consolidado. No reabrir refine. |

## D3 — Criterios observables

| ID | Criterio |
|----|----------|
| **CA-1** | Tras `systemctl --user restart` de cada centinela en alcance: termina limpio, `.lock` sin PID muerto, sin mensaje `lock huérfano pid=…; recuperando` en arranque inmediato. |
| **CA-2** | Reinicio controlado de flota: `daemon-heartbeat-audit` no emite `System_Fracture_Detected` / `F-ARGOS-ORPHAN-LOCK` por PID muerto transitorio en el primer muestreo; solo tras gracia/re-chequeo si el orphan persiste. |
| **CA-3** | Tests: (a) shutdown SIGTERM con borrado de lock; (b) ventana de gracia/confirmación en `daemon_heartbeat.rs`. |
| **CA-4** | Caso en `fracture-corpus` si aplica; `validacion.md` Argos APTO; `AUT-111` cerrado en Linear (`state_map.done`). |

## D4 — Fuera de alcance

- Rediseño general de Argos / nuevas firmas de fractura ajenas a orphan-lock.
- Discriminación adicional por `btime` del host.
- Centinelas fuera de la lista D-2.
- Reabrir o re-forjar `AUT-100`…`AUT-108` (permanecen como evidencia consolidada).
