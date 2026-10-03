---
document_id: HU-LINEAR-SYNC-FLOW
title: "HU - [OPERATIVO] Ampliación de Cápsula Linear: Sincronización de Flujos SddIA"
format: markdown
version: "1.0.0"
created: "2026-10-03"
status: "abierto"
priority: "alta"
process: "feature"
related:
  - SddIA/norms/entidades-dominio-ecosistema-sddia.md
  - SddIA/agents/cumulo.paths.json
---

### [OPERATIVO] Ampliación de Cápsula Linear: Sincronización de Flujos SddIA

#### 1. Propósito y Alcance
Ampliar la funcionalidad de la cápsula de Linear para establecer una sincronización bidireccional y determinista con el ciclo de vida del desarrollo de software del ecosistema SddIA. El objetivo es que Linear refleje fielmente el estado de las entidades operativas sin requerir intervención manual, actuando como un panel de control acoplado a la termodinámica del código.

#### 2. Configuración de Dominio (Proyecto)
Para garantizar la Ceguera Espacial y evitar la entropía entre distintos repositorios o contextos, la cápsula requerirá la inyección de la siguiente configuración de entorno:
* **Clave de asociación:** `LINEAR_PROJECT`
* **Directriz:** Toda petición de creación o actualización hacia Linear deberá inyectar esta clave para asegurar que la entidad se aloje estrictamente en el proyecto correspondiente de la plataforma.

#### 3. Diccionario Ontológico de Etiquetas (Labels)
Para mantener la coherencia visual y de filtrado en Linear, la cápsula inyectará automáticamente los siguientes prefijos a los issues, respetando el formato `{alias}-{item}`:

* **Historia de Usuario:** `HU-{item}`
* **Feature / Product Backlog Item:** `PBI-{item}`
* **Fix / Resolución de Error:** `FIX-{item}`
* **Mejora / Evolución:** `kaicen-{item}`
* **Deuda Técnica:** `IT-{item}`
* **Investigación / Spike:** `SPIKE-{item}`

#### 4. Motor de Estados (Matriz de Transición)
Los estados del flujo de Linear quedan fijados y homologados numéricamente para gobernar las transiciones desde el Orquestador SddIA:

1. **Backlog:** Pendiente de refinar y clarificar.
2. **Todo:** Refinado, clarificado y listo para forjar.
3. **In Progress:** Forja en proceso.
4. **In Review:** Forja finalizada, pendiente de revisión (Pull Request presentado).
5. **Done:** Revisado, consolidado y en Producción.
6. **Canceled:** Descartado o purgado.

#### 5. Coreografía de Flujos (Triggers de Ejecución)

**Flujo A: Historia de Usuario (HU)**
* **Creación:** Se inicializa en Estado 1 (Backlog).
* **Paso a Todo:** Al superar la fase de clarificación, transita al Estado 2 (Todo).
* **Generación de PBIs:** En el instante en que se forjan y asocian PBIs a la HU, o cuando el primer PBI inicia su ejecución, la HU transita al Estado 3 (In Progress).
* **Cierre de Ciclo:** Cuando el último PBI asociado alcanza el estado *Done*, la HU transita automáticamente al Estado 5 (Done).

**Flujo B: Product Backlog Item (PBI)**
* **Creación:** Ya sea derivado de una HU o creado de forma autónoma, se inicializa en Estado 1 (Backlog).
* **Refinamiento:** Tras generar su plan de acción, transita al Estado 2 (Todo).
* **Ejecución Activa:** Al iniciar la alteración de código (forja), transita al Estado 3 (In Progress).
* **Revisión (Condicional):** Si el flujo requiere revisión mediante Pull Request, la presentación del PR dispara la transición al Estado 4 (In Review).
* **Consolidación:** Al ejecutar el *merge* hacia master/main (o si finaliza directamente desde *In Progress* por ser un flujo sin PR), transita al Estado 5 (Done).
