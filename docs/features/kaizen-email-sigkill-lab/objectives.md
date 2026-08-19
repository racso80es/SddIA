---
feature_name: kaizen-email-sigkill-lab
created: "2026-08-19"
process: feature
phase: Estabilización de Requisitos
agents: mayeuta
branch_name: feat/kaizen-email-sigkill-lab
persist_ref: docs/features/kaizen-email-sigkill-lab
pbi_ref: "docs/todos/pending/[OPERATIVO] email-watcher — validación SIGKILL systemd lab (kalma2-mvp-sensorial-email).md"
document_id: PBI-KAIZEN-EMAIL-SIGKILL-01A
uuid: "a3f7c812-1e45-4b09-95d1-6e820f4dc301"
version: "1.0.0"
type: feature
parent_pbi: PBI-KALMA2-MVP-01A
parent_uuid: "c209c150-8ab4-4f0d-bcf7-8fa7a6101de0"
parent_branch: feat/kalma2-mvp-sensorial-email
parent_persist_ref: docs/features/kalma2-mvp-sensorial-email
kaizen_phase: Cosecha Kaizen
correlation_id: "2XyNciPL7yiQuKGFY77qJASEEBjTP572gFt1VjK2HQVY"
mayeuta_verdict: pending
status: stabilizing
---

# Objetivos — kaizen-email-sigkill-lab

## Misión

Cerrar la deuda **DEFER T9a** de `PBI-KALMA2-MVP-01A`: validar en entorno lab real (con `SDDIA_EMAIL_IMAP_HOST` configurado) que el centinela `email-watcher` sostiene latido continuo sin fractura y que el template systemd absorbe un SIGKILL y recupera el servicio en menos de 5 segundos.

## Alcance

| Incluye | Excluye |
|---------|---------|
| Lab heartbeat: ≥3 ciclos `Daemon_Heartbeat` sin fractura | Forja de nuevas entidades Core |
| Instalación template `sddia-email-watcher@.service` | Cambios en `codex-contract` o eventos |
| Validación SIGKILL → recuperación <5 s | Ola T6–T8 de `kalma2-mvp-sensorial-email` |
| Registro evidencia en `execution.md` de este `persist_ref` | Push / merge a `main` |

## Artefactos upstream ya presentes

- Template: `SddIA/templates/systemd/sddia-email-watcher@.service.template` (`Restart=always`, `RestartSec=5`, `WorkingDirectory=%f`)
- Centinela: `email-watcher` forjado en `feat/kalma2-mvp-sensorial-email`
- Proceso de auditoría: `daemon-heartbeat-audit` (sweep disponible)

## Criterio de Done

1. `Daemon_Heartbeat` emitido ≥3 veces consecutivas sin `fractures_emitted`
2. SIGKILL absorbido y servicio activo en <5 s (evidencia `systemctl status`)
3. `execution.md` en este `persist_ref` con resultados
4. PBI movido a `docs/todos/done/` y `validacion.md` con `global: APTO`

## Ley aplicada

- `features-documentation-pattern` v1.2.1
- DA-4: topología activa antes de mutar genoma
- DA-5: post-acuse CLI, cero polling
- Git exclusivamente vía `skill:git-manager`
