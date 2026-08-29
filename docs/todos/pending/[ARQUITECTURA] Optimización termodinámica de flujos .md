---
document_id: PBI-KAIZEN-CI-WORKFLOW-OPTIMIZATION
title: "[ARQUITECTURA] Optimización termodinámica de flujos CI/CD (push vs pull_request)"
format: markdown
version: "1.0.0"
created: "2026-08-29"
status: pending
refinement_status: proposed
priority: media
process: refactorization
executor_vehicle: feature
type: kaizen
dispatch: false
related:
  - .github/workflows/
---

# [ARQUITECTURA] Optimización termodinámica de flujos CI/CD (push vs pull_request)

## Mandato
Erradicar la duplicación de validaciones en GitHub Actions. Actualmente, el ecosistema ejecuta baterías idénticas de pruebas pesadas (integración física y End-to-End) simultáneamente para los eventos `push` y `pull_request`, generando sobrecarga térmica, consumo innecesario de minutos CI y riesgo de bloqueos por saturación en red (ej. IOTA). 

El objetivo es bifurcar la línea de montaje: `push` proveerá *Feedback Inmediato* (fast-fail) al desarrollador, mientras que `pull_request` actuará como la *Aduana de Integración* (deep audit) antes del merge.

## 1. Superficie de Impacto
- Directorio de flujos: `.github/workflows/`

## 2. Estrategia de Refactorización (Línea de Montaje)

### Ola A1: Segregación de Responsabilidades
Dividir el archivo monolítico actual (ej. `sddia-index-qa.yml`) en dos flujos especializados, o implementar condicionales estrictos a nivel de job:

| Enfoque de Validación | Disparador | Jobs Asignados |
| :--- | :--- | :--- |
| **Feedback Inmediato** (Fast-Fail local) | `on: push` | `verify-tools-index`, `wasi-runtime-smoke`, `eda-iota-smoke-simulate` |
| **Aduana de Integración** (Auditoría E2E) | `on: pull_request` | `verify-tools-index`, `wasi-runtime-smoke`, `eda-iota-smoke-simulate`, **`eda-bus-e2e-smoke`**, **`eda-iota-physical`** |

### Ola A2: Control de Concurrencia (Anti-Entropía)
Inyectar la directiva nativa `concurrency` en los flujos desencadenados por `push`. Esto abortará automáticamente cualquier ejecución previa en vuelo si se detecta un nuevo commit en la misma rama, purgando el gasto de minutos en validaciones de código obsoleto.

```yaml
concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true
3. Criterios de Aceptación (Protocolo de Acero)
[ ] CA1 (Sin Duplicación): Un commit subido a una rama con un PR abierto no dispara ejecuciones redundantes de eda-bus-e2e-smoke ni eda-iota-physical asociadas al evento push.

[ ] CA2 (Aduana Severa): Las pruebas de integración física y E2E se ejecutan invariablemente cuando se abre o actualiza un pull_request contra la rama principal.

[ ] CA3 (Cancelación Activa): Realizar dos push consecutivos en menos de un minuto a la misma rama cancela automáticamente la ejecución de GitHub Actions del primer commit.

[ ] CA4 (Trazabilidad): El genoma del repositorio refleja esta optimización sin alterar la lógica interna de los tests individuales.
