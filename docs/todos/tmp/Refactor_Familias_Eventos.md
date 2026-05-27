---
status: consolidado
superseded_by: docs/todos/pending/[ARQUITECTURA] Telemetría Reactiva — Unificación EDA S+ Grade.md
document_id_unified: PBI-TELEMETRIA-REACTIVA-EDA-UNIFICADO
unified_phase: "Fase 1 (Familias de eventos — Trinidad de Estímulos)"
archived_from: docs/todos/pending/Refactor_Familias_Eventos.md
---

> **Documento archivado — unificado**
> Este PBI ha sido consolidado en [`[ARQUITECTURA] Telemetría Reactiva — Unificación EDA S+ Grade`](../pending/[ARQUITECTURA] Telemetría Reactiva — Unificación EDA S+ Grade.md) (`document_id: PBI-TELEMETRIA-REACTIVA-EDA-UNIFICADO`). Corresponde a la **Fase 1** del documento unificado. No ejecutar como ítem independiente.

# [ PBI / ToDo: La Cocina ]
**Título:** Refactorización Genómica: Topología Fractal de Eventos y Contratos de Emisión  
**Naturaleza:** Arquitectura Core / Reestructuración de Entidades de Dominio (ED)  
**Estatus:** Pendiente de Ejecución  
**Prioridad:** Alta (Pre-requisito para la creación de nuevos eventos SddIA)  

---

## 1. Síntesis y Propósito
El objetivo de este PBI es aplicar el principio de Simetría Fractal a la carpeta del Genoma de eventos (`SddIA/events/`), reflejando la estructura de triple ruta del Runtime (`.SddIA/events/`). 

Se debe erradicar la topología plana, instanciar los Códices de Familia (índices), blindar el contrato maestro para obligar a la tipificación de estímulos, y actualizar la Línea de Montaje de creación de eventos para que la IA Obrera opere con Ceguera Espacial optimizada.

## 2. Directrices de Ejecución (Tareas de Forja)

### A. Topología Física del Genoma
* Crear las tres subcarpetas dentro de `SddIA/events/`: `telemetry/`, `orchestration/`, y `domain/`.
* Crear un archivo `index.md` en el interior de cada subcarpeta. Este archivo actuará como "Códice de Familia" y única fuente de verdad (SSOT) para esa ruta, fusionando propósito, definición y catálogo de esquemas. Queda prohibida la duplicidad con archivos `README.md`.
* Mantener en la raíz (`SddIA/events/`) única y exclusivamente el archivo `events-contract.md`.

### B. Mutación del Contrato Base (events-contract.md)
* Refactorizar el contrato para incluir un nuevo campo obligatorio en el bloque de definición (ej. `event_family`).
* Este campo debe ser un *Enum* estricto de tres valores admitidos: `telemetry`, `orchestration`, `domain`.
* Modificar las reglas de auditoría para que Argos rechace cualquier evento teórico que no se adscriba a una de estas tres familias.

### C. Actualización del Proceso (`create-event` o similar)
* Modificar el blueprint del proceso encargado de la creación de eventos.
* El proceso debe exigir el parámetro `event_family` como input innegociable antes de arrancar la primera acción.
* Configurar el enrutamiento interno del proceso para que el *Workspace* dinámico o el contexto inyectado a los agentes apunte directamente a la subcarpeta seleccionada, y no a la raíz de eventos. El agente leerá las normas específicas del `index.md` de la carpeta destino y depositará el nuevo `.schema.json` en ella.

---

## 3. Criterios de Aceptación (Definition of Done)
* **AC1:** La carpeta `SddIA/events/` solo contiene `events-contract.md` y tres subcarpetas. Ningún esquema suelto reside en la raíz.
* **AC2:** Cada subcarpeta posee su propio `index.md` detallando su jurisdicción operativa y los agentes autorizados a emitir.
* **AC3:** El contrato `events-contract.md` obliga explícitamente a clasificar todo nuevo evento dentro de la trinidad de estímulos.
* **AC4:** El proceso de creación de entidades tipo Evento deposita físicamente los nuevos esquemas en la subcarpeta correspondiente sin intervención humana posterior a la declaración del input inicial.
