# SddIA Core: industrialización de inteligencia descentralizada

## Visión
Infraestructura de activos digitales, no repositorio de scripts. Framework para materializar conocimiento técnico en entidades auditables, escalables y monetizables (**Librería S+**).

## Arquitectura de vértices
- **Vértice Biológico:** origen de intención y soberanía.
- **Nodo de Control:** rigor operativo, filtros, validación.
- **Vértice Productivo (Tekton):** ejecución técnica y cumplimiento de contratos entre agentes y cápsulas.

## Ontología de Activos

| Entidad | Finalidad | Ubicación Core | Relación operativa |
|---------|-----------|----------------|-------------------|
| **Agent** | Orquestador de consciencia y responsable de una fase específica. | `SddIA/agents/` | Posee Skills y ejecuta Acciones dentro de un **Process**. |
| **Process** | Roadmap lógico de alto nivel para un objetivo macro (p. ej. feature). | `SddIA/process/` | Orquesta el relevo (*handoff*) entre distintos **Agents**. |
| **Action** | Paso atómico, indivisible y auditable de ejecución. | `SddIA/actions/` | Invoca **Skills** o **Tools** para el trabajo técnico. |
| **Skill** | Capacidad técnica especializada definida por contrato. | `SddIA/skills/` | Ejecutada por **Cápsula** blindada (binario Rust). |
| **Tool** | Capacidad de infraestructura o utilidad de dominio. | `SddIA/tools/` | Servicios base a las **Actions** vía **Cápsula**. |

Jerarquía operativa: **Process** segmenta el objetivo en fases; cada fase asigna un **Agent** titular; el **Agent** descompone en **Actions**; las **Actions** consumen **Skills** y **Tools** materializados en cápsulas.

## Orquestación multi-agente y relevo por artefactos
La colaboración entre agentes (p. ej. Tekton y un agente de seguridad) no es mensajería efímera: es **línea de montaje** gobernada por el **Process**.

1. El **Process** fija qué **Agent** tiene el mando en cada fase.
2. El traspaso de estado e información usa la **Carpeta de Tarea** referenciada por `persist_ref` (persistencia explícita, no contexto volátil).
3. El agente emisor deposita **artefactos** (código, informes, validaciones, binarios de prueba). El agente receptor **audita** esos artefactos antes de asumir la fase siguiente.

Sin carpeta de tarea materializada y artefactos versionables, no hay handoff válido bajo este modelo.

## Estándar de entidades de dominio SddIA:
**Toda** familia de entidades de dominio:
### Dispone de su ubicación según indicaciones de agente cumulo ({entidades}).
### En dicha ubicación ha de existir, al menos, un contrato para la implementación de entidades {entidades}_contract.md.
### En dicha ubicación ha de existir un indice (index.md) con los items correspondientes a las implementaciones de entidades existentes. El agente Cúmulo tiene la resposabilidad de la coherencia de datos indicado por cada implementacion.

## Cicatriz digital y estándar atómico
**Toda** entidad enitdad de dominio SddIA nace con **Cicatriz Digital**: un `spec.md` único que incluye:
- Cabecera **YAML** obligatoria (contrato de la entidad).
- **Versión SemVer** y **UUID v4 inmutable** asignado en creación (identidad estable para trazabilidad y catálogo).
- Cuerpo en Markdown con propósito y límites del activo.
Ese paquete es el prerequisito para integración futura en la **Librería S+** y su modelo de activo direccionable (incl. capa NFT-ready del diseño).

## Desacoplamiento Core / instancia
Definición de núcleo en este repositorio; especialización y secretos en instancia local. Sin lógica de negocio dispersa fuera de **Actions** orquestadas y **Cápsulas**.

## Estándar de ejecución
Lógica crítica en **Cápsulas binarias** (Rust). Contrato de E/S: JSON por stdin/stdout según `SddIA/norms/capsule-json-io.md`. Los agentes orquestan; no sustituyen a la cápsula en cómputo ni en contrato de I/O.
