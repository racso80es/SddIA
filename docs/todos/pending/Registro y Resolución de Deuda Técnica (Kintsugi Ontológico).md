---
document_id: PBI-OPER-DEUDA-TECNICA-KINTSUGI-001
uuid: "4be8aeee-896a-4d2f-b2d3-3ee0d05fbd80"
title: "[OPERATIVO] Registro y Resolución de Deuda Técnica (Kintsugi Ontológico)"
format: markdown
version: "2.0.0"
created: "2026-08-28"
updated: "2026-08-28"
status: pending
priority: media
process: feature
type: feature
dispatch: false
suggested_branch: feat/jurisdiccion-deuda-tecnica-todos
persist_ref_suggested: docs/features/jurisdiccion-deuda-tecnica-todos
depends_on: []
related_pbis:
  - id: PBI-KAIZEN-FRACTURE-FANOUT-IDEMPOTENCIA
    rol: "Resolutor de ruta de PBI de fractura en Core. Este PBI NO duplica su alcance: consume el resolutor, no lo reimplementa."
  - id: PBI-DT-PACIENTE0-DEPLOY-PROCESS
    rol: "Habitante actual de docs/todos/DeudaTecnica/; sujeto de reclasificación en CA3."
  - id: PBI-DT-PACIENTE0-UNDEPLOY-PROCESS
    rol: "Ídem."
friction_ids:
  - F-TODOS-BUCKET-HUERFANO
  - F-TODOS-SIN-JURISDICCION
  - F-DEUDA-NO-FRACTURA-SIN-PORTADOR
  - F-TAXONOMIA-FRICCION-SIN-ENUM
related:
  - SddIA/engine/execute-process/src/engine/handlers/task_queue_manager.rs
  - SddIA/engine/execute-process/src/engine/phase_capsules.rs
  - SddIA/actions/materialize-fracture-pbi.md
  - SddIA/actions/enrich-fracture-pbi-kaizen.md
  - SddIA/events/domain/system-fracture-detected.md
  - SddIA/library/norms/features-documentation-pattern.md
  - SddIA/norms/obediencia-procesos.md
  - SddIA/core/cumulo.paths.json
  - SddIA/tools/tools-contract.md
source_audit: "Lectura directa de anclajes de ruta en task_queue_manager.rs y phase_capsules.rs; censo de docs/todos/*; rg de terminología contra README.md, CONSTITUTION_CORE.md y SddIA/norms/"
review_notes: "Filtro A aplicado 2026-08-28 — borrador v1 auditado; 6 afirmaciones corregidas o descartadas en §7"
---

# [OPERATIVO] Registro y Resolución de Deuda Técnica (Kintsugi Ontológico)

> **Refinamiento v2.0.0.** El borrador v1 proponía canalizar toda la deuda pendiente hacia `docs/todos/DeudaTecnica/`. La auditoría del genoma demuestra que ese bucket es **inalcanzable para el despachador y para el archivador**, por lo que un ítem depositado allí no puede cerrar ciclo. El déficit real es otro y está acotado en §3. Las afirmaciones descartadas quedan en §7 para evitar reincidencia.

## 1. Falla estructural y contexto

`docs/todos/` opera hoy con **seis buckets** sin norma que declare su jurisdicción:

| Bucket | Documentos | Reconocido por el runtime |
|--------|-----------|---------------------------|
| `pending/` | 9 | Sí — despacho y archivado |
| `done/` | 205 | Sí — destino de archivado |
| `DeudaTecnica/` | 3 | **No** |
| `kitchen/` | 5 | **No** |
| `tmp/` | 5 | **No** |
| `historias/` | 1 | **No** |

El runtime nativo solo conoce dos rutas:

- `extract_pbi_path` ancla exclusivamente en `docs/todos/pending/` y `docs/todos/done/` (`task_queue_manager.rs:364`). Un documento fuera de esos prefijos **no es despachable**: el TQM no extrae su ruta del estímulo.
- La cápsula de archivado exige `rel.contains("docs/todos/pending/")` antes de mover a `docs/todos/done/` (`phase_capsules.rs:1100`, `:1108`) y valida `pbi_archived` (`:1045`).

