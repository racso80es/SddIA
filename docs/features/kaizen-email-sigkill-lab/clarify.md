---
feature_name: kaizen-email-sigkill-lab
created: "2026-08-19"
process: feature
phase: Estabilización de Requisitos
agents: mayeuta
branch_name: feat/kaizen-email-sigkill-lab
persist_ref: docs/features/kaizen-email-sigkill-lab
document_id: PBI-KAIZEN-EMAIL-SIGKILL-01A
uuid: "a3f7c812-1e45-4b09-95d1-6e820f4dc301"
version: "1.0.0"
type: feature
parent_pbi: PBI-KALMA2-MVP-01A
parent_persist_ref: docs/features/kalma2-mvp-sensorial-email
kaizen_phase: Cosecha Kaizen
correlation_id: "2XyNciPL7yiQuKGFY77qJASEEBjTP572gFt1VjK2HQVY"
status: stabilized
mayeuta_verdict: ok
open_decisions: 0
ratification_required: 0
---

# Clarificación Mayeuta — kaizen-email-sigkill-lab

## D0. Apertura

Kaizen operativo de una sola ola. No reabre requisitos del padre (`PBI-KALMA2-MVP-01A`): los laudos D-01…D-13 y las ratificaciones R-01/R-02 son vinculantes y no se revisan aquí.

El perímetro es exclusivamente de laboratorio: instalar, ejecutar, medir y registrar evidencia. Cero forja de genoma nuevo.

Dossier padre (SSOT de contexto): `docs/features/kalma2-mvp-sensorial-email/{clarify,execution}.md`.

## D1. Herencia vinculante

Del padre `PBI-KALMA2-MVP-01A`:

- Arquitectura sensorial (T0–T5) finalizada y mergeada.
- `email-watcher` compilado y publicado en `feat/kalma2-mvp-sensorial-email`.
- Template systemd presente en `SddIA/templates/systemd/sddia-email-watcher@.service.template`.
- `daemon-heartbeat-audit` operativo (proceso indexado en Core).
- T9a declaró dos ítems DEFER por ausencia de `SDDIA_EMAIL_IMAP_HOST` en el host de CI.

## D2. Fricción única

### F-01 — Dependencia de entorno real

Ambos DEFER (heartbeat vivo y SIGKILL) requieren `SDDIA_EMAIL_IMAP_HOST` configurado en `.SddIA/.dev/.env` del host de lab. Sin esa variable el `email-watcher` no inicia y el ensayo systemd no tiene sentido.

**Resolución:** la ejecución es bloqueante hasta que Racso confirme el entorno lab activo. Una vez activo, la secuencia es:

1. `./start-sddia.sh` → verificar ≥3 `Daemon_Heartbeat` con `./sddia-run.sh --process daemon-heartbeat-audit --inputs '{"sweep":true}'`
2. `systemctl --user enable sddia-email-watcher@$(pwd)` (con template instalado) → `kill -SIGKILL <pid>` → `systemctl --user status` en <5 s

No se bloquea este `clarify.md`; se bloquea la fase de ejecución (Tekton/Argos).

## D3. Perímetro

| Tarea | Entrega |
|-------|---------|
| Lab-01 | Heartbeat ≥3 ciclos sin fractura (`daemon-heartbeat-audit sweep`) |
| Lab-02 | Template systemd instalado + SIGKILL absorbido <5 s |
| Lab-03 | `execution.md` en `docs/features/kaizen-email-sigkill-lab/` con evidencia raw |
| Cierre | `validacion.md` APTO + PBI a `docs/todos/done/` |

**Fuera de alcance:** forja de entidades, cambios en buses de eventos, modificación de cápsulas, merge a `main` sin PR.

## D4. Preguntas abiertas

Ninguna. La semilla es precisa; el único gate es disponibilidad del entorno lab (F-01). Decisión de desbloqueo: Racso.

## D5. Veredicto

Estabilizado. Cero preguntas de requisito pendientes. Handoff a Tekton: ejecutar Lab-01 y Lab-02 cuando el entorno lab esté activo; registrar en `execution.md`.
