---
feature_name: evolution-registry-gate
created: "2026-08-13"
purpose: Estabilización EV-AUD-001/002 — gate automático de registro y coherencia evolution
branch_name: feat/evolution-registry-gate
persist_ref: docs/features/evolution-registry-gate
pbi_ref: docs/todos/pending/[FEATURE] Evolution — gate automático de registro y coherencia (EV-AUD-001-002).md
document_id: 70f78d23-e209-4e41-9292-cb7421a934f6
phase: mayeuta-stabilization
agents: mayeuta
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
execution_id: 0bceeb41-64d1-4920-af9d-46a11c0455a2
---

# Clarificación — evolution-registry-gate

Transcript Mayeuta. Estabiliza el **qué** y el **por qué** del gate evolution antes de blueprint Dedalo. Sin diseño de cápsulas ni YAML de proceso.

## D0 — Semilla y evidencia

| Vector | Hecho |
|--------|--------|
| Hallazgos | EV-AUD-001 (SSOT ausente) + EV-AUD-002 (0/61 conformidad atómica completa). |
| Contrato+índice | PBI `4feb4ea2-…` en `docs/todos/done/`; `validacion.md` APTO; existen `normative_documents.evolution_contract` y `evolution_log`. |
| Validador lectura | `sddia-qa validate-evolution-contract` (solo lectura; **no** gate bloqueante en contrato v1.1.0). |
| Migración histórica | PBI `7bb37ff1-…` **sigue en pending** (EV-AUD-002/007). |
| Norma | `sddia-evolution-sync`: alta/baja/modificación material bajo `./SddIA/` exige detalle `{id_cambio}.md` + fila en índice; registro vía binario Rust `paths.skillCapsules.sddia-evolution-register`. |
| Problema residual | La obligación existe; **no** hay aduana verificable que falle commit/PR ante diff material sin evolution correlacionada o registro inválido. |

## D1 — Misión termodinámica (qué)

| Decisión | Laudo |
|----------|--------|
| Objetivo | Convertir la trazabilidad evolution en **regla automática, determinista y reproducible**. |
| Efecto observable | Ante cambio material bajo el árbol gobernado por Cúmulo (`./SddIA/` y exclusiones contractuales), el sistema **registra** o **rechaza** con código estable y diagnóstico estructurado; no depende de disciplina humana ni de bypass de IA obrera. |

## D2 — Precondición dura de dependencia (L-DEP)

| Opción | Veredicto |
|--------|-----------|
| Activar gate bloqueante con históricos no migrados | **Rechazada** — baseline fallaría de forma masiva (falsos positivos / bloqueo total del flujo). El PBI exige contrato+índice **y** migración cerrados. |
| Estabilizar requisitos ahora; activación bloqueante solo tras Done de `7bb37ff1-…` | **Adoptada** — `L-DEP`. |

| Vector | Laudo |
|--------|--------|
| Contrato+índice `4feb4ea2-…` | **Satisfactorio** (cerrado). |
| Migración `7bb37ff1-…` | **Abierta** → precondición dura para **encendido bloqueante** (pre-commit + CI fail). |
| Esta fase Mayeuta | Puede cerrar con requisito estable; Dedalo puede blueprint; **no** declarar Done del gate ni flip a fail-hard hasta migración cerrada. |
| Soft-mode / dry-run | Permitido en diseño posterior **solo** como andamiaje de pruebas; no sustituye el AC de fallo bloqueante. |

## D3 — Capacidades exigidas (sin blueprint)

| Capacidad | Obligación |
|-----------|------------|
| Registro atómico | Alta/actualización válida materializa **detalle + índice** de forma atómica (ambos o ninguno). |
| Validación contractual | Comprobar contrato, índice, UUID v4, fecha, hash según contrato vigente (`evolution_contract` v1.1+). |
| Correlación diff↔evolution | Detectar diff **material** bajo el árbol `./SddIA/` sin entrada evolution correlacionada → fallo. |
| Integridad de registro | Registro inválido, no indexado o hash inválido → fallo **antes** de commit/PR. |
| Aduanas | CLI `sddia-qa` es la Aduana Universal. Pre-commit = detonador inerte. **Prohibido** bypass para IA obrera. |
| Diagnóstico | Sobre `capsule-json-io` (machine-readable) con razón accionable y códigos estables. |
| Rutas | Exclusivamente vía Cúmulo (`directories.*`, `normative_documents.*`, `execution_capsules.skills`, `compiled_capsules`). |

## D4 — Semántica de “cambio material” y exclusiones (L-MATERIAL / L-EXCL)

