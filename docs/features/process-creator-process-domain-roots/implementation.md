---
feature_name: process-creator-process-domain-roots
created: "2026-08-10"
updated: "2026-08-10"
process: feature
branch_name: feat/process-creator-process-domain-roots
persist_ref: docs/features/process-creator-process-domain-roots
pbi_ref: docs/todos/pending/[ARQUITECTURA] process-creator — jurisdicción process_domain_roots (ABSTRACT-03 D7).md
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-03-D7-PROCESS-CREATOR
parent_document_id: PBI-SDDIA-DOMAIN-ABSTRACT-03
laudo: L-JURIS-MEMBERSHIP-PLUS-FLAG
phase: T0-T5
status: implemented
agents: tekton
---

# Implementation — process-creator-process-domain-roots

## T0 — Forja multi-root (factory)

| Path | Cambio |
|------|--------|
| `SddIA/engine/execute-process/src/forges/factory.rs` | `run_process_forge`: `load_paths_config` + classifier D1–D2; L-UNIQ-MULTI; L-INDEX-TARGET; update vía `resolve_process_path` / locate multi-root; outputs `resolved_process_root` + `process_jurisdiction_applied` |
| tests en mismo módulo | `ac_juris_*`, `ac_uniq_*`, `ac_smoke_*` + fixture Cúmulo en updates previos |

Classifier operativo:

```text
process_jurisdiction=domain | (ausente ∧ name ∈ process_membership SE)
  → dest = process_domain_roots[0] (o process_domain_root si multi)
else → dest = directories.process
```

## T1 — Genoma creator

| Path | Cambio |
|------|--------|
| `SddIA/process/process-creator.md` | **v1.2.0**: inputs `process_jurisdiction` / `process_domain_root`; outputs root resuelto; fases unicidad multi-root + índice destino |

Mutación bajo Raw Kernel DA-4: topología `docs/features/process-creator-process-domain-roots/` activa. `hash_signature` sellado `sha256:0fb74ad8b5b561f18292ce2648aa03f98aa969e64a94ea47caf1379b810b911b` (`sddia-qa recalc-process-hash-signatures --write`).

## T2 — Contrato / normas / evolution

| Path | Cambio |
|------|--------|
| `SddIA/process/process-contract.md` | Unicidad `aliases`/`name` → catálogo multi-root (unión roots) |
| `SddIA/norms/external-ai-constraints.md` | Fila `process_domain_roots`: escritura creators alineada |
| `SddIA/evolution/a3c7e91f-2b4d-4f8a-9c1e-7d6b0a5f3211.md` | Hito D7 + uuid creator `7c2d9e41-…` |

## AC-OVERLAY

**N/A (nuevo esquema).** Escritura reusa `load_paths_config` (Cúmulo + `.SddIA/local.paths.json`). Si instancia declara `directories.process_domain_roots`, **reemplaza** el array Core (contrato heredado ABSTRACT-03 §3.3).

## Fuera de alcance (AC-NONSCOPE)

Sin re-move de los 6 packing; sin EM/daemons/routes; sin residual Kalma2 Shell/git-manager.
