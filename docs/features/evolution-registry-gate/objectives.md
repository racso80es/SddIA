---
feature_name: evolution-registry-gate
created: "2026-08-13"
process: feature
branch_name: feat/evolution-registry-gate
persist_ref: docs/features/evolution-registry-gate
pbi_ref: docs/todos/pending/[FEATURE] Evolution — gate automático de registro y coherencia (EV-AUD-001-002).md
document_id: 70f78d23-e209-4e41-9292-cb7421a934f6
source_audit: docs/audits/evolution/2026-08-11.md
findings:
  - EV-AUD-001
  - EV-AUD-002
depends_on:
  - 4feb4ea2-b1ca-41c6-bc57-75457840eabf
  - 7bb37ff1-decd-4ec5-968b-344a5334f9eb
dependency_status:
  4feb4ea2-b1ca-41c6-bc57-75457840eabf: closed
  7bb37ff1-decd-4ec5-968b-344a5334f9eb: open
phase: blueprint
execution_id: 0bceeb41-64d1-4920-af9d-46a11c0455a2
---

# Objetivos — evolution-registry-gate

## Objetivo

Convertir la trazabilidad evolution en una **regla automática, determinista y reproducible**: todo cambio material bajo el árbol gobernado `./SddIA/` queda registrado (detalle + índice) o es **rechazado** por aduana (pre-commit y CI) con código estable y diagnóstico estructurado, sin bypass para IA obrera.

## Alcance

1. Implementar la cápsula Rust **WASI** `sddia-evolution-register` (lógica de dominio pura; resolución vía `execution_capsules.skills` + `compiled_capsules.wasm_root`).
2. CLI nativo `sddia-qa` (Aduana Universal): captura el árbol, inyecta JSON `capsule-json-io` por stdin, persiste writes; distinto de `validate-evolution-contract` (solo lectura).
3. Detectar diff material bajo `./SddIA/` sin entrada evolution correlacionada (**cotejo en la cápsula**, no en el hook).
4. Pre-commit = detonador inerte del CLI; CI invoca el mismo CLI; sin bypass para IA obrera.
5. Excluir únicamente artefactos definidos por el contrato / norma sync; garantizar cero falsos positivos al tocar solo `directories.evolution` (L-SELF).
6. Emitir diagnóstico estructurado (sobre JSON) con reason-codes estables.
7. Tests: alta, modificación, baja, duplicado, hash inválido, idempotencia, veredicto con diff inyectado, hook inerte.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-ATOMIC | Alta/actualización válida actualiza detalle e índice de forma atómica (ambos o ninguno). |
| AC-MATERIAL | Un cambio material sin evolution correlacionada falla con código estable. |
| AC-INVALID | Un registro inválido o no indexado falla antes del commit/PR. |
| AC-SELF | Cero falsos positivos al modificar únicamente `directories.evolution` (auto-registro / L-SELF). |
| AC-TESTS | Tests cubren alta, modificación, baja, duplicado, hash inválido, idempotencia, veredicto con diff inyectado y hook inerte. |
| AC-CUMULO | El gate consume rutas exclusivamente desde Cúmulo. |
| AC-ADUANA | Gate activo vía CLI en pre-commit y CI; sin bypass para IA obrera. |
| AC-INJECT | Diff y registro evolution llegan a la cápsula solo por stdin JSON (`capsule-json-io`). Cero Git / cero cálculo de diff dentro de WASI. |
| AC-HOOK-INERT | Hook solo invoca `sddia-qa`; aborta iff sobre `success: false` ∧ `exitCode > 0`. Sin inventario de paths ni lógica de dominio en el hook. |
| AC-WASI | Artefacto de dominio = `wasm32-wasip1`. El nativo (CLI) posee OS/Git; la cápsula no. |
| AC-DIAG | Diagnóstico estructurado machine-readable con reason-code estable. |
| AC-DEP | Fail-hard sobre baseline solo tras cierre de migración `7bb37ff1-…` (L-DEP). |
| AC-PR | Cascada documental + PBI archivado en rama según patrón v1.2.x. |

## Dependencias

| PBI | Estado | Rol |
|-----|--------|-----|
| `4feb4ea2-…` contrato+índice | **Cerrado** | SSOT contractual disponible. |
| `7bb37ff1-…` migración histórica | **Abierto** | Precondición dura para activación bloqueante (L-DEP). |

## Fuera de alcance

- Normalización física / renombre de históricos y extracción de borradores (PBI `7bb37ff1-…`).
- Sustituir el validador solo-lectura del corte de 61 por este gate (complementa; no lo borra).
- Retirada de claves Cúmulo `evolution_contract` / `evolution_log`.
- Confundir `directories.evolution` (ecosistema) con `paths.evolutionPath` / docs de producto.

## Restricciones

- Captura de árbol: CLI nativo (Aduana Universal). Cápsula WASI: **prohibido** Git.
- Mutación de genoma indexado solo vía `entity-manager` / cápsulas autorizadas.
- Prohibido inventar fechas, UUIDs o hashes.
- Prohibido activar fail-hard sobre baseline no migrado.
- Prohibido lógica de dominio o diff en el hook de pre-commit.
- El cuerpo de este documento es el `refined_requirements` de entrada a Dedalo.

## Ley aplicada

- `features-documentation-pattern` (frontmatter + un `.md` por fase).
- `SddIA/norms/sddia-evolution-sync.md` (obligación de registro; cápsula Rust).
- `SddIA/evolution/evolution_contract.md` v1.1.0 (esquema, hash, índice).
- `SddIA/norms/paths-via-cumulo.md` / `cumulo.paths.json`.
- `SddIA/norms/external-ai-constraints.md` (soberanía; sin bypass de aduana).
- `SddIA/norms/capsule-json-io.md` (E/S JSON de cápsulas).
