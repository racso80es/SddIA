[ PBI / ToDo: La Cocina ]
Título: Implementación Fase 1.0.0 - Telemetría Reactiva SddIA (Aduana Universal)
Tipo: Tarea de Arquitectura Core
Descripción: Establecer el motor de telemetría de SddIA basándose en el principio de Interceptación Central. Ninguna entidad se audita a sí misma.

Tareas de Forja (A fuego lento):

Añadir Cláusula Legal: Modificar SddIA/CONSTITUTION_CORE.md inyectando la "Prohibición de Invocación Directa y Peaje de Telemetría".

Refactorización del Motor CLI: Modificar execute-process (o equivalente) para que inicie la medición termodinámica (cronómetro) antes de ejecutar cualquier cápsula, acción o agente.

Emisión de Eventos Crudos: Programar la inyección del evento Raw_Execution_Finished en el bus .SddIA/events/ interceptando el exit code, el execution time y el asset_id.

Diseño del Juez: Preparar el contrato de Argos para que reaccione (despierte) ante estos eventos crudos, consolide el éxito/rendimiento y los registre en el Ledger de telemetría.