| Decisión | Laudo |
|----------|--------|
| Material | Alteración de contenido versionado bajo el árbol normativo `./SddIA/` que no esté en el conjunto de exclusión contractual. |
| Exclusiones | **Solo** las declaradas por el contrato evolution (y norma sync) — no inventar listas ad hoc en agentes. Dedalo debe anclar exclusiones al SSOT contractual. |
| Auto-registro (L-SELF) | Diff limitado a artefactos del protocolo bajo `directories.evolution` (detalle del cambio + índice + contrato cuando el propio cambio sea el registro) **no** debe exigir un segundo registro correlacionado en el mismo commit → **cero falsos positivos** al tocar solo `directories.evolution`. |
| Fuera de alcance de exclusión | Docs de producto (`paths.featurePath` / `docs/evolution` producto) no son `directories.evolution`; no se confunden (norma sync §2). |

## D5 — Códigos de fallo estables (L-CODES)

| Decisión | Laudo |
|----------|--------|
| Requisito | Fallos del gate emiten **código de salida / reason-code estable** documentado (p. ej. material-sin-evolution, registro-inválido, no-indexado, hash-inválido, duplicado). |
| Prohibido | Mensajes solo narrativos sin código; códigos efímeros por commit. |
| Diseño concreto de enum | Jurisdicción Dedalo/Tekton; Mayeuta fija la obligación de estabilidad. |

## D6 — Matriz de pruebas mínima (L-TESTS)

Obligatoria cobertura de: **alta**, **modificación**, **baja**, **duplicado**, **hash inválido**, **ejecución idempotente**. Además: “solo `directories.evolution`” (no FP), “diff material sin evolution” (sí fallo), **veredicto con `diff`/`registry` inyectados (sin Git en cápsula)**, **hook inerte**.

## D7 — Límites duros

| Prohibido en este ciclo |
|-------------------------|
| Sustituir o reabrir la migración física de históricos (`7bb37ff1-…`) dentro de este PBI. |
| Activar fail-hard en pre-commit/CI sobre baseline no migrado. |
| Bypass de aduana para IA obrera / hooks `--no-verify` como solución. |
| Rutas literales hardcodeadas fuera de Cúmulo. |
| Mutar genoma indexado por forja manual; alta de cápsula/skill vía `entity-manager` cuando el blueprint lo exija. |
| Confundir validador lectura existente con el gate bloqueante (el primero **no** cumple este PBI). |
| Cápsula WASI calculando diff Git o leyendo el working tree. |
| Hook de pre-commit con inventario de paths, ephemeral de diff o lógica de cotejo. |

## D8 — Entrega y Done

| Vector | Laudo |
|--------|--------|
| PR único | Cápsula/registro + validación gate + hooks/CI + tests + cascada documental + PBI archivado en la **misma** rama/PR. |
| Done | `features-documentation-pattern` v1.2.x: `validacion.md` APTO, `pbi_archived: true`, PBI en `docs/todos/done/` pre-merge. |
| Orden de backlog | Contrato (hecho) → migración (pendiente) → **este gate**. |

## D9 — Inyección desacoplada y detonador inerte (refino 2026-08-13)

| Vector | Laudo |
|--------|--------|
| **L-INJECT** | El árbol (diff material) lo captura el **CLI nativo**. Se inyecta en `request.diff` (+ `request.registry`) por stdin. La cápsula **coteja JSON contra JSON** y emite veredicto. |
| **L-WASI-DOMAIN** | `sddia-evolution-register` = `wasm32-wasip1`. Sin Git, sin cálculo de diff. Persistencia host = CLI nativo aplicando el JSON de alta/baja/modificación. |
| **L-HOOK-INERT** | Pre-commit: única función = invocar `sddia-qa`. Abort iff `success: false` ∧ `exitCode > 0`. Cero lógica de dominio en el hook. |
| **L-CLI-ARGOS** | `sddia-qa gate-evolution` es la chispa de Argos: captura, inyección WASI, eco del sobre, código de salida. CI usa el mismo CLI. |

## Handoff Dedalo

1. Consumir el cuerpo de `objectives.md` como `refined_requirements`.
2. Especificar contrato I/O de `sddia-evolution-register` (JSON stdin/stdout) y CLI `sddia-qa` como orquestador nativo (sin confundir con validate-evolution-contract solo-lectura).
3. Definir correlación diff↔entrada evolution y tabla de exclusiones anclada al contrato.
4. Reason-codes estables. Wiring: CLI captura+inyecta; hook inerte; CI = mismo CLI.
5. Marcar en `spec.md`/`plan.md` el **hold** `L-DEP`: fail-hard canónico de universo solo tras cierre de `7bb37ff1-…`.
6. **Refino operador:** L-INJECT / L-WASI-DOMAIN / L-HOOK-INERT anulan cápsula nativa y hook recolector.
