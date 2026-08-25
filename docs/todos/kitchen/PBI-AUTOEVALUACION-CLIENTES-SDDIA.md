---
document_id: PBI-AUTOEVALUACION-CLIENTES-SDDIA
title: "[ARQUITECTURA] Bucle de Autoevaluación y Telemetría en Clientes SddIA"
format: markdown
version: "1.0.0"
created: "2026-08-24"
status: "abierto"
priority: "alta"
process: "feature"
related:
  - SddIA/agents/mayeuta.md
  - SddIA/agents/radamanto.md
  - SddIA/agents/cerbero.md
  - SddIA/norms/entidades-dominio-ecosistema-sddia.md
---

### [ARQUITECTURA] Bucle de Autoevaluación y Telemetría en Clientes SddIA

#### 1. Contexto Arquitectónico
La topología actual de autoevaluación genera documentos `todo pending` en el entorno físico donde ocurre la fricción. Esto es termodinámicamente correcto en nuestro repositorio de desarrollo (Lab), pero representa una **alucinación arquitectónica** y un riesgo de Entropía Táctica si se traslada a las instancias de los clientes de la Librería SddIA. 

El cliente es un consumidor de inteligencia industrializada y carece del ancho de banda y la autoridad técnica para resolver fallos estructurales del motor. Este PBI define la migración hacia una **Topología de Autoevaluación Reactiva**, silenciando la materialización de artefactos pendientes en el cliente y enrutando la anomalía hacia el Core (Lab) mediante un puente de telemetría DLT.

#### 2. Superficie a Portar y Modificar

**Fase 1: Mutación del Archivo al "Log de Fricción" (Sistema Nervioso)**
*   **Anulación de I/O Física en Cliente:** Las Entidades de Dominio (ED) desplegadas en instancias `client` tienen estrictamente prohibido persistir sus fallos como archivos Markdown en `docs/todos/pending/`.
*   **Emisión Ciega:** Los errores de ejecución o rupturas de contrato se emitirán de forma transparente como eventos en el bus fractal (`.SddIA/events/telemetry/`).

**Fase 2: El Escudo de Mayeuta (Triaje Autónomo Local)**
*   **Asimilación Autónoma:** El agente Mayeuta escanea proactivamente este ruido técnico. 
*   **Destilación de Fricción Evolutiva:** Mayeuta aplica el Filtro C para descartar la latencia o fallos de red, extrayendo y encapsulando exclusivamente las anomalías éticas o los colapsos lógicos de la arquitectura.

**Fase 3: Radamanto y Contención RBAC (Self-Healing)**
*   **Certificación de Degradación:** Radamanto actúa como actuario de confianza. Si evalúa que los logs destilados por Mayeuta superan el umbral de tolerancia, sella estadísticamente el fallo y emite un evento `Tool_Degraded`.
*   **Revocación y Cuarentena:** Cerbero intercepta este evento y ejecuta un *Hard Override*, revocando inmediatamente los permisos RBAC de la entidad defectuosa en el entorno del cliente. 
*   **Intento de Reparación:** Tekton/Dédalo inician una reparación aislada en el *sandbox* local, validada empíricamente por Argos.

**Fase 4: El Puente DLT hacia el Core (Retorno a Tormentosa)**
*   **Telemetría ZKP:** Si la falla colapsa y no puede ser reparada localmente, la esencia del error se cifra y se emite de forma asíncrona hacia la red DLT (IOTA Rebased) mediante la cápsula `iota-immutable-publisher`. Se aplican Pruebas de Conocimiento Cero (ZKP) para garantizar la total invisibilidad de los datos del cliente frente al exterior.
*   **Materialización Soberana en el Lab:** Tormentosa intercepta la señal DLT entrante en el entorno de desarrollo (Lab). Es aquí, y solo aquí, donde el hallazgo supera el Filtro de Materialización y se consolida en el Cúmulo como un documento `todo pending` real para el Vértice Biológico.

#### 3. Touchpoints Físicos a Actualizar
*   **SddIA/agents/mayeuta.md:** Ampliar sus políticas operativas para procesar logs de fricción y ejecutar el cortafuegos técnico.
*   **SddIA/agents/radamanto.md:** Refinar los umbrales deterministas de telemetría para lanzar `Tool_Degraded` en instancias comerciales.
*   **SddIA/process/route-domain-event.md:** Condicionar el flujo de escritura física de un "todo" en función de la jerarquía de bóvedas (`SDDIA_ENV=lab` vs `SDDIA_ENV=client`).
*   **SddIA/tools/iota-immutable-publisher.md:** Integrar el soporte para el envío de telemetría de fricción ZKP hacia las coordenadas inmutables del Core.

#### 4. Criterios de Aceptación (Protocolo de Acero)
- [ ] Ante el colapso de una ED, las instancias configuradas con perfil `client` no generan archivos bajo `docs/todos/pending/`.
- [ ] La secuencia reactiva se cumple íntegramente: `Raw_Execution_Finished (fail)` -> `Mayeuta (Triaje)` -> `Radamanto (Tool_Degraded)`.
- [ ] El agente Cerbero revoca instantáneamente los permisos RBAC de la herramienta tras el sellado de Radamanto.
- [ ] La cápsula `iota-immutable-publisher` ancla la fricción destilada en la red IOTA Testnet preservando la privacidad estructural del cliente.
- [ ] El orquestador del Lab es capaz de leer la resonancia micelial DLT y materializar exitosamente un documento `todo pending` en su propio repositorio para que Tormentosa asuma la reparación.
