---
feature_name: evolution-history-normalization
created: "2026-08-14"
purpose: Estabilización EV-AUD-002/007 — migrar históricos al contrato vigente y extraer borradores
process: refactorization
phase: mayeuta-stabilization
agents: mayeuta
branch_name: refactor/evolution-history-normalization
persist_ref: docs/features/evolution-history-normalization
pbi_ref: docs/todos/pending/[REFACTOR] Evolution — migrar históricos y extraer borradores (EV-AUD-002-007).md
document_id: 7bb37ff1-decd-4ec5-968b-344a5334f9eb
correlation_id: 4b9de6b3-c400-49c8-86f2-55f08ec64ce4
source_audit: docs/audits/evolution/2026-08-11.md
findings:
  - EV-AUD-002
  - EV-AUD-007
depends_on:
  - 4feb4ea2-b1ca-41c6-bc57-75457840eabf
dependency_status:
  4feb4ea2-b1ca-41c6-bc57-75457840eabf: closed
blocks:
  - 70f78d23-e209-4e41-9292-cb7421a934f6
---

# Clarificación — evolution-history-normalization

Transcript Mayeuta. Estabiliza el **qué** y el **por qué** de EV-AUD-002/007 antes de blueprint Dedalo. Sin diseño de cápsulas, lotes de código ni YAML de proceso.

## D0 — Semilla y evidencia

| Vector | Hecho |
|--------|--------|
| Hallazgos | EV-AUD-002 (0/61 conformidad atómica completa; 9 UUID v4 inválidos) + EV-AUD-007 (2 borradores en universo oficial). |
| Contrato+índice | PBI `4feb4ea2-…` **cerrado**. Contrato v1.1.1 en `normative_documents.evolution_contract`. Índice en `normative_documents.evolution_log` (corte 61 + nota de delta). |
| Validador | `sddia-qa validate-evolution-contract` (lectura). Gate `sddia-qa gate-evolution` ya existe en modo **delta** (PBI `70f78d23-…` archivado); **no** certifica el universo legacy. |
| `persist_ref` inyectado | Vacío. Resuelto por Cúmulo: `paths.featurePath` + kebab del PBI → `docs/features/evolution-history-normalization`. |
| Rama | Input de proceso y PBI `suggested_branch`: `refactor/evolution-history-normalization`. Stub orquestador usó `feat/…`. **Laudo L-BRANCH:** la rama canónica es `refactor/…`. |
| Tres esquemas (corte) | **INV-A** (`uuid`/`date`/`created`/`type`); **INV-L** (`contrato_version`/`id_cambio`/`fecha`/`tipo_operacion` no canónico o hash vacío); **NOMBRE/UUID-INV** (filename no UUID v4 y/o identidad inválida). |
| Borradores | `entity-manager-eda-propuesta-analisis-temp.md`, `emit-domain-mutation-analisis-temp.md` (`tipo: analisis-temporal`, `estado: borrador`, clase `BORRADOR`). |
| SIN_FECHA | `migracion-execute-process-rust-p14-p15.md`. |
| UUID-INV oficiales (9, excl. borradores) | 7× NOMBRE + `e1f2a3b4-…` (variante RFC inválida) + `c8f2a1b3-…` (versión ≠ 4). |
| Delta post-corte no indexado (notas EV-AUD-001) | `0c19403d-…`, `83bbfdeb-…`, `a7c3e91f-2b4d-4e8a-…`. Alta canónica posterior ya en índice: `0bceeb41-…` (gate). |

## D1 — Misión (qué / por qué)

| Decisión | Laudo |
|----------|--------|
| Objetivo | Normalizar **todos** los registros oficiales bajo `directories.evolution` al esquema canónico del contrato vigente, **sin perder** identidad usable, fecha evidenciada, cuerpo Markdown ni trazabilidad de origen. |
| Por qué ahora | El contrato v1.1.1 clasifica legado y **prohíbe reescritura por el contrato mismo**. Este PBI es la **excepción autorizada** de mutación. Hasta cerrarlo, la auditoría periódica no puede declarar conformidad formal completa y el universo oficial sigue mezclado. |
| Efecto observable | 100 % de registros oficiales = `CANONICO`; 0 borradores en `directories.evolution`; índice completo; migrador idempotente. |

## D2 — Precondición (L-DEP)

