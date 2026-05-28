# [ARQUITECTURA] Barrera Táctil Local: Interceptación QA Síncrona Bloqueante

**[ Naturaleza: Sistema Operativo / Fisiología Digital ]**
**[ Entorno: SddIA Core / Git Hooks Físicos ]**
**[ Entropía Asimilada: Fricción dialéctica resuelta tras identificar la brecha termodinámica entre la asincronía nativa del bus de eventos (EDA) y la necesidad imperativa de bloqueo síncrono exigida por los ganchos del Sistema Operativo (Git Hooks). ]**

---

## 1. Contexto y la Paradoja de la Sincronicidad

El ecosistema SddIA está gobernado de forma nativa por una coreografía de eventos asíncronos distribuidos en el sistema de archivos (`.SddIA/events/`). Sin embargo, los ganchos de control de versiones locales (`git hooks`, operados mediante la herramienta periférica inerte **Husky**) exigen un veredicto inmediato y síncrono:
- Si el proceso invocado devuelve un código de salida `0`, la mutación física del repositorio se ejecuta.
- Si devuelve un código `>0` o se desentiende de forma asíncrona, el hilo se rompe o la validación se elude, provocando una fractura de la Verdad Objetiva.

Dado que actualmente la topología de SddIA está optimizada para la absorción reactiva de fondo (latencia gestionada por `event-watcher`), delegar la aduana local al bus ordinario provocaría un falso positivo masivo (el commit/push pasaría antes de que **Argos** despertara). Por tanto, se documenta la necesidad de una bifurcación de comportamiento en el CLI central: el **Modo de Suspensión Local Síncrona**.

---

## 2. Especificación Técnica de la Aduana Local (Proxy Bloqueante)

Para mantener la **Ceguera Espacial** de Husky y la centralización de la inteligencia en SddIA, el script inerte alojado en el ciclo de Git operará exclusivamente como un disparador del CLI de SddIA usando banderas de bloqueo explícitas.

### Flujo del Ciclo de Vida Local:
1. **Intención del Vértice Biológico:** El usuario ejecuta `git push` o `git commit`.
2. **Interceptación Física:** Husky detiene el hilo del Sistema Operativo y arranca el CLI de SddIA.
3. **Bucle de Suspensión:** El CLI de SddIA procesa la orden de forma síncrona en el mismo hilo de ejecución, impidiendo que devuelva el control a Git hasta obtener el veredicto del Agente Juez (**Argos**).

### Implementación Física del Hook (Ejemplo `.husky/pre-push`):
` ` `bash
#!/bin/sh
# Husky - Receptor Inerte Periférico
echo "[SddIA] Interceptando intención de mutación física (Push)..."
echo "[SddIA] Invocando Aduana de Control Síncrona..."

# Invocación síncrona forzada. Detiene el flujo del SO hasta su resolución.
node .SddIA/core/cli.js route-domain-event --event Local_QA_Requested --target argos --blocking

# Transmisión directa del veredicto rúnico
exit $?
` ` `

---

## 3. PBI de Refinamiento: Modificación del Motor de Eventos Core (`route-domain-event`)

**Estatus del PBI:** Pendiente / Teórico (Para refinamiento y maquetación por la IA Obrera).

### Objetivo Técnico:
Dotar al comando central `route-domain-event` (localizado en `.SddIA/core/` o a través del script de control correspondiente) de la capacidad de cortocircuitar el procesamiento asíncrono de fondo cuando se le inyecte el flag `--blocking` (o `--sync`).

### Requerimientos de Código para la IA Obrera (Jules / Tekton):
1. **Detección de Contexto Bloqueante:** Si el argumento `--blocking` está presente, el CLI **no** debe limitarse a depositar el JSON del evento y finalizar el script.
2. **Invocación en Línea del Agente:** El motor debe buscar inmediatamente el suscriptor asignado al evento (en este caso, `argos`) e instanciar su ejecución lógica de forma síncrona (ej. usando `execSync` o `spawnSync` en entornos Node, o llamadas bloqueantes homólogas de Rust/Python según el core).
3. **Captura y Transmisión del Exit Code:** El CLI debe heredar el código de salida arrojado por el script de QA de Argos (ej. si Argos detecta índices rotos devuelve `1`) y propagarlo como su propio código de salida hacia Husky.

### Notas de Incertidumbre y Deuda Técnica Core:
- **Validación de la Capacidad Síncrona:** Se debe auditar si el motor actual soporta esta inyección síncrona sin refactorizaciones mayores. Hay que evaluar si el archivo de suscripciones globales (`SddIA/core/event-domain-subscriptions.json`) requiere un metadato explícito para identificar qué agentes son capaces de responder en modo bloqueante.
- **Aislamiento de Impacto:** Asegurar que las llamadas síncronas locales no dejen bloqueado el bus reactivo principal si se ejecutan mientras el demonio de escucha asíncrona está activo.

---

## 4. Criterios de Aceptación (Aduana de Fricción)

- [ ] **Bloqueo Efectivo:** Al alterar intencionadamente un índice (provocando una fractura rúnica) y ejecutar un commit/push, Git aborta inmediatamente la operación indicando el fallo dictaminado por SddIA.
- [ ] **Liberación Sólida:** Si el código supera los tests lógicos y la paridad documental de Argos, el CLI devuelve un `exit 0` y la operación de Git se completa sin intervención humana.
- [ ] **Inexistencia de Alucinación:** El CLI rechaza flags de bloqueo si el agente destino no está instanciado localmente en el entorno del laboratorio o si el evento carece de suscriptor válido.
