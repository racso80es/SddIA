---
feature_name: kaizen-capsula-imap-triaje
created: "2026-08-19"
process: feature
branch_name: feat/kaizen-capsula-imap-triaje
persist_ref: docs/features/kaizen-capsula-imap-triaje
pbi_ref: docs/todos/pending/PBI-KAIZEN-CAPSULA-IMAP-TRIAJE.md
document_id: PBI-KAIZEN-CAPSULA-IMAP-TRIAJE
uuid: "9c25bb52-57a4-4ede-be43-41388a7576b2"
execution_id: "14fff213-bcee-4c26-ad17-53e5e585979b"
depends_on:
  - docs/features/kalma2-mvp-sensorial-email
adjacent_not_merged:
  - docs/todos/pending/PBI-globalizacion-consciencia-grafo.md
mayeuta_verdict: ok
dedalo_verdict: ok
laudo: no-dualidad-email-triaged
---

# Objetivos — kaizen-capsula-imap-triaje

## Misión

Elevar el **Guante IMAP** (centinela `email-watcher`, Paciente Cero) a activo de Grado S+ auditable —fallos de red/MIME encapsulados, sin panic— y cerrar la **primera línea de valor humano**: cuando el veredicto ya existente es `actionable`, el Vértice Biológico recibe un umbral interactivo (resumen + acciones rápidas) en Kalma2 y/o Telegram eferente, sin alerta plana ni dualidad de clases ECST.

## Punto objetivo

> **O-IMAP-TRIAJE:** El centinela permanece ciego y read-only; la ley sigue en `email-triage-matrix` + `email-triage-gateway`; el hueco de 01A (*Email_Triaged sin suscriptores / solo poll de status*) se cierra con elevación humana selectiva (`verdict=actionable`) y retorno de intención desde la UI, sin mutar el buzón desde el watcher.

## Alcance

| Dentro | Fuera |
|--------|-------|
| Resiliencia termodinámica del crate/daemon `email-watcher` (IMAP down, parse MIME, contrato JSON-io) | Reabrir G4/G5 / `kalma2-mvp-sensorial-email` |
| Fan-out humano de `Email_Triaged` con `verdict=actionable` | Clase `Actionable_Email_Detected` por default |
| Payload de umbral: resumen táctico + acciones rápidas como **evento de retorno** | Alojar la matriz en el centinela |
| Canal Kalma2 (WUI/bridge) y/o `send-telegram-notification` | Handler de correo en `telegram-watcher` |
| Ruido: constancia `Email_Triaged` + silencio hacia canales humanos | Hábitos/grafo (`PBI-ARQ-CONSCIENCIA-UNIVERSAL`) |
| | IMAP STORE/archivar real desde el centinela; SMTP de borrador en este ciclo salvo gap Dedalo |

## Objetivos medibles

| ID | Objetivo | Criterio |
|----|----------|----------|
| **O1** | Guante | Fallo IMAP/MIME no mata el proceso; salida alineada a `capsule-json-io` (`success:false`, `exitCode` ≠ 0) |
| **O2** | Ceguera | Watcher no clasifica, no lee la matriz, no orquesta |
| **O3** | No dualidad | Cero clase ECST nueva salvo laudo Dedalo documentado contra D1 |
| **O4** | Silencio de ruido | `noise`/`passive` no disparan notificación Kalma2/Telegram |
| **O5** | Elevación | `actionable` produce umbral enriquecido (resumen + botones) en canal humano |
| **O6** | Retorno | Clic de acción emite estímulo de retorno sin salir de la plataforma; no muta IMAP vía watcher |
| **O7** | Identidad | Suscriptores usan nombres canónicos (`kalma2-interact`, tool eferente Telegram); no `kalma2_interact_core` ni el daemon watcher |

## No objetivos

- Fusionar con el Grafo de Pensamiento Universal.
- Convertir `telegram-watcher` en consumidor de dominio de correo.
- Reescribir `email-triage-matrix` salvo gap normativo demostrado.
- Sustituir `GET /api/status` como veredicto terminal (PBI-044).

## Ley aplicada

- `features-documentation-pattern` v1.2.1 / proceso `feature` v1.3.2
- `capsule-json-io` v2.0
- `email-triage-matrix` v1.0.0 (ley fuera del Core; prohibido en cápsula del Centinela)
- G4 Paciente 0: IMAP read-only, ceguera lógica, sin `execute-process` en el watcher
- DA-2/DA-3: genoma vía `entity-manager`; DA-4 topología activa; DA-5 fire-and-forget
- Clarificaciones D0–D8 en `clarify.md` (laudo **no-dualidad-email-triaged**)
