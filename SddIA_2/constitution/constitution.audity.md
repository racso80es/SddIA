# Principio Constitucional: Gobernanza Ética y Auditoría Inmutable

## Propósito
Integrar un sistema de auditoría inmutable que actúe como la "conciencia" del sistema, validando y registrando acciones críticas para la transparencia.

## Definición
Este principio establece la necesidad de un registro criptográfico de acciones y un sistema de reglas de validación.

## Directrices
1. **Auditoría Inmutable:** Cada acción crítica debe ser registrada de forma irreversible (hash-chaining).
2. **Reglas de Validación:** Interceptar acciones para validar su conformidad con las normas del proyecto.
3. **Transparencia ("Caja de Cristal"):** Proporcionar visibilidad sobre el estado de salud del proyecto y las decisiones tomadas.

## Estado Actual
- **En GesFer.Admin.Front:** Auditoría vía SddIA (paths.auditsPath, agente auditor frontend). No aplica Logger.ts/Electron.
- **Reglas:** SddIA/norms y agentes (auditor, process-interaction).
