---
document_id: PBI-ARQ-CONSCIENCIA-UNIVERSAL
title: "[ARQUITECTURA] Globalización de la Consciencia del Usuario (Grafo de Pensamiento Universal)"
format: markdown
version: "1.0.0"
created: "2026-08-19"
status: "abierto"
priority: alta
process: feature
related:
  - SddIA/agents/mayeuta.md
  - SddIA/agents/cumulo.md
  - SddIA/norms/events-contract.md
---

### [ARQUITECTURA] Globalización de la Consciencia del Usuario (Grafo de Pensamiento Universal)

### pendiente de refinamiento. Prestar especial atención a posibles alucionaciones, incoherencias o inexactitudes.

#### 1. Especificación (`spec.md`)

**Propósito:**
Erradicar la entropía de los silos de información (el "Cerebro Dividido") descentralizando el conocimiento de las herramientas individuales y centralizándolo en un **Grafo de Pensamiento Universal**. La consciencia sobre los hábitos, preferencias y directrices del Vértice Biológico (Racso) debe ser transversal y agnóstica a la herramienta de origen o destino.

**El Problema Operativo:**
Si el Gestor de Correos aprende que "los correos de X son urgentes", ese conocimiento queda atrapado en el dominio del correo. Si mañana se despliega un Gestor de Calendario, este no sabrá que las reuniones con "X" también son urgentes. Las herramientas no deben "aprender"; deben ser actuadores ciegos.

**La Solución S+ Grade (Arquitectura de Flujo):**
Se establece un pipeline ontológico de cuatro fases, orquestado a través del Sistema Nervioso de Eventos (ECST):

1. **Estímulo Ciego (La Herramienta):** La herramienta (ej. Gestor de Correos) recibe una corrección del usuario. La herramienta **no guarda nada**. Se limita a inyectar un evento de fricción genérico en el bus (`.SddIA/events/domain/`).
2. **Triaje y Razonamiento (Mayeuta):** El agente Mayeuta actúa como el "embudo perceptual". Suscrito a los eventos de fricción, escanea el caos, filtra el ruido técnico y destila la intención pura en un Tripleto Semántico (Sujeto -> Predicado -> Objeto).
3. **Persistencia Soberana (Cúmulo):** Mayeuta no escribe en la base de datos. Transfiere el Tripleto Semántico a **Cúmulo**. Cúmulo, como custodio de la SSOT, inyecta/actualiza/elimina el nodo en la base de datos vectorial de grafos (ej. LanceDB).
4. **Inyección Topológica (El Orquestador):** En ejecuciones futuras de *cualquier* herramienta, el Orquestador (Aduana Universal) consulta a Cúmulo, extrae el sub-grafo de contexto relevante y lo inyecta en el *payload* de la herramienta antes de despertarla.

---

#### 2. Clarificación y Lógica de Razonamiento (`clarify.md`)

**El Cerebro de Mayeuta (Motor de Digestión Ontológica):**
Para que Mayeuta sepa qué hacer con la entropía entrante, su contrato (`mayeuta.md`) debe ser expandido con un **Protocolo de Triaje Entrópico** riguroso. Ante cada evento de fricción, Mayeuta debe razonar y emitir un dictamen hacia Cúmulo bajo una de las siguientes cuatro operaciones (CRUD Ontológico):

*   **A. IGNORAR (Filtro C - Descarte de Ruido):**
    *   *Escenario:* El evento es anecdótico, técnico o efímero (ej. "Borra este correo que es spam", "Ordena la lista por fecha solo por hoy").
    *   *Acción:* Mayeuta clasifica el evento como ruido transitorio. No genera conocimiento. Se descarta para proteger la economía termodinámica.
*   **B. PERSISTIR / AÑADIR (Filtro A y B superados):**
    *   *Escenario:* Se detecta un nuevo patrón o regla de oro (ej. "Los correos de Juan Pérez sobre el proyecto GesFer siempre son máxima prioridad").
    *   *Acción:* Mayeuta destila: `[Juan_Pérez] -> [Prioridad_Máxima] -> [Contexto:GesFer]`. Se transfiere a Cúmulo para creación de nodo.
*   **C. ACTUALIZAR / MATIZAR:**
    *   *Escenario:* El usuario refina una regla existente (ej. "A partir de ahora, los informes de marketing ya no me interesan los viernes, solo los lunes").
    *   *Acción:* Mayeuta debe consultar el grafo existente (vía Cúmulo), identificar la contradicción temporal y emitir una orden de mutación de la arista relacional.
*   **D. ELIMINAR (Poda Ontológica):**
    *   *Escenario:* El usuario revoca explícitamente un hábito o Cúmulo detecta un "Secuestro Semántico" de reglas fósiles que ya no aplican.
    *   *Acción:* Se instruye a Cúmulo la eliminación del nodo para evitar la amnesia termodinámica por sobresaturación.

**El Custodio (Cúmulo):**
Cúmulo es el único con permisos de I/O sobre el motor de grafos vectoriales. Valida la integridad de la petición de Mayeuta y ejecuta la inserción. Ninguna herramienta externa (ej. Tekton o cápsulas Rust) puede tocar la memoria a largo plazo de forma directa.

---

#### 3. Plan de Implementación (`plan.md`)

**Fase 1: Infraestructura Base (Contratos y Eventos)**
1.  Definir en `events-contract.md` el nuevo evento de dominio: `User_Friction_Detected` (Emitido por las herramientas ciegas/interfaces hacia el bus).
2.  Definir el evento interno: `Ontological_Mutation_Requested` (Emitido por Mayeuta hacia Cúmulo, conteniendo la directriz CRUD y el Tripleto Semántico).

**Fase 2: Evolución de Mayeuta (El Triaje)**
1.  Refactorizar el contrato `SddIA/agents/mayeuta.md`. Inyectar en sus `capabilities` el marco de decisión "Ignorar / Persistir / Actualizar / Eliminar" detallado en la clarificación.
2.  Implementar la lógica en Mayeuta para que no genere código, sino que responda estructurando datos semánticos a partir de lenguaje natural (NLP a Tripletos).

**Fase 3: Evolución de Cúmulo (El Grafo)**
1.  Vincular el agente Cúmulo a la base de datos vectorial subyacente (LanceDB / Grafo).
2.  Dotar a Cúmulo de la *capability* para interpretar los `Ontological_Mutation_Requested` y ejecutar las sentencias físicas en la base de datos.
3.  Dotar a Cúmulo de un endpoint de consulta (`Query_Subgraph`) para extraer contextos.

**Fase 4: Inyección en la Aduana Universal (CLI/Orquestador)**
1.  Modificar el Orquestador (`execute-process` nativo en Rust). Antes del *spawn* del subproceso (la cápsula ciega), el Orquestador emite un `Query_Subgraph` a Cúmulo pasando las entidades clave del payload actual.
2.  El resultado del grafo se inyecta en el objeto JSON de entrada de la herramienta (ej. bajo el nodo `"system_context": {}`).
3.  Prueba de fuego (Suite de Caos): Crear una herramienta *dummy* y verificar que hereda una regla creada a través del gestor de correos sin conocer su origen.
