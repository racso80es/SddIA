---
document_id: PBI-KALMA2-MVP-01A
uuid: "c209c150-8ab4-4f0d-bcf7-8fa7a6101de0"
title: "[OPERATIVO] Kalma2 MVP 01A — Circuito sensorial de correo (Paciente 0)"
format: markdown
version: "1.2.0"
status: done
priority: alta
process: feature
parent_pbi: PBI-KALMA2-MVP-01
feature_slug: kalma2-mvp-sensorial-email
persist_ref: docs/features/kalma2-mvp-sensorial-email
dossier_ref: docs/features/kalma2-mvp-paciente-0
spec_ref: docs/features/kalma2-mvp-paciente-0/spec.md
plan_ref: docs/features/kalma2-mvp-paciente-0/plan.md
clarify_ref: docs/features/kalma2-mvp-paciente-0/clarify.md
phases: "T0-topologia,T1-ratificacion-ssot,T2-ley-y-codice,T3-eventos,T4-centinela,T5-triaje,T9a-aduana-sensorial,T10-post-auditoria"
created: "2026-08-17"
updated: "2026-08-18"
---

# Kalma2 MVP 01A — Circuito sensorial de correo

Primera ola de `PBI-KALMA2-MVP-01`. Cierra el circuito completo **correo → veredicto → WUI** con la ley del triaje fuera del Core.

La especificación del genoma no se duplica aquí: reside en `spec.md` del dossier compartido. Este documento fija alcance, gates y Done de la ola.

## Ampliaciones post-auditoría (2026-08-18)

Hallazgos del lab IMAP en Paciente 0. Incorporados en la misma ola 01A.

| ID | Hallazgo | Resolución | Estado |
|----|----------|------------|--------|
| **A-01** | Primer arranque con `UID SEARCH ALL` backfilla todo el INBOX | Primer sondeo limitado a **60 días** (`SDDIA_EMAIL_INITIAL_LOOKBACK_DAYS`, IMAP `SINCE`). Histórico anterior: ola futura bajo demanda del usuario | Implementado |
| **A-02** | Lock huérfano tras `--once` interrumpido | `DaemonRuntime::bootstrap` recupera lock si PID muerto; `_run_daemon.sh` limpia antes de arrancar; cleanup en `start-sddia.sh` | Implementado |
| **A-03** | E2E automático requiere daemons en loop | `start-sddia.sh` levanta `event-watcher` + `email-watcher` (condicional IMAP) + Kalma2 | Implementado |
| **A-04** | Secretos IMAP en bóveda global | Credenciales en `{instancia}/.SddIA/.dev/.env` (`env_hierarchy.instance`) | Hecho (manual) |
| **A-05** | `email-watcher` ausente de ignición unificada | `start-sddia.sh` + `start-sddia.md` v1.2.2 + launcher `SddIA/scripts/daemons/email-watcher.sh` | Implementado |
| **A-06** | Catch-up UID bloquea correo nuevo en UID alto | `UNSEEN` prioritario + `SDDIA_EMAIL_MAX_UIDS_PER_POLL` (50) + watermark contiguo + skip si `.eml` existe | Implementado |

## Alcance

| Fase | Entrega | Referencia |
|------|---------|------------|
| T0 | Topología documental + rama | `plan.md` T0 |
| T1 | `codex-contract` v1.2.0 (R-01) + `process_domain_roots` (R-02) | `spec.md` §9.3, §9.4 |
| T2 | Norma `email-triage-matrix` + códice `codex-kalma2-assistant` con bloque `dlt` | `spec.md` §5, §6 |
| T3 | Clases ECST `Email_Received` y `Email_Triaged` + suscripción | `spec.md` §3, §9.2 |
| T4 | Centinela `email-watcher` (IMAP read-only) + template systemd | `spec.md` §4 |
| T5 | Proceso empacado `email-triage-gateway` + skill `agenda-manager` | `spec.md` §7, §8.4 |
| T9a | Aduana sensorial | `plan.md` T9 |
| T10 | Post-auditoría IMAP (A-01…A-06) | Esta sección |

**Fuera de alcance:** toda la tubería de sincronización de activos (`PBI-KALMA2-MVP-01B`); backfill de histórico de correo anterior a la ventana inicial de 60 días (ola futura).

## Entidades a forjar

`email-triage-matrix`, `codex-kalma2-assistant`, `email-received`, `email-triaged`, `email-watcher` (+ cápsula Rust), `email-triage-gateway`, `agenda-manager`, plantilla `sddia-email-watcher@.service.template`.

Todas vía `execute-process --process entity-manager`. Los UUID de `spec.md` son reservas; prevalece el emitido por la forja.

## Criterios de aceptación

- [x] **Ventana inicial (A-01):** primer sondeo limitado a 60 días (`SINCE`); arranques incrementales por UID; sin `ALL`.
- [x] **Ignición unificada (A-05):** `./start-sddia.sh` arranca `email-watcher` cuando `SDDIA_EMAIL_IMAP_HOST` está en bóveda.
- [x] **Bóveda instancia (A-04):** credenciales IMAP solo en `.SddIA/.dev/.env`.
- [x] **Trazabilidad sin fugas:** correo entrante → `Email_Received` → veredicto → visible en la WUI (`5e7e24e0-…` lab).
- [x] **Ceguera lógica (G4):** la cápsula del Centinela no contiene invocación de `execute-process`, ni lectura bajo `SddIA/`, ni comando IMAP de escritura, ni ruta absoluta del host.
- [x] **Ceguera espacial:** único acoplamiento `WorkingDirectory=%f`; cero rutas de cliente en `SddIA/`.
- [x] **No destructividad:** cero operaciones de escritura sobre el buzón.
- [x] **Peaje termodinámico (G5):** correo con cabeceras de lista resuelto con `decision_path: deterministic` y coste en ceros; el `execution_report` prueba que la fase de clasificación LLM no se ejecutó.
- [x] **Blindaje antiverbosidad:** correo comercial verboso no obtiene veredicto `actionable` (unit).
- [x] **Privacidad del bus:** el payload porta `body_ref`, nunca el cuerpo íntegro.
- [ ] **Resiliencia:** `SIGKILL` ⇒ resurrección en <5 s; el servicio sobrevive al bloqueo de sesión (template systemd; lab OS defer).
- [x] **Idempotencia:** reinicio del Centinela sin reemisión de correo ya procesado (watermark + skip `.eml`).
- [x] **Identidad de Activo (G2):** códice con `canonical_hash == hash_signature`, `mint_status: pre-mint`, `token_id: null` y `composition[]` resoluble.
- [x] **Cicatriz Digital:** toda entidad con `uuid` v4, SemVer, `contract`, `hash_signature` y fila en su `index.md`.
- [x] **Soberanía de interacción:** el prompt de Kalma2 coexiste con el flujo de correo en background.

## Done

Un único PR mergeado en `main`, con `validacion.md` en `global: APTO` y `pbi_archived: true`, y este PBI en `docs/todos/done/` en la misma rama (`task-closure-documental`).
