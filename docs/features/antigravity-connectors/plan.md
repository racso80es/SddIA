---
feature_name: antigravity-connectors
created: "2026-09-03"
process: feature
branch_name: feat/antigravity-connectors-8989250975201761652
persist_ref: docs/features/antigravity-connectors
status: executed
---

# Blueprint — refactor antigravity-connectors

No parchear el esqueleto Jules in-place como si fuera genoma válido (DA-2: `{name}.md` y cobertura EDA ya están ensuciados). Re-forja + reescritura de crates + higiene EDA.

```text
R0  Laudos L1–L5 tácitos (defaults PBI) — STOP solo si Racso veta L1
R1  entity-manager create tool gemini-http-infer
R2  entity-manager update/create skill antigravity-cli-executor (sello real)
R3  entity-manager delete/retire skill antigravity-http-connector
R4  Reescribir crates + tests lab (mock outbound / stub agy)
R5  Extender outbound_lab (URL Gemini) — crate sddia-io (no ED indexada)
R6  Índice skills/tools + WASI exclude coherente + evolution
R7  Higiene eda-coverage (UUIDs reales, hash post-forja, podar ae2927f5)
R8  validacion.md NO_APTO→APTO solo con CA de código + lab; CI Gemini/agy real = PENDIENTE o fuera de gate
```

## Fase R0 — Congelar defaults

Sin nuevo laudo: L1 tool HTTP, L2 `system-operations`, L3 sin DI, L4 sin Vertex, L-PERSIST este directorio. Si Racso exige dos skills, abortar R1 y documentar en este clarify.

## Fase R1 — Tool HTTP

`./sddia-run.sh --process entity-manager` `entity_class: tool` `gemini-http-infer`. Inputs: context `system-operations`, schemas I/O alineados a spec §1.1 / sobre 2.0.

## Fase R2 — Skill CLI

Misma cadena, `entity_class: skill`, nombre `antigravity-cli-executor`. No editar el `.md` Jules a mano. El creator debe emitir `hash_signature` ≠ ceros.

## Fase R3 — Retirar skill HTTP

Cadena entity-manager para borrar/deprecar `antigravity-http-connector` (md + crate + exclusión WASI de ese nombre). Un solo HTTP vivo: el tool.

## Fase R4 — Física de crates

- HTTP: cliente nativo; mock lab; timeout; nunca `curl`.
- CLI: argv print-mode; whitelist; sandbox default; parseo sobre `agy` (`status`/`response`/`usage`).
- Tests unitarios sin red. Fixture stub `agy` en `persist_ref/_smoke-agy-print.json` si hace falta plantilla.

## Fase R5 — `outbound_lab`

Añadir `lab_mock_gemini_url()` simétrico a Telegram. Flag existente `lab_mock_outbound_enabled()`. Prohibido `find_repo_root_from_cwd` en las cápsulas nuevas.

## Fase R6 — Índices y evolution

Filas índice coherentes con YAML. Registro `SddIA/evolution/` con UUID de las ED. `build-wasi-capsules.sh`: exclude solo crates nativos que queden (CLI + HTTP tool).

## Fase R7 — EDA

`--scan` huérfanos = 0 para las ED de este ciclo. Eliminar o reemplazar `ae2927f5` / `deadbeef`. UUIDs de cobertura = frontmatter post-forja.

## Fase R8 — Validación

`validacion.md` en esta rama. `pbi_archived: true` solo cuando el PBI esté en `docs/todos/done/` **en el mismo PR**. CA de red Google real no bloquean `global: APTO` si el mock lab cubre el contrato; documentar como fuera de gate.

## STOP

Prohibido DCC / PR de cierre con el código Jules actual. Prohibido bisturí IDE sobre `SddIA/skills/*.md` o `eda-coverage.json`.
