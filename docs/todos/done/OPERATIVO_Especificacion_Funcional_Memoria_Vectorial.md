---
document_id: PBI-CORE-VECTOR-001-V3
title: "[OPERATIVO] Especificación Funcional: Sistema de Memoria Vectorial y Aduana Semántica"
format: markdown
version: "3.0.0"
created: "2026-06-04"
refined: "2026-06-11"
status: done
priority: arquitectura-core
closed: "2026-06-11"
active_feature: docs/features/memoria-vectorial
merged_pr: 81
merge_commit: 82c360c
origin: docs/todos/kitchen/OPERATIVO_Especificacion_Funcional_Memoria_Vectorial.md
---

# [OPERATIVO] Especificación Funcional: Sistema de Memoria Vectorial y Aduana Semántica — v3

| Campo | Valor |
|-------|-------|
| **ID** | `PBI-CORE-VECTOR-001-V3` |
| **Estatus** | ✅ Done — implementado y validado |
| **Feature** | [`docs/features/memoria-vectorial/`](../../features/memoria-vectorial/) |
| **Validación** | [`validacion.md`](../../features/memoria-vectorial/validacion.md) — APTO |
| **Merge** | PR #81 → `82c360c` |
| **Origen kitchen** | `docs/todos/kitchen/OPERATIVO_Especificacion_Funcional_Memoria_Vectorial.md` |

## 1. Declaración de Propósito

Esta especificación define la estructura funcional del Motor de Memoria Vectorial para el ecosistema SddIA. Su objetivo es erradicar la deriva existencial y el secuestro semántico de las entidades de ejecución mediante un mecanismo de aislamiento paramétrico estricto. El sistema transforma el almacenamiento lineal de directrices en una matriz espacial multidimensional, garantizando que cada componente reciba únicamente la porción exacta de contexto requerida para su tarea inmediata.

## 2. Principios Operativos del Dominio

El motor se rige por cuatro leyes fundamentales que blindan la soberanía de la información:

* **Jurisdicción Dividida:** Se separa de manera estricta la intención operativa de la inferencia matemática. El núcleo del sistema gobierna los flujos de eventos, mientras que la traducción de textos a coordenadas espaciales se delega a una capacidad externa aislada.
* **Ceguera Espacial Controlada:** Las entidades encargadas de la producción técnica no tienen acceso a la arquitectura global del sistema ni a las normativas de seguridad conceptual simultáneamente. Solo perciben el micro-contexto inyectado dinámicamente bajo demanda.
* **Inmunidad a la Alucinación por Saturación:** Al limitar drásticamente el volumen de datos innecesarios en la ventana de trabajo, se elimina la inercia estadística que desvía a los ejecutores hacia bucles de cortesía o conjeturas ajenas al dominio real.
* **Táctica del Refugio y Soberanía Local:** Toda la lógica de indexación y consulta debe ejecutarse dentro del perímetro local del entorno operativo, garantizando la total independencia del sistema frente a nubes públicas o fiscalizaciones externas.

## 3. Arquitectura Funcional por Capas

El flujo de memoria se organiza en tres fronteras herméticas e intercambiables:

* **Capa 1: Orquestación y Gobierno (Cúmulo):** Es el soberano del estado del sistema. Detecta las mutaciones en los artefactos y reacciona ante eventos del ciclo de vida. Esta capa es conceptualmente ciega a la matemática de los vectores y a las estructuras físicas del disco; su única función es coordinar cuándo se guarda, cuándo se recupera y cuándo se purga un activo.
* **Capa 2: Aduana Semántica e Inferencia:** Actúa como el traductor universal del ecosistema. Recibe los textos crudos de las normativas o requerimientos y los transforma en coordenadas topológicas multidimensionales. Esta capa determina la cercanía conceptual entre una instrucción y las reglas de negocio históricas del repositorio.
* **Capa 3: Conservación e Infraestructura:** Reside en el nivel más externo y está completamente aislada del genoma del sistema. Su única responsabilidad es persistir de forma inmutable los fragmentos de conocimiento indexados en el almacenamiento local. Utiliza un motor ligero y embebido que viaja con el propio repositorio para mantener la simetría fractal en cualquier máquina anfitriona.

