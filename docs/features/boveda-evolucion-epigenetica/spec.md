---
feature_name: boveda-evolucion-epigenetica
created: "2024-06-04"
process: feature
impacts_doc: true
---

# Especificación Técnica — Bóveda de Evolución Epigenética

## 1. Arquitectura y Componentes

El subsistema "Bóveda de Evolución Epigenética" constará de los siguientes dominios lógicos, respetando la Arquitectura Hexagonal y la inyección de dependencias de SddIA.

### 1.1. Motor de Almacenamiento Vectorial (Evolution)
- **Tecnología:** LanceDB (incrustado).
- **Ruta Física:** `.SddIA/vector_store/evolution/`.
- **Estructura del Esquema:** Deberá definir campos que incluyan el `id` (hash SHA-256), la polaridad (`Simetría Eficiente` | `Fractura Estructural`), vector embedding semántico de la traza, metadata operativa (tiempos, costo termodinámico) y el payload original del evento.

### 1.2. Generador de Identidad Determinista
- Algoritmo que recibe el payload serializado y los metadatos contextuales para producir un hash SHA-256 inmutable que fungirá como la clave primaria del registro en la base de datos vectorial.
- Sin factor de entropía aleatoria (Cero UUIDs).

### 1.3. Proxy de Captura Pasiva (Event Listener)
- Módulo encargado de monitorizar el bus reactivo en `.SddIA/events/`.
- Interceptará eventos relacionados a finalización de procesos, errores del orquestador, y reportes de fricción termodinámica.
- Deberá operar de manera asíncrona o no bloqueante en relación con el flujo principal del orquestador (Edge Telemetry).

### 1.4. Clasificador de Polaridad Espacial
- Algoritmo evaluador que determinará el grado de "éxito" o "fracaso" de la traza.
- Enrutará el registro bajo la categoría `Simetría Eficiente` o `Fractura Estructural` dependiendo de las métricas reportadas en la telemetría del evento capturado.

## 2. Flujo de Datos

1. **Emisión:** Un componente en los bordes del sistema finaliza su ejecución y emite un evento estándar (YAML frontmatter + Markdown payload) al directorio `.SddIA/events/`.
2. **Captura:** El Proxy de Captura Pasiva detecta el nuevo evento, lo procesa y extrae las métricas operativas clave.
3. **Clasificación y Hashing:**
   - El clasificador etiqueta el evento (Simetría Eficiente o Fractura Estructural).
   - Se calcula el hash SHA-256 del contenido canónico para obtener el `id` determinista.
4. **Cristalización Semántica:** Se invoca la Aduana Semántica local (All-MiniLM-L6-v2 nativo) para generar el vector embebido del cuerpo del evento.
5. **Persistencia:** El registro estructurado se guarda en la colección LanceDB ubicada en `.SddIA/vector_store/evolution/`.

## 3. Consideraciones No Funcionales
- La implementación futura requerirá que la lógica base sea compatible con la transición a Rust WASI (wasm32-wasip1), utilizando los bindings nativos correspondientes sin depender de wrappers Python.
- Los secretos o configuraciones para los modelos nativos, de ser necesarios, deberán ser gestionados estrictamente vía `.env`, nunca en los payloads de los eventos.

### Impacto en Documentación

- `docs/features/boveda-evolucion-epigenetica/` — spec, plan, implementation, objectives, validacion
- `docs/features/memoria-vectorial/` — paridad documental coalescente
- `docs/features/grafo-pensamiento/` — spec, validacion
- `SddIA/events/domain/thought-persisted.md` — evento ECST grafo
- `SddIA/events/domain/vector-memory-indexed.md` — evento ECST bóveda
- `SddIA/core/event-subscriptions.json` — suscripciones EDA
- `SddIA/core/event-domain-subscriptions.json` — suscripciones dominio
- `.gitignore` — exclusión `.SddIA/vector_store/`
