---
document_id: PBI-KAIZEN-CI-WORKFLOW-OPTIMIZATION
title: "[ARQUITECTURA] Optimización termodinámica de flujos CI/CD (push vs pull_request)"
format: markdown
version: "1.1.0"
created: "2026-08-29"
status: done
refinement_status: refined
priority: media
process: refactorization
executor_vehicle: feature
type: kaizen
dispatch: false
related:
  - .github/workflows/sddia-index-qa.yml
---

# [ARQUITECTURA] Optimización termodinámica de flujos CI/CD (push vs pull_request)

## Mandato
Erradicar la duplicación de validaciones en GitHub Actions. Hoy, el flujo monolítico `sddia-index-qa.yml` se dispara con los mismos globs de rama (`main`, `feat/**`, `fix/**`) tanto en `push` como en `pull_request`. Un commit empujado a una rama con PR abierto ejecuta **dos veces** la batería completa (incluidas las pruebas pesadas E2E y de anclaje físico), generando sobrecarga térmica, consumo de minutos CI y riesgo de saturación en la red IOTA.

Objetivo: bifurcar la línea de montaje. `push` a ramas de trabajo provee *Feedback Inmediato* (fast-fail); `pull_request` actúa como *Aduana de Integración* (deep audit) previa al merge; `push` a `main` conserva un *Guardián* post-merge.

## 1. Superficie de Impacto
- Archivo único de flujos: `.github/workflows/sddia-index-qa.yml`
- No se altera la lógica interna de las cápsulas ni de los comandos `sddia-qa`.

## 2. Estado real del genoma (verificado)

Jobs actualmente definidos y su peso térmico:

| Job (real) | Contenido clave | Peso |
| :--- | :--- | :--- |
| `sddia-index-integrity` | steps `verify-tools-index`, `verify-process-integrity`, tests `evolution-register`, `cargo build --workspace` + `verify-compiled-capsules` | Medio-alto |
| `eda-iota-smoke-simulate` | build centinelas/orquestador/QA + `run-iota-ci-smoke --simulate` | Ligero-medio |
| `wasi-runtime-smoke` | build cápsulas WASI + nativos + `run-wasi-ci-smoke` + `gate-evolution` (delta y universo) | Alto |
| `eda-bus-e2e-smoke` | build WASI + nativos + `run-eda-e2e-lab` + `event-sweeper --once` | Alto (E2E) |
| `eda-iota-physical` | build nativos + `iota-immutable-publisher` + `run-iota-ci-smoke --require-physical` | Alto (externo) |

Correcciones respecto a la v1.0.0 de este PBI:
- `verify-tools-index` es un **step**, no un job; el job contenedor real es `sddia-index-integrity` (antes ausente en la tabla).
- `eda-iota-physical` **ya posee** un `if:` fork-guard (`head.repo.full_name == github.repository`) y hace *exit 0* si `IOTA_WALLET_SECRET` no está configurado; la duplicación real que ataca este PBI es la disparada por el par `push`/`pull_request` sobre la misma rama, no una ausencia total de gobierno.

## 3. Estrategia de Refactorización (Línea de Montaje)

Enfoque preferente: **condicionales a nivel de job** dentro del archivo único (mantiene DRY y una sola SSOT de flujo), en lugar de fragmentar en varios `.yml`.

### Ola A1: Segregación por evento

| Fase | Disparador | Jobs ejecutados |
| :--- | :--- | :--- |
| **Feedback Inmediato** (fast-fail) | `push` → `feat/**`, `fix/**` | `sddia-index-integrity`, `eda-iota-smoke-simulate`, `wasi-runtime-smoke` |
| **Aduana de Integración** (deep audit) | `pull_request` → `main` | los tres anteriores **+ `eda-bus-e2e-smoke` + `eda-iota-physical`** |
| **Guardián** (post-merge) | `push` → `main` | conjunto completo |

Materialización con `if:` en los jobs pesados (E2E y físico), preservando el fork-guard existente de `eda-iota-physical`:

```yaml
# eda-bus-e2e-smoke
if: github.event_name == 'pull_request' || github.ref == 'refs/heads/main'

# eda-iota-physical (fusiona la nueva condición con el fork-guard preexistente)
if: >-
  (github.event_name == 'pull_request' || github.ref == 'refs/heads/main') &&
  (github.event_name != 'pull_request' ||
   github.event.pull_request.head.repo.full_name == github.repository)
```

Efecto: los `push` a `feat/**`/`fix/**` dejan de ejecutar E2E y anclaje físico (fuente de la duplicación), mientras que la Aduana de PR y el Guardián de `main` los conservan íntegros.

### Ola A2: Control de Concurrencia (Anti-Entropía)

Concurrency es **a nivel de workflow**; con un solo archivo que atiende `push` y `pull_request` hay que evitar (a) cancelar auditorías de PR en vuelo y (b) colisiones entre eventos. Se segrega el grupo por evento y se restringe la cancelación a `push`:

```yaml
concurrency:
  group: ${{ github.workflow }}-${{ github.event_name }}-${{ github.ref }}
  cancel-in-progress: ${{ github.event_name == 'push' }}
```

Así, un nuevo commit en la misma rama purga el `push` obsoleto sin abortar la Aduana del PR asociado.

## 4. Criterios de Aceptación (Protocolo de Acero)

- [ ] **CA1 (Sin Duplicación):** un commit empujado a una rama `feat/**`/`fix/**` con PR abierto NO dispara `eda-bus-e2e-smoke` ni `eda-iota-physical` bajo el evento `push`.
- [ ] **CA2 (Aduana Severa):** `eda-bus-e2e-smoke` y `eda-iota-physical` se ejecutan invariablemente al abrir o actualizar un `pull_request` contra `main` (respetando el fork-guard: el físico solo ancla en PRs del propio repositorio y con `IOTA_WALLET_SECRET` presente).
- [ ] **CA3 (Guardián de main):** un `push` a `main` (merge) ejecuta el conjunto completo, evitando regresiones post-merge.
- [ ] **CA4 (Cancelación Activa):** dos `push` consecutivos (<1 min) a la misma rama de trabajo cancelan la ejecución del primer commit, sin afectar ejecuciones de `pull_request` en curso.
- [ ] **CA5 (Trazabilidad):** el cambio se limita a `.github/workflows/sddia-index-qa.yml`, sin alterar la lógica de los comandos `sddia-qa` ni de las cápsulas, y queda registrado en `SddIA/evolution/`.
