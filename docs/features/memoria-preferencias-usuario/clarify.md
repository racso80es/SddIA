---
feature_name: memoria-preferencias-usuario
created: "2026-08-27"
process: feature
purpose: Estabilización Mayeuta — PBI-ARQ-CONSCIENCIA-UNIVERSAL v2.0.0
branch_name: feat/memoria-preferencias-usuario
persist_ref: docs/features/memoria-preferencias-usuario
pbi_ref: docs/todos/pending/PBI-globalizacion-consciencia-grafo.md
document_id: PBI-ARQ-CONSCIENCIA-UNIVERSAL
uuid: "7ad2ef99-4c50-4b6d-9cc1-313d3338bb1b"
execution_id: "56eb29e0-e2f5-46d1-90c6-48b918a1af8a"
mayeuta_verdict: ok
---

# Clarificación — memoria-preferencias-usuario

Transcript Mayeuta (2026-08-27). Semilla PBI v2.0.0 ya refinada. Este ciclo no reabre el “qué”; sella DoR y elimina residuales de ejecución.

---

## D0 — Apertura formal

| Pregunta | Decisión |
|----------|----------|
| Proceso | `feature` v1.3.2 |
| `feature_name` | `memoria-preferencias-usuario` |
| Rama | `feat/memoria-preferencias-usuario` |
| `persist_ref` | `docs/features/memoria-preferencias-usuario` |
| `document_id` | `PBI-ARQ-CONSCIENCIA-UNIVERSAL` |
| Init lab | `./sddia-run.sh --process feature` + `SDDIA_LAB_SKIP_PBI_ARCHIVE=1` + `SDDIA_LAB_SKIP_DELIVERY_CLOSE=1` + `SDDIA_AGENT_RUNTIME_COMMAND=` |
| `execution_id` | `56eb29e0-e2f5-46d1-90c6-48b918a1af8a` |
| Fase | Estabilización Mayeuta + Diseño Dedalo (esta sesión). **Tekton no arranca.** |

---

## D1 — DoR: consentimiento y ámbito

| Vacío PBI §13 | Laudo |
|---------------|-------|
| Aprobación del alcance local de instancia | El mandato de iniciar implementación **cierra** el consentimiento táctico: memoria **solo** en `.SddIA/` de esta instancia; cero federación entre clones. |
| Cifrado en reposo / backup | Amenaza documentada en `spec.md` §7. MVP: store gitignored + umask de instancia; cifrado en reposo **no** es CA de este ciclo (deuda explícita `DEUDA-PREF-CRYPTO`). |

---

## D2 — Productor y consumidor piloto (reales)

| Rol | Activo existente | Justificación |
|-----|------------------|---------------|
| Productor | `kalma2-bridge` (canal correo / umbral humano) | Intención explícita. El `email-watcher` permanece ciego. `email-quick-action-ingest` **no** es SSOT de hábitos (solo proof de archive/draft/delegate). |
| Consumidor | `telegram-fallback-responder` | Canal distinto (CA-08). Declara `requires_capability` de consulta; fail-open a contexto vacío. |

Prohibido inventar un gestor de calendario para la prueba de fuego.

---

## D3 — Dualidad con Grafo de Pensamiento (H6)

| Activo | Ontología | Uso |
|--------|-----------|-----|
| `ThoughtNode` / `ThoughtGraphRepository` | Razonamiento interno, contrapoder Tormentosa | **Ortogonal.** Prohibido persistir preferencias aquí. |
| `KnowledgeChunk` / `VectorStore` | Fragmentos de activos técnicos | No es perfil de usuario. |
| `EvolutionEvent` / `evolution/` | Telemetría de ejecución | Ortogonal; no porta `value` de preferencia. |
| `UserPreference` (este ciclo) | Directrices del Vértice Biológico | Puerto y colección **nuevos**. |

`LanceDbThoughtRepo` es placeholder (`Ok(())`). Este PBI **no** lo “completa” de pasada; si el adaptador de preferencias usa LanceDB, debe tener reapertura demostrable **propia**.

---

## D4 — Eventos: no Query_Subgraph

Consulta de subgrafo es **síncrona** vía cápsula + DI. Modelarla como ECST acoplaría latencia de bus al spawn y violaría “solo opt-in”.

Clases de dominio candidatas (forja posterior, no en esta sesión):

- `User_Preference_Change_Requested`
- `User_Preference_Changed`

Cero suscriptor `iota-immutable-publisher` en el MVP (CA-11).

---

## D5 — Mayeuta no escribe memoria (H4)

Mayeuta produce `PreferenceProposal` (NLP → predicado controlado). Persistencia = proceso + cápsula. Cúmulo orquesta; no embebe I/O LanceDB en el agente.

---

## D6 — Capacidades: no inventar en código

No existen `memory:*` en `capability-taxonomy` v1.0.6. Dedalo declara IDs **propuestos**; Tekton los materializa **antes** de que ningún process los `requires_capability`, vía mutación gobernada de la norma + schemas + bindings.

---

## D7 — Rutas de bus (H1–H3)

Instancias ECST: `eda_fractal.domain` → `./.events/domain/` (y coexistencia `eda_bus.pending` según familia/ruta del emisor canónico). `.SddIA/events/` no es cola. Emisión solo por acción/proceso indexado.
