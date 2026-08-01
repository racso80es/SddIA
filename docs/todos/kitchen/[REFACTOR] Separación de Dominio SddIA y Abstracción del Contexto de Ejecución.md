Document ID: PBI-SDDIA-DOMAIN-ABSTRACT-01
UUID: 7d81a9f2-3c4e-4b1a-8f0e-2d9c1b4e6a3f
Title: "[REFACTOR] Desacoplamiento del Motor SddIA frente al Contexto de Software (Hacia el Sistema Nervioso Universal)"
Format: markdown
Version: "1.0.0"
Created: "2026-07-20"
Status: "pendiente-kitchen"
Priority: Alta (Estratégica)
Process: refactor
Suggested Feature Name: sddia-domain-abstraction
Suggested Branch: refactor/sddia-domain-abstraction
Depends On: - docs/features/kalma2-full-cycle

docs/features/vanguardia-soberania-local
Destination Path: docs/todos/kitchen/PBI_Separacion_Dominio_SddIA.md

1. Clarificación Estratégica (Filtro B - Táctica del Refugio y Soberanía)
El ecosistema SddIA nació forjado bajo el rigor de la compilación de software, el control de repositorios (Git) y la orquestación de código. Sin embargo, su verdadera naturaleza es la de un Sistema Nervioso Central Reactivo gobernado por una Arquitectura Orientada a Eventos (EDA).

Este PBI establece la Separación de Dominio Estricta:

El Motor Core (SddIA/core/ y SddIA/engine/): Debe purgar cualquier supuesto implícito de que el objetivo del bus o de los eventos esté atado a un Pull Request, una rama o un artefacto de código fuente. El motor solo procesa chispazos canónicos, paquetes JSON (capsule-json-io.md) y estados termodinámicos.

Los Códices de Dominio (La Cantera Fractal): Los propósitos operativos (como la gestión de software, la asistencia personal, la domótica, la mensajería asíncrona o el procesamiento de notas) se desvinculan del arranque base y pasan a ser Códices de Dominio Inyectables (SddIA/library/codexes/).

2. Objetivos S+ Grade
Agnosticismo del Motor EDA: Lograr que el Task Queue Manager (TQM) y los watchers del sistema nervioso procesen eventos de dominio generalista sin requerir la instanciación forzosa de un entorno de desarrollo de código (como objectives.md orientado a software por defecto).

Modularización de Códices: Extraer la lógica específica de programación (feature, bug-fix, refactorization) hacia un códice desacoplado (ej. codex-software-engineering.md), dejando espacio limpio para registrar nuevos códices de vida y asistencia (ej. codex-personal-assistant.md).

Contratos de E/S Universales: Garantizar que cualquier skill o tool periférica (como el conector de Telegram o el puente de Kalma2) interactúe con el bus bajo el mismo estándar inmutable, permitiendo que un prompt de asistencia personal dispare acciones sin tocar el genoma core.

3. Especificación Técnica y Hitos de Ejecución (Futura Forja)
Hito 1: Desacoplamiento del Arranque de Workspace (workspace_init)
Refactorizar el motor de inicialización para que no asuma por defecto la creación de estructuras de Git o PRs a menos que el Códice de Dominio activo lo exija explícitamente.

Hito 2: Taxonomía de Códices de Dominio (SddIA/library/codexes/)
Formalizar el contrato de los Códices para que actúen como enrutadores semánticos limpios, permitiendo al usuario alternar o solapar perfiles operativos (desarrollador, asistente personal, monitor de red) a demanda desde Kalma2.

Hito 3: Generalización de Eventos de Dominio
Extender la familia de eventos en el Bus (.SddIA/events/domain/) para soportar mutaciones de estado no ligadas al código fuente, manteniendo la trazabilidad inmutable y el triaje de seguridad de Cerbero y Argos.

4. Criterios de Aceptación (Validación Rúnica Previa)
El motor de SddIA puede arrancar, escuchar el Bus de Eventos y procesar un evento de dominio no técnico (ej. un recordatorio o notificación de asistente) sin generar errores de compilación o requerir un repositorio Git activo.

La carpeta SddIA/process/ contiene exclusivamente flujos agnósticos o desacoplados mediante códices, eliminando el acoplamiento rígido de arranque.

El Core en Rust compila limpiamente (cargo build --release) manteniendo la Ceguera Espacial absoluta.

# Refinamiento 1.0
Document ID: PBI-SDDIA-DOMAIN-ABSTRACT-01
Title: "[ARQUITECTURA] Separación de Dominio SddIA y Abstracción del Contexto de Ejecución"
Format: markdown
Version: "2.0.0" (Refactorizado)
Status: "pendiente-kitchen"
Priority: Alta (Prerrequisito Bloqueante)
Process: refactor
Suggested Branch: refactor/sddia-domain-abstraction

# 1. Clarificación Estratégica (Filtro B - Soberanía Estructural)
El ecosistema SddIA abandona su concepción legacy como "herramienta de repositorios de código" para formalizarse como un Sistema Nervioso Central Reactivo (Arquitectura Orientada a Eventos - EDA) de propósito general. 

Este PBI establece la Separación de Dominio Estricta: el Motor Core (`SddIA/core/` y `SddIA/engine/`) debe purgar cualquier supuesto implícito de que el estímulo entrante proviene de un Pull Request o requiere un repositorio Git. El motor se limita a arrancar, enrutar la entropía física a través de Cúmulo/Cerbero, e invocar la ejecución S+ Grade, independientemente de si el dominio es código fuente, gestión de correo o la domótica del entorno.

# 2. Especificación Técnica y Hitos de Ejecución (Física del Motor)

## Hito 1: Desacoplamiento del Orquestador (`workspace_init` / `execute-process`)
- **Directriz:** Refactorizar el motor de inicialización para erradicar la dependencia estricta de Git. 
- **Acción:** El orquestador no intentará clonar, validar ramas, ni instanciar contextos de versionado a menos que el Códice de Dominio activo así lo demande explícitamente a través del Enrutamiento Semántico.

## Hito 2: Agnosticismo del Bus de Eventos (`.SddIA/events/domain/`)
- **Directriz:** Generalizar la matriz de consumo del Bus.
- **Acción:** Los *daemons* y el motor de enrutamiento deben ser capaces de procesar estructuras JSON estándar (La Chispa) sin fallar por ausencia de metadatos de repositorios. Un evento tipo `Email_Received` o `Prompt_Submitted` tiene la misma autoridad térmica que un `PullRequest_Merged`.

## Hito 3: Poda Ontológica de Dependencias (Filtro C)
- **Directriz:** Eliminar de la capa Core cualquier acoplamiento a casos de uso específicos (como despliegues Tekton para lenguajes específicos). La capa Core es solo la autopista de información; el cargamento lo define la Librería (Códices).

# 3. Aduana Ontológica y Criterios de Aceptación (S+ Grade)
- [ ] **Independencia del Dominio:** El motor de SddIA puede arrancar, escuchar el Bus de Eventos y procesar un evento de dominio simulado en vacío sin generar errores de compilación (`cargo build --release`) y sin requerir la existencia de una carpeta `.git`.
- [ ] **Cierre de Ciclo Limpio:** La carpeta `SddIA/process/` del Core solo contiene contratos agnósticos.
- [ ] **Hermeticidad Anti-Panic:** Si se inyecta un evento sin el Códice de Dominio adecuado, Cerbero deniega la ejecución por "Falta de Autoridad" en lugar de provocar un colapso del sistema (Ceguera Espacial mantenida).
