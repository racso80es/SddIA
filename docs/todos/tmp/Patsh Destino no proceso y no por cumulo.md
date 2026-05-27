---
status: consolidado
superseded_by: docs/todos/pending/[ARQUITECTURA] Telemetría Reactiva — Unificación EDA S+ Grade.md
document_id_unified: PBI-TELEMETRIA-REACTIVA-EDA-UNIFICADO
unified_phase: "Fase 2 (Workspaces dinámicos — anti-sesgo de origen)"
archived_from: docs/todos/pending/Patsh Destino no proceso y no por cumulo.md
---

> **Documento archivado — unificado**
> Este PBI ha sido consolidado en [`[ARQUITECTURA] Telemetría Reactiva — Unificación EDA S+ Grade`](../pending/[ARQUITECTURA] Telemetría Reactiva — Unificación EDA S+ Grade.md) (`document_id: PBI-TELEMETRIA-REACTIVA-EDA-UNIFICADO`). Corresponde a la **Fase 2** del documento unificado. No ejecutar como ítem independiente.

# [ PBI / ToDo: La Cocina ]
**Título:** Refactorización Ontológica: Espacios de Trabajo Dinámicos (Workspaces) vs. Directorios Estáticos  
**Naturaleza:** Arquitectura Core / Evolución a Fase de Industrialización  
**Estatus:** Pendiente de Ejecución  
**Prioridad:** Alta (Bloqueante para la generalización operativa)  

---

## 1. Síntesis y Propósito
El objetivo de este PBI es erradicar el "Sesgo de Origen" en el motor SddIA. Se debe abandonar la nomenclatura y dependencia de rutas rígidas ligadas al desarrollo de software (ej. `paths.featurePath`, `paths.fixPath`) para abrazar un modelo universal basado en **Espacios de Trabajo Aislados (Workspaces) dinámicos e impermanentes**. 

Cualquier proceso (sea ingeniería, legal, investigación o gestión documental) debe poder instanciar su propio territorio operativo sin ensuciar el ecosistema general ni romper la Ceguera Espacial de las Entidades de Dominio.

## 2. Directrices de Ejecución (Tareas de Forja)

### A. Actualización del Contrato de Procesos (Genoma)
* Modificar el estándar de definición de procesos (`SddIA/process/process-contract.md`).
* A partir de ahora, cada proceso (`spec.md` o `spec.json`) debe declarar obligatoriamente un parámetro `workspace_template`. 
* *Ejemplo de declaración:* `workspace_template: ".SddIA/workspaces/{process_name}/{execution_id}/"`

### B. Instanciación en la Aduana (Motor de Orquestación)
* Al arrancar la ejecución de un proceso, el Orquestador inerte (CLI) será el único responsable de parsear la plantilla definida en el contrato.
* El Orquestador generará un identificador único (UUID o Hash temporal de ejecución) y materializará la creación de la carpeta física en el sistema operativo antes de invocar la primera Acción.

### C. Inyección de Contexto (Conservación de la Ceguera Espacial)
* Las Entidades de Dominio (Agentes Obreros, Auditores) tienen terminantemente prohibido asumir o adivinar dónde están trabajando.
* El Orquestador, al emitir el Evento de Dominio que despierta al agente, debe inyectar la coordenada espacial absoluta del *Workspace* en el *payload* del evento.
* El agente leerá y mutará artefactos operando **única y exclusivamente** dentro de las fronteras de esa coordenada inyectada.

### D. Purga del SSOT (Única Fuente de Verdad)
* Refactorizar el archivo de rutas maestro gestionado por Cúmulo (`SddIA/agents/cumulo.paths.json`). 
* Eliminar las referencias estáticas a "features" o "fixes" y definir la raíz universal de operaciones como `.SddIA/workspaces/`.

---

## 3. Criterios de Aceptación (Definition of Done)
* **AC1:** Un proceso no ligado a desarrollo de software se puede ejecutar sin arrojar errores de ruta.
* **AC2:** El motor CLI crea dinámicamente la carpeta del Workspace inyectando un UUID único por cada ejecución de proceso.
* **AC3:** Las instrucciones enviadas a Tekton, Dédalo o Argos ya no mencionan directorios absolutos del repositorio, sino que limitan su visión exclusivamente al *Workspace* inyectado en el contexto de su evento táctico.

## NOTAS:
* Desde cumulo si que se indicará la ruta base de tal manera que la indicada por el proceso será parcialrelativa.
