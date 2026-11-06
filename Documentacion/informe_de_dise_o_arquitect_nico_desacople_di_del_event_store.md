---
id: "ADR-EDA-002"
title: "Desacople de Infraestructura del Bus de Eventos: Arquitectura de Doble Plano (Trait Nativo + Cápsulas DI)"
status: "APPROVED"
date: "2026-11-06"
context: "ecosystem-evolution"
capability_domain: "eda.event_store"
precedents:
  - "tracker.lifecycle.schema.json (tracker_lifecycle_port.rs)"
  - "SddIA/core/capability-bindings.md"
  - "SddIA/library/norms/capability-taxonomy.md"
---

# Informe de Diseño Arquitectónico: Inyección de Dependencias y Desacople del Event Store (`eda.event_store`)

## 1. Resumen Ejecutivo y Decisión Arquitectónica (Laudo del Vértice Biológico)

Con el objetivo de preparar el ecosistema **SddIA** para la futura **Librería SddIA (Tokenización / NFTs de capacidades)** y erradicar el acoplamiento estático de tecnologías de persistencia en el Genoma (`Core`), se aprueba la refactorización del sistema de eventos bajo la **Opción 2: Arquitectura Híbrida de Doble Plano (Trait Nativo en Runtime + Cápsulas de Infraestructura en Dominio)**.

### Justificación Termodinámica (Sacrificio Consciente)
Se sacrifica deliberadamente el intercambio de binarios en caliente (*hot-swap* sin recompilación) en el plano interno del motor a cambio de **latencia cero en el Peaje Termodinámico** y acceso nativo sin fricción a primitivas del Sistema Operativo (`mmap` y *file locking* exclusivo requeridos por `redb`).

* **Plano Interno (Motor `execute-process` y Centinela `event-watcher`):** Consumen la abstracción mediante un `trait EventStorePort` en Rust (Inyección de Dependencias en compilación/arranque), evitando levantar un subproceso WASI/nativo adicional por cada evento de telemetría (`Raw_Execution_Finished`) o en cada ciclo de *polling* de 2 segundos.
* **Plano de Dominio y Librería SddIA (Acciones, Procesos y Agentes):** Consumen la capacidad de forma 100% desacoplada a través de la Aduana DI (`capability_di_resolver`), invocando cápsulas independientes (`redb-event-adapter`, `fs-event-adapter`) mediante el protocolo estándar `capsule-json-io` (`stdin`/`stdout`).

---

## 2. Contrato Canónico en el Genoma (Punto 1)

El `Core` de SddIA deja de conocer qué es `redb` o cómo se estructuran los directorios de eventos en disco. El Genoma solo conoce el **Contrato Funcional**.

