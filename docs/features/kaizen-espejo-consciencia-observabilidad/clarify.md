---
feature_name: kaizen-espejo-consciencia-observabilidad
created: "2026-08-29"
process: feature
purpose: Estabilización Mayeuta — Espejo de Consciencia (mapa × territorio)
version_clarify: "1.0.0"
uuid: 97d96117-49cf-4db7-b860-acd65bee216a
execution_id: "a15ad28b-27a3-491c-902e-f78c100ffd43"
document_id: PBI-KAIZEN-ESPEJO-CONSCIENCIA-001
pbi_ref: docs/todos/pending/[KAIZEN] Espejo de Consciencia: Proyección de Salud y Observabilidad del Ecosistema.md
---

# Clarificación — kaizen-espejo-consciencia-observabilidad

Semilla: PBI v1.2.0. Init lab `execution_id` `a15ad28b-27a3-491c-902e-f78c100ffd43`. Relé IDE (`SDDIA_AGENT_RELAY_IDE=1`); archive/delivery omitidos.

## D0 — Apertura

| Pregunta | Decisión |
|----------|----------|
| Proceso | `feature` v1.3.2 |
| `feature_name` | `kaizen-espejo-consciencia-observabilidad` |
| Rama | `feat/kaizen-espejo-consciencia-observabilidad` |
| `persist_ref` | `docs/features/kaizen-espejo-consciencia-observabilidad` |
| Norma documental | `features-documentation-pattern` v1.2.1 |

## D1 — Tres vectores, no uno

El territorio no es Radamanto. Fuentes de instancia:

| Vector | Dueño | Artefacto (Cúmulo) |
|--------|-------|-------------------|
| Vida Centinelas | Argos / `daemon-heartbeat-audit` | `daemons_instance.state` + `heartbeat-audit.json` |
| Termodinámica skill/tool | Radamanto / `radamanto-batch` | `radamanto.stats` → `entities[].status` |
| Gobernanza | Cerbero | `radamanto.revoked_entities` |

## D2 — Placeholder ≠ fallo

Sin ejecuciones en `stats.json` → **GRIS**. ROJO solo: `missed_cycles >= 3`, `deprecated`, revocada. Marca de contrato placeholder = Fase 2.

## D3 — Infra SSOT vivo, panel no

`directories.infrastructure_adapters` + `index.md` existen (PBI-ARCH-INFRA-ADAPTERS-SSOT-001, `cumulo.paths.json` v1.7.0). El alcance Fuera / DD-7 del PBI **sigue**: MVP **no** pinta filas de adaptadores. CA7 se cumple por omisión de familia, no por ceguera SSOT. IOTA como **tool** sí entra (está en `tools/index.md`).

## D4 — Fusión: dónde y cuándo

DD-5(a): módulo Core Rust, no cápsula. DD-3: merge lícito on-demand sobre artefactos de instancia.

| Capa | Cuándo | Quién |
|------|--------|-------|
| Map-snapshot | Eventos `Domain_Entity_{Created\|Updated\|Deleted}` + seed CLI | Cúmulo (lee `index.md` de tools/skills/daemons) |
| Territorio | Ya vivo | Argos / Radamanto / Cerbero |
| Fusión | `GET /api/system-health` (y proceso `query-ecosystem-health` para seed/rebuild) | Bridge + handler nativo; **misma** función Rust |

Prohibido `execute-process` síncrono dentro de `daemon-heartbeat-audit` / `radamanto-batch`. No suscribir fusión a `Daemon_Heartbeat` (alta frecuencia).

## D5 — Sin evento nuevo

No forjar `Ecosystem_State_Changed` en MVP. Disparo de mapa = suscripciones dominio ya existentes. Territorio = ficheros.

## D6 — Ceguera espacial

Bridge/WUI: solo `.SddIA/**` + Cúmulo paths JSON. Cero parseo de `{name}.md` de genoma. El snapshot es el único inventario esperado.

## D7 — Superficie Kalma2

Réplica del pulso cognitivo: pull `GET /api/system-health`, panel contiguo, sin SSE obligatorio (el territorio no es cola de telemetría). Sin botones de intervención (Despertador Inerte).
