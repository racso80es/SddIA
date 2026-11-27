---
feature_name: aut-132-gate-anti-anemia-reglas-contractuales
created: "2026-11-27"
process: refactorization
branch_name: refactor/aut-132-gate-anti-anemia-reglas-contractuales
persist_ref: docs/refactors/aut-132-gate-anti-anemia-reglas-contractuales
tracker_ref: AUT-132
historia_ref: AUT-125
execution_id: "47facb2e-84b4-48b8-ba38-0429be4605fd"
---
# Objetivos — AUT-132

Endurecer el peaje `gate-entity-no-anemia` / `entity_no_anemia` para que las reglas contractuales por familia sean efectivas, el escaneo de delta sea correcto y el peaje DCC/PPR/CLI sea in-process y JSON-determinista (hijo de AUT-125; sin tocar forjas ni remediación masiva de genoma).

| ID | Objetivo |
| --- | --- |
| O-1 | Keywords contractuales efectivas: `purpose_kw` + `second_kw` en headings `##` distintos, sin atajos por conteo/longitud |
| O-2 | `scan_changed`: rango `base...HEAD`, filtro de borrados, clasificación por rank y límite de directorio `/` |
| O-3 | Frontmatter estricto (`---` inicial + cierre) y `FRONTMATTER_YAML_INVALID` explícito |
| O-4 | CLI `gate-entity-no-anemia`: `--base` estricto y errores en modo `--json` |
| O-5 | Peaje PPR/`phase_capsules` in-process con `base_ref` configurable |
| O-6 | Limpieza de scripts ad-hoc de remediación y traza evolution con AUT-132 |
| O-7 | Suite de tests `entity_no_anemia` / `gate_entity_no_anemia` verde (regresión del REJECT) |

## Alcance (manifiesto)

Estabilización Mayeuta → diseño Dedalo → ejecución Tekton → verificación Argos. Git solo vía `skill:git-manager`. Fuera: AUT-131 (forjas), AUT-133 (línea base → 0).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
- Documentación: `features-documentation-pattern` (un `.md` por fase, frontmatter YAML).
- KM / `docs/todos/`: solo `agent:cumulo` o `Kaizen_Alert_Required` (Mayeuta no siembra TODOs).
