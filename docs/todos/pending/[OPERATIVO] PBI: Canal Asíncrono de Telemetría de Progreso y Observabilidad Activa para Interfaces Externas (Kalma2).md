[OPERATIVO] PBI: Canal Asíncrono de Telemetría de Progreso y Observabilidad Activa para Interfaces Externas (Kalma2)
0. Pendiente de refinamiento. Aplicar especial atención a incongruencias e inexactitudes.

1. Especificación (Spec)
Propósito: Dotar al ecosistema SddIA de un mecanismo de expresión asíncrono y desacoplado para la emisión de trazas de progreso en tiempo real durante la ejecución de procesos (ej. kalma2-process-requested o procesamiento de PBIs).

Alcance Táctico:

Definición del contrato y esquema semántico para cápsulas de telemetría efímera (.SddIA/events/telemetry/).

Inyección de trazas de progreso por fases (Spec, Clarify, Plan, Implementation, Validation) desde el orquestador en Rust sin violar la Ceguera Espacial.

Implementación de un canal de difusión Server-Sent Events (SSE) en kalma2-bridge alimentado por eventos del sistema de archivos.

Consumo y renderizado reactivo de trazas cromáticas en la WUI de Kalma2.

Circuito de limpieza termodinámica (sweeper) para eventos de telemetría huérfanos o finalizados.

2. Clarificación y Restricciones de Dominio (Clarify)
Principio de Ceguera Espacial: El orquestador SddIA ignora la existencia de clientes externos. La emisión es estrictamente de tipo fire-and-forget.

Diferenciación de Eventos:

Eventos de Dominio (/domain/): Representan hechos inmutables, requieren consenso/libro mayor (delivery_state) y persistencia/anclaje DLT.

Eventos de Telemetría (/telemetry/): Representan el flujo volátil de ejecución. Se descartan sin reintentos en caso de fallo del receptor.

Esquema JSON del Evento de Telemetría (telemetry_trace.json):

JSON
{
  "event_id": "UUID-v4",
  "process_id": "UUID-v4-del-proceso-padre",
  "timestamp": "ISO-8601",
  "phase": "Spec | Clarify | Plan | Implementation | Validation | Closure",
  "severity": "info | warn | error | kaizen_alert",
  "source_agent": "cerbero | mayeuta | dedalo | tekton | argos | cumulo",
  "message": "Descripción legible del hito operativo",
  "metadata": {}
}
3. Plan de Acción Técnico (Plan)
Fase 1: Estandarización y Esquema de Telemetría
Creación de la norma y contrato semántico en .SddIA/library/norms/capability-contracts/telemetry.trace.schema.json.

Confirmar el aislamiento del directorio runtime .SddIA/events/telemetry/ respecto a /domain/ y su inclusión en .gitignore.

Fase 2: Emisión en Motor de Ejecución (Rust Core)
Extender el módulo de orquestación en Rust (sddia-core / execute-process) con la función emit_telemetry_trace(phase, severity, agent, message).

Inyectar chispazos de telemetría automática al inicio y fin de cada fase del ciclo de vida (Spec, Clarify, Plan, Implementation, Validation).

Fase 3: El Puente Sensorial Reactivo (kalma2-bridge)
Incorporar un observador de sistema de archivos (notify / inotify) en kalma2-bridge apuntando a .SddIA/events/telemetry/.

Exponer el endpoint HTTP GET /api/events/stream?process_id={id} configurado como Server-Sent Events (text/event-stream).

Transmitir inmediatamente la traza JSON entrante a la conexión SSE activa filtrando por process_id.

Fase 4: Integración en la Interfaz de Usuario (Kalma2 WUI)
Suscribir el cliente Web (index.html / app.js) al flujo SSE tras enviar una petición de procesamiento.

Implementar el panel de consola reactiva:

Sustituir el polling pasivo a /api/status por la recepción continua de trazas.

Formateo cromático según severity (info: azul/blanco, warn: amarillo, error: rojo, kaizen_alert: violeta).

Auto-scroll y badge visual del agente emisor ([Tekton], [Argos], etc.).

Fase 5: Higiene Termodinámica y Poda
Actualizar el daemon event-sweeper para barrer las trazas en .SddIA/events/telemetry/ cuyos process_id correspondan a ejecuciones cerradas o expiradas (tiempo mayor a T+N minutos).

4. Criterios de Aceptación y Validación (Validation - Grade S+)
[ ] Desacople Absoluto: Si kalma2-bridge colapsa o está apagado, execute-process completa la tarea exactamente en el mismo tiempo sin errores ni reintentos bloqueantes.

[ ] Latencia de Refresco: Las trazas se reflejan en la WUI en menos de 100ms desde su escritura física en el bus de telemetría.

[ ] Integridad Estructural: Ningún evento de telemetría contamina el bus de dominio (.SddIA/events/domain/) ni requiere anclaje DLT.

[ ] Compilación e Higiene: Código en Rust libre de panics o warnings (cargo check --release). Ausencia de acumulación de archivos residuales tras la poda.