| Vector | Laudo |
|--------|--------|
| Contrato+índice `4feb4ea2-…` | **Satisfactorio** (cerrado). Migración **desbloqueada**. |
| Gate `70f78d23-…` | Archivado en modo delta. Este ciclo **no** reabre el gate ni cambia reason-codes. Sí habilita que una auditoría periódica posterior reporte conformidad formal del universo oficial. |

## D3 — Manifiesto reversible (L-MANIFEST)

| Decisión | Laudo |
|----------|--------|
| Obligatorio | Antes de cualquier renombre o reescritura: manifiesto determinista `old_path → {new_path, id_cambio, lote, acción}`. |
| Reversible | Toda fila tiene inverso `new_path → old_path`. Prohibida sustitución masiva sin este mapa. |
| UUID nuevos | Solo si ni filename ni `id_cambio`/`uuid` son UUID v4 RFC 4122. El v4 se **genera una vez** y queda **congelado en el manifiesto**. Segunda ejecución **lee** el manifiesto; no regenera. |
| Identidad conservada | Filename UUID v4 válido → `id_cambio`. Frontmatter UUID v4 válido + filename no UUID → renombrar al UUID existente (p. ej. `event-bus-audit-process.md` → `8d577a50-…`). Conflicto entre dos UUID v4 válidos → ambas en el mapa; Dedalo elige con regla explícita y deja alias. |
| Alias | Cada renombre deja rastro auditable: campo de origen en el registro canónico **y** fila en el manifiesto global. El mapa **no** vive como fichero no-UUID bajo `directories.evolution` (recontaminaría el universo). Territorio: `directories.documentation` (tarea y/o `paths.auditsPath`). |

## D4 — Lotes = tres esquemas + extracción

| Lote | Universo | Acción |
|------|----------|--------|
| L1 INV-A | Cabecera atómica parcial | Mapear alias → canónico; filename `{id_cambio}.md`; calcular `hash_integrity`; `tipo_operacion` según D6. Cuerpo intacto. |
| L2 INV-L | Proto-contrato | Enum → canónico; hash vacío → hash real del payload canónico; `contrato_version` vigente. Cuerpo intacto. |
| L3 NOMBRE/UUID-INV | Identidad rota | Reparar UUID (D3); renombrar; completar canónico. Incluye SIN_FECHA (D5). |
| L4 BORRADOR | EV-AUD-007 | Extraer (D7). No convertir en registro oficial. |

Cada lote exige pruebas propias **antes** del siguiente. CANONICO ya válido (p. ej. `0bceeb41-…`): **no reescribir** cuerpo ni hash; solo asegurar fila de índice.

Delta post-corte no indexado entra en el lote que le corresponda por esquema y **debe** quedar indexado al cierre.

## D5 — Fechas (L-DATE)

| Caso | Laudo |
|------|--------|
| `fecha` / `date` / `created` presente y parseable | Conservar valor; normalizar representación a `YYYY-MM-DD` o ISO-8601 ya válido. No alterar el día evidenciado. |
| Ausente o inválida (SIN_FECHA) | Recuperar de evidencia git (primera aparición versionada del path). Registrar la derivación en manifiesto + `relacionado`. |
| Sin evidencia | **Bloquear el ítem**; prohibido inventar timestamp. |

## D6 — Tipología (L-TIPO)

Contrato §3 (migración futura) se ejecuta aquí.

| Origen | Destino |
|--------|---------|
| `alta` / `baja` / `modificacion` | Identidad. |
| `feature`, `bug-fix`, `refactorizacion`, `feature-milestone`, `process`, `evolucion-proceso`, `type` atómico (`corrective-milestone`, …) | **`modificacion`** por defecto. |
| Cuerpo/frontmatter evidencian retirada de artefactos | `baja` (+ `rutas_eliminadas` / `commit_referencia_previo` si existen). |
| `analisis-temporal` / `estado: borrador` | No migrar a enum oficial → L4. |

Prohibido inferir `alta` por heurística de novedad. En duda: `modificacion`.

## D7 — Borradores (L-DRAFT)

