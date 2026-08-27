---
document_id: PBI-KAIZEN-ESPEJO-CONSCIENCIA-001
title: "[KAIZEN] Espejo de Consciencia: Proyección de Salud y Observabilidad del Ecosistema"
format: markdown
version: "1.0.0"
created: "2026-08-27"
status: "propuesto"
priority: "critica"
process: feature
type: kaizen
dispatch: false
suggested_branch: feat/kaizen-espejo-consciencia-observabilidad
depends_on: []
related:
  - SddIA/agents/radamanto.md
  - SddIA/agents/cumulo.md
  - SddIA/library/norms/read-models-y-proyecciones/spec.md
  - SddIA/interfaces/kalma2-bridge/
---

# [KAIZEN] Espejo de Consciencia: Proyección de Salud y Observabilidad del Ecosistema

## 0. Pendiete de refinar. Dedicar espacial antención a posibles alucinaciones, incoherencias o inexatitudes.
## 1. Falla Estructural y Contexto

El Vértice Biológico sufre de "Ceguera Espacial" respecto a la materialización física del sistema. SddIA ha evolucionado hacia un motor asíncrono y descentralizado, pero su observabilidad es nula. 

Actualmente, el usuario no tiene forma de saber desde la interfaz (Kalma2) si un adaptador es un *placeholder* teórico (como ha ocurrido con LanceDB), si un centinela (daemon) ha dejado de emitir latidos (`daemon-heartbeat`), o si Radamanto ha degradado una herramienta (`domain-entity-degraded`) por fallos termodinámicos recurrentes. El sistema es opaco; opera como una caja negra que consume la atención del humano al obligarlo a auditar logs crudos o revisar código fuente para confirmar el estado del entorno.

## 2. Objetivo Medible

Erradicar la opacidad del ecosistema construyendo un **Panel de Observabilidad (Espejo de Consciencia)** accesible desde Kalma2. El sistema debe compilar una proyección en tiempo real (Read Model) que cruce la topología esperada con el estado físico real, permitiendo al usuario visualizar el estatus exacto de la trinchera en un solo vistazo.

El flujo será exitoso si el usuario puede preguntar a Kalma2 (o acceder a una vista dedicada) y obtener una matriz visual con:
1. El estado de los conectores de infraestructura (LanceDB, IOTA).
2. El estatus de vida de los Daemons (basado en la telemetría de heartbeats).
3. El estado de las cápsulas y herramientas (Activa, Degradada, Deprecada, Placeholder).

## 3. Decisiones Arquitectónicas Obligatorias

### 3.1. Proyección de Estado (Read-Model), No Acoplamiento
Kalma2 y el `kalma2-bridge` **no** deben acceder directamente al sistema de archivos para parsear contratos o leer la base de datos de telemetría. Deben invocar una nueva acción/skill (`query-ecosystem-health`) que consuma un **Read Model** mantenido por Radamanto y Cúmulo. 

### 3.2. Fusión Ontológica: Mapa vs. Territorio
El estado del ecosistema nace de la fricción entre dos Nodos:
* **Cúmulo (El Mapa):** Provee el listado absoluto de entidades de dominio esperadas según los índices oficiales (`SddIA/tools/index.md`, `cumulo.paths.json`).
* **Radamanto (El Territorio):** Provee el estatus termodinámico de dichas entidades, cruzando sus umbrales de fallo, eventos de degradación y tiempos desde el último heartbeat.

La acción de diagnóstico debe cruzar ambos vectores. Si Cúmulo dice que "LanceDB Adapter" existe, pero su ejecución solo devuelve respuestas simuladas o vacías continuadas, el reporte debe marcarlo en **ROJO (Placeholder / Fallo Físico)**.

### 3.3. Representación Visual en Kalma2
El `kalma2-bridge` transmitirá la proyección como un JSON estructurado hacia el frontend. Kalma2, utilizando Vanilla JS y HTML plano, renderizará un cuadro de mandos con semántica de colores estricta:
* **VERDE (S+ Grade):** Activo, cápsula comprobada, daemon latiendo.
* **AMARILLO (Fricción):** Degradado (Self-Healing en proceso), latencia alta, o advertencias de cumplimiento.
* **ROJO (Entropía):** Caído, Placeholder no implementado, Daemon muerto (sin latido).
* **GRIS (Letargo):** Entidad teórica (en diseño) o deshabilitada voluntariamente.

## 4. Alcance

### Dentro
- Creación de la skill/acción de lectura `query-ecosystem-health` o similar.
- Diseño del Read Model JSON que consolida el estado de la infraestructura, daemons, skills y tools.
- Actualización de `kalma2-bridge` para habilitar un nuevo endpoint (ej. `/api/system-health`).
- Actualización de la interfaz material (Kalma2) para renderizar el panel de forma asíncrona a petición del usuario.
- Lógica simple en Radamanto/Cúmulo para actualizar este estado pre-calculado cuando ocurren eventos clave (ej. `domain-entity-updated`, `daemon-heartbeat`).

### Fuera
- Gráficos históricos de rendimiento a largo plazo (corresponde a otro nivel de observabilidad).
- Intervención manual desde la UI (ej. botones para "reiniciar daemon"). La UI de Kalma2 sigue siendo un "Despertador Inerte" de lectura e inyección de prompts, no un panel de control de DevOps. El reinicio de servicios pertenece a las rutinas de Self-Healing del Core.

## 5. Criterios de Aceptación (Protocolo de Acero)

| ID | Criterio | Verificación |
|----|----------|--------------|
| OBS-CA1 | La acción `query-ecosystem-health` retorna un JSON con el mapa completo cruzado con los estados de salud. | Ejecución por CLI devuelve JSON válido. |
| OBS-CA2 | Si se detiene manualmente un daemon (`event-watcher`), el panel lo refleja como "Muerto/Rojo" tras expirar el timeout del heartbeat. | E2E Lab Smoke Test. |
| OBS-CA3 | Herramientas en estado de "Placeholder" (como detectado en LanceDB) se reportan visiblemente diferentes a las operativas. | Validación visual y de payload. |
| OBS-CA4 | Kalma2 renderiza el estado de forma clara, sin dependencias externas pesadas, preservando su naturaleza ligera. | Revisión de artefacto `.SddIA/client/`. |
| OBS-CA5 | El bridge y Kalma2 no violan la Ceguera Espacial: piden el estado a la API, no navegan por los directorios del Core. | Auditoría de `app.js` y `kalma2-bridge.py/rs`. |
