# [ARQUITECTURA] Rediseño del Proceso pull-request-review y Aduana de Materialización

> **Feature activa:** `docs/features/pull-request-review-redesign/` — rama `feat/pull-request-review-redesign` (iniciada 2026-05-22).
> **Triaje genoma:** existe `SddIA/process/pull-request-review.md` v1.0.0 (placeholder, sin handler lab); requiere reescritura v2, no borrado.

## 1. Declaración de Propósito de Dominio
El proceso `pull-request-review` actúa como la **Aduana de Fricción** del ecosistema SddIA. Su propósito fundamental no es simplemente automatizar la integración o actuar como una tubería de CI/CD ciega, sino bloquear de forma determinista la inyección de entropía degenerativa (alucinaciones, atajos de código, deuda técnica oculta o violaciones de la Constitución del Core) antes de que una intención de desarrollo se consolide como un hecho inmutable dentro del repositorio físico.

Este proceso encarna el principio de **Gobernanza S+ Grade**, delegando la inspección analítica a los agentes conscientes del panteón (**Argos** y **Cerbero**) y eliminando cualquier toma de decisiones heurística por parte de scripts inertes locales.

---

## 2. Objetivos Técnicos y de Dominio (Estructura del TODO / Backlog de Implementación)

### [x] Fase 1: Intercepción y Triaje Multidimensión (Filtros de Entrada)
- **[x] Captura de Estímulo Orgánico:** Suscriptor `pull-request-review` en `event-subscriptions.json` + watcher.
- **[x] Validación de Dimensión Documental:** Handler `pr-review-documental` (frontmatter + artefactos base).
- **[x] Validación de Dimensión Técnica y Funcional:** Handler `pr-review-technical` + `verify-process-integrity`.
- **[x] Validación de Dimensión de Seguridad (Acceso RBAC):** Handler `pr-review-rbac` (Cerbero lab stub).

### [x] Fase 2: Aislamiento y Gestión de Bloqueos (Fallas Duras)
- **[x] Interrupción Determinista del Flujo:** `delivery_state: failed` + `status_code: 1`.
- **[x] Inyección de Feedback Correctivo por Argos:** `argos_feedback` en handler veredicto (lab).

### [x] Fase 3: Absorción de Entropía No-Bloqueante (Gestión Kaizen)
- **[x] Analizador de Deuda y Pasivos Tácticos:** Flag `SDDIA_LAB_PR_REVIEW_KAIZEN`.
- **[x] Cosecha Automatizada por Cúmulo:** Persistencia en `docs/todos/[OPERATIVO]`.

### [x] Fase 4: Materialización Soberana y Cierre de Ciclo
- **[x] Handoff accept-pr:** Fase 7 (sin merge directo en aduana); watcher omite fusión con `SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF`.

> Evidencia: `docs/features/pull-request-review-redesign/validacion.md` — E2E `62bcb6e1-f995-4edf-95d6-3745c7503303`.

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
│+ Comentarios││Materializar │              │en Cúmulo    ││Entropía     │
└─────────────┘└─────────────┘              └─────────────┘└─────────────┘

```
