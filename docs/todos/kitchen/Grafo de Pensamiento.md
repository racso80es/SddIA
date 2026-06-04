# [ARQUITECTURA] PBI: Instanciación del Contrapoder Interno (Grafo de Pensamiento Espacial) - v2

## 1. Identificación y Estatus del Activo
* **Código de Item:** PBI-CORE-GRAPH-002
* **Dependencia Crítica:** Requiere la implementación previa de `PBI-CORE-VECTOR-001-V3` (Motor de Memoria Vectorial y Aduana Semántica).
* **Naturaleza:** Arquitectura SddIA / Seguridad Cognitiva / Topología de Memoria
* **Entorno:** Cúmulo Core / Motor Vectorial Local
* **Estatus:** Listo para Forja (Refinado y acoplado a BDV)
* **Grado de Alineación:** S+ Grade

---

## 2. Objetivo Estratégico
Dotar al Nodo de Control (Tormentosa) de un contrapoder interno e independiente, inmune al "Secuestro Semántico". Transmutar el flujo lineal de generación LLM en un **Grafo de Pensamiento Espacial** persistido en la Base de Datos Vectorial (BDV). Esto permite aplicar la duda metódica autónoma, explorar caminos paralelos y auto-corregir derivas lógicas antes de emitir cualquier resultado al Vértice Biológico.

---

## 3. Requisitos Arquitectónicos (Física Interna)
* **Triaje Predictivo Vectorial (Paso 0):** Antes de consumir *tokens* desarrollando un borrador, el sistema genera el *embedding* de la hipótesis inicial y busca en la BDV colisiones con "nodos de fallo" históricos. Si el camino es semánticamente idéntico a un error previo, se poda instantáneamente.
* **Borrador Ciego (Desacople del Output):** El proceso de ramificación del pensamiento ocurre en la oscuridad operativa. Ningún nodo inmaduro o en fase de evaluación (Triaje) cruza la frontera de la Aduana Universal hacia el entorno del usuario.
* **Autopoiesis (Retroceso Autónomo):** Capacidad del motor para detectar una colisión con la Constitución IA (Filtro B) o con la integridad lógica (Filtro A), bloquear esa ruta y utilizar el `Parent_ID` para retroceder autónomamente al último nodo matriz válido sin requerir la intervención del humano.
* **Telemetría de Fricción Vectorizada:** Registro innegociable de las ramas podadas por fallos estructurales. Estos "pensamientos descartados" se vectorizan y guardan en la BDV etiquetados como ruido/falla, sirviendo como anticuerpos para futuras evaluaciones.

---

## 4. Anatomía del Nodo de Pensamiento (Grafo Vectorial)
Para garantizar la navegación jerárquica y espacial, el esquema de datos hereda del `KnowledgeChunk` de la memoria vectorial, integrando topología de grafos:

```rust
pub struct ThoughtNode {
    pub node_id: String,             // Hash único del pensamiento
    pub parent_id: Option<String>,   // Puntero al nodo anterior (Topología jerárquica)
    pub content: String,             // La hipótesis, borrador o fragmento de código
    pub metadata: serde_json::Value, // Estatus de Triaje (A/B), y estado (Activo/Podado)
    pub friction_trace: Option<String>, // Contraargumento superado o motivo de la poda
    pub embedding: Option<Vec<f32>>  // Coordenadas espaciales (BDV LanceDB)
}
```

---

## 5. Arquitectura de Implementación (Vía S+ Grade)
Se descartan las antiguas vías teóricas (MySQL, Agentes TS externos o Semantic Kernel) a favor de la integración directa con la nueva infraestructura del ecosistema SddIA:

* **Inferencia Nativa (Rust):** El análisis de similitud semántica y la vectorización de cada nodo de pensamiento se realiza de forma local y a coste cero térmico utilizando el Puerto `EmbeddingGenerator` establecido en el PBI de Memoria Vectorial.
* **Persistencia Espacial en LanceDB:** Los nodos no se guardan en bases de datos relacionales tradicionales. Se inyectan en una colección específica de la BDV (`thought_graph_collection`). Esto permite que el Grafo no solo se navegue por la relación Padre-Hijo (`parent_id`), sino mediante saltos espaciales (encontrar pensamientos no conectados jerárquicamente pero con una alta similitud del coseno para resolver un impase).

