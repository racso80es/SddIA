---
feature_name: memoria-preferencias-usuario
created: "2026-08-27"
process: feature
branch_name: feat/memoria-preferencias-usuario
persist_ref: docs/features/memoria-preferencias-usuario
pbi_ref: docs/todos/done/[ARQUITECTURA] Globalización de la Consciencia del Usuario (Grafo de Pensamiento Universal).md
document_id: PBI-ARQ-CONSCIENCIA-UNIVERSAL
uuid: "7ad2ef99-4c50-4b6d-9cc1-313d3338bb1b"
execution_id: "56eb29e0-e2f5-46d1-90c6-48b918a1af8a"
depends_on:
  - docs/features/memoria-vectorial
  - docs/features/grafo-pensamiento
adjacent_not_merged:
  - docs/features/kaizen-capsula-imap-triaje
mayeuta_verdict: ok
dedalo_verdict: ok
---

# Objetivos — memoria-preferencias-usuario

## Misión

Instanciar una **memoria soberana de preferencias** del Vértice Biológico (`UserPreference`), local a la instancia, reutilizable entre canales. Las herramientas permanecen actuadores ciegos: no aprenden perfiles propios. El grafo interno `ThoughtNode` no se reutiliza.

## Punto objetivo

> **O-PREF:** Una corrección explícita capturada en el canal correo queda persistida con autoridad, ámbito y procedencia; un proceso de otro canal (Telegram) la recupera sin conocer el origen; una revocación posterior impide su reinyección. Inferencias no confirmadas nunca se activan.

## Alcance

| Dentro | Fuera |
|--------|-------|
| Modelo `UserPreference` + puerto + adaptador durable con prueba de reapertura | Reutilizar `ThoughtNode` / `thought_graph_collection` |
| Escritura gobernada (propuesta → activación / supersede / revocación / purga) | Autoaprendizaje vinculante por frecuencia |
| Lectura **opt-in** vía capacidad DI registrada | Inyección en todas las ejecuciones del orquestador |
| Productor piloto: `kalma2-bridge` (intención humana de correo) | Watcher IMAP como emisor o almacén |
| Consumidor piloto: `telegram-fallback-responder` | Calendario ficticio / gestor de calendario nuevo |
| Store local gitignored; telemetría sin valores personales | Anclaje DLT de `value`; memoria entre usuarios/instalaciones |

## Objetivos medibles

| ID | Objetivo | Criterio (PBI) |
|----|----------|----------------|
| **O1** | Ontología propia | CA-01, CA-02 |
| **O2** | Canales ciegos | CA-03 |
| **O3** | Autoridad | CA-04, CA-05, CA-06 |
| **O4** | Persistencia real | CA-07 |
| **O5** | Cruce de dominio | CA-08 |
| **O6** | Opt-in + fallo declarado | CA-09, CA-14 |
| **O7** | Precedencia determinista | CA-10 |
| **O8** | Privacidad | CA-11, CA-15 |
| **O9** | Contratos | CA-12, CA-13 |

## No objetivos

- Convertir `LanceDbThoughtRepo` placeholder en SSOT de hábitos.
- Consulta síncrona modelada como evento ECST (`Query_Subgraph` rechazado).
- Alta informal de `capability_id` fuera de `capability-taxonomy`.
- Confirmar DoR de consentimiento más allá de este ciclo: el estímulo de implementación equivale a mandato de alcance local de instancia.

## Ley aplicada

- `features-documentation-pattern` v1.2.1 / proceso `feature` v1.3.2
- `events-contract` v1.1.0 — Clase = `{name}.md` bajo `directories.events/{family}/`
- `capsule-json-io` v2.0
- `capability-taxonomy` — AC-NO-INVENT; alta gobernada + `Domain_Entity_Updated`
- `CONSTITUTION_CORE` — Triaje C/A/B; datos personales no transmutados a ruido DLT
- DA-2/DA-3: genoma vía `entity-manager`; DA-4 topología activa; DA-5 fire-and-forget
- SSOT rutas: `SddIA/core/cumulo.paths.json`