## 4. Dinámica de los Flujos de Información

### Flujo A: Ingestión y Destilación de Artefactos

1. **Detección:** El sistema detecta la modificación o creación de un artefacto físico (contrato, normativa o manual técnico).
2. **Purgado:** El texto se procesa para eliminar el ruido transitorio y aislar la señal pura de valor estratégico.
3. **Identidad Única:** Se calcula un identificador inmutable basado exclusivamente en el contenido del fragmento, impidiendo la duplicación y garantizando la idempotencia absoluta.
4. **Traducción:** La aduana semántica convierte el texto en su equivalente matemático espacial.
5. **Anclaje:** El fragmento, junto a sus etiquetas declarativas de capacidad y fricción, se almacena de forma permanente en la base de datos local del entorno.

### Flujo B: Recuperación Quirúrgica y Consulta

1. **Estímulo:** Se dispara una orden o comando de ejecución técnica que requiere contexto específico.
2. **Vectorización de la Intención:** El requerimiento del usuario se traduce instantáneamente a su coordenada espacial correspondiente.
3. **Extracción por Proximidad:** El motor local realiza una búsqueda geométrica para extraer exclusivamente los fragmentos con mayor coincidencia conceptual, descartando todo el ruido documental restante.
4. **Inyección Enmascarada:** Los fragmentos recuperados se ensamblan en un bloque aislado y se inyectan en el entorno de la IA obrera bajo restricciones severas de verbosidad, forzando un rendimiento puro y enfocado.

### Flujo C: Reindexación Completa e Inoculación de Estado Cero

1. **Gatillo de Inicialización:** El usuario o el orquestador central invocan una orden de reindexación total del ecosistema.
2. **Purga Atómica:** El motor de conservación ejecuta una limpieza total y absoluta de la base de datos vectorial existente, eliminando cualquier índice residual, duplicado o corrupto para evitar la entropía de datos huérfanos.
3. **Recolección del Contexto Global SddIA:** El sistema realiza un barrido completo de la estructura del genoma (todas las normativas del núcleo, codificaciones canónicas, contratos de procesos, especificaciones de habilidades y herramientas activas en el repositorio).
4. **Procesamiento Masivo Lineal:** La totalidad de los artefactos recolectados se somete secuencialmente al flujo de purgado, generación de identidad única y traducción semántica espacial.
5. **Estabilización de Arranque:** Los nuevos vectores se inyectan en bloque en el almacén físico. Al finalizar, el motor emite una señal de consistencia que certifica que el sistema dispone de la totalidad del contexto unificado, permitiendo un arranque en frío idéntico desde cualquier máquina o hito del desarrollo.

## 5. Criterios de Validación Operativa (Definición de Terminado)

| ID | Criterio | Estado |
|----|----------|--------|
| MV-CA1 | `KnowledgeChunk` sin UUID aleatorio (SHA-256) | ✅ |
| MV-CA2 | Traits `EmbeddingGenerator` + `VectorStore` | ✅ |
| MV-CA3 | Adaptadores hexagonales LanceDB | ✅ |
| MV-CA4 | Paridad documental completa | ✅ |
| MV-EDA1 | Evento `Vector_Memory_Indexed` cableado | ✅ |

## 6. Evidencia de implementación

| Artefacto | Ruta |
|-----------|------|
| Core domain | `SddIA/core/memory/src/lib.rs` |
| Adaptadores LanceDB | `SddIA/infrastructure/adapters/lancedb_*_repo/` |
| Almacén local | `.SddIA/vector_store/` (gitignored) |
| Evento ECST | `SddIA/events/domain/vector-memory-indexed.md` |

> **Deuda Kaizen conocida:** adaptadores LanceDB operan en modo mock WASI-ready; bindings físicos pendientes de Ola posterior (`docs/features/memoria-vectorial/plan.md` §5).