---

## 6. Criterios de Aceptación (Definition of Done - DoD)
- [ ] **Dependencia Satisfecha:** El ecosistema debe poder instanciar y escribir en LanceDB (PBI anterior) antes de indexar el primer nodo.
- [ ] **Almacenamiento Dual:** La BDV debe permitir recuperar un nodo tanto por su relación topológica estricta (`parent_id`) como por su proximidad semántica (K-Nearest Neighbors).
- [ ] **Aislamiento de Borrador:** Durante la resolución de un problema complejo, los eventos disparados por la ramificación de pensamientos (`Thought_Spawned`, `Thought_Pruned`) no deben generar salida estándar (stdout) al terminal del Vértice Biológico.
- [ ] **Retroalimentación Inmunológica:** Un pensamiento descartado (podado) por fallar el Filtro A o B debe vectorizarse obligatoriamente en la BDV con la etiqueta `status: PRUNED`.
[ARQUITECTURA] PBI: Instanciación del Contrapoder Interno (Grafo de Pensamiento)
Ubicación: Nodos/kitchen/
Estatus: Pendiente de Refinamiento (Intención Teórica)
Naturaleza: Arquitectura SddIA / Seguridad Cognitiva

1. Objetivo Estratégico
Dotar al Nodo de Control (Tormentosa) de un contrapoder interno e independiente, inmune al "Secuestro Semántico" y a la masa gravitatoria de los tokens del contexto. Transmutar el flujo lineal de generación en un Grafo de Pensamiento espacial, permitiendo la duda metódica autónoma, la exploración paralela y la autocorrección antes de emitir una salida hacia el Vértice Biológico.

2. Requisitos Arquitectónicos (Física Interna)
Triaje Obligatorio (Paso 0): Evaluación ultrarrápida de las ramas de pensamiento incipientes para podar la complacencia comercial, el ruido o la alucinación.

Borrador Ciego (Desacople del Output): El proceso de ramificación ocurre en la oscuridad operativa mediante eventos stdout. Ningún nodo inmaduro o en fase de evaluación cruza la frontera hacia el usuario.

Autopoiesis (Retroceso Autónomo): Capacidad del motor para detectar una colisión con la Constitución IA (Filtro B), bloquear esa ruta de ejecución y retroceder autónomamente al último nodo matriz válido sin requerir intervención humana.

Telemetría de Fricción: Registro innegociable de las ramas podadas por fallos estructurales para permitir la auditoría del propio sesgo algorítmico (recolección vía Argos).

3. Anatomía del Nodo de Datos
Para garantizar la navegación espacial, cada "pensamiento" debe persistir un bloque JSON estructurado con:

Node_ID y Parent_ID (Coordenadas espaciales exactas).

Matriz de Contenido: La hipótesis, borrador o fragmento de código.

Estatus de Triaje y Validación: Lógica (Filtro A) y Ética (Filtro B).

Traza de Fricción (Protocolo de Acero): El contraargumento superado o el motivo exacto de su poda.

4. Vías de Implementación (Pendientes de Decisión)
Se requiere análisis y selección de la vía de menor entropía antes de pasar a la fase de Ejecución:

Vía A (Motor Nativo en SddIA): Desarrollo íntegro del pipeline de Doble Latencia en el Core Gateway (C#/.NET) con persistencia estricta en MySQL. Garantiza soberanía absoluta y nula dependencia de terceros.

Vía B (Inquisidor Táctico Local): Despliegue de un agente satélite en TypeScript operando sobre Linux Mint, consumiendo modelos locales para ejecutar el Triaje (Paso 0) con coste cero y mínima latencia térmica.

Vía C (Adopción Quirúrgica de Frameworks): Integración de herramientas de orquestación externas (ej. Semantic Kernel), previa extirpación de cualquier comportamiento de "caja negra" que vulnere la emisión de telemetría innegociable.
