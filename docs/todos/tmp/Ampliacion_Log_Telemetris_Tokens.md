---
status: consolidado
superseded_by: docs/todos/pending/[ARQUITECTURA] Telemetría Reactiva — Unificación EDA S+ Grade.md
document_id_unified: PBI-TELEMETRIA-REACTIVA-EDA-UNIFICADO
unified_phase: "Fase 5 (Cumplimiento termodinámico — recibos de tokens)"
archived_from: docs/todos/pending/Ampliacion_Log_Telemetris_Tokens.md
---

> **Documento archivado — unificado**
> Este PBI ha sido consolidado en [`[ARQUITECTURA] Telemetría Reactiva — Unificación EDA S+ Grade`](../pending/[ARQUITECTURA] Telemetría Reactiva — Unificación EDA S+ Grade.md) (`document_id: PBI-TELEMETRIA-REACTIVA-EDA-UNIFICADO`). Corresponde a la **Fase 5** del documento unificado. No ejecutar como ítem independiente.

# [ PBI / ToDo: La Cocina ]
**Título:** Auditoría de Cumplimiento Termodinámico (Contratos de Recibo en ED)  
**Naturaleza:** Arquitectura Core / Gobernanza Reactiva  
**Estatus:** Pendiente de Ejecución (Vinculado a Fase 1.0.0 Telemetría Base)  
**Prioridad:** Media (Evolución de la "Física del Valor")  

---

## 1. Síntesis y Propósito
Este PBI define el mecanismo asíncrono para auditar si una Entidad de Dominio (ED) cumple con su promesa de entregar métricas de consumo (ej. tokens de LLM) tras su ejecución, sin bloquear el motor físico del ecosistema. 

La infraestructura (CLI) debe ser tolerante a la omisión de estos datos para no detener la Línea de Montaje, pero el ecosistema debe ser capaz de detectar la infracción a nivel de contrato y generar un Evento de Dominio de alerta (`Telemetry_Compliance_Breached`) para la posterior actuación de Cerbero o Radamanto.

## 2. Directrices de Ejecución (Tareas de Forja)

### A. Mutación del Contrato Base de ED (El Genoma)
* Ampliar el estándar de contratos de las entidades (`skills-contract.md`, `actions-contract.md`).
* Añadir una propiedad declarativa obligatoria en el `spec.json` o `spec.md` denominada `telemetry_provided` (boolean) o `telemetry_schema` (para definir qué valores exactos se compromete a devolver, ej. `prompt_tokens`).

### B. Tolerancia en la Aduana (CLI Inerte)
* El Orquestador (`execute-process`) interceptará el `stdout` de la cápsula. 
* Si la cápsula devuelve el bloque `telemetry_receipt`, el CLI lo anexa al evento crudo `Raw_Execution_Finished`. 
* **Directriz de Falla Suave:** Si la cápsula omite el bloque, el CLI **no arrojará error**. Simplemente emitirá el evento crudo únicamente con las métricas físicas (tiempo de reloj y exit code), dejando el recibo de tokens vacío.

### C. El Bucle de Auditoría Asíncrona (El Juez de Contratos)
* Instanciar una regla de auditoría (asignada temporalmente a Argos o a un sub-proceso de Radamanto) suscrita al bus de telemetría.
* Al leer un `Raw_Execution_Finished`, el agente cruzará la información real devuelta contra el contrato original (`spec`) de la ED invocada.
* **Detonación de Dominio:** Si el contrato estipulaba `telemetry_provided: true` y el recibo llegó vacío, el agente emitirá un Evento de Dominio de nivel 3: `Telemetry_Compliance_Breached`.

### D. Gobernanza Futura (Placeholder)
* Queda pendiente definir la reacción del ecosistema ante el evento `Telemetry_Compliance_Breached` (ej. degradación de reputación de la herramienta, bloqueo tras 'N' infracciones, o auto-reparación por parte de Tekton).

---

## 3. Criterios de Aceptación (Definition of Done)
* **AC1:** El CLI ejecuta herramientas que no devuelven tokens sin detener el proceso ni marcar la ejecución como fallida.
* **AC2:** Un contrato de ED puede declarar explícitamente si genera o no recibos termodinámicos.
* **AC3:** El sistema inyecta exitosamente el evento `Telemetry_Compliance_Breached` en la ruta `.SddIA/events/domain/` cuando se detecta el incumplimiento del contrato.
