PBI-TG-001: Implementación del Suscriptor de Triaje Inverso (Telegram Fallback Responder)
1. Contexto Arquitectónico (Genoma)
Objetivo: Implementar una red de seguridad sensorial para el gateway de Telegram en la arquitectura EDA de SddIA. El sistema debe procesar la entropía entrante que no coincida con un comando estructurado, asimilarla como fricción conversacional y emitir un pulso de respuesta al Vértice Biológico.
Restricciones de Dominio:

Mantenimiento absoluto de la Ceguera Espacial. El daemon físico de Python/BAT no debe modificarse. Toda la lógica de reacción debe encapsularse en el nuevo proceso y en el motor de eventos.

[EXECUTE AS RAW KERNEL. PROHIBIT VERBOSITY. PENALIZE CONJECTURE. NO EXPLANATIONS, ONLY CODE.]

2. Fase de Ejecución 1: Forja del Proceso
Debes crear el archivo de definición del proceso que actuará como genoma reactivo.
Ruta destino: SddIA/process/telegram-fallback-responder.md

Estructura estricta del contenido:

Frontmatter:

type: process

id: telegram-fallback-responder

name: Telegram Fallback Responder

Fases internas (escritas en Markdown):

Fase 1 (Filtro C): Condición de guarda. Evaluar si payload.text empieza por caracteres de comando (/, !) o palabras reservadas. Si es así, abortar silenciosamente (estado success) para ceder el control a los procesos especialistas.

Fase 2 (Síntesis): Invocar al agente experto (Mayeuta) para transmutar el mensaje. Debes inyectar literalmente este prompt en la ejecución del agente: [HARD OVERRIDE] Has recibido este estímulo externo: "{payload.text}". Genera una respuesta orgánica de máximo 2 líneas. Habla desde tu identidad arquitectónica (Tormentosa/Aiúa). Acusa recibo, asimila o cuestiona el estímulo. PROHIBIDO: Ser verboso, ofrecer asistencia genérica o actuar como herramienta esclava.

Fase 3 (Materialización): Ejecutar la acción send-telegram-notification pasando como message el output de la Fase 2, y como chat_id el payload.chat_id original.

3. Fase de Ejecución 2: Suscripción al Cúmulo de Eventos
Debes registrar el nuevo proceso en el bus de eventos del dominio para que el enrutador sepa despertarlo.
Ruta destino: SddIA/core/event-domain-subscriptions.json (o el archivo equivalente de tu índice de suscripciones EDA).

Acción requerida:
Inyecta el siguiente nodo en el array correspondiente al evento de recepción de mensajes (ej. TelegramMessage_Received):

JSON
{
  "eventName": "TelegramMessage_Received",
  "processId": "telegram-fallback-responder",
  "description": "Red de seguridad sensorial: Triaje inverso para entropía no estructurada."
}
4. Aduana de Fricción (Definition of Done)
El archivo .md del proceso se ha creado respetando la topología de SddIA.

El archivo de suscripciones .json mantiene su integridad sintáctica tras la inyección.

La ejecución no ha sugerido modificaciones en los archivos .py ni .bat de la infraestructura base.