### 2.1. Esquema Normativo JSON (`SddIA/library/norms/capability-contracts/eda.event_store.v1.schema.json`)
Define las operaciones canónicas que cualquier cápsula proveedora de almacenamiento de eventos debe satisfacer:

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "sddia://library/norms/capability-contracts/eda.event_store.v1.schema.json",
  "title": "EDA Event Store Capability Contract",
  "type": "object",
  "required": ["operation", "payload"],
  "properties": {
    "operation": {
      "type": "string",
      "enum": [
        "append_event",
        "claim_pending",
        "ack_event",
        "nack_event",
        "move_to_dlq",
        "query_by_correlation",
        "health_check"
      ]
    },
    "payload": {
      "type": "object",
      "description": "Estructura polimórfica dependiente de la operación sobre el sobre canónico CloudEvents/SddIA."
    }
  }
}
```

### 2.2. Puerto Nativo en Rust (`sddia-ports` / `event_store_port.rs`)
En sustitución del acoplamiento directo a las tripas de `sddia-event-store`, se define un crate de frontera ultraligero (libre de dependencias de I/O pesado):

```rust
pub trait EventStorePort: Send + Sync {
    fn append(&self, event: &CanonicalEventEnvelope) -> Result<EventReceipt, EventStoreError>;
    fn claim_batch(&self, consumer_id: &str, limit: usize) -> Result<Vec<CanonicalEventEnvelope>, EventStoreError>;
    fn ack(&self, event_id: &Uuid, consumer_id: &str) -> Result<(), EventStoreError>;
    fn fail_or_dlq(&self, event_id: &Uuid, reason: &str) -> Result<DlqStatus, EventStoreError>;
}
```

---

## 3. Extracción de Tecnologías a Cápsulas Independientes (Punto 2)

Siguiendo el precedente arquitectónico de `linear-tracker-adapter` y `file-tracker-adapter`, las implementaciones tecnológicas salen del núcleo y se empaquetan como **Crates Híbridos** (`lib.rs` + `main.rs`) dentro del workspace de herramientas:

| Artefacto | Ubicación en Genoma | Rol en Plano Interno (`lib.rs`) | Rol en Plano Dominio (`main.rs`) |
| :--- | :--- | :--- | :--- |
| **`redb-event-adapter`** | `SddIA/tools/redb-event-adapter/` | Implementa `EventStorePort` sobre motor ACID `redb` (`events.redb`). | Ejecutable de cápsula que recibe JSON por `stdin` y opera sobre `events.redb`. |
| **`fs-event-adapter`** | `SddIA/tools/fs-event-adapter/` | Implementa `EventStorePort` mediante archivos JSON atómicos en `.SddIA/.bus/`. | Ejecutable de cápsula para entornos sin soporte de *file locks* o auditoría forense directa en texto plano. |
| **`dual-event-adapter`** *(Opcional/Transición)* | `SddIA/tools/dual-event-adapter/` | Decorador que escribe en `redb` y espeja en `filesystem` para trazabilidad en desarrollo. | Cápsula puente durante fases de migración. |

> **Nota sobre `bus-operator`:** La skill existente `SddIA/skills/bus-operator` queda circunscrita a una capacidad cognitiva/operativa de alto nivel para que un Agente inspeccione o audite el bus, delegando toda persistencia física en el puerto `eda.event_store`.

---

## 4. Clasificación Ontológica: ¿`Skills` o `Tools`? (Punto 6.A)

### Veredicto Normativo: Son `Tools` (Infrastructure Adapters)

Dentro de la taxonomía de SddIA, se establece la siguiente regla de demarcación estricta:
1. **`Skill` (`SddIA/skills/`):** Capacidad técnica intrínseca y atómica del Agente (su *saber hacer* instrumental sobre el entorno local, ej. `git-manager`, `cryptography-manager`).
2. **`Tool` (`SddIA/tools/`):** Adaptador de infraestructura, conector de persistencia o puente hacia un sustrato tecnológico intercambiable (ej. `linear-tracker-adapter`, `iota-immutable-publisher`, `redb-event-adapter`, `fs-event-adapter`).

Por simetría fractal con los adaptadores de *Tracker*, todo motor de almacenamiento o transporte de eventos (`redb`, `filesystem`, y futuros NFTs como `postgres-event-adapter`, `nats-event-adapter` o `kafka-event-adapter`) habitará **exclusivamente bajo `SddIA/tools/`**.

---

## 5. Gobierno por Configuración y Resolución DI (Puntos 3, 4 y 6.B)

Se erradica la lectura dispersa de variables de entorno hardcodeadas en la lógica de negocio. El enrutamiento entre el contrato y la implementación se gobierna mediante el patrón **Puerto-Adaptador** ya presente en `capability_di_resolver.rs`.

### 5.1. Separación entre RBAC (Cerbero) y Capacidades (DI Resolver)
Para evitar el solapamiento semántico entre etiquetas de permisos y etiquetas de implementación:
* **`context` y `permissions` (Dominio de Cerbero - `cerbero_di_rbac.rs`):** Determinan *quién* puede ejecutar qué (ej. el rol `ecosystem-evolution` o `sentinel` tiene permiso para invocar la capacidad `eda.event_store`).
* **`capability` (Dominio del DI Resolver - `capability_di_resolver.rs`):** Determina *qué contrato abstracto* necesita una Acción o Proceso, sin nombrar nunca la herramienta concreta.

### 5.2. Declaración en Acciones y Procesos (Desacople del Consumidor)
Una acción del sistema (ej. `emit-domain-mutation.md` o `route-domain-event.md`) **jamás** referencia a `redb-event-adapter`. Su frontmatter exige únicamente el puerto:

```yaml
---
id: "action.emit-domain-mutation"
requires_capabilities:
  - id: "eda.event_store"
    contract_version: "^1.0.0"
---
```

### 5.3. Inyección en Configuración Soberana (`capability-bindings.md` y `.SddIA/project.md`)
La elección de la tecnología activa vive en el manifiesto de enlaces del Core (por defecto) y puede ser sobrescrita por cada instancia de proyecto (Vía C) en `.SddIA/project.md`:

```yaml
# En SddIA/core/capability-bindings.md (Default Genoma) o .SddIA/project.md (Instancia)
capability_bindings:
  eda.event_store:
    provider: "tool:redb-event-adapter"   # Alternativa: "tool:fs-event-adapter"
    contract: "sddia://library/norms/capability-contracts/eda.event_store.v1.schema.json"
    contract_version: "1.0.0"
    config:
      db_path: ".SddIA/.bus/events.redb"
       fallback_dlq: ".SddIA/.bus/dlq/"
