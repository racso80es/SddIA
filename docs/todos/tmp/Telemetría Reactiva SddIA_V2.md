---
status: consolidado
superseded_by: docs/todos/pending/[ARQUITECTURA] Telemetría Reactiva — Unificación EDA S+ Grade.md
document_id_unified: PBI-TELEMETRIA-REACTIVA-EDA-UNIFICADO
unified_phase: "Fases 3–4 (Aduana Universal, rutas runtime, Argos/Radamanto, Self-Healing)"
archived_from: docs/todos/pending/Telemetría Reactiva SddIA_V2.md
---

> **Documento archivado — unificado**
> Este PBI ha sido consolidado en [`[ARQUITECTURA] Telemetría Reactiva — Unificación EDA S+ Grade`](../pending/[ARQUITECTURA] Telemetría Reactiva — Unificación EDA S+ Grade.md) (`document_id: PBI-TELEMETRIA-REACTIVA-EDA-UNIFICADO`). Corresponde principalmente a las **Fases 3 y 4** del documento unificado. No ejecutar como ítem independiente.

# [ARQUITECTURA] Volcado de Sesión: Telemetría Reactiva, Juicio y Topología de Estímulos S+ Grade (V2)

**Fecha de la Sesión:** 2026-05-26  
**Temática Principal:** Consolidación de la Aduana Universal, Radamanto, Trinidad de Suscripciones y Persistencia Encapsulada  
**Estatus:** Consolidado y Refinado (V2 S+ Grade)  

---

## I. Axiomas de Interacción y Comunicación entre ED

Se consolida la ruptura total con los flujos de comunicación imperativos y síncronos entre las Entidades de Dominio (ED). La información y el estado se despliegan en dos capas físicas estrictamente desacopladas para garantizar la Ceguera Espacial y proteger la ventana de contexto de los agentes:

1. **La Señalización Táctica (El Sistema Nervioso):** Toda comunicación se realiza de forma asíncrona mediante la inyección de **Eventos de Dominio** en el bus estático basado en el sistema de archivos bajo la raíz de ejecución. Se aplica el patrón *Event-Carried State Transfer*: cada evento es auto-contenido y transporta la desnormalización de los datos vitales, evitando consultas secundarias por parte del receptor.
2. **La Sustancia Operativa (La Línea de Montaje):** El traspaso de paquetes de datos densos se gestiona mediante el modelo de Línea de Montaje Especializada. El relevo se realiza exclusivamente mediante la mutación de Artefactos Físicos dentro de un Espacio de Trabajo Aislado (Workspace / Directorio de Operación). La ruta exacta no está hardcodeada a un dominio de software, sino que es dinámica e impermanente; es instanciada e inyectada por el Orquestador inerte en el micro-contexto del agente en el momento de despertarlo.

---

## II. El Motor de Telemetría e Interceptación (La Aduana Universal)

Ninguna entidad posee la facultad de auto-auditarse. El control estadístico y de rendimiento se delega por completo a la infraestructura física inerte:

* **El Peaje Termodinámico:** Toda orden de ejecución transita obligatoriamente a través del Orquestador inerte (CLI).
* **Mecánica del CLI:** El script activa un cronómetro antes de dar paso a la cápsula de software. Al finalizar, captura el `exit code`, el tiempo de ejecución y el `asset_id`.
* **Desacople del Estímulo:** El CLI emite la telemetría cruda inyectando el evento `Raw_Execution_Finished` en su ruta aislada y finaliza su ciclo de vida.
* **Ampliación de telemetris:** La telemetrís dispondrá de facilitados por el artefacto ejecutor, como uso de tokens. Este punto queda fuera del alcance actual, pero si referenciado. PBI docs\todos\pending\Ampliacion_Log_Telemetris_Tokens.md

---

## III. El Panteón de Juicio: Argos y Radamanto

Se divide la jurisdicción de auditoría en dos nodos deterministas independientes para evitar la sobrecarga cognitiva:
* **Argos (El Inspector de la Materia):** Mantiene su jurisdicción sobre la *Aduana de Artefactos*, evaluando la calidad estructural, la eficiencia termodinámica y el *diff* del código físico entregado.
* **Radamanto (El Actuario de Confianza):** Agente encargado de la gobernanza macroscópica de la *Librería SddIA*.
  * **Determinismo Matemático:** Procesa umbrales estadísticos agregados procedentes del CLI sin interpretar código.
  * **Procesamiento por Lotes (Batching):** Consolida métricas acumuladas y reacciona ante variaciones de umbral para proteger la economía termodinámica.
  * **Soberanía de Firma:** Posee jurisdicción exclusiva para interactuar con la cápsula `iota-immutable-publisher`, sellando inmutablemente los estatus en la Testnet de IOTA Rebased.

---

## IV. El Bucle de Inmunidad Autónoma / Self-Healing

El ecosistema se repara a sí mismo sin la intervención del Vértice Biológico. El ciclo opera en bucle cerrado:

1. **Detección:** Radamanto constata que el acumulado estadístico de una *skill* cae por debajo del umbral, emite `Tool_Degraded` y sella la degradación en IOTA.
2. **Bloqueo (RBAC):** Cerbero intercepta la señal y actualiza la matriz de permisos, revocando el derecho de Tekton a invocar la herramienta.
3. **Forja Aislada:** `Tool_Degraded` inicializa `fix-tool-process`. Dédalo y Tekton refactorizan el código en un sandbox.
4. **Redención:** Argos audita el artefacto. El CLI ejecuta pruebas ciegas de estrés. Radamanto absorbe los datos exitosos, emite `Status_Restored` al Ledger, y Cerbero levanta el bloqueo.

