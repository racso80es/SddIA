---
document_id: PBI-CORE-EVOLUTION-003-V2
title: "[ARQUITECTURA] PBI: Bóveda de Evolución Epigenética (Aprendizaje Continuo Basado en Telemetría)"
format: markdown
version: "2.0.0"
created: "2026-06-04"
refined: "2026-06-11"
status: done
priority: arquitectura-core
closed: "2026-06-11"
depends_on:
  - PBI-CORE-VECTOR-001-V3
  - PBI-CORE-GRAPH-002
active_feature: docs/features/boveda-evolucion-epigenetica
merged_pr: 81
merge_commit: 82c360c
origin: docs/todos/kitchen/Bóveda de Evolución Epigenética.md
---

# [ARQUITECTURA] PBI: Bóveda de Evolución Epigenética (Aprendizaje Continuo Basado en Telemetría) — v2

| Campo | Valor |
|-------|-------|
| **ID** | `PBI-CORE-EVOLUTION-003-V2` |
| **Estatus** | ✅ Done — implementado y validado |
| **Dependencias** | `PBI-CORE-VECTOR-001-V3` ✅ · `PBI-CORE-GRAPH-002` ✅ |
| **Feature** | [`docs/features/boveda-evolucion-epigenetica/`](../../features/boveda-evolucion-epigenetica/) |
| **Validación** | [`validacion.md`](../../features/boveda-evolucion-epigenetica/validacion.md) — APTO |
| **Merge** | PR #81 → `82c360c` |
| **Origen kitchen** | `docs/todos/kitchen/Bóveda de Evolución Epigenética.md` |

## 1. Identificación y Estatus del Activo

* **Código de Item:** PBI-CORE-EVOLUTION-003-V2
* **Dependencias Críticas:**
  * `PBI-CORE-VECTOR-001-V3` (Motor de Memoria Vectorial y Aduana Semántica) — ✅ satisfecha.
  * `PBI-CORE-GRAPH-002` (Grafo de Pensamiento Espacial) — ✅ satisfecha.
* **Naturaleza:** Arquitectura SddIA / Evolución Ontológica / Aprendizaje por Refuerzo Local
* **Entorno:** Cúmulo Core / Bus de Eventos Reactivo (`.SddIA/events/`)
* **Grado de Alineación:** S+ Grade

## 2. Objetivo Estratégico

Establecer un circuito cerrado de retroalimentación, maduración y crecimiento autónomo para el ecosistema SddIA. El propósito es recopilar de forma transparente la telemetría operativa de los bordes del sistema (indicadores de éxito, tiempos de ejecución, coste termodinámico de tokens y bloqueos) y traducirla a una matriz espacial de experiencia. Esta base de conocimiento dinámico permitirá al orquestador consultar sus aciertos y fracturas del pasado inmediato para refinar estrategias, optimizar el enrutamiento semántico y auto-corregirse de forma continua sin requerir reentrenamientos paramétricos.

## 3. Topología Fractal del Almacenamiento (Estructura Multi-Índice)

La Base de Datos Vectorial local (LanceDB) se estructura mediante tres grupos de colecciones físicamente independientes:

```text
.SddIA/
└── vector_store/
    ├── artifacts/          <-- Contexto Estático: Normas canónicas, habilidades y contratos (PBI 1)
    ├── thoughts/           <-- Contexto Transitorio: Nodos y borradores activos del Grafo (PBI 2)
    └── evolution/          <-- Contexto Acumulativo: Experiencia, éxitos y telemetría de fricción (PBI 3)
```

## 4. Componentes implementados

| Componente | Descripción | Ruta |
|------------|-------------|------|
| `EvolutionEvent` | Entidad con id SHA-256 determinista | `SddIA/core/memory/src/models/evolution_node.rs` |
| `SpatialPolarity` | `EfficientSymmetry` / `StructuralFracture` | `evolution_node.rs` |
| `EvolutionProxyService` | Proxy de captura pasiva de telemetría | `SddIA/core/memory/src/services/evolution_proxy.rs` |
| `EvolutionStore` | Puerto hexagonal de persistencia | `SddIA/core/memory/src/ports.rs` |
| `LanceDbEvolutionRepo` | Adaptador físico LanceDB | `SddIA/infrastructure/adapters/lancedb_evolution_repo/` |

## 5. Flujo de Datos

1. **Emisión:** Un componente en los bordes del sistema finaliza su ejecución y emite un evento estándar al directorio `.SddIA/events/`.
2. **Captura:** El Proxy de Captura Pasiva detecta el nuevo evento y extrae métricas operativas.
3. **Clasificación y Hashing:** El clasificador etiqueta el evento (`Simetría Eficiente` o `Fractura Estructural`) y calcula el hash SHA-256 determinista.
4. **Cristalización Semántica:** La Aduana Semántica local genera el vector embebido del cuerpo del evento.
5. **Persistencia:** El registro se guarda en `.SddIA/vector_store/evolution/`.

## 6. Criterios de Aceptación (Definition of Done)

| ID | Criterio | Estado |
|----|----------|--------|
| BE-CA1 | `EvolutionEvent` id SHA-256 determinista | ✅ |
| BE-CA2 | Polaridad espacial (`EfficientSymmetry` / `StructuralFracture`) | ✅ |
| BE-CA3 | Puerto `EvolutionStore` hexagonal | ✅ |
| BE-CA4 | Adaptador LanceDB ruta `evolution/` | ✅ |
| BE-CA5 | Proxy captura pasiva (`capture_event`) | ✅ |
| BE-EDA1 | Evento ECST `Vector_Memory_Indexed` | ✅ |
| BE-EDA2 | Suscripción en `event-subscriptions.json` | ✅ |

## 7. Evidencia de implementación

| Artefacto | Ruta |
|-----------|------|
| Evento ECST | `SddIA/events/domain/vector-memory-indexed.md` |
| Suscripciones EDA | `SddIA/core/event-subscriptions.json` |
| Ignore vector store | `.gitignore` → `.SddIA/vector_store/` |