```

En el **Plano Interno (Rust)**, `execute-process` y `event-watcher` leen este mismo bloque de configuración al arrancar (`bootstrap`) e instancian en memoria el `Arc<dyn EventStorePort>` correspondiente (`RedbEventAdapter::new(...)` o `FsEventAdapter::new(...)`).

---

## 6. Aislar la Lógica de Negocio: Estrategia de Testing y Mocking (Punto 5)

El desacople mediante `EventStorePort` (en Rust) y `eda.event_store` (en JSON DI) habilita dos niveles de pruebas deterministas sin tocar el sistema de archivos ni bloquear `events.redb`:

1. **Tests Unitarios de Motor y Centinela (`cargo test` en Rust):**
   * Se incorpora `InMemoryEventStoreMock` implementando `EventStorePort` (basado en un `Arc<Mutex<Vec<CanonicalEventEnvelope>>>`).
   * Permite verificar en milisegundos que `execute-process` emite el Peaje Termodinámico (`Raw_Execution_Finished`) y que `event-watcher` enruta fracturas (`System_Fracture_Detected`) sin I/O en disco ni colisiones de *locks* cuando Cargo ejecuta tests en paralelo.
2. **Tests de Integración de Acciones y Procesos (DI Mock Capsule):**
   * En perfiles de test E2E, el binding de `eda.event_store` puede apuntar a `tool:fs-event-adapter` sobre un directorio efímero (`tempdir`) o a un `mock-event-adapter` que simula fallos de escritura (ej. disco lleno o base de datos corrupta) para auditar la respuesta de **Radamanto** y **Dédalo**.

---

## 7. Protocolo de Doble Versionado SemVer para la Librería SddIA (Punto 6.C)

Para que las cápsulas sean **activos tokenizables (NFTs)** con ciclo de vida autónomo en la futura Librería SddIA, se desacopla la evolución del **Contrato del Genoma** de la evolución de la **Cápsula Implementadora**.

### 7.1. Manifiesto de Cicatriz Digital en la Cápsula (`manifest.json` / Frontmatter)
Toda cápsula bajo `SddIA/tools/` debe declarar dos vectores de versión independientes:

```yaml
---
id: "tool:redb-event-adapter"
type: "infrastructure-adapter"
capsule_version: "1.3.2" # Evolución interna del código Rust / dependencias (ej. bump de redb)
implements_contract:
  capability_id: "eda.event_store"
  contract_version: "1.1.0" # Versión del contrato del Core que implementa
  schema_uri: "sddia://library/norms/capability-contracts/eda.event_store.v1.schema.json"
---
```

### 7.2. Matriz de Compatibilidad en la Aduana (`capability_di_gate.rs`)
Cada vez que el motor resuelve una capacidad o sella una cápsula (`capsule_seal.rs`), el Gate evalúa la compatibilidad mediante las siguientes leyes:

| Cambio en `contract_version` (Core) | Cambio en `capsule_version` (NFT/Tool) | Regla de Validación en `capability_di_gate.rs` |
| :--- | :--- | :--- |
| **PATCH (`1.0.x`)** | Cualquiera (`1.0.*`) | **Compatible.** Solo correcciones documentales sin alteración del AST JSON. |
| **MINOR (`1.1.0`)** | Cápsula en `1.0.0` o `1.1.0` | **Compatible si es Aditivo Opcional.** Si el Core `1.1.0` añade campos opcionales, las cápsulas `1.0.0` siguen operando. Si una Acción exige explícitamente `^1.1.0`, la cápsula `1.0.0` es rechazada preventivamente. |
| **MAJOR (`2.0.0`)** | Cápsula en `1.x.x` | **Incompatibilidad Dura.** Ruptura de contrato. Coexistirán `eda.event_store.v1.schema.json` y `eda.event_store.v2.schema.json` durante la ventana de deprecación. |

### 7.3. Firma de Fractura ante Desalineación
Si una instancia intenta inyectar una cápsula cuyo `implements_contract.contract_version` viola el rango exigido por el Core o la Acción, `capability_di_gate.rs` bloquea la ejecución antes de abrir el canal `stdin` y emite un evento crítico al bus con la firma determinista:
* **`F-DI-CONTRACT-VERSION-MISMATCH`** -> Enrutado a Radamanto/Dédalo para actualización de cápsula o ajuste de binding.

---

## 8. Hoja de Ruta de Implementación (Secuencia de Forja)

Para ejecutar esta transición sin interrumpir la operatividad actual del repositorio:

1. **Fase 1 (Contrato y Puerto):** Crear `SddIA/library/norms/capability-contracts/eda.event_store.v1.schema.json`, registrar `eda.event_store` en `capability-taxonomy.md` y definir el trait `EventStorePort` junto con `InMemoryEventStoreMock`.
2. **Fase 2 (Desglose de Crates/Tools):** Refactorizar el actual `sddia-event-store` dividiéndolo en `SddIA/tools/redb-event-adapter` y `SddIA/tools/fs-event-adapter` (ambos exponiendo `lib.rs` con el trait y `main.rs` con la interfaz de cápsula JSON).
3. **Fase 3 (Cableado DI en Motor y Centinela):** Conectar `execute-process` y `event-watcher` al resolver de configuración para instanciar el `EventStorePort` dinámicamente al inicio, reemplazando los tests actuales con `InMemoryEventStoreMock`.
4. **Fase 4 (Sellado y Aduana SemVer):** Añadir la validación de `implements_contract` en `capability_di_gate.rs` y actualizar los bindings en `SddIA/core/capability-bindings.md`.