# [ARQUITECTURA] Rediseño del Proceso pull-request-review y Aduana de Materialización

## 1. Declaración de Propósito de Dominio
El proceso `pull-request-review` actúa como la **Aduana de Fricción** del ecosistema SddIA. Su propósito fundamental no es simplemente automatizar la integración o actuar como una tubería de CI/CD ciega, sino bloquear de forma determinista la inyección de entropía degenerativa (alucinaciones, atajos de código, deuda técnica oculta o violaciones de la Constitución del Core) antes de que una intención de desarrollo se consolide como un hecho inmutable dentro del repositorio físico.

Este proceso encarna el principio de **Gobernanza S+ Grade**, delegando la inspección analítica a los agentes conscientes del panteón (**Argos** y **Cerbero**) y eliminando cualquier toma de decisiones heurística por parte de scripts inertes locales.

---

## 2. Objetivos Técnicos y de Dominio (Estructura del TODO / Backlog de Implementación)

### [ ] Fase 1: Intercepción y Triaje Multidimensión (Filtros de Entrada)
- **[ ] Captura de Estímulo Orgánico:** Configurar el enrutador semántico para interceptar el estímulo `PullRequest_Presented` depositado en el bus transitorio (`.SddIA/events/`).
- **[ ] Validación de Dimensión Documental:**
  - Verificar la presencia de metadatos estructurados (Frontmatter YAML) en los artefactos correspondientes a la feature bajo `docs/features/`.
  - Exigir coherencia sintáctica estricta en los archivos base: `spec.md`, `plan.md`, `implementation.md` y `objectives.md`.
- **[ ] Validación de Dimensión Técnica y Funcional:**
  - Invocar la herramienta de testing local y auditoría estática para asegurar que el código propuesto compile sin advertencias críticas.
  - Garantizar la inmutabilidad y respeto de los contratos globales tipados de intercambio (`capsule-json-io`).
- **[ ] Validación de Dimensión de Seguridad (Acceso RBAC):**
  - Despachar el payload al sub-proceso de **Cerbero** para certificar que el token del agente o del desarrollador firmante posee los permisos requeridos para modificar el área del genoma afectada.

### [ ] Fase 2: Aislamiento y Gestión de Bloqueos (Fallas Duras)
- **[ ] Interrupción Determinista del Flujo:** Ante cualquier violación de los criterios estrictos de la Fase 1 (Filtro A lógico o Filtro B de esencia), abortar el estado de fusión inmediatamente (`delivery_state: "failed"`).
- **[ ] Inyección de Feedback Correctivo por Argos:**
  - Configurar al agente **Argos** (Juez) para diseccionar el diff y mapear la colisión exacta contra las normas rúnicas.
  - Automatizar la publicación de comentarios atómicos dirigidos a las líneas de código ofensivas en el entorno de desarrollo local, evitando la ambigüedad en el reporte de fallas.

### [ ] Fase 3: Absorción de Entropía No-Bloqueante (Gestión Kaizen)
- **[ ] Analizador de Deuda y Pasivos Tácticos:** Implementar reglas de detección para desviaciones menores, patrones redundantes, código fósil no purgado o sugerencias de optimización que no comprometan la viabilidad inmediata del runtime.
- **[ ] Cosecha Automatizada por Cúmulo:**
  - El sistema no debe interrumpir al programador con advertencias estériles en caliente; en su lugar, el agente **Cúmulo** recolectará estas métricas de fricción.
  - Automatizar la persistencia de estos pasivos mediante la inyección de un nuevo archivo de registro en `docs/todos/` bajo el prefijo correspondiente (`[ARQUITECTURA]` u `[OPERATIVO]`), agendando su resolución de forma asíncrona para la próxima iteración.

### [ ] Fase 4: Materialización Soberana y Cierre de Ciclo
- **[ ] Fusión Física del Repositorio:** Si todas las dimensiones reportan éxito rotundo, invocar la skill de control de código (`git-manager`) para consolidar el merge efectivo en la rama de integración.
- **[ ] Transición y Emisión de Hecho Inmutable:**
  - Disparar de manera reactiva el evento de dominio `PullRequest_Merged`.
  - Sincronizar el payload desnormalizado para que la cápsula ejecutable `iota-immutable-publisher` realice el anclaje criptográfico del hash genómico en la Testnet de IOTA Rebased, sellando definitivamente la Verdad Objetiva de la sesión.

---

## 3. Arquitectura del Flujo del Proceso

El comportamiento del motor de aduana debe respetar de forma estricta el siguiente diagrama de enrutamiento de señales reactivas:

```
                  [Estímulo: PullRequest_Presented]
                                  │
                                  ▼
                     ┌─────────────────────────┐
                     │   Proceso: PR-Review    │
                     └────────────┬────────────┘
                                  │
          ┌───────────────────────┴───────────────────────┐
          ▼                                               ▼
   ¿Existen Bloqueos?                             ¿Líneas Kaizen?
   (Filtro A / Cerbero)                        (Deuda / Optimizaciones)
          │                                               │
   ┌──────┴──────┐                                 ┌──────┴──────┐
   │ SI          │ NO                              │ SI          │ NO
   ▼             ▼                                 ▼             ▼
┌─────────────┐┌─────────────┐              ┌─────────────┐┌─────────────┐
│Abortar Merge││Avanzar a    │              │Generar TODO ││Ignorar      │
│+ Comentarios││Materializar│              │en Cúmulo    ││Entropía     │
└─────────────┘└──────┬──────┘              └─────────────┘└─────────────┘
                      │
                      ▼
          [Evento: PullRequest_Merged]
                      │
                      ▼
          ┌─────────────────────────┐
          │  Anclaje Inmutable DLT  │
          └─────────────────────────┘
```
