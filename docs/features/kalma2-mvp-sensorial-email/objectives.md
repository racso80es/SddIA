---
feature_name: kalma2-mvp-sensorial-email
created: "2026-08-18"
process: feature
branch_name: feat/kalma2-mvp-sensorial-email
persist_ref: docs/features/kalma2-mvp-sensorial-email
pbi_ref: docs/todos/pending/[OPERATIVO] Kalma2 MVP 01A — Circuito sensorial de correo (Paciente 0).md
document_id: PBI-KALMA2-MVP-01A
uuid: "c209c150-8ab4-4f0d-bcf7-8fa7a6101de0"
parent_pbi: PBI-KALMA2-MVP-01
parent_uuid: "d7d00838-9ee6-472f-a164-95dcba2ceb80"
dossier_ref: docs/features/kalma2-mvp-paciente-0
execution_id: "fa4dde03-a0ec-426f-ade7-850246ba7575"
---

# Objetivos — kalma2-mvp-sensorial-email

## Misión

Cerrar el circuito **correo → `Email_Received` → veredicto `Email_Triaged` → WUI** sin intervención en terminal. La ley del triaje vive fuera del Core (norma + códice). Ola A de `PBI-KALMA2-MVP-01`.

## Alcance

| Incluye | Excluye |
|---------|---------|
| T0–T5 + T9a (gates G0–G5, G9a) | T6–T8, G6–G8, G9b (`PBI-KALMA2-MVP-01B`) |
| Forja: norma, códice, 2 ECST, centinela, proceso empacado, skill agenda | Tubería `asset:fetch` / `sync-client-assets` / WUI sync |

SSOT de genoma: `docs/features/kalma2-mvp-paciente-0/spec.md`. No se reabre. UUIDs allí = reservas; prevalece forja.

## Ley aplicada

- `features-documentation-pattern` v1.2.1
- `codex-software-engineering` (ciclo feature)
- DA-2/DA-3: entidades vía `entity-manager` (daemon: `daemon-creator`; no hay clase daemon en el gestor)
- DA-4: topología 01A activa antes de mutar genoma
- DA-5: post-acuse CLI, cero polling
- G4 ceguera lógica; G5 peaje termodinámico; IMAP read-only
