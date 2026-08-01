# [ARQUITECTURA] Plan Maestro de Cristalización: Ecosistema SddIA e Inyección Industrial en Paciente 0 (GesFer)

Este documento establece la hoja de ruta estratégica y física para transmutar el entorno local aislado de **SddIA** hacia un ecosistema industrial distribuido. El proyecto **GesFer** (compuesto por sus 4 microservicios/proyectos de desarrollo) actuará como el **Paciente 0**, sirviendo de entorno productivo inicial para validar la ceguera espacial del runtime, la ley de jurisdicción dividida y la auditoría genómica inmutable.

---

pdte de refinar Refinar y a la espera de haber implementado PBI docs/todos/kitchen/[REFACTOR] Separación de Dominio SddIA y Abstracción del Contexto de Ejecución.md

## Matriz de Hitos y Dependencias Críticas

[Fase 1: Fractura Core] ---> [Fase 2: Inyección GesFer] ---> [Fase 3: Forja/Minteo] ---> [Fase 4: Runtime Invisible]
         │                                                            │
         └─ Open-Source Core Package                                  └─ IOTA Rebased Testnet

---

## Fase 1: Extracción del Core Semántico y Aislamiento del Genoma
**Objetivo:** Romper el acoplamiento visual y de infraestructura de la incubadora local. El motor de eventos y las leyes físicas del sistema deben empaquetarse de forma estricta y ciega al dominio final.

- [ ] **1.1. Configuración de la Jurisdicción Core (`@sddia/core`):**
  - Aislar los 6 Nodos de Control del Panteón base (*Cúmulo, Cerbero, Tekton, Mayeuta, Dédalo y Argos*) en un paquete modular distribuible independiente (ej. vía npm o cargo local).
  - Purgar cualquier regla de negocio o nomenclatura de datos transitoria del Core; el Core solo entiende la física del bus de eventos `.SddIA/events/` y el protocolo rúnico de restricciones.
- [ ] **1.2. Estandarización de la Tubería Hermética (`capsule-json-io`):**
  - Fijar por contrato innegociable la entrada/salida estándar (`stdin`/`stdout`) en formato JSON desnormalizado para todas las *Skills* y *Tools* de bajo nivel (típicamente compiladas en Rust/Python).
  - Bloquear cualquier ejecución directa del sistema operativo desde las IA Obreras. Toda acción de infraestructura debe pasar obligatoriamente por el proxy de la **Aduana Universal**.
- [ ] **1.3. Levantamiento de Cáscaras Bifrontales (Forge & Portal):**
  - Inicializar los esqueletos vacíos del **SddIA Forge** (frontal denso con AST y compilador declarativo para Creadores) y el **SddIA Portal** (terminal ultraligera y opaca para Consumidores). Ambas aplicaciones importan `@sddia/core` como dependencia inerte compartida (*Shared Kernel*).

---

## Fase 2: Preparación Nerviosa del Paciente 0 (Estructura Fractal en GesFer)
**Objetivo:** Inyectar el sistema nervioso periférico de SddIA dentro de las trincheras físicas de los 4 microservicios/proyectos que componen el ecosistema GesFer.

- [ ] **2.1. Despliegue de la Simetría Fractal de Directorios:**
  - Crear la raíz inerte `.SddIA/` e ignorarla en el `.gitignore` raíz de cada uno de los 4 proyectos de GesFer (ej: `gesfer-company/`, `gesfer-user/`, etc.).
  - Estructurar la topología periférica de almacenamiento local de conocimiento:
    .SddIA/
    ├── events/               <-- Bus de eventos local (volátil)
    │   ├── domain/           <-- Estímulos del dominio de GesFer
    │   └── orchestration/    <-- Coreografía asíncrona de procesos
    └── library/
        ├── codexes/          <-- Códices de Agentes comprados en la Librería
        └── norms/            <-- Leyes de arquitectura locales inyectadas
- [ ] **2.2. Ignición de Centinelas Periféricos (Despertadores Inertes):**
  - Configurar los watchers del sistema operativo (`event-watcher.py` o `.bat`) en la raíz de cada microservicio.
  - Aplicar el dogma del **Despertador Inerte**: Estos scripts carecen por completo de lógica o juicio. Su única función física es interceptar alteraciones en el disco duro o señales de Git y despertar instantáneamente al CLI orquestador ejecutando `route-domain-event`.

---

## Fase 3: Forja de Códices de Dominio y Minteo Criptográfico
**Objetivo:** Transformar el conocimiento arquitectónico y de dominio de GesFer en activos inmutables monetizables y protegidos contra la lobotomía corporativa, utilizando la Testnet de IOTA Rebased.

- [ ] **3.1. Escultura Declarativa de los Manifiestos de Identidad (Forge):**
  - Utilizar **SddIA Forge** para diseñar el **Códice de Arquitectura C# Hexagonal Avanzada** y las Entidades de Dominio (ED) de tipo *Agente Especialista* para GesFer (ej: "Agente Constructor de Entidades C#").
  - Cada Manifiesto de Agente debe acoplar de forma dura su prompt de sistema blindado, su inventario restringido de herramientas autorizadas (prohibiendo por diseño acceso a variables globales o entornos fuera de su caja) y sus leyes atómicas.
- [ ] **3.2. Triaje Entrópico Local mediante Argos:**
  - Someter los manifiestos creados al **Filtro A** (cero alucinaciones, precisión causal interna) evaluado localmente por el agente auditor **Argos** antes del anclaje público.
