---
document_id: PBI-KALMA2-MVP-01A
uuid: "c209c150-8ab4-4f0d-bcf7-8fa7a6101de0"
title: "[OPERATIVO] Kalma2 MVP 01A — Circuito sensorial de correo (Paciente 0)"
format: markdown
version: "1.0.0"
status: pendiente
priority: alta
process: feature
parent_pbi: PBI-KALMA2-MVP-01
feature_slug: kalma2-mvp-sensorial-email
persist_ref: docs/features/kalma2-mvp-sensorial-email
dossier_ref: docs/features/kalma2-mvp-paciente-0
spec_ref: docs/features/kalma2-mvp-paciente-0/spec.md
plan_ref: docs/features/kalma2-mvp-paciente-0/plan.md
clarify_ref: docs/features/kalma2-mvp-paciente-0/clarify.md
phases: "T0-topologia,T1-ratificacion-ssot,T2-ley-y-codice,T3-eventos,T4-centinela,T5-triaje,T9a-aduana-sensorial"
created: "2026-08-17"
---

# Kalma2 MVP 01A — Circuito sensorial de correo

Primera ola de `PBI-KALMA2-MVP-01`. Cierra el circuito completo **correo → veredicto → WUI** con la ley del triaje fuera del Core.

La especificación del genoma no se duplica aquí: reside en `spec.md` del dossier compartido. Este documento fija alcance, gates y Done de la ola.

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

**Fuera de alcance:** toda la tubería de sincronización de activos (`PBI-KALMA2-MVP-01B`).

## Entidades a forjar

`email-triage-matrix`, `codex-kalma2-assistant`, `email-received`, `email-triaged`, `email-watcher` (+ cápsula Rust), `email-triage-gateway`, `agenda-manager`, plantilla `sddia-email-watcher@.service.template`.

Todas vía `execute-process --process entity-manager`. Los UUID de `spec.md` son reservas; prevalece el emitido por la forja.

## Criterios de aceptación

- [ ] **Trazabilidad sin fugas:** correo entrante → `Email_Received` → veredicto → visible en la WUI, sin intervención en terminal.
- [ ] **Ceguera lógica (G4):** la cápsula del Centinela no contiene invocación de `execute-process`, ni lectura bajo `SddIA/`, ni comando IMAP de escritura, ni ruta absoluta del host.
- [ ] **Ceguera espacial:** único acoplamiento `WorkingDirectory=%f`; cero rutas de cliente en `SddIA/`.
- [ ] **No destructividad:** cero operaciones de escritura sobre el buzón.
- [ ] **Peaje termodinámico (G5):** correo con cabeceras de lista resuelto con `decision_path: deterministic` y coste en ceros; el `execution_report` prueba que la fase de clasificación LLM no se ejecutó.
- [ ] **Blindaje antiverbosidad:** correo comercial verboso no obtiene veredicto `actionable`.
- [ ] **Privacidad del bus:** el payload porta `body_ref`, nunca el cuerpo íntegro.
- [ ] **Resiliencia:** `SIGKILL` ⇒ resurrección en <5 s; el servicio sobrevive al bloqueo de sesión.
- [ ] **Idempotencia:** reinicio del Centinela sin reemisión de correo ya procesado.
- [ ] **Identidad de Activo (G2):** códice con `canonical_hash == hash_signature`, `mint_status: pre-mint`, `token_id: null` y `composition[]` resoluble.
- [ ] **Cicatriz Digital:** toda entidad con `uuid` v4, SemVer, `contract`, `hash_signature` y fila en su `index.md`.
- [ ] **Soberanía de interacción:** el prompt de Kalma2 coexiste con el flujo de correo en background.

## Done

Un único PR mergeado en `main`, con `validacion.md` en `global: APTO` y `pbi_archived: true`, y este PBI movido a `docs/todos/done/` en la misma rama (`task-closure-documental`).
