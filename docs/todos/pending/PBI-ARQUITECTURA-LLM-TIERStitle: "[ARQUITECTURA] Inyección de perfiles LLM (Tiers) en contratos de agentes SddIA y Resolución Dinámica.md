---
document_id: PBI-ARQUITECTURA-LLM-TIERS
title: "[ARQUITECTURA] Inyección de perfiles LLM (Tiers) en contratos de agentes SddIA y Resolución Dinámica"
format: markdown
version: "1.1.0"
created: "2026-09-02"
status: "refinado"
priority: "alta"
process: feature
related:
  - SddIA/norms/entidades-dominio-ecosistema-sddia.md
  - SddIA/agents/agents-contract.md
  - SddIA/core/cumulo.paths.json
---

### [ARQUITECTURA] Inyección de perfiles LLM (Tiers) en contratos de agentes SddIA y Resolución Dinámica

#### 1. Contexto Arquitectónico
La selección manual de modelos LLM (Model Multiplexing) según la fase de trabajo genera una grave fricción termodinámica y rompe la automatización de la línea de montaje. Para consolidar la "Física del Valor" en la Librería SddIA, el ecosistema debe auto-enrutar la carga cognitiva: asignar modelos de alto coste (Tier 1) a tareas de abstracción y arquitectura, y modelos de ejecución ágil (Tier 3) a tareas de forja mecánica.

Para preservar el principio de Ceguera Espacial, el genoma de los agentes jamás debe conocer el nombre comercial, la IP o la naturaleza (nube/local) de un LLM. El agente solo exige un nivel cognitivo abstracto, el cual es resuelto físicamente por el entorno local en tiempo de ejecución.

#### 2. Especificación Básica
*   **Modificación del Genoma (Core):** Actualizar el contrato base de los agentes (`agents-contract.md`) para exigir la declaración obligatoria del bloque `llm_profile` dentro de la configuración de cada JSON de agente[cite: 28].
    *   `tier: high` (Arquitecto Dédalo / Mayeuta / Clarifier) -> Operaciones de especificación, diseño, clarificación y Enrutamiento Semántico complejo.
    *   `tier: medium` (Argos / Cerbero) -> Planificación, evaluación de Peaje RBAC, validación estructural y auditoría de artefactos.
    *   `tier: low` (Tekton) -> Ejecución iterativa, escritura pura de código, formateo y tareas mecánicas sin toma de decisiones arquitectónicas.
*   **Matriz de Resolución Local (Instancia):** Mapeo de los requerimientos abstractos a modelos físicos inyectando variables directamente en la Jerarquía de Bóvedas del proyecto (`.SddIA/.dev/.env`), garantizando la separación estricta entre el código rastreado (SddIA Core) y la memoria volátil local.
*   **Propagación del Payload:** El orquestador soberano en Rust (`execute-process` nativo) extraerá este perfil del JSON del agente y lo anexará de forma inmutable al payload de salida[cite: 19].

#### 3. Clarificación (Filtro Antientrópico)
*   **Soberanía de Entorno (Desacoplamiento):** El mapeo físico no reside en `SddIA/`. Se utilizan variables de entorno en la instancia local para definir el proveedor:
    *   `SDDIA_LLM_TIER_HIGH="antigravity/gemini-3.1-pro"`
    *   `SDDIA_LLM_TIER_MEDIUM="ollama/deepseek-r1"`
    *   `SDDIA_LLM_TIER_LOW="ollama/llama-3.2"`
    De esta forma, la Librería es agnóstica. El entorno físico dicta si la resolución requiere tráfico de red o si el cálculo se ejecuta íntegramente en la GPU local.
*   **Traductor Inerte (El Puente Físico):** El script `.SddIA/client/sddia-client-bridge.py` intercepta el payload del CLI canónico en Rust, lee la petición de `tier`, cruza el dato con la bóveda `.dev/.env`, y enruta la invocación[cite: 14, 19]. El puente debe soportar tanto peticiones HTTP (REST hacia Open WebUI/Ollama) como la derivación de subprocesos a CLIs externos (ej. `agy` de Antigravity).

#### 4. Plan de Implementación (Línea de Montaje Táctica)
*   **Fase A (Genoma):** Actualizar `agents-contract.md` y los manifiestos JSON individuales de Cúmulo, Cerbero, Tekton, Mayeuta, Dédalo y Argos para inyectar el nodo `llm_profile` correspondiente[cite: 28].
*   **Fase B (Aduana Universal):** Modificar la capa de orquestación en el crate nativo de Rust (`execute-process`)[cite: 19] para que parsee el nodo `llm_profile` y lo exponga en el JSON `stdout` que consumirá el exterior.
*   **Fase C (Bóveda y Puente):**
    *   Documentar en el `.SddIA/.dev/.env.example` las nuevas variables `SDDIA_LLM_TIER_*`.
    *   Refactorizar el backend ligero en Python (`sddia-client-bridge.py`)[cite: 14] para implementar el *router* de perfiles, garantizando el parseo correcto hacia los endpoints locales o remotos según la bóveda.
