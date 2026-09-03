---
feature_name: antigravity-connectors
created: "2026-09-02"
updated: "2026-09-03"
process: feature
branch_name: feat/antigravity-connectors-8989250975201761652
persist_ref: docs/features/antigravity-connectors
execution_id: "7d6ca13f-95e4-4b47-8457-6f37d54f3c3d"
pbi_ref: "docs/todos/pending/[OPERATIVO] Forja de cápsulas nativas para integración dual con Google Antigravity (HTTP y CLI).md"
document_id: PBI-CAPSULES-ANTIGRAVITY-NATIVE
pbi_uuid: "7f966f32-5502-4bd7-b252-44849f29f5d3"
pbi_version: "1.2.0"
status: audit-no-apto
---

# Objetivos — antigravity-connectors

## Misión

Materializar los dos actuadores del PBI `PBI-CAPSULES-ANTIGRAVITY-NATIVE` v1.2.0 (refinado): tool HTTP Gemini `generateContent` y skill transductora de `agy` en print/headless. Martillos ciegos; sin `provides llm:interact`.

## Estado 2026-09-03

La rama contiene un **esqueleto Jules** (`skill:antigravity-http-connector` + `skill:antigravity-cli-executor`) **NO_APTO** frente al PBI v1.2.0. Ver `clarify.md` (auditoría) y `plan.md` (refactor). Prohibido cerrar entrega sobre el código actual.

El PBI permanece en `docs/todos/pending/` (no está en `done/`). `persist_ref` vigente = este directorio; ignorar `persist_ref_suggested` del PBI (`docs/features/capsules-antigravity-native`).

## Alcance (manifiesto)

- Ciclo `feature` ya inicializado (`execution_id` `7d6ca13f-…`).
- Genoma solo vía `entity-manager` → `tool-creator` / `skill-creator` (DA-2/DA-3).
- Topología documental: `features-documentation-pattern` v1.2.1.

## Ley aplicada

- Git vía `skill:git-manager`. Troncal = `main` (no existe `master`).
- Ceguera espacial: no leer `env_hierarchy`; env inyectado por orquestador.
- Códice: no inventar `capability_id`; no rebind de `llm:interact`.
