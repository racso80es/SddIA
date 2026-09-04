---
feature_name: arquitectura-llm-tiers
created: "2026-09-04"
process: feature
base: main
scope: arquitectura-llm-tiers
version_spec: "1.0.0"
pbi_uuid: "8a3b5c7d-4e2f-41a9-9b6e-7c1d3e5f8a2b"
pbi_version: "1.3.0"
execution_id: "ff0407a0-0458-4461-acc3-1beeb94e1aa0"
---

# Especificación — arquitectura-llm-tiers

SSOT: PBI v1.3.0.

## 1. Contrato y genoma

`agents-contract.md` v1.1.0: `llm_profile` opcional `{ tier: high|medium|low|none, description?: string }`.

Siete `{name}.md` declaran el bloque. Argos documenta L-ARGOS-SYNTHESIS en cuerpo.

## 2. `invoke_agent_phase`

Tras `agent_names`, leer `{repo}/SddIA/agents/{name}.md` (`parse_frontmatter`). Fail-soft si falta el fichero.

Insertar `llm_profiles` en el JSON `AGENT_PHASE` existente (`repo_root`, `persist_ref`, `execution_id`, … intactos).

Si **todos** los agentes de la fase son `tier: none` (o `cerbero`/`cumulo`/`radamanto` sin YAML aún): `status: executed`, `note: deterministic-agent-no-llm`, **sin spawn**.

## 3. Harness

`resolve_phase_model(doc)`: mayor tier cognitivo de `llm_profiles` × `agents` → `SDDIA_LLM_TIER_{HIGH|MEDIUM|LOW}` no vacío → else `SDDIA_AGENT_RUNTIME_MODEL` (default harness vigente). Backend SDK usa ese id. Backend CLI: no añadir `--model` (degradación documentada).

## 4. Bóveda

Starter-kit: tres claves comentadas, valor vacío. Fallback documentado: `SDDIA_AGENT_RUNTIME_MODEL`. Cero marcas como valor-ley.
