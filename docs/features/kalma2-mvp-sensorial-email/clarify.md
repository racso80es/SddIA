---
feature_name: kalma2-mvp-sensorial-email
created: "2026-08-18"
process: feature
purpose: "Mayeuta 01A — confirmación de ola; laudos del dossier padre vinculantes"
phase: Estabilización de Requisitos
agents: mayeuta
branch_name: feat/kalma2-mvp-sensorial-email
persist_ref: docs/features/kalma2-mvp-sensorial-email
document_id: PBI-KALMA2-MVP-01A
uuid: "c209c150-8ab4-4f0d-bcf7-8fa7a6101de0"
parent_pbi: PBI-KALMA2-MVP-01
dossier_ref: docs/features/kalma2-mvp-paciente-0
status: stabilized
mayeuta_verdict: ok
open_decisions: 0
ratification_required: 0
ratification_inherited: "R-01 R-02 (2026-08-17)"
execution_id: "fa4dde03-a0ec-426f-ade7-850246ba7575"
---

# Clarificación Mayeuta — Ola A (circuito sensorial)

## D0. Apertura

Semilla operativa, no v0. El padre (`PBI-KALMA2-MVP-01`) ya pasó Filtro Antientrópico: 13 fisuras, 13 decisiones, R-01/R-02 concedidas. Esta ola **no reabre** laudos. Solo acota el perímetro ejecutable y dos fricciones de forja.

Dossier padre (SSOT): `docs/features/kalma2-mvp-paciente-0/{clarify,spec,plan}.md`.

## D1. Herencia vinculante

D-01…D-13 del padre aplican. Para 01A son operativos: D-01, D-02, D-04, D-05, D-06, D-07, D-08, D-09, D-11, D-12. Diferidos a 01B: D-03, D-13.

R-01 y R-02: concedidas. T1 es ejecución material, no nueva ratificación.

## D2. Fricciones de ola (no de requisito)

### F-01 — `entity-manager` sin clase `daemon`

El PBI ordena «todas las entidades vía `entity-manager`». El gestor no declara `entity_class: daemon`. `daemon-creator` está indexado en Core.

**Resolución:** T4 via `./sddia-run.sh --process daemon-creator`. No se inventa clase en el gestor. Resto (norm, codex, event, process, skill) sí vía `entity-manager`.

### F-02 — Contrato de familia sin creator

`codex-contract.md` no es `domain-codex`; está excluido del índice. No hay `contract-creator`.

**Resolución:** T1 R-01 = mutación in-ciclo del contrato (precedente normas Core sin creator) + T9 evolution. Prohibido forjarlo como códice.

### F-03 — Genoma no se duplica

D5.1 del padre: la especificación de entidades no se copia por ola. `spec.md` de 01A es perímetro, gates y punteros a §§ del dossier.

## D3. Perímetro 01A

| Fase | Entrega |
|------|---------|
| T0 | Topología `persist_ref` 01A + rama `feat/kalma2-mvp-sensorial-email` |
| T1 | `codex-contract` v1.2.0 (R-01) + `process_domain_roots` (R-02) |
| T2 | `email-triage-matrix` + `codex-kalma2-assistant` (`dlt` pre-mint) |
| T3 | `email-received`, `email-triaged` + suscripción |
| T4 | `email-watcher` + cápsula IMAP RO + template systemd |
| T5 | `email-triage-gateway` empacado + `agenda-manager` + binding `agenda:persist` |
| T9a | Aduana sensorial (bus, heartbeat, resiliencia, idempotencia, e2e) |

**Fuera:** T6–T8, `asset:fetch`, `github-raw-fetcher`, `POST /api/sync-assets`, botón WUI sync.

## D4. Veredicto

Estabilizado. Cero preguntas abiertas. Handoff a Dédalo: blueprint de ola, sin reescribir el genoma padre.
