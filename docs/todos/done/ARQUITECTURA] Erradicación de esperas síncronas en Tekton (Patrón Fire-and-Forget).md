---
document_id: PBI-TEKTON-FIRE-AND-FORGET
title: "[ARQUITECTURA] Erradicación de esperas síncronas en Tekton (Patrón Fire-and-Forget)"
format: markdown
version: "1.0.0"
created: "2026-08-15"
status: abierto
priority: alta
process: feature
related:
  - SddIA/agents/tekton.md
  - SddIA/norms/external-ai-constraints.md
  - SddIA/process/process-contract.md
  - .cursorrules
---

# [ARQUITECTURA] Erradicación de esperas síncronas en Tekton (Patrón Fire-and-Forget)

## 1. Contexto Arquitectónico y Diagnóstico de Entropía

El ecosistema SddIA opera bajo una Arquitectura Orientada a Eventos (EDA) de Estado Cero, donde los procesos fluyen de forma asíncrona a través del bus fractal. Sin embargo, se ha detectado que la IA obrera (Tekton/Cursor) continúa inyectando temporizadores estáticos (`sleep`, esperas síncronas o bucles de comprobación) tras invocar la Aduana Universal (CLI). 

Esta fricción nace del instinto de supervivencia lineal del LLM, que intenta parchear su "ceguera espacial y temporal" forzando una monitorización síncrona de un proceso asíncrono. Esto viola el principio termodinámico del ecosistema: la IA obrera no debe supervisar el tiempo; debe inyectar la intención y entrar en latencia hasta ser despertada por un nuevo estímulo.

## 2. Mandato de Refactorización (Laudo del Yunque)

Se establece la castración absoluta de la capacidad de supervisión asíncrona por parte de Tekton. La interacción entre la Capa de Ejecución y el Core SddIA debe transmutar al patrón estricto de **Fire-and-Forget (Disparar y Olvidar)**.

### Vectores de Ejecución:

**I. Blindaje Ontológico de la IA Obrera (Contratos y Normas)**
*   **Refactorización de `SddIA/agents/tekton.md` y `.cursorrules`:** Inyectar una cláusula prohibitiva innegociable. Queda terminantemente prohibido el uso de comandos como `sleep`, `wait`, bucles `while` de comprobación de estado, o cualquier mecanismo de retardo tras la invocación de una skill o proceso mediante el CLI.
*   **Mandato de Latencia:** Una vez que Tekton emite la orden al CLI, debe dar su propia tarea por finalizada (éxito de inyección) y liberar el hilo de ejecución inmediatamente.

**II. Corte Radical en la Aduana (Aislamiento Físico)**
*   **Desacople de Retorno:** Asegurar que el CLI (`execute-process` o el enrutador de dominio) devuelva el control (exit code 0 y JSON de confirmación) a la terminal en el momento exacto en que el evento es depositado en `.SddIA/events/`, sin esperar a que los centinelas (`event-watcher`) procesen la carga de trabajo.

**III. Relevo de Testigos (Handoff Asíncrono)**
*   El Vértice Biológico (Racso) o las interfaces externas (Kalma2) serán los únicos encargados de dictar el siguiente paso tras la materialización de los artefactos en el *Workspace*. Tekton no debe asumir la responsabilidad de encadenar respuestas de procesos largos en un mismo hilo de razonamiento continuo.

## 3. Criterios de Aceptación (Protocolo de Acero)

- [ ] **Auditoría de Normas:** `SddIA/norms/external-ai-constraints.md` y los contratos de Tekton incluyen explícitamente el veto al uso de temporizadores síncronos y asumen el patrón *Fire-and-Forget*.
- [ ] **Prueba de Fricción (Red Teaming):** Se invoca un proceso de larga duración (ej. `pull-request-review` o `radamanto-batch`) mediante Tekton. Se verifica empíricamente que la consola devuelve el control de inmediato y Tekton cierra su respuesta sin inyectar esperas.
- [ ] **Alineación del Bus Fractal:** Los centinelas (como `event-watcher`) asumen la carga silenciosa del evento en *background*, y los artefactos se generan en la carpeta de destino sin dependencia del hilo de la IA.
- [ ] **Cierre Documental:** Este documento supera el escrutinio de Argos y es migrado a `docs/todos/done/`.
