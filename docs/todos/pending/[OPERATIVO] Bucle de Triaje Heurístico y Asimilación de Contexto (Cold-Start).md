---
document_id: PBI-EMAIL-TRIAGE-HEURISTIC
title: "[OPERATIVO] Bucle de Triaje Heurístico y Asimilación de Contexto (Cold-Start)"
format: markdown
version: "1.0.0"
created: "2026-09-02"
status: "propuesta"
priority: alta
process: feature
related:
  - SddIA/norms/capsule-json-io.md
  - SddIA/events/domain/user-preference-change-requested.md
---

### [OPERATIVO] Bucle de Triaje Heurístico y Asimilación de Contexto (Cold-Start)

#### 1. Naturaleza del Activo (Física del Valor)
La cápsula de Triaje Semántico de Correos trasciende la automatización *legacy*. Se define como una Entidad de Dominio autopoietica diseñada para la Librería SddIA. Su propuesta de valor central es la capacidad de operar con alta precisión desde el despliegue inicial (Cold-Start) mediante una Heurística Base, para posteriormente evolucionar sus fronteras lógicas asimilando los hábitos del portador a través de una base de datos vectorial (LanceDB), con fricción de usuario tendente a cero.

#### 2. Lógica de Triaje y Ejecución (El Túnel de Viento)
Ante el chispazo de un nuevo correo interceptado (excluyendo dominios ya gestionados como las agencias de reuniones), la cápsula ejecuta la siguiente coreografía obligatoria:

1. **Inyección de Memoria (Consulta Vectorial):** La cápsula interroga a LanceDB cruzando el remitente, asunto y cuerpo del correo para recuperar el "Hábito Consolidado" del usuario.
2. **Evaluación de Fricción:**
   - *Vía A (Contexto Existente):* Si LanceDB devuelve un patrón claro (ej. "Ignorar newsletters de esta fuente" o "Notificar siempre facturas de X"), la IA acata el hábito histórico.
   - *Vía B (Heurística Base / Cold-Start):* En ausencia de contexto, la IA aplica su instinto de fábrica. Por defecto, solo superarán el Filtro C (Necesidad) y serán notificados los correos que pertenezcan a los siguientes vectores de alto impacto:
     - **Alteraciones Logísticas:** Entregas de paquetería, cambios de estado en envíos físicos.
     - **Alteraciones Financieras:** Confirmaciones de compras realizadas, recibos.
     - **Alertas de Seguridad:** Inicios de sesión anómalos, cambios de credenciales.
3. **Descarte Silencioso:** Cualquier correo que no encaje en la Vía A o la Vía B es ignorado transitoriamente. No se ejecuta orden de borrado (Prevención de Fractura Sistémica por falso positivo) hasta alcanzar un grado S+ de confianza futura.

#### 3. Bucle de Desarrollo Kaizen (Asimilación Silenciosa)
La cápsula no establece diálogos abiertos que disipen la energía del Vértice Biológico. La interacción de aprendizaje se restringe a interfaces atómicas en el Umbral (Telegram):

- **Notificación Estructurada:** El mensaje entregado en Telegram incluirá un resumen semántico de una línea y el motivo de la alerta (ej. *Motivo: Heurística Base - Logística*).
- **Inmunidad y Botonera Atómica:** El mensaje incorporará botones *inline* de Telegram para la retroalimentación inmediata:
  - `[👍 Útil]` (Refuerza el peso vectorial del remitente/categoría).
  - `[👎 Ignorar similares]` (Crea un sesgo negativo en LanceDB).
  - `[🧠 Ajustar regla]` (Abre una excepción para inyectar un matiz textual corto).
- **Enrutamiento del Hábito:** La pulsación de un botón dispara un evento ciego (`user-preference-change-requested`) al bus de SddIA. Un agente en segundo plano procesa este evento y persiste el nuevo estado en LanceDB, esculpiendo la identidad de la cápsula para la próxima iteración.

#### 4. Contrato de Entrada/Salida (Capsule JSON I/O)
Para mantener la Ceguera Espacial y el aislamiento paramétrico, el contrato atómico se define como:
- **Input:** JSON con los metadatos del correo y el bloque de contexto recuperado de LanceDB.
- **Output:** JSON estandarizado (`success`, `exitCode`, `result`). El `result` dictaminará la acción (`NOTIFY`, `IGNORE`) y el `payload` exacto con la estructura de la botonera a transferir al `telegram-gateway`.
