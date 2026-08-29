---
document_id: PBI-ARCH-INFRA-ADAPTERS-SSOT-001
uuid: b7e4c1a9-2f83-4d6e-9a15-3c8f0d2b6e47
title: "[ARQUITECTURA] Gobernanza SSOT de infrastructure/adapters — indexación Cúmulo y observabilidad"
format: markdown
version: "1.1.0"
created: "2026-08-29"
refined: "2026-08-29"
status: refined
refinement_status: refined
priority: media
process: feature
type: architecture
dispatch: false
suggested_branch: feat/infra-adapters-ssot-governance
persist_ref_suggested: docs/features/infra-adapters-ssot-governance
depends_on: []
blocks:
  - PBI-KAIZEN-ESPEJO-CONSCIENCIA-001
related:
  - SddIA/core/cumulo.paths.json
  - SddIA/agents/cumulo.md
  - SddIA/infrastructure/adapters/lancedb_thought_repo/
  - SddIA/infrastructure/adapters/lancedb_evolution_repo/
  - docs/todos/pending/[KAIZEN] Espejo de Consciencia: Proyección de Salud y Observabilidad del Ecosistema.md
  - docs/todos/pending/[ARQUITECTURA] LanceDB — integración física real y memoria vectorial efectiva.md
---

# [ARQUITECTURA] Gobernanza SSOT de `infrastructure/adapters` — indexación Cúmulo y observabilidad

## 0. Refinado (2026-08-29)
Deuda de `PBI-KAIZEN-ESPEJO-CONSCIENCIA-001` §DD-7. DAs cerradas en `docs/features/infra-adapters-ssot-governance/{clarify,spec}.md` (`execution_id` `eb646386-6dc9-43d8-9b08-630de228a192`). Path canónico de este PBI: `docs/todos/pending/PBI-ARCH-INFRA-ADAPTERS-SSOT-001.md` (sin `/` en el filename).

## 1. Contexto y Fricción (origen de la deuda)

El árbol `SddIA/infrastructure/` (hoy: `adapters/lancedb_thought_repo`, `adapters/lancedb_evolution_repo`) **no está registrado** en `directories` de `SddIA/core/cumulo.paths.json` (SSOT topológico). Consecuencias verificadas:

1. **Zona ciega para Cúmulo.** Cúmulo solo gobierna las rutas declaradas en `directories`/`contracts`. Los adaptadores de infraestructura son, desde la óptica del SSOT, **Entropía/Código Fósil**: sin `index.md`, sin contrato, sin `uuid`, sin `{name}.md`.
2. **Bloqueo de observabilidad.** El Espejo de Consciencia (DD-7) **excluye** los conectores de infraestructura (LanceDB, etc.) porque mapearlos "a ciegas" violaría la soberanía de rutas y la Anti-Alucinación Espacial de Cúmulo (`cumulo.md` §4). Sin gobernanza SSOT, el panel no puede mostrar su salud sin inventar rutas.
3. **Ambigüedad de identidad.** Un adaptador no es hoy ni `tool`, ni `skill`, ni `daemon` catalogado; es un crate Rust suelto (`sddia-infrastructure-lancedb-thought`, ver `Cargo.toml`) fuera del catálogo de entidades atómicas.

Esta deuda es de **gobernanza topológica**, ortogonal a `PBI-CORE-LANCEDB-REAL-001` (integración física real + inclusión en build). Ambas pueden coexistir; esta habilita que la infraestructura sea **auditable y observable**, aquella la hace **funcional**.

## 2. Objetivo Medible

Dotar a `SddIA/infrastructure/**` de existencia soberana en el SSOT para que Cúmulo pueda indexarlo, auditarlo y exponer su topología a consumidores (Espejo de Consciencia). Éxito = Cúmulo resuelve la ruta de infraestructura desde `cumulo.paths.json`, valida la integridad de sus entradas y publica un inventario consultable; el Espejo puede entonces incorporar la fila "conectores de infraestructura" sin walk "a ciegas".

## 3. Decisiones a Resolver en Refinamiento (no cerradas)

