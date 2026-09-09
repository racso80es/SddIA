---
feature_name: tool-ephemeral-cache-purger
created: "2026-09-09"
process: feature
branch_name: feat/tool-ephemeral-cache-purger
persist_ref: docs/features/tool-ephemeral-cache-purger
pbi_ref: docs/todos/pending/[FEATURE] Tool: ephemeral-cache-purger (Saneamiento Termodinámico).md
execution_id: "7f37a724-5d10-41df-9cc9-cf22dc275951"
document_id: PBI-FEATURE-TOOL-CACHE-PURGER
pbi_uuid: "0987ac64-2c95-41c4-9ca7-d777446034cb"
pbi_version: "1.2.0"
status: in-progress
---

# Objetivos — tool-ephemeral-cache-purger

## Misión

Cápsula nativa que inventaría y, bajo jail determinista, purga `cargo-target` huérfanos en `/tmp/cursor-sandbox-cache/<hash>/`. Cero inferencia LLM sobre rutas. Dos tiempos orquestados por acción. Evidencia de tests + CI antes de merge.

## Alcance (manifiesto)

- Ciclo `feature` inicializado (`execution_id` `7f37a724-…`).
- Tool `ephemeral-cache-purger` + crate Rust + tests de jail.
- Acción `purge-sandbox-cache` + handler nativo `execute-process`.
- Cierre documental en rama + DCC + PR. `accept-pr` condicionado a CI verde.

## Ley aplicada

- Git vía `skill:git-manager`. Troncal `main`.
- DA-2/DA-4: topología `objectives.md` en rama antes de mutar genoma (`tools/`, `actions/`).
- `features-documentation-pattern` v1.2.1: un PR; `validacion.md` APTO solo con CA-CI verde (`run_id`).
- `CONSTITUTION_CORE` Filtro A: no declarar Done sobre diffs locales.
- `actions-contract` §2/§2bis: la acción no invoca Argos.
- `sddia-io` es el envelope de la tool.

## Criterios (PBI v1.2.0)

PURGE-CA1…CA8 según PBI. CA7 = fixture, no `df` de host root. CA8 = CI `run_id`.
