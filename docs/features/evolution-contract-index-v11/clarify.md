---
feature_name: evolution-contract-index-v11
created: "2026-08-11"
purpose: Estabilización EV-AUD-001 — restaurar contrato e índice canónico evolution sin mutar históricos
branch_name: feat/evolution-contract-index-v11
persist_ref: docs/features/evolution-contract-index-v11
pbi_ref: docs/todos/pending/[ARQUITECTURA] Evolution — restaurar contrato e índice canónico (EV-AUD-001).md
document_id: 4feb4ea2-b1ca-41c6-bc57-75457840eabf
execution_id: c906d516-f708-48bc-87b3-19980a9a11ab
phase: mayeuta-stabilization
agents: mayeuta
source_audit: docs/audits/evolution/2026-08-11.md
findings:
  - EV-AUD-001
blocks:
  - 7bb37ff1-decd-4ec5-968b-344a5334f9eb
  - 70f78d23-e209-4e41-9292-cb7421a934f6
---

# Clarificación — evolution-contract-index-v11

Transcript Mayeuta. Estabiliza el **qué** y el **por qué** de EV-AUD-001 antes de blueprint Dedalo.

## D0 — Semilla y evidencia

| Vector | Hecho |
|--------|--------|
| Hallazgo | EV-AUD-001 crítico: Cúmulo declara `normative_documents.evolution_contract` y `evolution_log` apuntando a archivos ausentes. |
| Rutas Cúmulo | `SddIA/evolution/evolution_contract.md` · `SddIA/evolution/Evolution_log.md` (`cumulo.paths.json` 1.6.1). |
| Norma vigente | `SddIA/norms/sddia-evolution-sync.md` ya exige contrato **v1.1**, filename `{id_cambio}.md` (UUID v4) y fila en el índice. |
| Universo AC | Corte auditoría `docs/audits/evolution/2026-08-11.md`: **61** registros oficiales sobre `main@9d9abd8`. |
| Esquemas observados | INV-A (atómico incompleto), INV-L (`contrato_version`/`id_cambio`/`fecha`/`tipo_operacion`), NOMBRE (filename no UUID), UUID-INV. |
| Borradores en universo | `entity-manager-eda-propuesta-analisis-temp.md`, `emit-domain-mutation-analisis-temp.md` (EV-AUD-007). |
| SIN_FECHA | `migracion-execute-process-rust-p14-p15.md`. |
| Dependientes | Bloquea migración históricos (`7bb37ff1-…`) y gate automático (`70f78d23-…`). |

## D1 — Estrategia de restauración (no retirada)

| Opción | Veredicto |
|--------|-----------|
| Retirar claves Cúmulo | **Rechazada** — rompería `sddia-evolution-sync`, constitución starter-kit y touchpoints que ya asumen las claves. |
| Materializar SSOT mínima en rutas declaradas | **Adoptada** — restaurar archivos; no mutar `cumulo.paths.json` en este ciclo salvo incoherencia demostrada (no hay). |

## D2 — Contrato v1.1+ migrable

| Decisión | Laudo |
|----------|--------|
| Versión mínima | **`evolution_contract.md` ≥ v1.1**, explícitamente migrable desde esquemas legacy detectados. |
| Esquema canónico objetivo | Un único esquema frontmatter para **registros nuevos** y como **destino** de la migración futura (PBI `7bb37ff1-…`). |
| Campos obligatorios canónicos | Identidad (`id_cambio` UUID v4), fecha de registro, tipología de operación, descripción breve, referencias a artefactos/entidades afectadas, huella (`hash_integrity` o equivalente declarado), versión de contrato (`contrato_version`). |
| Enum tipología canónica | Alinear a `sddia-evolution-sync`: `alta` \| `baja` \| `modificacion`. |
| Alias legacy tipología | Mapear sin mutar: `feature`/`bug-fix`/`refactorizacion`/`type` atómico/`tipo` → clasificación de compatibilidad (no reescritura física en este PR). |
| Alias fecha | Aceptar lectura de `fecha` \| `date` \| `created`; valor inválido o ausente → clase `SIN_FECHA` / no conforme, **sin inventar fecha**. |
| Alias identidad | Aceptar lectura de `id_cambio` \| `uuid` (y filename UUID si el frontmatter carece); UUID ausente/no v4 → `UUID-INV`. |
| Hash | Campo obligatorio en esquema canónico; históricos con vacío/`""` se clasifican como parcial/legacy, no se rellenan aquí. |
| Referencias | Campo o lista canónica (`relacionado` / `artefactos_afectados` / `related_entities`); ausencia = no conformidad formal, no borrado. |

