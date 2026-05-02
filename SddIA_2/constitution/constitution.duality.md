# Principio Constitucional: Dualidad Operativa

## Propósito
Separar claramente las responsabilidades del sistema entre el "Jefe" (Control) y la "Calma" (Ejecución), permitiendo la auditoría independiente y un flujo de datos robusto.

## Definición
Este principio establece la separación entre modos de operación y el canal de comunicación entre ellos.

## Directrices
1. **Separación de Poderes:** El modo Boss (configuración, decisiones estratégicas) y el modo Calm (ejecución autónoma) operan con reglas distintas.
2. **Canal de Comunicación:** La comunicación entre componentes debe ser estandarizada y auditable.
3. **Modos de Operación:** El sistema soporta modos operativos distintos gestionados por el estado global.

## Estado Actual
- **En GesFer.Admin.Front:** No aplica IPC ni Electron (frontend web). El principio de dualidad Boss/Calm se conserva a nivel conceptual para el ecosistema SddIA.
- **Modos:** Aplicables a nivel de agentes SddIA y procesos, no a nivel de arquitectura de la app frontend.
