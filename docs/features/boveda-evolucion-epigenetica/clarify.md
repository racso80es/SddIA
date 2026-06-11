---
feature_name: boveda-evolucion-epigenetica
created: "2024-06-04"
process: feature
---

# Clarificación — Bóveda de Evolución Epigenética

## Entendimiento del Problema

Se requiere la construcción de la Bóveda de Evolución Epigenética, un subsistema de aprendizaje continuo basado en telemetría para SddIA. Este componente actuará como una capa de "memoria de experiencia", capturando de forma pasiva los eventos generados por el sistema y cristalizándolos en vectores que representan el éxito o fracaso de acciones pasadas.

La premisa principal es otorgar al orquestador la capacidad de consultar su historial operativo para auto-corregirse y mejorar su enrutamiento y toma de decisiones, implementando un circuito de retroalimentación autónomo sin necesidad de modificar o reentrenar modelos paramétricos.

## Decisiones Clave y Restricciones Ontológicas

1.  **Topología de Almacenamiento (Estructura Multi-Índice Fractal):**
    *   La persistencia de esta bóveda operará sobre el motor LanceDB, requiriendo un aislamiento físico estricto.
    *   Los datos se almacenarán obligatoriamente en el directorio local: `.SddIA/vector_store/evolution/`. Esto mantiene la separación respecto al contexto estático (artifacts) y transitorio (thoughts).

2.  **Identidad Criptográfica (Determinismo Estricto):**
    *   Para garantizar la simetría fractal y la inmutabilidad, todo identificador generado dentro de este subsistema utilizará exclusivamente un hash SHA-256 determinista basado en el contenido o contexto del evento.
    *   Se prohíbe terminantemente la generación y el uso de identificadores aleatorios (UUIDs).

3.  **Mecanismo de Captura Pasiva (Proxy Reactivo):**
    *   La bóveda no interrumpirá el flujo normal de ejecución. Se implementará un mecanismo de proxy que actúe como "listener" o recolector sobre el bus de eventos reactivos ubicado en `.SddIA/events/`.
    *   Extraerá telemetría operativa como indicadores de éxito, tiempos, bloqueos y coste termodinámico.

4.  **Polaridades Espaciales (Clasificación Epigenética):**
    *   Cada experiencia cristalizada será clasificada semánticamente en una de dos polaridades:
        *   **Simetría Eficiente:** Trazas que representan operaciones exitosas o de baja fricción.
        *   **Fractura Estructural:** Trazas que representan errores, bloqueos, alto coste térmico o podas.
