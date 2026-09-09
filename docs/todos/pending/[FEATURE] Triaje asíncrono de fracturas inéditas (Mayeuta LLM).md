---
document_id: PBI-FEATURE-ASYNC-FRACTURE-CLARIFICATION
title: "[FEATURE] Triaje asíncrono de fracturas inéditas (Mayeuta LLM)"
format: markdown
version: "1.0.0"
created: "2026-09-09"
status: "abierto"
priority: media
process: feature
related:
  - SddIA/agents/mayeuta.md
  - SddIA/actions/enrich-fracture-pbi-kaizen.md
  - SddIA/skills/mayeuta-llm.md
  - SddIA/norms/capsule-json-io.md
---

# [FEATURE] Triaje asíncrono de fracturas inéditas (Mayeuta LLM)

## 1. Contexto Arquitectónico y Motivación
Actualmente, el circuito de enriquecimiento Kintsugi (handler nativo en Rust) opera con una estricta Ceguera Espacial y latencia de ~20 ms. Ante una entropía inédita que carece de un cubo léxico mapeado, el motor sella la fractura correctamente pero emite un diagnóstico genérico: «requiere laudo humano». 

Esta delegación pasiva satura el ancho de banda del Vértice Biológico. Se requiere inyectar el motor semántico (`mayeuta-llm`) para analizar la traza y el contexto, pero **aislándolo termodinámicamente** del instante del nacimiento de la fractura. El LLM no debe penalizar la latencia del bus EDA ni corromper el determinismo del hash original.

## 2. Especificación (spec.md)
Implementar una topología de análisis en dos tiempos mediante un nuevo proceso asíncrono:

*   **Detonante:** Cuando el handler nativo finaliza la creación de un PBI de fractura y el veredicto incluye el flag de «requiere laudo humano» (o carece de cubo léxico), el CLI debe emitir un evento de orquestación secundario (ej. `Fracture_Clarification_Requested`).
*   **Proceso (`clarify-unknown-fracture`):** Un nuevo proceso que escucha este evento. Su única responsabilidad es invocar a Mayeuta.
*   **Acción (`append-mayeuta-hypothesis`):** Mayeuta ejecuta la skill `llm:interact` (pasando la traza literal y el estado del repositorio). La instrucción exige la generación de un análisis estructurado.
*   **Inyección en Artefacto:** El resultado devuelto por la cápsula LLM se anexa al final del PBI de la fractura bajo el encabezado estricto `### Hipótesis Semántica (Mayeuta)`.

## 3. Clarificación y Restricciones (Filtro Antientrópico)
Para blindar el ecosistema contra alucinaciones y secuestros semánticos, se establecen las siguientes Leyes de Acero:

*   **Inmutabilidad del Sello (Capa 1):** Mayeuta tiene estrictamente prohibido alterar, sobrescribir o recalcular el `fracture_hash`, `friction_id`, o la traza de error literal generada por el handler nativo.
*   **Modo Lectura/Anexo (Capa 2):** Mayeuta opera como un consultor externo. Su output se inyecta como un anexo consultivo, nunca como una reescritura del frontmatter del documento.
*   **Fail-Open Mandatario:** La invocación a `mayeuta-llm` depende de red externa y configuraciones de host. Si la cápsula falla (timeout, error HTTP, CLI de instancia ausente), el proceso `clarify-unknown-fracture` debe morir silenciosamente (exitCode > 0, capturado) sin emitir nuevas fracturas sistémicas en bucle. El PBI original queda intacto con su «requiere laudo humano».
*   **Prohibición de Orquestación:** Mayeuta emite hipótesis y sugiere comandos de mitigación en su bloque de texto, pero **no** tiene autoridad para disparar `execute-process` ni alterar código fuente por sí mismo.

## 4. Plan de Implementación (Línea de Montaje)
1.  **Skill (`mayeuta-llm.md`):** Asegurar que el contrato de la skill acepta un payload con `fracture_filepath` y `trace_context`. 
2.  **Acción (`actions/append-mayeuta-hypothesis.md`):** Definir el prompt del sistema que limitará la verbosidad de Mayeuta, exigiéndole formato Markdown limpio: Causa probable, Componentes Implicados, Solución Propuesta.
3.  **Proceso (`process/clarify-unknown-fracture.md`):** Crear el orquestador que lee el PBI, lanza la acción, y utiliza `filesystem-manager` para anexar el texto al final del archivo original.
4.  **Enrutamiento (`route_domain_core` / `enrich_fracture`):** Modificar el cierre de `enrich_fracture_pbi_kaizen.rs` para que, si el cubo es desconocido, empuje el evento secundario al bus de orquestación (`.events/orchestration/`).

## 5. Criterios de Aceptación (Protocolo de Acero)
* [ ] La emisión de una traza inédita genera el PBI determinista en < 50ms, sin esperar al LLM.
* [ ] El proceso asíncrono anexa correctamente el bloque `### Hipótesis Semántica (Mayeuta)` al documento, sin romper el frontmatter YAML.
* [ ] Si la skill `llm:interact` sufre un timeout o falla de credenciales, no se ensucia el log del sistema ni se corrompe el PBI (tolerancia a fallos validada).
* [ ] El análisis de Mayeuta respeta la limitación de verbosidad (Filtro C) y no intenta modificar la arquitectura por su cuenta.
