# [ARQUITECTURA] Puente Sensorial Telegram: Ingesta Externa y Notificación

## 1. Contexto y Visión S+ Grade
Este PBI define la integración bidireccional entre el ecosistema SddIA y Telegram. El objetivo es dotar al Vértice Biológico de un mecanismo de inyección de voluntad (TODOs, aprobaciones) y monitorización reactiva (notificaciones de PRs, fallas) sin necesidad de presencia física en la terminal. Todo ello manteniendo la Ceguera Espacial de las herramientas y aislando el motor de eventos de la entropía de la red pública.

## 2. Configuración de Entorno y Seguridad
La integración requiere las siguientes variables de entorno en el archivo .env del nodo local:
- TELEGRAM_BOT_TOKEN: Token de autorización provisto por BotFather.
- TELEGRAM_ALLOWED_CHAT_ID: Identificador estricto del canal o usuario autorizado. Cualquier interacción desde un ID no registrado se considerará una intrusión y será bloqueada en la Capa 0 (Daemon físico).

## 3. Arquitectura del Flujo Eferente (Salida)
Entidad: Tool send-telegram-notification
- Naturaleza: Script inerte sin jurisdicción lógica.
- Input (JSON): {"message": "Texto a enviar", "parse_mode": "MarkdownV2"}
- Ejecución: Realiza una petición POST a la API de Telegram (/sendMessage) usando el token y el chat_id preconfigurados en el entorno.
- Consumo: Agentes como *Argos* (Auditor) invocarán esta tool al procesar eventos como PullRequest_Presented o System_Fracture_Detected.

## 4. Arquitectura del Flujo Aferente (Entrada)
Se divide estrictamente en dos capas para separar la física de la lógica:

### 4.1. Demonio Físico (El Centinela Inerte)
- Naturaleza: Script Python ejecutado en background (ej. telegram-watcher.py).
- Mecánica: Bucle de Long Polling contra el endpoint /getUpdates de Telegram.
- Jurisdicción: Filtra mensajes por TELEGRAM_ALLOWED_CHAT_ID. Si es válido, extrae el texto puro y ejecuta el CLI de SddIA: 
  execute-process telegram-gateway --payload "{\"text\": \"<texto_limpio>\"}".
- Aislamiento: El daemon no conoce el estado de SddIA, ni qué significa el mensaje. Su memoria se limita a registrar el último update_id procesado.

### 4.2. Proceso Lógico (Telegram Gateway ED)
- Naturaleza: Aduana cognitiva (Proceso SddIA).
- Mecánica: Recibe el payload en texto plano. Analiza la intención semántica del texto mediante el orquestador (o un agente como Mayeuta/Tekton dependiendo de la complejidad).
- Transmutación: - Si el texto sigue un patrón predefinido (ej. "TODO: Revisar auditorías"), emite un evento de dominio estandarizado: Kaizen_Idea_Captured o Manual_Task_Requested.
  - Si el texto es una consulta, extrae el contexto y despacha a la acción correspondiente.
- Inyección: Inyecta el evento tipado y limpio en .SddIA/events/ para que la Ola C lo coreografíe.

## 5. Criterios de Aceptación (DoD)
1. Inmunidad: Un mensaje de un chat_id desconocido no debe activar ninguna acción ni consumir tokens de IA.
2. Idempotencia: El reinicio del demonio físico no debe reprocesar mensajes antiguos de Telegram (gestión correcta del update_id).
3. Ceguera Espacial: La tool de salida no debe conocer el motivo de la notificación, solo ejecuta el envío.
4. Desacople: El Gateway ED debe ser capaz de emitir al menos un evento básico (ej. Manual_Task_Requested) y depositarlo correctamente en el bus local.


# Referencias
https://core.telegram.org/bots/api
TELEGRAM_BOT_TOKEN = { A indicar}
TELEGRAM_ALLOWED_CHAT_ID = {A Indicar}
