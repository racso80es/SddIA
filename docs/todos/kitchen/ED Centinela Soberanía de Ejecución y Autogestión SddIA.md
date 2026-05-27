# [ARQUITECTURA] ED Centinela: Soberanía de Ejecución y Autogestión SddIA

## 1. Visión S+ Grade y Contexto
Transformar los scripts de monitorización físicos (*watchers*, *daemons*) de "satélites inertes y dispersos" a Entidades de Dominio (ED) gobernadas por contrato. SddIA debe adquirir la capacidad táctica de arrancar, auditar y purgar sus propios procesos de frontera, garantizando que el bus de eventos mantenga su latido sin depender de la intervención manual del Vértice Biológico.

## 2. Definición del Contrato del Centinela (La Ley Física)
Todo proceso periférico que aspire a ser un Centinela de SddIA debe registrarse mediante un contrato estricto en la topología SddIA/daemons/.
        
### 2.1. Anatomía de la Cápsula (Propuesta spec.json)
\\\`json
{
  "domain_entity": "Centinela",
  "id": "telegram-watcher",
  "description": "Demonio de escucha de long-polling para el canal aferente de Telegram",
  "execution": {
    "entrypoint": "scripts/daemons/telegram-watcher.py",
    "runtime": "python3",
    "heartbeat_interval_seconds": 30
  },
  "jurisdiction": "Aislada - Ceguera Lógica. Solo inyecta eventos físicos en el bus"
}
\\\`

### 2.2. Obligaciones Termodinámicas
1. Telemetría Obligatoria: El demonio es lógica y espacialmente ciego, pero es responsable de inyectar un evento Daemon_Heartbeat en .SddIA/events/ periódicamente.
2. Identidad Física: Al despertar, el Centinela debe crear un archivo .lock con su PID en .SddIA/daemons/status/ para garantizar que no existan ejecuciones duplicadas.
3. Idempotencia: Debe ser capaz de morir y reiniciar sin volver a procesar estímulos antiguos del entorno.

## 3. Especificación Técnica: Capa de Control Táctico
El Core SddIA requiere un actuador para manipular estos procesos en el sistema operativo subyacente.

- ED Acción governance-daemon-manager: Un subproceso interno del Core con autoridad para hacer spawn (arrancar), status (auditar) y kill (destruir) de los procesos definidos en los contratos.
- Protocolo de Purga (Kill-Switch): Gestión estricta de las señales SIGTERM y SIGKILL. Cuando el motor SddIA entra en latencia, todos los procesos hijos amparados bajo la ED Centinela deben apagarse limpiamente. Se prohíbe la generación de procesos zombie.

## 4. Desglose de Tareas de Ejecución (Kaizen)

| ID | Tarea | Prioridad | Definición de Hecho (DoD) |
| :--- | :--- | :--- | :--- |
| CEN-01 | Contrato Base: Crear SddIA/daemons/daemons-contract.md y estructura de directorios. | Crítica | El documento define las normas de aislamiento, telemetría y el formato JSON explícitamente. |
| CEN-02 | Acción Manager: Desarrollar daemon-manager.py invocable vía orquestador. | Alta | SddIA puede lanzar execute-process governance-daemon-manager --action start --id telegram-watcher. |
| CEN-03 | Políticas de Purga (Kill Switch): Implementar manejo de señales de terminación. | Alta | Al detener el Core, ningún proceso hijo Centinela queda huérfano consumiendo memoria o bloqueando puertos. |
| CEN-04 | Refactor Inertes Actuales: Adaptar event-watcher y github_bridge_watcher al nuevo estándar. | Media | Los demonios legacy emiten heartbeats al bus y poseen su propio spec.json. |
| CEN-05 | Triaje Argos: Enseñar al agente auditor (Argos) a leer la telemetría térmica. | Baja | Argos levanta una alerta System_Fracture_Detected si un Centinela crítico omite su latido durante 3 ciclos seguidos. |

## 5. Control de Riesgos (Filtro A)
- Riesgos de Permisos (SO): Se debe garantizar que la manipulación de procesos se ciña al espacio de usuario y no solicite escalada de privilegios a root/sudo en el entorno, protegiendo el aislamiento de la terminal.
- Saturación Entrópica del Bus: Evaluar si la inyección continua de Daemon_Heartbeat ensucia el historial de eventos principal. Si es así, se redirigirá la telemetría a una subcarpeta específica .SddIA/events/telemetry/ que escape al registro histórico DLT.