Consecuencia empírica: `Done = un único PR mergeado + validacion.md APTO + PBI en docs/todos/done/` (`features-documentation-pattern` v1.2.1, §`pbi_archived`) es **insatisfacible** para un ítem nacido en `DeudaTecnica/`. Un registro de deuda alojado ahí es un cementerio, no una cola.

Ninguna norma ni entidad del genoma menciona `DeudaTecnica/`, `kitchen/`, `tmp/` ni `historias/`: `rg` sobre `SddIA/` devuelve cero coincidencias. Son convenciones tácitas sostenidas por memoria humana.

## 2. Lo que ya existe (no se reconstruye)

| Órgano | Artefacto | Referencia |
|--------|-----------|------------|
| Detección de fractura | `System_Fracture_Detected` → `eda_bus.pending` | `SddIA/events/domain/system-fracture-detected.md` |
| Materialización de deuda | Cúmulo escribe el PBI `bug-fix` en `docs/todos/pending/` | `SddIA/actions/materialize-fracture-pbi.md` (`target_path`) |
| Enriquecimiento diagnóstico | Mayeuta añade causa raíz y veredicto evolutivo | `SddIA/actions/enrich-fracture-pbi-kaizen.md` |
| Despacho | Extracción de ruta de PBI desde el estímulo | `task_queue_manager.rs:362-373` |
| Archivado | `pending/` → `done/` con sello `pbi_archived` | `phase_capsules.rs:1045-1134` |
| Contrato documental | `objectives/plan/implementation/validacion` en `.md` con frontmatter | `features-documentation-pattern` v1.2.1 |
| Escalado ante colapso | Protocolo Kintsugi: detener, emitir fractura, delegar al bus | `SddIA/norms/obediencia-procesos.md` § Escalado ante fallo |

**El motor de resolución de deuda por fractura ya está forjado.** Este PBI no lo reconstruye.

## 3. Déficit real (alcance acotado)

| ID | Déficit | Evidencia |
|----|---------|-----------|
| `F-TODOS-BUCKET-HUERFANO` | 14 documentos viven en buckets que el runtime no ve; no son despachables ni archivables | `task_queue_manager.rs:364`; `phase_capsules.rs:1100` |
| `F-TODOS-SIN-JURISDICCION` | Ninguna norma define qué bucket admite qué tipo de documento ni su ciclo de vida | `rg "todos/(kitchen\|tmp\|historias\|DeudaTecnica)" SddIA/` → 0 |
| `F-DEUDA-NO-FRACTURA-SIN-PORTADOR` | La deuda por **decisión humana de aplazamiento** no emite evento, luego el fan-out Kintsugi nunca la materializa; hoy depende de redacción manual sin identidad ni taxonomía | Kintsugi se dispara solo desde `System_Fracture_Detected` |
| `F-TAXONOMIA-FRICCION-SIN-ENUM` | `friction_ids` y `tech_debt_ids` se usan ad-hoc (2 de 9 en `pending/`, 2 de 3 en `DeudaTecnica/`) sin catálogo ni validación | Censo de frontmatter en `docs/todos/` |

## 4. Taxonomía de la deuda (corregida)

| Tipo de fricción | Origen empírico | Resolución esperada (Grado S+) |
|------------------|-----------------|-------------------------------|
| **Deuda arquitectónica** | Atajo tomado para estabilizar una PoC o sortear un bloqueo local (script suelto, lógica de negocio dentro del agente) | Refactorización hacia cápsula determinista (`SddIA/skills/` o `SddIA/tools/`) con E/S JSON por stdin/stdout y emisión al bus fractal |
| **Falla de cobertura** | Caso límite no contemplado o cápsula incompleta | Densificación de la **definición atómica `{name}.md`** de la entidad (contrato `tools-contract` v1.5.0 §1 / `skills-contract`) y recompilación del artefacto resuelto vía `compiled_capsules` |
| **Deuda de gobernanza** | Evento sin `event_family` correcto, telemetría mal clasificada o ruido en `./.events/` | Ajuste a la **Trinidad de Estímulos** (`telemetry` \| `orchestration` \| `domain`, enum estricto en `events-contract.md`) y purga vía `event-bus-audit` |

