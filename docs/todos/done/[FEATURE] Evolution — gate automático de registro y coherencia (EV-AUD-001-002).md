---
document_id: 70f78d23-e209-4e41-9292-cb7421a934f6
title: Evolution — gate automático de registro y coherencia
type: feature
status: done
priority: high
created: "2026-08-11"
refined: "2026-08-13"
closed: "2026-08-13"
suggested_branch: feat/evolution-registry-gate
source_audit: docs/audits/evolution/2026-08-11.md
findings:
  - EV-AUD-001
  - EV-AUD-002
depends_on:
  - 4feb4ea2-b1ca-41c6-bc57-75457840eabf
  - 7bb37ff1-decd-4ec5-968b-344a5334f9eb
constraints:
  - inject-diff-json
  - hook-inert-detonator
  - capsule-wasi-domain-only
persist_ref: docs/features/evolution-registry-gate
execution_id: 0bceeb41-64d1-4920-af9d-46a11c0455a2
---

# Evolution — gate automático de registro y coherencia

## Problema

La norma exige registrar cambios materiales bajo `SddIA/`, pero no existe una aduana verificable que garantice contrato, índice y correlación con el diff.

## Objetivo

Convertir la trazabilidad evolution en una regla automática, determinista y reproducible.

## Alcance

1. Implementar o restaurar la cápsula Rust **WASI** `sddia-evolution-register` (lógica de dominio pura).
2. Añadir validación `sddia-qa` (Aduana Universal / CLI nativo) de contrato, índice, UUID, fecha y hash.
3. Detectar diff material bajo `SddIA/` sin entrada evolution correlacionada.
4. Integrar el gate en pre-commit y CI sin bypass para IA obrera.
5. Excluir explícitamente artefactos definidos por contrato.
6. Emitir diagnóstico estructurado y accionable (sobre `capsule-json-io` v2.0).

## Especificación de desacoplamiento (2026-08-13)

### Inyección desacoplada del diff

La cápsula `sddia-evolution-register` es **WASI**. **Prohibido** que calcule el diff de Git, invoque `git` o lea el working tree.

El **orquestador soberano nativo** (CLI Rust con acceso al SO: `sddia-qa` / Aduana Universal) captura el estado del árbol (diff staged o `base...HEAD`) y lo **inyecta** como JSON puro en el `request` del envelope `capsule-json-io.md` vía **stdin** de la cápsula, junto con el JSON del registro evolution (detalle + índice) ya resuelto vía Cúmulo.

La cápsula **solo** aplica lógica de dominio: cotejar el JSON del diff material contra el JSON del registro evolution y emitir el veredicto en el sobre de respuesta (`success`, `exitCode`, `reason_code`).

Persistencia atómica de detalle/índice (si `operation` es alta/modificación/baja): la cápsula calcula el nuevo estado como JSON; el CLI nativo aplica el write en el host. La cápsula no es el peón de Git ni el de syscall de árbol.

### Hook como detonador inerte

El hook de pre-commit es **únicamente** el despertador inerte:

1. Invocar la Aduana Universal (`sddia-qa`), chispa que despierta a Argos para la validación.
2. **Prohibido** en el hook: calcular diff, listar paths, escribir ephemeral de inventario, ramificar lógica de negocio.
3. Abortar el commit **solo** si el CLI retorna sobre JSON con `success: false` **y** `exitCode > 0` (coherente con `capsule-json-io`: `exitCode === 0` ⟺ `success === true`).

CI invoca el mismo CLI; no duplica la lógica de la cápsula.

## Criterios de aceptación

- Alta/actualización válida actualiza detalle e índice de forma atómica (aplicación nativa del JSON emitido por la cápsula).
- Un cambio material sin evolution falla con código estable.
- Un registro inválido o no indexado falla antes del commit/PR.
- No hay falsos positivos al modificar únicamente `directories.evolution`.
- Tests cubren alta, modificación, baja, duplicado, hash inválido, ejecución idempotente, **veredicto con diff inyectado** (sin Git dentro de la cápsula) e **hook inerte**.
- El gate consume rutas exclusivamente desde Cúmulo.
- La cápsula WASI no contiene llamadas a Git ni cálculo de diff; el árbol llega solo por stdin JSON.
- El hook de pre-commit no contiene lógica de dominio ni inventario de paths; aborta solo ante `success: false` ∧ `exitCode > 0`.

## Dependencias

Requiere contrato, índice y migración histórica cerrados para evitar bloquear el baseline (fail-hard canónico del universo). El gate sobre **delta inyectado** no certifica los 61 históricos.

## Cierre

Archivado en rama `feat/evolution-registry-gate`. `validacion.md` APTO. Universo 61 sigue fuera del fail-hard (PBI `7bb37ff1-…` abierto).
