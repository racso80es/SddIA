---
document_id: PBI-KAIZEN-CAPSULA-IMAP-TRIAJE
title: "[KAIZEN] Evolución de Cápsula IMAP y Activación de Triaje Interactivo"
format: markdown
version: "1.0.0"
created: "2026-08-19"
status: "abierto"
priority: alta
process: feature
related:
  - SddIA/norms/capsule-json-io.md
  - SddIA/events/events-contract.md
---

### [KAIZEN] Evolución de Cápsula IMAP y Activación de Triaje Interactivo

#### 1. Especificación (`spec.md`)

**Propósito:**
Elevar la cápsula de gestión de correos (Paciente Cero) de un simple lector pasivo a un activo de Grado S+ auditable, y habilitar la primera línea de valor tangible para el Vértice Biológico mediante notificaciones accionables libres de entropía.

**Objetivo A: Maestría Física de la Cápsula (El Guante Perfecto)**
*   **Resiliencia Termodinámica:** La cápsula debe manejar caídas de red IMAP, adjuntos corruptos y errores de *parsing* MIME sin lanzar un *panic!* fatal. Todo fallo debe encapsularse estrictamente en el contrato `capsule-json-io` (devolviendo `success: false` y un `exitCode` controlado), garantizando la estabilidad del Orquestador.
*   **Aislamiento de Cómputo:** Su única misión es recibir un payload JSON de contexto y el buzón bruto, cruzar los datos de forma determinista y devolver un objeto estructurado. No guarda estado ni toma decisiones estratégicas.

**Objetivo B: Valor Funcional (Triaje Interactivo)**
*   **Erradicación de la Alerta Plana:** Prohibido notificar cada entrada del buzón. Se implementa el evento ECST `Actionable_Email_Detected`.
*   **Enrutamiento Táctico:** Cuando la cápsula determine que un correo supera el umbral de relevancia (según la matriz inyectada), el sistema enrutará este evento hacia `kalma2_interact_core` o `telegram-watcher`.
*   **Interacción en el Umbral:** La interfaz mostrará un resumen táctico y ofrecerá "Acciones Rápidas" (ej. redactar respuesta, archivar, delegar) para resolver la fricción con un solo clic.

---

#### 2. Clarificación y Lógica de Razonamiento (`clarify.md`)

**Sobre la Entropía Funcional:**
Una notificación pasiva que solo indica "Tienes un correo" transfiere la carga termodinámica al Vértice Biológico, obligándolo a romper su contexto para evaluar el ruido. La cápsula debe asumir esa carga. Al emitir un `Actionable_Email_Detected`, el correo ya ha pasado el Filtro C (Eficiencia). Si no es relevante, la cápsula lo marca como procesado en silencio.

**Sobre el Valor del Activo (NFT-ready):**
La robustez de esta cápsula en su manejo de errores de red y formatos corruptos es lo que le otorga valor de mercado dentro de la Librería SddIA. Un consumidor no adquiere hábitos (que pertenecen al Grafo de Pensamiento Universal), sino un motor físico inquebrantable capaz de integrarse con cualquier agente Cúmulo externo.

---

#### 3. Plan de Implementación (`plan.md`)

**Fase 1: Blindaje de la Cápsula IMAP**
1.  Auditar y refactorizar el código de la cápsula IMAP actual para asegurar la captura absoluta de excepciones (red, decodificación).
2.  Garantizar el cumplimiento innegociable del contrato de salida JSON (`capsule-json-io.md`).

**Fase 2: Motor de Triaje Ciego**
1.  Implementar la lógica determinista para cruzar la matriz de contexto (inyectada por el Orquestador) con el asunto/remitente de los correos no leídos.
2.  Clasificar internamente entre `RUIDO` (Filtro C negativo) y `FRICCION_RELEVANTE`.

**Fase 3: Propagación y Notificación (Kalma2 / Telegram)**
1.  Registrar el evento `Actionable_Email_Detected` en `events-contract.md`.
2.  Desarrollar el handler en `kalma2_interact_core` y `telegram-watcher` para capturar este evento.
3.  Diseñar el payload de la interfaz de usuario para presentar el resumen y los botones de acción (`[Archivar]`, `[Generar Borrador]`).

---

#### 4. Criterios de Aceptación (Protocolo de Acero)

*   [ ] La cápsula IMAP procesa la bandeja sin dependencias externas vulnerables y jamás rompe el hilo del sistema con errores no controlados.
*   [ ] Los correos clasificados como ruido no generan ningún evento de dominio hacia Kalma2 o Telegram.
*   [ ] Un correo relevante dispara una notificación enriquecida en la interfaz del usuario.
*   [ ] La interfaz (Kalma2/Telegram) ofrece opciones de respuesta rápida, permitiendo desencadenar un evento de retorno sin salir de la plataforma.