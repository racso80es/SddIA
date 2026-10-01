---
document_id: PBI-SDDIA-LINEAR-CORE-001
title: "[ARQUITECTURA] Forja de la Cápsula Linear Tracker y Orquestación del Ciclo de Vida"
format: markdown
version: "1.0.0"
created: "2026-10-01"
status: "abierto"
priority: alta
process: feature
related:
  - SddIA/norms/capsule-json-io.md
  - SddIA/process/delivery-close-cycle.md
---

### [ARQUITECTURA] Forja de la Cápsula Linear Tracker y Orquestación del Ciclo de Vida

#### 1. Descripción General
**Como** Arquitecto del Core SddIA y agentes orquestadores (Tekton, Argos, Cúmulo),
**Quiero** disponer de una cápsula física (Tool) inmutable `linear-tracker-adapter` y refactorizar los procesos del ciclo de desarrollo,
**Para** erradicar la dependencia de Git como base de datos de estado de los requerimientos, permitiendo a los agentes interactuar nativamente con la API GraphQL de Linear para leer PBIs y anclar confirmaciones sin generar commits documentales espurios.

#### 2. Criterios de Aceptación (Protocolo de Acero)
- [ ] **Contrato y Definición (SSOT):** El contrato de la herramienta (`spec.md`, `spec.json`) está anclado en `SddIA/tools/linear-tracker-adapter/`, documentando explícitamente el uso de GraphQL para tres operaciones atómicas: `fetch_issue`, `update_issue_state`, y `create_comment` (para anclaje de hashes).
- [ ] **Frontera de Invocación y Secretos:** La cápsula (Rust WASI o binario nativo) cumple estrictamente el estándar de entrada/salida de `SddIA/norms/capsule-json-io.md`. No contiene credenciales hardcodeadas; consume `LINEAR_API_KEY` directamente de la bóveda de la instancia en tiempo de ejecución.
- [ ] **Delegación de Orquestación (Procesos):** Los contratos de procesos base de SddIA (ej. `SddIA/process/delivery-close-cycle.md` o análogos de bug-fix) han sido refactorizados. Sustituyen las directrices de "actualizar el archivo markdown" por "invocar `linear-tracker-adapter` para transicionar el estado del ticket y anclar el SHA del commit".
- [ ] **Actualización del Peaje RBAC:** El catálogo de Cerbero ha sido actualizado para autorizar explícitamente a los agentes pertinentes (Tekton, Argos) el uso de la nueva tool de infraestructura.
- [ ] **Ceguera Operativa Probada:** La tool se limita a ejecutar peticiones GraphQL ciegas a partir del `stdin` y devuelve un payload JSON estándar por `stdout` sin tomar decisiones de dominio.
