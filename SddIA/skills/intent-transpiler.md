---
uuid: "4f0edfe0-4380-442b-962d-9e98f8ecf956"
name: "intent-transpiler"
version: "2.0.0"
contract: "skills-contract v1.3.0"
context: "knowledge-management"
capabilities:
  - "intent-structuring"
  - "ssot-path-resolution"
  - "feature-topology-gate"
hash_signature: "sha256:45213b2a116ac38aa505d5273844d7ba5ca4cb2d4d227c91809493a5d3a1ba6e"
inputs:
  - "raw_instruction": "string; texto libre del Vértice Biológico (UTF-8)"
  - "intent_hints": "array<string>; pistas opcionales: develop, fix, refactor, feature, bug-fix"
outputs:
  - "success": "boolean"
  - "exitCode": "integer"
  - "data.structured_directive": "string; instrucción condensada para Tekton/Jules"
  - "data.event_uris": "array<string>; rutas relativas resueltas vía cumulo.paths.json"
  - "data.required_process": "string|null; feature | bug-fix | refactorization"
  - "data.persist_ref": "string|null; docs/features|fixes/{name}"
  - "error": "string; diagnóstico si success es false"
---

# Skill: intent-transpiler

## Propósito

Skill intermediaria de **prevención cognitiva** (PBI Snapshot Fricción Jules): procesa la instrucción del Vértice Biológico antes del núcleo de ejecución de la IA obrera. Mantiene ceguera espacial sobre negocio del cliente; estructura, enruta y condiciona según SSOT SddIA.

## Operación

1. Recibir `raw_instruction` y opcionalmente `intent_hints`.
2. Resolver `required_process` cruzando `SddIA/norms/interaction-triggers.json`.
3. Inferir `persist_ref` kebab-case bajo `docs/features/` o `docs/fixes/` según proceso.
4. Mapear `target_paths` exclusivamente desde `SddIA/core/cumulo.paths.json` — prohibido inventar rutas.
5. Emitir `structured_directive` sin verbosidad; incluir aviso DA-4 si no existe `objectives.md` en `persist_ref`.
6. Devolver envelope JSON (`success`, `exitCode`, `data` | `error`).

## Límites

- No muta genoma ni disco; solo produce directiva estructurada.
- No sustituye el orquestador nativo `execute-process` ni `entity-manager`.
- Si `persist_ref` no existe, `required_process: feature` y `structured_directive` debe ordenar inicialización de feature antes de mutaciones.

## Referencias

- `SddIA/norms/external-ai-constraints.md` § DA-4
- Feature origen: `docs/features/snapshot-friccion-laboratorio-jules/`