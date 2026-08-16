---
document_id: 4f7ff349-c25c-4365-a6b1-73798528b1d8
title: process-creator — materialización contractual completa
type: bug-fix
status: cerrado
closed: "2026-08-16"
priority: critical
created: "2026-08-11"
process: bug-fix
persist_ref: docs/fixes/process-creator-full-contract-forge
fix_ref: docs/fixes/process-creator-full-contract-forge
branch: fix/process-creator-full-contract-forge
source_audit: docs/audits/evolution/2026-08-11.md
findings:
  - EV-AUD-003
---

# process-creator — materialización contractual completa

## Problema

`process-creator` declara que materializa inputs, outputs, workspace y fases, pero `run_process_forge` genera un stub con una única `Fase inicial`. El hash se calcula sobre fases que no coinciden con el artefacto escrito.

## Resolución

CREATE serializa el payload contractual y sella `hash_signature` con `sha256_phases_integrity` sobre las fases escritas (paridad `verify-process-integrity`). `entity-manager` propaga workspace/inputs/outputs/invocations. Fixture `evolution-audit` recreable. PR #178.

## Criterios de aceptación

- [x] CREATE conserva exactamente el payload contractual solicitado.
- [x] El proceso recién forjado supera `verify-process-integrity` sin parche manual.
- [x] Índice y frontmatter coinciden en UUID, versión, contexto y descripción.
- [x] Un fallo de materialización no puede devolver éxito ni handoff parcial.
- [x] Tests cubren Core, domain, DI, inputs/outputs complejos e idempotencia.
- [x] `evolution-audit` puede recrearse en fixture sin laudo de excepción.