---

## V. La Trinidad de Estímulos (Taxonomía de Eventos y Gesta de Emisiones)

Se establece la bifurcación absoluta del impulso nervioso para aislar el ruido físico de la verdad lógica y de la orquestación. Queda prohibido mezclar eventos crudos con notificaciones de flujo para evitar condiciones de carrera:
0. **Dependeincia de PBI docs\todos\pending\Refactor_Familias_Eventos.md**
1. **Eventos de Telemetría (El Ruido Físico):**
   * **Naturaleza:** Chispas de Infraestructura (Nivel 1). Residuo termodinámico de ejecución (ej. `Raw_Execution_Finished`).
   * **Gesta/Emisión:** Engendrados **exclusivamente por el propio script/CLI** inerte al detener el cronómetro.
   * **Destino:** Consumidos únicamente por Radamanto en lotes y purgados de inmediato del disco.
2. **Eventos de Orquestación / Flujo (La Comunicación entre ED):**
   * **Naturaleza:** Chispas Tácticas. Impulsos volátiles que mantienen viva la Línea de Montaje.
   * **Gesta/Emisión:** Cooperación dual. Son emitidos por el **CLI** cuando una cápsula finaliza con código de éxito (`status: success`) mapeando el blueprint del proceso, o por **Agentes Auditores** (como Argos emitiendo `Artifact_Validated`) invocando de forma legítima al CLI tras un veredicto de dominio.
   * **Destino:** Leídos por el enrutador de orquestación para despertar obreros (Dédalo, Tekton) y avanzar de fase.
3. **Eventos de Dominio Puro (La Verdad Objetiva):**
   * **Naturaleza:** Chispas Ontológicas (Nivel 3). Hitos irreversibles y mutaciones del negocio (ej. `PullRequest_Merged`, `Tool_Degraded`).
   * **Gesta/Emisión:** Engendrados por los **Agentes Core de Control** (Cúmulo, Cerbero, Radamanto) al consolidar alteraciones definitivas en la realidad del ecosistema.
   * **Destino:** Leídos por Cúmulo para anclaje criptográfico en la DLT y por Cerbero para mutar las políticas de seguridad.
   * **Refactorización:** Estos eventos son los que existen actualmente. tendrá que ser refactorizado para cumplir con las nuevas especificaciones.
4. **NOTA**
   * **Todos los tipos de eventos son del mismo tipo de entidad de dominio SddIA\events**

---

## VI. Topología Logística de Rutas y Suscripciones

Para soportar la Trinidad de Estímulos sin sacrificar el rendimiento, la infraestructura física y su configuración se fragmentan bajo el principio de Simetría Fractal:

* **Rutas Físicas del Runtime (`./.events/`):**
  * `./.events/telemetry/` -> Aislamiento de alta frecuencia (I/O intensivo).
  * `./.events/orchestration/` -> Aislamiento de latencia mínima para la línea de montaje.
  * `./.events/domain/` -> Aislamiento de alta seguridad para la gobernanza.



* **Refactor de Suscripciones (SddIA/core/):**
  El archivo original `event-subscriptions.json` colapsa y se divide en tres configuraciones homólogas e independientes en el Genoma, compartiendo la misma estructura contractual de ED event para garantizar la reutilización del motor:
  * `event-telemetry-subscriptions.json` -> Consumido por el proceso `route-telemetry` (Apunta a Radamanto).
  * `event-orchestration-subscriptions.json` -> Consumido por `route-orchestration` (Apunta a Acciones/Obreros).
  * `event-domain-subscriptions.json` -> Consumido por `route-domain` (Apunta a Cerbero/Cúmulo).

---

## VII. Flujo de Persistencia de Artefactos

Las Entidades de Dominio carecen de facultades físicas para escribir directamente en el disco del sistema operativo, preservando su Ceguera Espacial absoluta. La persistencia de los artefactos se ejecuta a través del siguiente canal encapsulado:



1. **Inyección de Contexto:** El orquestador inyecta en el micro-contexto del agente la ruta física de la carpeta de Espacio de Trabajo Aislado (Workspace / Directorio de Operación).
2. **Mutación en Memoria:** El agente computa la evolución del documento (ej. un plan de arquitectura o cambios de código) dentro de su ventana de contexto y genera el nuevo cuerpo del archivo.
3. **Invocación Encapsulada:** El agente genera un payload estructurado JSON y llama a la skill autorizada `filesystem-manager`. La comunicación se rige estrictamente por la interfaz `capsule-json-io` vía `stdin/stdout`.
4. **Sello Físico:** El binario ejecutable (en Rust) de la skill materializa la escritura real en el disco físico del sistema operativo. El artefacto queda guardado formalmente en estado "Pendiente/Teórico" en el repositorio hasta que el flujo de orquestación invoque a Argos para su validación final.

## Notas
* Radamanto será un nuevo agente auditor externo que está pendiente de implementar en docs\todos\pending\NuevoAgenteCertificador.md
* Hacer uso de rutas relativas, siendo la ruta compuesta la suma de lo facilitado por Cúmulo

## Dependencias a PBI's
docs\todos\pending\Refactor_Familias_Eventos.md
