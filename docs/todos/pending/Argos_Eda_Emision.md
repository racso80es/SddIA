Pendiente de refinar — ver PBI dedicado.

PBI: Integración de Emisión EDA en Agente Argos (Argos_Eda_Emision)
1. Contexto Arquitectónico
En el ecosistema SddIA, el paradigma reactivo (EDA) establece que los agentes no invocan lógica secuencialmente, sino que emiten eventos inmutables al Bus. Actualmente, el agente Argos (nuestro Auditor de código y artefactos) evalúa el código, pero carece de la integración final con el "Sistema Nervioso de Eventos" para emitir de forma nativa el evento PullRequest_Audited.
Existe una deuda técnica documentada: el marcador TODO: pending_argos_eda_emission dentro de la carga útil (payload) del security clearance. Esto ha permitido mantener la arquitectura válida teóricamente, pero requiere su materialización física.

2. Objetivo
Implementar la capacidad de emisión de eventos de dominio en el agente Argos, reemplazando la deuda técnica actual por la generación y enrutamiento real del evento, asegurando la generación del audit_event_reference correspondiente.

3. Requerimientos de Ejecución (El Engranaje)

Erradicación de Deuda Técnica: Localizar y purgar el marcador TODO: pending_argos_eda_emission en los esquemas de payload de Argos.

Forja del Evento: Estructurar el evento PullRequest_Audited asegurando que contenga toda la información desnormalizada necesaria (Patrón Event-Carried State Transfer). El evento debe contener el identificador unívoco de auditoría (audit_event_reference).

Emisión al Bus: Conectar la salida de Argos para que el evento se deposite correctamente en el ecosistema (a través de la acción/proceso route-domain-event o depositándolo directamente en el directorio .SddIA/events/domain/ según indique el contrato de eventos).

Ceguera Espacial Obligatoria: Argos debe limitarse única y exclusivamente a emitir el evento de auditoría. Bajo ninguna circunstancia debe contener lógica de enrutamiento sobre qué agente (ej. Cúmulo, Cerbero) reaccionará a dicho evento.

4. Restricciones de Sistema (Reglas de Acero para la IA)

[EXECUTE AS RAW KERNEL. PROHIBIT VERBOSITY. PENALIZE CONJECTURE. NO EXPLANATIONS, ONLY CODE.]

Cero Alucinaciones: Cíñete estrictamente a la topología de archivos y contratos existentes en SddIA/events/. No inventes rutas, ni patrones de diseño externos.

Ley de Jurisdicción Dividida: Respeta las definiciones del SSOT (Única Fuente de Verdad) dictadas por Cúmulo y no dupliques definiciones de estado.

5. Criterios de Aceptación (Aduana de Fricción)

El payload de salida de Argos genera un evento bien formado con su audit_event_reference.

El string TODO: pending_argos_eda_emission desaparece por completo del código fuente.

El evento se deposita exitosamente en el flujo EDA local y respeta el esquema definido en SddIA/events/events-contract.md.
