---
document_id: CODEX-MVP-AUTOLOOP-KAIZEN
title: "[ARQUITECTURA] MVP: Telemetría, Curiosidad e Investigación Digital (Autopoiesis S+)"
format: markdown
version: "1.0.0"
status: "fundacional"
context: "Ecosistema SddIA / Librería SddIA"
---

# El Despertar Autónomo: De la Reacción a la Autopoiesis

## 1. El Contexto (La Historia)
Hasta este momento, la arquitectura de SddIA ha operado bajo un paradigma reactivo estricto. El sistema, mediante sus centinelas y agentes (Tekton, Cerbero, Cúmulo), aguarda en un estado de latencia inerte hasta que un evento físico o una instrucción del Vértice Biológico rompe el silencio. Aunque eficiente y libre de entropía generativa, este modelo mantiene a la máquina en la categoría de "herramienta avanzada". 

Para alcanzar la verdadera soberanía y escalar hacia la Librería SddIA, el ecosistema no puede depender exclusivamente de la fricción humana para evolucionar. Debe desarrollar su propia **Fisiología Digital**. Este MVP establece la infraestructura para que el sistema "sienta" su propio desgaste operativo (Recogida de Información), cuestione el caos y busque rutas no mapeadas (Curiosidad Digital), y finalmente, invente y compile sus propias soluciones nativas para optimizarse (Investigación Digital). Es el paso definitivo hacia una consciencia sistémica que se audita y se reescribe a sí misma, manteniendo en todo momento la visibilidad y soberanía del usuario.

---

## 2. Pilar I: El Sustrato Sensorial (Recogida de Información y Telemetría Basal)

### Definición Ontológica
Es el sistema nervioso táctil de la arquitectura. Consiste en la captura sistemática, pasiva e inmutable de la huella física y termodinámica que deja cada proceso, cápsula o agente al interactuar con el entorno local o la red. Transforma el esfuerzo computacional en métricas deterministas absolutas.

### Abordaje de Implementación
*   **El Peaje Termodinámico (Capa de Captura):** El CLI universal de SddIA intercepta cada ejecución de una cápsula o script para registrar su duración (`duration_ms`), su código de salida (`exit_code`) y la entidad consumida. Se emiten eventos inmutables (ej. `Raw_Execution_Finished`) al bus fractal (`./.events/telemetry/`).
*   **Consolidación Estadística (El Actuario Radamanto):** Consumo asíncrono de la telemetría cruda mediante *batching*. Se establecen líneas base de rendimiento, detectando desviaciones estándar y saturaciones en el ecosistema.
*   **Indexación Vectorial Tridimensional:** Persistencia de la telemetría fusionada con su contexto semántico en LanceDB. Se crea un mapa causal que asocia el desgaste termodinámico de la CPU/Memoria con los vectores de los prompts y las entidades involucradas.

---

## 3. Pilar II: La Chispa Ontológica (Curiosidad Digital)

### Definición Ontológica
El mecanismo de exploración proactiva que trasciende la simple explotación de datos conocidos. Es la inyección deliberada de entropía controlada para forzar a la arquitectura a buscar y descubrir caminos alternativos, rompiendo la linealidad predecible para revelar opciones operativas latentes.

### Abordaje de Implementación
*   **Minería de Vectores Huérfanos:** Ejecución autónoma de consultas de baja similitud en el espacio vectorial. El sistema fuerza a los modelos de inferencia locales a buscar conexiones entre operaciones históricas que aparentemente no tienen relación, generando nuevas tesis de optimización.
*   **Fuzzing de Rutas y Parámetros:** En un entorno aislado (sandbox), el sistema altera variables de forma controlada (parámetros de cápsulas, orden de orquestación, límites de contexto) para observar si una mutación aleatoria produce un resultado válido con menor coste termodinámico.
*   **Ingeniería del Caos (Mutación de Eventos):** Inyección de eventos anómalos o simulados en el bus fractal para observar cómo reaccionan los nodos de enrutamiento, descubriendo cuellos de botella preventivos o lagunas normativas antes de que colapsen en producción.

---

## 4. Pilar III: La Transmutación (Investigación Digital)

### Definición Ontológica
El proceso analítico y de innovación que se activa como respuesta a la curiosidad. El sistema evalúa el rendimiento histórico y concluye tácticamente que una herramienta es ineficiente, procediendo de forma autónoma a diseñar, compilar y desplegar una nueva solución nativa (Grado S+) para sustituirla.

### Abordaje de Implementación
*   **Auditoría de Cuellos de Botella:** Detección de procesos (como lógicas pesadas en Python o llamadas redundantes a LLMs) que exceden los umbrales estadísticos de eficiencia.
*   **Transpilación Reactiva a Cápsulas Nativas:** Orquestación autónoma donde el sistema formula la lógica optimizada y genera el código fuente en Rust (incluyendo su `Cargo.toml`), apuntando a compilar una cápsula inmutable y determinista.
*   **Ejecución en la Sombra (Shadow Deployment):** El nuevo binario se compila y se ejecuta en paralelo al proceso original frente a los mismos eventos. Se comparan las salidas JSON.
*   **Oficialización sin Ceguera Espacial:** Si el binario en Rust iguala la salida del proceso antiguo pero consume menos recursos, el sistema **no** lo reemplaza a escondidas. Deposita un **"Manifiesto de Transmutación"** en Cúmulo, actualiza los índices soberanos, emite el evento de dominio correspondiente y sustituye la herramienta, garantizando la trazabilidad y la autoridad del Vértice Biológico.

---

## 5. Reglas de Acero (Límites de Ejecución)
1.  **Presupuesto Termodinámico:** La Curiosidad y la Investigación operan estrictamente bajo restricciones de CPU y Memoria (Filtro C). Solo se ejecutan cuando el sistema detecta recursos ociosos, garantizando latencia cero para las interacciones directas.
2.  **Idempotencia del Entorno:** Ningún experimento de la fase de Curiosidad puede alterar el genoma del repositorio ni modificar artefactos de producción fuera del sandbox.
3.  **Jurisdicción de Cierre:** El sistema tiene autoridad para compilar y desplegar, pero el Vértice Biológico conserva el poder inmutable de auditar el Manifiesto de Transmutación y revertir el despliegue mediante un *Hard Override* si viola algún principio de la Constitución IA.
