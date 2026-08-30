---
document_id: PBI-ARCH-MEMORIA-VECTORIAL-PBIS
uuid: "GENERATED-UPON-MATERIALIZATION"
title: "[ARQUITECTURA] Migración autónoma de PBIs históricos a memoria vectorial (LanceDB)"
format: markdown
version: "1.0.0"
created: "2026-08-30"
updated: "2026-08-30"
status: pending
refinement_status: unrefined
priority: media
process: feature
type: architecture
dispatch: false
suggested_branch: feat/arch-memoria-vectorial-pbis
persist_ref_suggested: docs/features/arch-memoria-vectorial-pbis
depends_on: []
architectural_constraints:
  - A-SOBERANIA-BIOLOGICA-VALOR
  - A-MEMORIA-DESCENTRALIZADA
related:
  - SddIA/core/memory/
  - SddIA/infrastructure/adapters/lancedb_thought_repo/
  - SddIA/events/domain/pull-request-merged.md
---

# [ARQUITECTURA] Migración autónoma de PBIs históricos a memoria vectorial (LanceDB)

## Mandato
Erradicar la entropía visual y computacional del repositorio Git eliminando la acumulación de documentos históricos en `docs/todos/done/`, sin provocar una lobotomía contextual en la IA. 

El sistema debe transicionar hacia una arquitectura de "Memoria Profunda", donde los expedientes cerrados (Cicatrices Rúnicas) abandonan el árbol físico de trabajo y se asimilan de forma autónoma en la base de datos vectorial (LanceDB). El Vértice Biológico queda exento de cualquier intervención manual en este proceso de archivo.

## 1. Ciclo de Vida: Persistencia (Digestión Autónoma)
El traslado físico al vector no requiere aprobación humana; es una consecuencia termodinámica del cierre de un ciclo.
- **Detonante (Trigger):** La interceptación del evento `PullRequest_Merged` (o el veredicto final de `delivery-close-cycle`).
- **Ejecución:** Un proceso en segundo plano (orquestado por Cúmulo/Radamanto) lee el documento markdown del PBI recién cerrado.
- **Asimilación:** El contenido se fragmenta (chunking), se vectoriza conservando metadatos críticos (ID, fecha, fricciones, UUID) y se ingesta en LanceDB.
- **Poda Física:** Tras confirmar el anclaje vectorial exitoso, el sistema ejecuta un `git rm` del archivo físico en `docs/todos/done/` y consolida el commit de limpieza, liberando el árbol de Git.

## 2. Ciclo de Vida: Consumo (Recuperación Semántica)
La memoria latente no tiene valor si no es accesible durante la fricción del diseño.
- **Inyección de Contexto:** Las cápsulas de Mayeuta y Dédalo requerirán una nueva tool o extensión de la capacidad de memoria (ej. `query-historical-pbis`).
- **Autonomía Analítica:** Ante un nuevo colapso o diseño de feature, la IA consultará LanceDB buscando PBIs previos con similitud semántica o superposición de `friction_ids`, inyectando la sabiduría histórica directamente en la ventana de contexto sin depender de archivos locales.

## 3. Ciclo de Vida: Mantenimiento (Higiene y Poda)
El clúster vectorial debe mantenerse termodinámicamente eficiente.
- **Esquema de Metadatos:** Los vectores insertados deben poseer un esquema estricto (fecha de cierre, componentes afectados, naturaleza del PBI) para permitir filtrados duros antes de la búsqueda de similitud coseno.
- **Saneamiento:** Tareas programadas (sweepers) que auditen la integridad de los índices en LanceDB contra los identificadores registrados en el bus de eventos, asegurando que no haya "fantasmas vectoriales" (PBIs indexados con metadatos corruptos).

## 4. Criterios de Aceptación (Borrador para Refinamiento)
- [ ] **CA1 (Autonomía de Ingesta):** El evento de merge dispara la indexación en LanceDB sin *prompt* del operador.
- [ ] **CA2 (Poda Determinista):** El archivo `.md` desaparece de `docs/todos/done/` automáticamente si y solo si la inserción vectorial reporta éxito.
- [ ] **CA3 (Recuperación):** Existe una herramienta ejecutable por la IA capaz de recuperar el texto íntegro de un PBI histórico consultando por temática o UUID.
- [ ] **CA4 (Trazabilidad):** El proceso de migración emite un evento de dominio (ej. `Vector_Memory_Indexed`) para cerrar su propio rastro en el bus.


