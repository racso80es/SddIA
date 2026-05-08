---
contract_version: "1.0.0"
entity_type: "knowledge"
jurisdiction: "Core SddIA"
---

# Contrato unificado de conocimiento (Knowledge)

Este contrato define la familia **knowledge**: artefactos normativos y reutilizables que **no** orquestan ejecución (no sustituyen a processes/actions/skills/tools).

## 1. Familias

| Familia | Propósito |
|---------|-----------|
| **pattern** | Recurso reutilizable de diseño (receta, estructura, convención documentada). Orientado a consistencia y velocidad; típicamente no bloquea merges por sí solo. |
| **principle** | Norma técnica con potencial **bloqueante** cuando se declare explícitamente (`blocking_for_pr`). Puede activar escrutinio adicional en revisión de cambios. |
| **template** | Configuración predefinida o esqueleto parametrizable asociado a un **proceso** u operación recurrente (inputs/outputs esperados en su spec). |

## 2. Identidad atómica común

Todo documento de conocimiento catalogable debe declarar en cabecera YAML:

* **`uuid`**: UUID v4.
* **`name`**: Identificador kebab-case coherente con carpeta y fichero.
* **`version`**: SemVer.
* **`contract`**: `knowledge-contract v1.0.0` (o versión vigente indexada en `cumulo.paths.json` → `contracts.knowledge`).
* **`family`**: uno de `pattern` \| `principle` \| `template`.
* **`inputs`** / **`outputs`**: obligatorios cuando la familia sea **template** y el artefacto exponga parámetros de instanciación; opcionales en pattern/principle según el caso.

## 3. Ubicación (SSOT)

* **Patterns** → `directories.patterns` (`SddIA/patterns`), un **`index.md`** por familia como catálogo maestro.
* **Principles** → `directories.principles` (`SddIA/principles`), con **`index.md`**.
* **Templates** → `directories.templates` (`SddIA/templates`), con **`index.md`**.

Los artefactos concretos se materializan bajo `<familia>/<norm_id>/spec.md` salvo convención explícita futura acordada por Cúmulo.

## 4. Frontera con processes / actions / skills / tools

* La **knowledge** es **dato normativo**: guías, principios, plantillas.
* **No** define envelope I/O de acciones, **no** declara `capabilities` de skills/tools, **no** reemplaza fases de `process-contract`.
* La orquestación sigue siendo responsabilidad de **processes** + **agents** + **actions** autorizadas.

## 5. Gobernanza

* **Indexación:** `agent:cumulo` mantiene sincronía entre ficheros físicos y cada `index.md` de familia.
* **Principios bloqueantes:** si `blocking_for_pr: true` (o equivalente acordado), el proceso **`pull-request-review`** debe enrutar escrutinio a **`agent:argos`** según el contrato de procesos y las políticas vigentes.
* **Coherencia:** violaciones de familia, rutas fuera del SSOT o metadatos discordantes se reportan como **Ruido de Sistema** y bloquean reconocimiento hasta corrección.
