---
feature_name: antigravity-connectors
created: "2026-09-02"
updated: "2026-09-03"
process: feature
branch_name: feat/antigravity-connectors-8989250975201761652
persist_ref: docs/features/antigravity-connectors
execution_id: "7d6ca13f-95e4-4b47-8457-6f37d54f3c3d"
pbi_ref: "docs/todos/done/[OPERATIVO] Forja de cápsulas nativas para integración dual con Google Antigravity (HTTP y CLI).md"
document_id: PBI-CAPSULES-ANTIGRAVITY-NATIVE
pbi_uuid: "7f966f32-5502-4bd7-b252-44849f29f5d3"
pbi_version: "1.2.0"
status: executed
---

# Objetivos — antigravity-connectors

## Misión

Materializar los dos actuadores del PBI `PBI-CAPSULES-ANTIGRAVITY-NATIVE` v1.2.0 (refinado): tool HTTP Gemini `generateContent` y skill transductora de `agy` en print/headless. Martillos ciegos; sin `provides llm:interact`.

## Estado 2026-09-03

Refactor ejecutado: tool `gemini-http-infer` + skill `antigravity-cli-executor` (print-mode). Skill HTTP Jules retirada. Tests lab verdes. Ver `execution.md` / `validacion.md`.

## Alcance (manifiesto)

- Ciclo `feature` ya inicializado (`execution_id` `7d6ca13f-…`).
- Genoma solo vía `entity-manager` → `tool-creator` / `skill-creator` (DA-2/DA-3).
- Topología documental: `features-documentation-pattern` v1.2.1.

## Ley aplicada

- Git vía `skill:git-manager`. Troncal = `main` (no existe `master`).
- Ceguera espacial: no leer `env_hierarchy`; env inyectado por orquestador.
- Códice: no inventar `capability_id`; no rebind de `llm:interact`.