- **DA-1 · Alta en SSOT.** Añadir clave `directories.infrastructure` (y/o `directories.infrastructure_adapters`) en `cumulo.paths.json`. Único agente autorizado a proponer el cambio: **Cúmulo** (soberanía de rutas). ¿Se modela como `directories` puro o también como `execution_capsules`/`products` (crates)?
- **DA-2 · Modelo de identidad del adaptador.** Opciones a evaluar:
  - (a) **Entidad atómica ligera:** cada adaptador con `{name}.md` + `uuid` + `type` + `contract` (nuevo `type: adapter` o reutilizar taxonomía existente) e `index.md` gobernado por Cúmulo.
  - (b) **Registro tabular:** un único `SddIA/infrastructure/adapters/index.md` que catalogue crates sin promover cada uno a entidad atómica plena.
  Decidir el mínimo viable que satisfaga Cúmulo sin sobre-forjar.
- **DA-3 · Contrato.** ¿Se crea `contracts.infrastructure` (p. ej. `infrastructure-contract.md`) análogo a los contratos existentes, o se subordina a un contrato existente? Definir cabecera YAML mínima exigible.
- **DA-4 · Señal de estado para observabilidad.** Definir qué campo declara el estado real de un adaptador (`status: placeholder|active|deprecated`) para que el Espejo lo pinte sin heurística. Enlaza con `PBI-KAIZEN-ESPEJO-CONSCIENCIA-001` §DD-4 (hoy un placeholder LanceDB devuelve `Ok(vec![])`/`Ok(None)` y no es distinguible por telemetría).
- **DA-5 · Forja legítima.** El alta de entidades/índice debe ejecutarse vía `entity-manager`/`execute-process`, no por mutación manual del genoma (norma motor `external-ai-constraints.md`).

## 4. Alcance

### Dentro
- Propuesta y aplicación (vía Cúmulo/`entity-manager`) del alta de `SddIA/infrastructure/**` en `cumulo.paths.json`.
- Definición del modelo de identidad/índice de adaptadores (DA-2) y su contrato mínimo (DA-3).
- Catalogación de los adaptadores existentes (`lancedb_thought_repo`, `lancedb_evolution_repo`) bajo el estándar elegido, con `status` declarado (DA-4).
- Cicatriz de decisión en `SddIA/evolution/{uuid}.md` (soberanía de rutas + taxonomía nueva).

### Fuera
- Integración física de LanceDB y su inclusión en el grafo de build/CI: pertenece a `PBI-CORE-LANCEDB-REAL-001`.
- Implementación del panel del Espejo de Consciencia: pertenece a `PBI-KAIZEN-ESPEJO-CONSCIENCIA-001` (este PBI solo lo desbloquea).
- Migración de datos vectoriales o embeddings.

## 5. Criterios de Aceptación (Protocolo de Acero)

| ID | Criterio | Verificación |
|----|----------|--------------|
| INF-CA1 | `cumulo.paths.json` declara la(s) ruta(s) de infraestructura; el cambio lo propone Cúmulo y queda versionado con SemVer del SSOT. | Diff de `cumulo.paths.json` + cicatriz de evolución. |
| INF-CA2 | Existe modelo de identidad/índice para adaptadores (DA-2) con su contrato mínimo (DA-3); Cúmulo lo audita sin marcar "Ruido de Sistema". | Ejecución de auditoría de Cúmulo APTA. |
| INF-CA3 | Los dos adaptadores LanceDB quedan catalogados con `uuid`/`status` coherentes con su estado real (placeholder hoy). | Revisión de índice/entidades + `src/lib.rs`. |
| INF-CA4 | Un consumidor (p. ej. la fusión del Espejo) puede resolver la topología de infraestructura desde el SSOT, sin walk "a ciegas" del árbol. | Lectura vía ruta indexada; ausencia de rutas cableadas. |
| INF-CA5 | La decisión (alta SSOT + taxonomía) queda registrada en `SddIA/evolution/` con UUID. | Artefacto de evolución. |
| INF-CA6 | Cierre documental en un único PR. | `validacion.md` APTO, `pbi_archived: true`, PBI movido a `done/`. |
