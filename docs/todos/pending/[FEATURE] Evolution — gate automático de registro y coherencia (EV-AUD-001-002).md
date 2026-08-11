---
document_id: 70f78d23-e209-4e41-9292-cb7421a934f6
title: Evolution — gate automático de registro y coherencia
type: feature
status: pending
priority: high
created: "2026-08-11"
suggested_branch: feat/evolution-registry-gate
source_audit: docs/audits/evolution/2026-08-11.md
findings:
  - EV-AUD-001
  - EV-AUD-002
depends_on:
  - 4feb4ea2-b1ca-41c6-bc57-75457840eabf
  - 7bb37ff1-decd-4ec5-968b-344a5334f9eb
---

# Evolution — gate automático de registro y coherencia

## Problema

La norma exige registrar cambios materiales bajo `SddIA/`, pero no existe una aduana verificable que garantice contrato, índice y correlación con el diff.

## Objetivo

Convertir la trazabilidad evolution en una regla automática, determinista y reproducible.

## Alcance

1. Implementar o restaurar la cápsula Rust `sddia-evolution-register`.
2. Añadir validación `sddia-qa` de contrato, índice, UUID, fecha y hash.
3. Detectar diff material bajo `SddIA/` sin entrada evolution correlacionada.
4. Integrar el gate en pre-commit y CI sin bypass para IA obrera.
5. Excluir explícitamente artefactos definidos por contrato.
6. Emitir diagnóstico estructurado y accionable.

## Criterios de aceptación

- Alta/actualización válida actualiza detalle e índice de forma atómica.
- Un cambio material sin evolution falla con código estable.
- Un registro inválido o no indexado falla antes del commit/PR.
- No hay falsos positivos al modificar únicamente `directories.evolution`.
- Tests cubren alta, modificación, baja, duplicado, hash inválido y ejecución idempotente.
- El gate consume rutas exclusivamente desde Cúmulo.

## Dependencias

Requiere contrato, índice y migración histórica cerrados para evitar bloquear el baseline.
