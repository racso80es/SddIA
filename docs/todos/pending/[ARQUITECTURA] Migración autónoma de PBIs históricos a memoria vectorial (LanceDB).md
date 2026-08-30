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