## D3 — Colisión estándar atómico global vs `{uuid}.md`

| Conflicto | Resolución |
|-----------|------------|
| Estándar atómico global | Entidades genoma: filename `{name}.md` + frontmatter `id`/`uuid`/`type`/`version`. |
| Jurisdicción evolution | Norma sync + práctica histórica: filename **`{id_cambio}.md`** = UUID v4. |
| Laudo **L-JURISDICTION** | El contrato evolution declara **excepción jurisdiccional** bajo `directories.evolution`: el nombre físico canónico es `{uuid}.md` / `{id_cambio}.md`. El `id` lógico (kebab) vive en frontmatter cuando exista; **no** se exige `{name}.md` para registros evolution. |
| Fuera de este PR | Renombrar físicos NOMBRE → UUID (pertenece a migración `7bb37ff1-…`). |

## D4 — Semántica de borradores

| Decisión | Laudo |
|----------|--------|
| Definición | Artefacto bajo `directories.evolution` marcado como borrador/análisis temporal (filename `*-temp*`, `estado` borrador, o tipología `analisis-temporal`) **sin** pretensión de registro oficial normativo. |
| Este PR | **No extraer** ni mover borradores (EV-AUD-007 → PBI migración). |
| Representación | Índice y validador los **clasifican explícitamente** (`BORRADOR` / equivalente) y siguen contando en el universo del corte si estaban en los 61. |
| SIN_FECHA | Fila/clase explícita; no se inventa timestamp. |

## D5 — `Evolution_log.md` (índice maestro)

| Decisión | Laudo |
|----------|--------|
| Cabecera | Debe declarar vínculo al contrato (`contrato_version` / ruta lógica Cúmulo) y columnas machine-readable mínimas. |
| Columnas mínimas | `id_cambio` (o marcador UUID-INV), `fecha` (o `SIN_FECHA`), `resumen`, `clase_formato` (INV-A/INV-L/NOMBRE/UUID-INV/BORRADOR/…), `ruta_relativa`. |
| Cardinalidad | **Exactamente 1 fila por registro oficial del corte de 61**. |
| Orden | Fecha descendente; empates por ruta ascendente (misma regla que `evolution-audit`). |
| Mutación de detalle | El índice **referencia**; no reescribe cuerpos de registros. |

## D6 — Validador modo lectura

| Decisión | Laudo |
|----------|--------|
| Efecto | **Solo lectura** sobre `directories.evolution` + contrato + índice. |
| Universo | Clasificar los **61** del corte auditoría (lista determinista anclada al informe o inventario derivado sin alterar ficheros). |
| Salida | Clasificación por registro (conforme / legacy-compatible / borrador / sin-fecha / uuid-inv / nombre-no-uuid / incompleto) + totales; **cero escrituras** a registros. |
| Gate CI bloqueante | **Fuera de alcance** (PBI gate `70f78d23-…`). Aquí: herramienta/QA invocable + evidencia en el mismo PR. |

## D7 — Límites duros

| Prohibido en este ciclo |
|-------------------------|
| Normalización física / renombre masivo de históricos. |
| Rellenar fechas, UUIDs o hashes inventados. |
| Gate CI bloqueante. |
| Retirar claves Cúmulo. |
| Mezclar migración EV-AUD-002/007 o gate EV-AUD-001/002 en el mismo alcance funcional (sí documentación/QA de **esta** restauración). |
| Mutar genoma vía forja manual; cápsulas/registro vía skill Rust cuando Dedalo lo especifique. |

## D8 — Entrega y Done

| Vector | Laudo |
|--------|--------|
| PR único | Contrato + índice + validador lectura + QA + cascada documental + cierre PBI en la **misma** rama/PR. |
| Done | `features-documentation-pattern` v1.2.x: `validacion.md` APTO, `pbi_archived: true`, PBI en `docs/todos/done/` pre-merge. |

## Handoff Dedalo

1. Especificar frontmatter canónico v1.1+ y tabla de alias legacy → canónico (sin mutación física).
2. Redactar cuerpo de `evolution_contract.md` y plantilla de cabecera/`Evolution_log.md`.
3. Diseñar validador lectura (ubicación vía Cúmulo/`paths.skillsRustPath` o QA existente; sin gate CI).
4. Definir inventario determinista de 61 filas a partir del corte auditoría.
5. Emitir `spec.md` + `plan.md`; no anticipar migrador de históricos.
