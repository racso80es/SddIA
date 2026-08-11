---
document_id: 4f7ff349-c25c-4365-a6b1-73798528b1d8
title: process-creator — materialización contractual completa
type: bug-fix
status: pending
priority: critical
created: "2026-08-11"
suggested_branch: fix/process-creator-full-contract-forge
source_audit: docs/audits/evolution/2026-08-11.md
findings:
  - EV-AUD-003
---

# process-creator — materialización contractual completa

## Problema

`process-creator` declara que materializa inputs, outputs, workspace y fases, pero `run_process_forge` genera un stub con una única `Fase inicial`. El hash se calcula sobre fases que no coinciden con el artefacto escrito.

## Objetivo

Hacer que la forja nativa produzca exactamente el proceso solicitado y elimine la necesidad de remediación manual post-creator.

## Alcance

1. Persistir `workspace_template`, `inputs`, `outputs`, `phases`, `requires_capability`, `delegates_to` y aliases recibidos.
2. Calcular `hash_signature` sobre las fases efectivamente escritas.
3. Validar contrato antes de actualizar el índice o emitir handoff.
4. Mantener jurisdicción Core/domain y unicidad multi-root.
5. Alinear `process-creator.md`, factory y `entity-manager`.

## Criterios de aceptación

- CREATE conserva exactamente el payload contractual solicitado.
- El proceso recién forjado supera `verify-process-integrity` sin parche manual.
- Índice y frontmatter coinciden en UUID, versión, contexto y descripción.
- Un fallo de materialización no puede devolver éxito ni handoff parcial.
- Tests cubren Core, domain, DI, inputs/outputs complejos e idempotencia.
- `evolution-audit` puede recrearse en fixture sin laudo de excepción.
