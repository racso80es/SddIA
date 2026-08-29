---
feature_name: kaizen-ci-workflow-optimization
created: "2026-08-29"
process: refactorization
phase: design
agents: dedalo
base: main
scope: sddia-index-qa-push-pr-segregation
branch_name: feat/kaizen-ci-workflow-optimization
persist_ref: docs/features/kaizen-ci-workflow-optimization
pbi_ref: docs/todos/pending/[ARQUITECTURA] Optimización termodinámica de flujos .md
document_id: PBI-KAIZEN-CI-WORKFLOW-OPTIMIZATION
uuid: d664b94d-3ce8-4b66-a4a7-0ff10570acf9
version_spec: "1.0.0"
status: dedalo_locked
---

# Spec — kaizen-ci-workflow-optimization

## 1. Misión técnica

Condicionar jobs pesados de `.github/workflows/sddia-index-qa.yml` y añadir `concurrency` anti-entropía. Los jobs ligeros/medios (`sddia-index-integrity`, `eda-iota-smoke-simulate`, `wasi-runtime-smoke`) permanecen en todos los disparadores actuales.

## 2. Diagnóstico (genoma verificado)

| Hecho | Evidencia |
|-------|-----------|
| Un único workflow | `.github/workflows/sddia-index-qa.yml` |
| Sin `concurrency` | Ausente en el archivo |
| Duplicación | Mismos globs en `on.push` y `on.pull_request` |
| Físico ya gobernado | `if:` fork-guard L159–161; *exit 0* si `IOTA_WALLET_SECRET` vacío |
| Jobs | 5; `verify-tools-index` es step de `sddia-index-integrity` |

## 3. Laudos Dedalo

| Ref | Decisión |
|-----|----------|
| **L-SINGLE-FILE** | Un YAML. No split. |
| **L-PR-IF** | No recortar `on:`. Segregación por `if:` de job. |
| **L-HEAVY-IF** | Pesados = `eda-bus-e2e-smoke`, `eda-iota-physical`. |
| **L-FORK-COMPOSE** | El `if:` del físico es conjunción: (PR ∨ `refs/heads/main`) ∧ fork-guard legado. |
| **L-CONC-EVENT** | Grupo workflow+event_name+ref; cancel solo `push`. |
| **L-NO-QA** | Cero cambios en crates `sddia-qa` / cápsulas. |

## 4. Contrato YAML

### 4.1 Concurrency (raíz, tras `on:` / antes o junto a `permissions:`)

```yaml
concurrency:
  group: ${{ github.workflow }}-${{ github.event_name }}-${{ github.ref }}
  cancel-in-progress: ${{ github.event_name == 'push' }}
```

### 4.2 `eda-bus-e2e-smoke`

```yaml
if: github.event_name == 'pull_request' || github.ref == 'refs/heads/main'
```

En `push` a `feat/**`/`fix/**`, `github.ref` es `refs/heads/feat/...` → job omitido.

### 4.3 `eda-iota-physical`

Sustituir el `if:` actual por:

```yaml
if: >-
  (github.event_name == 'pull_request' || github.ref == 'refs/heads/main') &&
  (github.event_name != 'pull_request' ||
   github.event.pull_request.head.repo.full_name == github.repository)
```

El step de secreto (`exit 0` si vacío) no se toca.

## 5. Matriz de ejecución

| Evento | Fast-fail (3 jobs) | E2E | Físico |
|--------|--------------------|-----|--------|
| `push` `feat/**`/`fix/**` | sí | no | no |
| `pull_request` (cualquier base listada) | sí | sí | sí si no-fork |
| `push` `main` | sí | sí | sí (no es PR; fork-guard no aplica) |

## 6. Fuera de spec

Cambiar globs `on:`; segundo workflow; timeouts; runners; cache keys; lógica `sddia-qa`.