> Matiz de precisión: `./.events/progress/` existe como ruta runtime bajo `eda_fractal` en `cumulo.paths.json`, pero **no es un `event_family`**; el genoma solo declara tres familias (`SddIA/events/{telemetry,orchestration,domain}/`).

## 5. Criterios de aceptación

- **CA1** — Norma de jurisdicción de `docs/todos/` forjada vía `norm-creator` (prohibida la escritura manual del genoma): declara para cada bucket si es despachable, archivable o inerte, y el ciclo de vida admitido.
- **CA2** — La norma no introduce un tercer estado de cierre: `Done` sigue siendo PBI en `docs/todos/done/` con `validacion.md` APTO y `pbi_archived: true` en el mismo PR (`features-documentation-pattern` v1.2.1).
- **CA3** — Los 3 documentos de `DeudaTecnica/` reclasificados con laudo explícito: los dos `[DEUDA] Paciente 0` son **semillas de proceso** (`process_candidate`), no deuda accionable; `Optimizacion_BioIA.md` triado a `pending/`, a semilla o a descarte.
- **CA4** — Portador canónico de deuda no-fractura definido: `type: deuda` + `tech_debt_ids` en un PBI bajo `pending/`, con enum documentado de prefijos de fricción.
- **CA5** — Verificación física, no declarativa: para cada documento migrado, `extract_pbi_path` devuelve su ruta y la cápsula de archivado la acepta. Evidencia en `validacion.md` con salida de CLI, no con afirmación del operador.
- **CA6** — Cero solapamiento con `PBI-KAIZEN-FRACTURE-FANOUT-IDEMPOTENCIA`: la idempotencia del fan-out y el resolutor de ruta en Core pertenecen a ese PBI (`A-RESOLUTOR-EN-CORE`). Si este PBI necesita el resolutor, lo consume.

## 6. Invariantes

| Regla | Motivo |
|-------|--------|
| Un ítem se declara resuelto solo con evidencia física (veredicto de Argos o telemetría del CLI) | Argos opera bajo Principio de Evidencia Determinista; prohibido sustituir cápsula de validación por razonamiento LLM (`SddIA/agents/argos.md` §2) |
| Prohibida la mutación manual de `SddIA/norms/`, `actions/`, `process/` | Blindaje IA obrera; toda entidad se forja vía `execute-process` → `entity-manager` |
| Rutas resueltas contra `cumulo.paths.json`, nunca inferidas | Ceguera Espacial: la coordenada absoluta se inyecta en `workspace_path` (README § Orquestación multi-agente, punto 3) |
| Ante colapso de proceso oficial: detener y emitir fractura | Protocolo Kintsugi (`obediencia-procesos.md`); prohibido bypass raw (`gh`, `git`, `curl`) |
| Hito registrado en `SddIA/evolution/` vinculando el `uuid` de este PBI | Registro de evolución obligatorio |

## 7. Auditoría del borrador v1 (afirmaciones corregidas o descartadas)