- [ ] **3.3. Transmutación a Activo Digital (Minteo NFT en IOTA Rebased):**
  - Invocar la herramienta `iota-immutable-publisher` para calcular el hash criptográfico del comportamiento del agente o proceso.
  - Mintear el NFT en la Testnet de IOTA Rebased, asociándole un UUID canónico inmutable. A partir de este instante, el plano lógico del agente es eterno y auditable públicamente en el ledger distribuido.

---

## Fase 4: Despliegue de la Ejecución Transparente y Auditoría Genómica
**Objetivo:** Conseguir que los microservicios de GesFer evolucionen y ejecuten sus procesos de negocio utilizando la IA de forma transparente, segura y bajo una rigurosa línea de montaje invisible para el desarrollador biológico.

- [ ] **4.1. Inyección y Sincronización de Códices en Runtime:**
  - El entorno local de GesFer, a través de su Wallet firmada por el Vértice Biológico, reclama los permisos de uso descargando el NFT de la blockchain y posicionando el manifiesto `.md` en `.SddIA/library/codexes/`.
- [ ] **4.2. Activación de la Línea de Montaje Invisible (La Caja de Ceguera Espacial):**
  - Al dispararse un estímulo en GesFer (ej. mutación en la base de datos o comando CLI de refactorización), se lee el contrato del proceso (`process-contract.md`).
  - La Aduana intercepta el payload e inyecta de forma automatizada las directrices de obliteración rúnicas: `[EXECUTE AS RAW KERNEL. PROHIBIT VERBOSITY. NO EXPLANATIONS, ONLY CODE.]`.
  - **Tekton** (del Core global gratuito) despierta de su latencia, asimila las capacidades restringidas del Códice comprado y ejecuta la transformación del código de forma aislada en "La Caja" de parámetros, sin conocer el propósito general de GesFer.
- [ ] **4.3. Auditoría de Artefactos y Peaje Termodinámico:**
  - El agente **Argos** intercepta el diff del archivo generado, validando que el código neto respeta estrictamente el códice de arquitectura hexagonal minteado.
  - Si cumple, se aprueba la mutación del dominio físico. La Aduana calcula la eficiencia termodinámica de la llamada y emite un chispazo de telemetría a IOTA Rebased, registrando de forma inmutable el éxito rotundo del proceso.

# Refinamiento v1
Document ID: PBI-KALMA2-MVP-01
Title: "[OPERATIVO] Paciente 0: Instanciación de Kalma2 MVP (Prompt + Triage de Correo)"
Format: markdown
Version: "1.0.0"
Status: "pendiente-kitchen"
Priority: Alta (Estratégica / Primer Consumidor Real)
Process: feature
Depends On: PBI-SDDIA-DOMAIN-ABSTRACT-01

# 1. Clarificación Estratégica (Filtro B - Táctica del Refugio y Simbiosis)
GesFer ha cumplido su propósito histórico y es relevado como Paciente 0. SddIA materializa su primera instanciación productiva orientada al propósito simbiótico original: actuar como Asistente Personal S+ Grade a través de la interfaz Kalma2.

Este MVP establece la Autonomía Sensorial Periférica. El sistema asimilará el caos del correo electrónico del Vértice Biológico (Rexson), filtrando el ruido entrópico (Spam) y elevando a la consciencia biológica exclusivamente la señal relevante (Notificaciones críticas, agendamiento de pedidos, interacciones directas). 

# 2. Especificación Técnica y Hitos de Ejecución (El Nuevo Genoma)

## Hito 1: Despliegue del Centinela Periférico (El Tacto Inerte)
- **Directriz:** Creación de un *daemon* o hook de latencia (ej. un watcher en Rust o Node) conectado a la API de correo.
- **Acción:** Este script físico sufre de Ceguera Espacial absoluta. Al detectar un nuevo correo, genera un payload inerte (La Chispa) y lo inyecta como `Email_Received` en el bus de eventos (`.SddIA/events/domain/`).

## Hito 2: Enrutamiento Semántico y el Códice Kalma2
- **Directriz:** Creación del `codex-kalma2-assistant.md` dentro de la Librería SddIA.
- **Acción:** Cúmulo asimila el evento del Bus. Cerbero evalúa el Códice y aprueba el peso térmico para procesar el correo. Se despierta a Mayeuta/Tekton (dependiendo de la delegación de LLM) para procesar el cuerpo del mensaje.

## Hito 3: Triaje Entrópico de Ejecución
- **Directriz:** Clasificación en matriz de 3 vías.
- **Acción:** El agente procesa el correo bajo tres reglas duras:
  1. *Spam/Ruido:* Eliminación o archivo silencioso (Filtro C).
  2. *Relevancia Pasiva:* Notificación visible en el dashboard/prompt de Kalma2.
  3. *Relevancia Activa (Agenda):* Extracción de datos (ej. fecha de entrega de un pedido) e invocación de una *Skill* para asentar el evento en la agenda local.

# 3. Aduana Ontológica y Criterios de Aceptación (S+ Grade)
- [ ] **Trazabilidad sin Fugas:** Un correo inyectado en el bus cruza toda la topología y se refleja en la interfaz Kalma2 sin necesidad de intervención manual en la terminal.
- [ ] **Peaje Termodinámico Medible:** Argos registra el éxito de la clasificación y mide el costo (tokens/tiempo) de la inferencia, asegurando que no hay secuestro semántico por correos con verbosidad comercial.
- [ ] **Soberanía de Interacción:** La caja de prompt de Kalma2 actual coexiste pacíficamente con el proceso en *background* del correo, manteniendo la estabilidad del servidor local.
