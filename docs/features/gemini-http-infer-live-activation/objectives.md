---
feature_name: gemini-http-infer-live-activation
created: "2026-09-04"
process: feature
branch_name: feat/gemini-http-infer-live-activation
persist_ref: docs/features/gemini-http-infer-live-activation
pbi_ref: docs/todos/pending/[OPERATIVO] Activación live gemini-http-infer — catálogo de modelos y humo de instancia.md
execution_id: "0926e45d-db83-42ea-8a5b-3bafcdb00b57"
document_id: PBI-GEMINI-HTTP-INFER-LIVE-ACTIVATION
pbi_uuid: "d7f1f8aa-0d51-4b2d-ac27-ed17c8a23d09"
pbi_version: "1.2.0"
status: in-progress
---

# Objetivos — gemini-http-infer-live-activation

## Misión

Residual de `PBI-CAPSULES-ANTIGRAVITY-NATIVE`: activación live + defectos de integración (ureq 4xx, L-MODEL, candidato vacío, argv `agy`, matcher auth). PBI v1.2.0.

## Alcance (manifiesto)

- Ciclo `feature` inicializado (`execution_id` `0926e45d-…`).
- Crates: delivery de EDs ya forjadas. No `entity-manager update` completo (regenera UUID).
- Starter-kit: comentarios Gemini, cero secretos.
- Humo HTTP vía `--tool`; skill SUCCESS live no gate.

## Ley aplicada

- Git vía `skill:git-manager`. Troncal `main`.
- DA-2/DA-4: topología `objectives.md` en rama. `{name}.md` solo con `hash_refresh_only` si se toca.
- Cero `provides` / rebind `llm:interact`.