| # | Afirmación v1 | Veredicto | Evidencia |
|---|---------------|-----------|-----------|
| 1 | «Ruta de destino: `docs/todos/DeudaTecnica/[OPERATIVO] Consolidacion-Deuda-Tecnica.md`» | **Descartada** | Bucket invisible para despacho (`task_queue_manager.rs:364`) y archivado (`phase_capsules.rs:1100`). Además el nombre propuesto no coincidía con el fichero real. |
| 2 | «Amnesia Termodinámica» | **No SSOT** | Término sin una sola aparición en el repo fuera del propio borrador. En SddIA «termodinámico» designa el **Peaje Termodinámico** (telemetría, README) y la eficiencia del DEP (`entidad-digital-passport.md`). Sustituido por *pérdida de contexto evolutivo*. |
| 3 | «Filtro de Acero» | **Inexistente (híbrido)** | La Constitución define el **Protocolo de Acero (Yunque Rúnico)** §5 y el **Triaje Entrópico** con filtros **C, A, B**. No existe un «Filtro de Acero». |
| 4 | «Densificación de los contratos `spec.md`» para Tools | **Corregida** | La definición de una Tool es `SddIA/tools/{name}.md` (`tools-contract` v1.5.0 §1: `name`, `uuid`, `version`, `contract_ref`, `implementation_path_ref`). `spec.md` es artefacto de `docs/features/` \| `docs/fixes/` y de patterns, no contrato de entidad. |
| 5 | «Trinidad de Estímulos (Telemetry, Orchestration, Domain)» | **Exacta** | README §Trinidad de Estímulos; enum estricto en `events-contract.md`. Único matiz en §4 (`progress` es ruta runtime, no familia). |
| 6 | «Ceguera Espacial: inyectar solo el micro-contexto necesario» | **Conflación** | Ceguera Espacial = las Entidades de Dominio **no conocen rutas del repositorio**; la coordenada se inyecta como `workspace_path` (README § Orquestación multi-agente, punto 3; `argos.md` §1). La minimización de contexto al operador IA es otra cosa (enrutamiento semántico / `external-ai-constraints.md`). Ambas aplican, pero no son la misma norma. |
| 7 | «Redundancia en las rutinas de Cerbero» | **No verificada** | Cerbero es peaje RBAC de intercepción pura (`cerbero.md` §5); no ejecuta rutinas periódicas. Sin evidencia (`event-bus-audit`) no se admite como deuda catalogada. |
| 8 | «El motor de resolución» (sin sujeto) | **Precisada** | El motor existente es el fan-out Kintsugi (`materialize-fracture-pbi` + `enrich-fracture-pbi-kaizen`) más el TQM. Nombrado en §2. |
| 9 | «Persistir en la memoria de la sesión y en el repositorio» | **Corregida** | La memoria de sesión no es SSOT ni sobrevive al proceso. Persistencia = repositorio (`docs/todos/`) + `SddIA/evolution/`. |
| 10 | «Confirmación empírica a través de Argos o de la telemetría del CLI» | **Exacta — conservada** | Alineada con `argos.md` §2 (evidencia determinista). Promovida a invariante en §6. |

Nota de forma: el borrador v1 llegó como un único párrafo con las tablas colapsadas en texto corrido, sin frontmatter ni `document_id`. Incumplía el estándar atómico `{name}.md` con frontmatter YAML e `uuid`.

## 8. Riesgos

| Riesgo | Mitigación |
|--------|-----------|
| Migrar 14 documentos a `pending/` inunda la cola y dispara despachos no deseados | `dispatch: false` obligatorio en la migración; triaje previo (CA3) descarta lo inerte |
| Solapamiento con el resolutor de ruta del PBI de fan-out | CA6: consumo, no reimplementación |
| Formalizar un bucket extra reproduce la fragmentación que se pretende cerrar | La norma debe justificar cada bucket superviviente o eliminarlo |

## 9. Referencias

| Ref | Uso |
|-----|-----|
| `SddIA/engine/execute-process/src/engine/handlers/task_queue_manager.rs:362-373` | Anclaje de ruta de PBI |
| `SddIA/engine/execute-process/src/engine/phase_capsules.rs:1045-1134` | Gate `pbi_archived` y movimiento a `done/` |
| `SddIA/library/norms/features-documentation-pattern.md` v1.2.1 | Definición de Done y cierre documental en rama |
| `SddIA/actions/materialize-fracture-pbi.md` | Materialización automática en `pending/` |
| `SddIA/norms/obediencia-procesos.md` | Escalado Kintsugi ante colapso |
| `SddIA/CONSTITUTION_CORE.md` §5 | Protocolo de Acero / Triaje Entrópico |
| `SddIA/core/cumulo.paths.json` | SSOT de topología (`directories.documentation`, `eda_fractal`) |
