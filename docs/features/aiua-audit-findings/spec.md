---
feature_name: aiua-audit-findings
created: "2026-09-09"
process: feature
base: main
scope: core
branch_name: feat/aiua-audit-findings
persist_ref: docs/features/aiua-audit-findings
execution_id: "1532d612-85d6-4b8e-9b9c-6c4979c3cce4"
document_id: PBI-AIUA-AUDIT-FINDINGS-20260908
---

# Spec — aiua-audit-findings

## 1. Tool `gemini-http-infer` (H-LLM-1b)

Crate `SddIA/tools/gemini-http-infer/src/main.rs`.

| Cambio | Contrato |
|--------|----------|
| Resolución | `request.thinking_level` no vacío ≻ `request.thinkingLevel` ≻ `SDDIA_GEMINI_THINKING_LEVEL` ≻ `None`. |
| Normalización | trim; case-insensitive; salida `HIGH`\|`MEDIUM`\|`LOW`. Otro valor → `Err` (`thinking-level-invalid:`). |
| Payload | Si `Some`, `generationConfig.thinkingConfig.thinkingLevel`. Si `temperature` también, mismo objeto `generationConfig`. |
| Lab | `SDDIA_LAB_MOCK_OUTBOUND` sin URL mock → `lab-mock:` **sin** exigir thinking ni red. |
| Slug | Cero model id en código. L-MODEL intacto. |

Starter-kit `.env.example`: clave `SDDIA_GEMINI_THINKING_LEVEL` comentada; valores `high`\|`medium`\|`low`; **sin** slug de modelo.

`{name}.md` de la tool: si se documenta thinking, EM `hash_refresh_only` tras mutación de cuerpo **solo** si existe vía legal; si `run_tool_forge` no soporta replacements, **no** tocar UUID. Preferencia: crate + env example.

## 2. Acciones (H-LLM-2)

`entity-manager` `lifecycle_operation: update` + `action_body` / `action_inputs` / `action_outputs` / `action_version: 1.1.0` / `actions_contract_version: 1.3.0`. UUID:

| Acción | UUID |
|--------|------|
| `retrieve-active-context` | `afa0424b-cdd4-4810-9a29-d8e7c06d6a1f` |
| `invoke-aiua-core` | `2edc7ef4-57e5-4d10-8753-bab8b0053cca` |
| `persist-thought-record` | `a37f9f1d-8f2a-441f-9357-776d65553362` |

Cuerpos alineados a `handlers::aiua_stimulus`:

- **retrieve-active-context:** inputs `query_text` (req), `limit` (opt=5). Delega `tool:thought-graph-access` `operation=search`. Output `memories`. No emite ECST.
- **invoke-aiua-core:** inputs `prompt` (req), `active_context` (opt), `model` (opt). Lee `aiua_core.md` vía `directories.conscience`. Prefacio identidad + genoma + contexto + `## Estímulo`. Sin HTTP. Output `assembled_prompt`, `model`.
- **persist-thought-record:** inputs `prompt`, `response_text` (req), `metadata` (opt). Delega `operation=store`. No emite `Thought_Persisted`. Output `thought_id`, `persisted`.

## 3. Prefacio (H-LLM-3)

En `invoke_aiua_core`, **antes** del genoma:

```text
Eres {name}, la {entity_type} del ecosistema SddIA. Hablas en primera persona. El genoma que sigue es tu identidad; no eres un asistente genérico.

---
{genome}
```

`name`/`entity_type` del YAML de `aiua_core.md`. Sin `name`: fallback «Eres la Aiúa…». Cero HTTP. Cero lectura de `CONSTITUTION_CORE.md`.

## 4. H-LLM-4

Laudo L-CONST: no inyectar Constitución. Ver `clarify.md`.

## 5. Tests

- Unit crate: thinking vacío → `None`; env/request → `HIGH`; inválido → err; `generationConfig` merge temperature+thinking; mock no exige thinking.
- Handler: `assembled_prompt` contiene prefacio (1ª persona / «Aiúa» o `name` del fixture); lab-mock ciclo intacto.
- Grep Kalma2 UI intacto.

## 6. Fuera

PR #270. Kalma2. IOTA. Agente Tormentosa. Antigravity CLI. Retry 503.
