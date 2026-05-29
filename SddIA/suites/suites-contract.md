---
uuid: "c1d2e3f4-a5b6-4789-c012-3456789abcde"
name: "suites-contract"
version: "1.0.0"
contract: "knowledge-contract v1.0.0"
nature: "family-contract"
scope: "core"
category: "domain-entity"
---

# Contrato — Entidad de Dominio Suite

## 1. Propósito

Define la estructura canónica de las **Suites** S+ Grade: activos declarativos que orquestan secuencias de procesos audit con estrategia de concurrencia y tolerancias a fallos.

## 2. Frontmatter obligatorio

| Campo | Tipo | Reglas |
|-------|------|--------|
| `uuid` | string (UUID v4) | Obligatorio |
| `name` | string (kebab-case) | Coincide con nombre de archivo sin extensión |
| `version` | SemVer | Obligatorio |
| `contract` | string | `suites-contract v1.0.0` |
| `context` | array | Al menos un contexto RBAC válido |
| `hash_signature` | string | `sha256:` sobre canon §4 |
| `execution_strategy` | enum | `fail_fast` \| `run_all` |
| `atomic_nodes` | array | No vacío; ver §3 |

## 3. `atomic_nodes[]`

Cada elemento:

| Campo | Tipo | Obligatorio |
|-------|------|:-----------:|
| `process_name` | string | Sí — debe existir en `SddIA/process/index.md` |
| `expected_exit_code` | integer | Sí — típicamente `0` |
| `timeout_ms` | integer | No — default orquestador `300000` |

**Prohibido** referenciar tools directamente — solo procesos (Atomicidad Diagnóstica).

## 4. Canon de integridad (`hash_signature`)

```text
SHA-256(JSON ordenado de: { atomic_nodes, execution_strategy, version })
```

## 5. Índice

`SddIA/suites/index.md` — columnas: Archivo | uuid | name | version | execution_strategy | node_count