| Decisión | Laudo |
|----------|--------|
| Destino | Subárbol de `directories.documentation` **fuera** de `directories.evolution`. Ancla Cúmulo existente: `paths.auditsPath` (p. ej. bajo `docs/audits/evolution/…`). |
| Prohibido | Nueva clave en `cumulo.paths.json`. Usar `paths.evolutionPath` (la norma sync la cita; **no existe** en Cúmulo 1.6.1). Formalizar borradores como registros `CANONICO`. |
| Efecto | 0 archivos `*-temp*` / clase `BORRADOR` en `directories.evolution`. Filas oficiales del índice las eliminan. El manifiesto guarda `old_path → ruta documental`. Contenido preservado. |
| Referencias internas | Actualizar punteros que apunten a las rutas viejas (incl. referencias cruzadas entre los dos borradores y menciones en docs/índice). |

## D8 — Índice, hashes, idempotencia, auditoría

| Vector | Laudo |
|--------|--------|
| `Evolution_log.md` | Una fila por **cada** registro oficial presente tras L1–L4 (corte + delta + altas canónicas − borradores). Cero huérfanos, cero duplicados `id_cambio`, cero colisiones de path. `clase_formato: CANONICO` en oficiales. Cabecera alineada a `contrato_version` vigente. |
| Hash | Obligatorio canónico. Se **calcula** del payload (frontmatter sin el campo + cuerpo, UTF-8, LF) **después** de normalizar. Prohibido copiar hashes vacíos o inventar hex. |
| Idempotencia | Segunda ejecución del migrador sobre el mismo manifiesto → **diff vacío** (registros + índice + mapa). |
| Auditoría periódica | Tras el PR, `validate-evolution-contract` sobre el universo oficial reporta conformidad formal completa (0 INV-A/INV-L/NOMBRE/UUID-INV/BORRADOR/SIN_FECHA en oficiales). |
| Contrato | Actualizar §3: la cláusula «históricos no se reescriben» deja de describir el universo oficial post-migración. La tabla de alias permanece como arqueología / lectura del mapa, no como permiso para dejar no-canónicos. |

## D9 — Conservación de contenido y referencias

| Conservar | Mutar |
|-----------|--------|
| Cuerpo Markdown (sentido y texto) | Frontmatter → esquema canónico v1.1.1. |
| Fecha evidenciada, UUID v4 válido | Filename → `{id_cambio}.md`. |
| Relación a PBI/artefactos/commits | `relacionado` canónico (unión de alias `related_entities` / `artefactos_afectados`). |
| Trazabilidad de origen | Alias en registro + manifiesto. |

Actualizar referencias internas (docs, índice, punteros entre registros) a los nuevos paths/UUID. No reescribir historia narrativa.

## D10 — Límites duros

| Prohibido en este ciclo |
|-------------------------|
| Sustitución masiva sin manifiesto reversible y pruebas por lote. |
| Inventar fechas, UUIDs (fuera del v4 congelado en manifiesto) o hashes. |
| Mutar `cumulo.paths.json` o índices soberanos de genoma. |
| Reabrir diseño del gate evolution / reason-codes. |
| Convertir `*-analisis-temp.md` en registros oficiales. |
| Alterar el significado del cuerpo Markdown. |
| Dejar entradas oficiales sin índice o índice con filas huérfanas. |
| Colocar el mapa de redirección como fichero no-UUID bajo `directories.evolution`. |

## D11 — Entrega y Done

| Vector | Laudo |
|--------|--------|
| PR único | Migración + extracción + índice + mapa + pruebas de lote/idempotencia + cascada documental + PBI archivado en la **misma** rama/PR. |
| Git | Solo `skill:git-manager`. Rama `refactor/evolution-history-normalization`. |
| Done | `features-documentation-pattern` v1.2.x: `validacion.md` APTO, `pbi_archived: true`, PBI en `docs/todos/done/` pre-merge. |

## Handoff Dedalo

1. Especificar formato del manifiesto (campos, ancla bajo `directories.documentation`, freeze de UUID).
2. Definir reglas de mapeo L1–L3 sobre contrato v1.1.1 (alias, enum D6, hash, filename).
3. Inventariar universo **actual** (corte 61 + delta post-corte + CANONICO posteriores − L4), no solo el informe 2026-08-11.
4. Diseñar extracción L4 hacia `paths.auditsPath` y actualización de referencias.
5. Definir batería: pruebas por lote + segunda pasada diff vacío + validador/auditoría de conformidad oficial.
6. Emitir `spec.md` + `plan.md`. No ejecutar la migración en esta fase.