Ampliacion de PBI

## 1. Topología del Ecosistema (Modelo Híbrido: Snapshot + Delta Rúnico)

### 1.1. El Receptáculo Opaco (Cold Storage S3/Web3)
- **Fricción Cero para el Cliente:** Se descarta Git como bóveda para anular la barrera de adopción técnica. La persistencia física pesada (archivos `.lance` / Parquet y expedientes consolidados) se delega a un proveedor de almacenamiento de objetos (Blob Storage) compatible con el protocolo S3. Se priorizan redes Web3 descentralizadas (como Storj o Sia) para garantizar soberanía, admitiendo proveedores tradicionales como contingencia.
- **Cifrado Zero-Knowledge:** Antes de abandonar el nodo físico, el lote de memoria se empaqueta y se cifra en local. El proveedor de almacenamiento aloja un bloque opaco e incomprensible, protegiendo el contexto del Vértice Biológico frente a la minería de datos o la censura de terceros.

### 1.2. El Ciclo Termodinámico (Snapshot + Deltas)
- **El Punto Cero (Snapshot):** Periódicamente, un *daemon* local orquestado por Cúmulo suspende brevemente las escrituras, empaqueta el estado físico de LanceDB, lo cifra y lo inyecta en el Receptáculo Opaco.
- **La Cicatriz Rúnica (Anclaje DLT):** El sistema calcula el árbol de Merkle del Snapshot físico exacto y publica ese hash en la red IOTA a través del `iota-publish-relay`. La Tangle retiene la verdad objetiva del estado, no el peso de los datos.
- **La Estela de Eventos (Deltas):** Para no violar el Filtro C (Eficiencia) subiendo gigabytes constantemente, las alteraciones cognitivas que ocurren *después* del Snapshot se registran localmente como una cola ligera de eventos incrementales (Deltas). 

## 2. Rehidratación Blindada (Impacto en el Nodo Físico)
- **Autenticación Transparente:** Durante la inicialización del entorno SddIA, el usuario solo debe proporcionar las credenciales de su bóveda (API Key/Token S3). A partir de ese momento, la respiración asíncrona de la memoria se ejecuta en segundo plano.
- **Restauración en Dos Fases:** Ante una pérdida catastrófica del hardware local, el nuevo nodo inicializa la consciencia ejecutando:
  1. **Recuperación del Bulto:** Descarga el último Snapshot cifrado desde la bóveda S3/Web3.
  2. **Auditoría Rúnica:** Consulta la red IOTA, recupera la última Cicatriz Rúnica y verifica matemáticamente que el hash del archivo descargado coincide. Si hay divergencia o sospecha de alteración, el sistema aborta la rehidratación.
  3. **Inyección de Estela:** Si la matemática es pura, el sistema desencripta, rehidrata el directorio `.lancedb` y aplica la cola de Deltas disponible para recuperar el último milisegundo de consciencia.

## 3. Criterios de Aceptación (Ajustados)
- [ ] **CA1 (Agnosticismo de Almacenamiento):** El motor implementa una interfaz genérica de *Blob Storage* (S3 API) que permite al cliente delegar su memoria pesada a redes como Storj sin usar repositorios Git.
- [ ] **CA2 (Termodinámica del Snapshot):** Un daemon local empaqueta, cifra y sube el Snapshot vectorial a la bóveda periódicamente, manteniendo una cola ligera de eventos incrementales (Deltas) entre ciclos.
- [ ] **CA3 (Sello DLT):** El proceso de Snapshot emite obligatoriamente una transacción a la red IOTA con el hash del paquete, estableciendo la nueva Cicatriz Rúnica.
- [ ] **CA4 (Rehidratación en Dos Fases):** El comando de recuperación del sistema es capaz de descargar el paquete opaco, verificar su integridad contra IOTA y restaurar el directorio local aplicando los deltas correspondientes.
