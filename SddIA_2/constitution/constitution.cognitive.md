# Principio Constitucional: Núcleo Cognitivo

## Propósito
Dotar al proyecto de una capacidad de "memoria" y contexto persistente, permitiendo que el sistema aprenda y se adapte a través de las sesiones.

## Definición
Este principio define la infraestructura para la persistencia del conocimiento del proyecto, el aprendizaje y la gestión del contexto dinámico.

## Directrices
1. **Memoria Persistente:** Utilizar documentación estructurada (MD + JSON) para retener el estado crítico del proyecto y las decisiones tomadas.
2. **Contexto Dinámico:** Inyectar el historial de interacciones en el flujo de trabajo actual para tomar decisiones informadas.
3. **Indexación Semántica:** Estructurar logs y datos para su fácil consumo por modelos de IA.

## Estado Actual
- **En GesFer.Admin.Front:** Estado del cliente vía React Query (TanStack Query) y React Context (`src/contexts/`). Persistencia documental vía SddIA (paths, Cúmulo).
- **Contexto:** Gestionado por contextos React y documentación de tareas (features, fixes).
