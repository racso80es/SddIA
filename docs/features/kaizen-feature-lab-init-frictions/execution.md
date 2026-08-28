---
feature_name: kaizen-feature-lab-init-frictions
created: "2026-08-28"
process: feature
execution_id: "80a3ca0d-80c5-4662-ab12-2afe757478c8"
items_applied:
  - T1
  - T2
  - T3
  - T4
  - T5
  - T6
  - T7
pr_url: "https://github.com/racso80es/SddIA/pull/209"
delivery_state: ready_for_merge
---

# Execution — kaizen-feature-lab-init-frictions

## Registro

| Fase plan | Estado | Evidencia |
|-----------|--------|-----------|
| T1 | aplicado | `cargo test -p execute-process agent_runtime::` — 12/12 |
| T2 | aplicado | LAB-CA1 / LAB-CA3 verdes en smoke |
| T3 | aplicado | LAB-CA6 (workspace resoluble) + test `persist-execution-id-conflict` |
| T4 | aplicado | LAB-CA8 / LAB-CA9 verdes; `entity-manager` v1.0.2 forjado |
| T5 | aplicado | LAB-CA10 (gate dirty) / LAB-CA11 verdes |
| T6 | aplicado | suite 268/268; smokes 9/9 |
| T7 | cerrado | PR [#209](https://github.com/racso80es/SddIA/pull/209); evolution `f66bad66`; CI re-ejecutándose tras fix gate |

## Verificación local

```bash
cd SddIA && CARGO_TARGET_DIR="$PWD/target" cargo build --workspace -q
cd SddIA && CARGO_TARGET_DIR="$PWD/target" cargo test -p execute-process --lib
docs/features/kaizen-feature-lab-init-frictions/.tmp/smoke-lab-init.sh
```

Resultado: `268 passed; 0 failed; 1 ignored` · `== resultado: fail=0 ==`

## Tabla de smokes

| Criterio | Estado | Cobertura |
|----------|--------|-----------|
| LAB-CA1 / CA2 | PASS | relevo IDE fuerza `simulated`; 134 ms con `SDDIA_AGENT_RUNTIME_COMMAND="sleep 999"` |
| LAB-CA3 | PASS | `_sddia_load_vault` no pisa variables de entorno preexistentes |
| LAB-CA4 | PASS | `timeout_kills_hanging_command` |
| LAB-CA5 | PASS | recuento de huérfanos del grupo antes/después del timeout |
| LAB-CA6 | PASS | `execution_id` del acuse resuelve `.SddIA/workspaces/feature/{id}` |
| LAB-CA7 | PASS | test `persist-execution-id-conflict` (suite) |
| LAB-CA8 | PASS | `daemon` en `PILOT_CLASSES` |
| LAB-CA9 | PASS | censo del pie de `daemons/index.md` = número de filas |
| LAB-CA10 | PASS | init sobre árbol sucio aborta con `dirty-worktree` |
| LAB-CA11 | PASS | snapshot no captura `??` ajenos bajo `docs/todos/` |

## Fricciones del laboratorio (registro para evolución)

Coste real del ciclo, en orden de impacto. Los tres primeros son defectos del propio
laboratorio, no del cambio implementado.

| Fricción | Efecto | Mitigación aplicada |
|----------|--------|---------------------|
| `cargo build` sin `CARGO_TARGET_DIR` escribía en un target distinto al que resuelve `sddia-run.sh` | Los smokes corrían contra un binario del día anterior, sin el flag de relevo; colgaban y dejaban procesos huérfanos | `CARGO_TARGET_DIR` explícito en el script de smoke |
| `cd` sin subshell en el script de smoke | El CWD quedaba fuera de la raíz y `./sddia-run.sh` no resolvía | Subshells en todos los bloques que cambian de directorio |
| Fixture obsoleto en `verify_process_integrity` | Un fallo permanente enmascaraba el estado real de la suite | Fixture completado con `cumulo.paths.json` |
| `pgrep` con `set -euo pipefail` | Ausencia de coincidencias abortaba el script | `{ pgrep … \|\| true; } \| wc -l` |
| `ac_smoke_domain_no_core_executable` exige la cápsula `cryptography-manager` compilada bajo `SddIA/target` | Falso rojo intermitente si solo se construye `-p execute-process` | Ejecutar `cargo build --workspace` antes de la suite |
| Fricción evolution gate CI | `gate-evolution --range` falló por 18 paths sin registro; corregido en `f151ebf` | Entrada `f66bad66-2861-4603-b790-843859dd46a2` vía `sddia-qa evolution-register` |

## Higiene de rama

Revertidos por ajenos al PBI: `docs/features/plumb-cid/`, `docs/fixes/x/`, y las entradas
`sha256:deadbeef` que los smokes inyectaron en `eda-coverage.json`. El único cambio conservado
en ese fichero es el sello de `entity-manager` (`62f08bbd`), coherente con el bump a v1.0.2.
