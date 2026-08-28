---
document_id: PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL
uuid: "6d64bcc7-b677-4c43-b239-928e279d2a04"
title: "[KAIZEN] Aduana evolution local inexistente — hooks sin instalar, --if-touched invertido y fase de impacto stub"
format: markdown
version: "1.0.0"
created: "2026-08-28"
status: "pendiente"
priority: "alta"
process: bug-fix
type: kaizen
dispatch: false
suggested_branch: fix/kaizen-aduana-evolution-local
incident_ref: "PR #209 — wasi-runtime-smoke rojo en el paso evolution gate (delta) con 18 findings EVOL_MATERIAL_UNREGISTERED; ninguna de las cuatro capas locales lo detectó"
friction_ids:
  - F-HOOKS-NO-INSTALADOS
  - F-IF-TOUCHED-CONDICION-INVERTIDA
  - F-IMPACT-ASSESSMENT-STUB
  - F-DCC-SIN-GATE-EVOLUTION
depends_on:
  - PBI-KAIZEN-FEATURE-LAB-INIT-FRICTIONS
related:
  - SddIA/scripts/qa/git-hooks/install-hooks.sh
  - SddIA/scripts/qa/git-hooks/pre_push_gate.sh
  - SddIA/scripts/qa/git-hooks/pre_commit_gate.sh
  - SddIA/tools/sddia-qa/src/gate_evolution.rs
  - SddIA/engine/execute-process/src/engine/phase_capsules.rs
  - SddIA/engine/execute-process/src/engine/delivery_close.rs
  - SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md
  - .github/workflows/sddia-index-qa.yml
  - docs/todos/done/[KAIZEN] Tekton — aduana local evolution y veto de polling CI.md
source_audit: "Auditoría de las cuatro capas de control tras el rojo de PR #209; inspección de .git/hooks, core.hooksPath, run_gate y capsule_delivery_impact_assessment"
---

# [KAIZEN] Aduana evolution local inexistente — hooks sin instalar, `--if-touched` invertido y fase de impacto stub

## 1. Falla Estructural y Contexto

El PR [#209](https://github.com/racso80es/SddIA/pull/209) llegó a CI con 18 mutaciones
bajo `SddIA/` y ninguna entrada correlacionada en `SddIA/evolution/`. El job
`wasi-runtime-smoke` falló en su último paso, `evolution gate (delta)`, con
`EVOL_MATERIAL_UNREGISTERED` para los 18 paths.

El smoke WASI en sí pasó. Lo que falló fue el gate de evolución acoplado a ese job.

Lo relevante no es el rojo, que se corrigió registrando `f66bad66-…`, sino que
**existen cuatro capas de control diseñadas para impedirlo y ninguna estaba operativa**.
El primer verificador real del ciclo fue GitHub Actions, con el PR ya abierto.

Esto contradice el `.cursorrules`, que declara: *"La aduana física (pre-commit / hooks PR)
refuerza esta norma"*.

## 2. Las cuatro capas y por qué ninguna disparó

### F-HOOKS-NO-INSTALADOS — no hay hooks en el clon

`.git/hooks/` contiene únicamente los `.sample` que git crea al inicializar, con fecha
`jun 9 16:20`. `git config --get core.hooksPath` está vacío. El instalador
`SddIA/scripts/qa/git-hooks/install-hooks.sh` nunca se ejecutó en este working copy.

Consecuencia: ni `pre-commit` ni `pre-push` existen. De las cuatro capas, esta es la
única que habría bloqueado el commit, porque `pre_commit_gate.sh` invoca
`gate-evolution --json` sobre el índice, sin atenuantes.

### F-IF-TOUCHED-CONDICION-INVERTIDA — el pre-push no habría bloqueado ni instalado

`pre_push_gate.sh:49` ejecuta `gate-evolution --json --range --if-touched`. En
`gate_evolution.rs:365`, `--if-touched` corta con `success: true` y exit 0 cuando el
rango **no** toca `directories.evolution`.

La condición de disparo es la inversa de la condición de riesgo: el gate solo audita
cuando ya registraste evolución. Tocar `SddIA/` material sin registrar nada sale por
`skipped: if-touched`.

El workflow de CI llama `gate-evolution --json --range` **sin** `--if-touched`
(`sddia-index-qa.yml:110-114`). Local y CI no ejecutan el mismo gate, así que el
invariante "árbol limpio → gate local ≡ job delta" que declaraba
`PBI-KAIZEN-TEKTON-EVOLUTION-GATE-NO-POLL` no se cumple para este caso.

### F-IMPACT-ASSESSMENT-STUB — la fase de impacto del ciclo no mira el diff

`delivery-close-cycle` incluye la fase "Impacto SddIA condicional", cuyo contrato en
`delivery-close-cycle.md:118` especifica *"diff name-only contra `origin/<target_branch>`;
filtra prefijo `SddIA/`"*.

La implementación (`phase_capsules.rs:489-495`) devuelve siempre `impact: "none"`,
`sddia_paths: []`, con la nota `"git diff omitido en stub Rust; paridad lab vía skip"`.
En el cierre del PR #209 reportó impacto nulo con 18 ficheros bajo `SddIA/` en el diff.

Agravante: la fase figura en `is_dcc_secondary_phase`, de modo que su fallo se degrada a
`fail_soft` en cuanto hay push o `pr_url`. Aunque se implementara el diff, no bloquearía.

### F-DCC-SIN-GATE-EVOLUTION — el ciclo de cierre no verifica evolución

Las siete fases del ciclo son Snapshot final, Impacto SddIA condicional, Aduana EDA
genómica, Publicación remota, Apertura en forja, Sello Presentación ECST e Higiene local.

Hay una aduana genómica que ejecuta `audit-eda-coverage --scan`, pero **ninguna fase
ejecuta `gate-evolution`**. El ciclo verifica cobertura EDA y no verifica evolución,
mientras CI verifica ambas. Esa asimetría garantiza que el fallo se manifieste siempre
después del push, cuando ya es caro.

## 3. Impacto

- Todo ciclo que mute `SddIA/` sin registrar evolución llega a CI en rojo.
- El rojo aparece en `wasi-runtime-smoke`, cuyo nombre no sugiere evolución, lo que
  desvía el diagnóstico hacia el build WASI (histórico de falsos positivos:
  `[FIX] wasi-runtime-somke.md`, `[FIX] eda-bus-e2e-smoke.md`).
- Coste por incidente: un push extra, un ciclo de CI completo y el tiempo de diagnóstico.
- Riesgo normativo: la afirmación del `.cursorrules` sobre la aduana física induce a
  confiar en un control inexistente.

## 4. Criterios de aceptación

| ID | Criterio |
|----|----------|
| AEL-CA1 | Existe verificación de instalación de hooks; un clon sin `.git/hooks/pre-commit` se detecta y se reporta con la orden de remedio |
| AEL-CA2 | `--if-touched` deja de saltarse el gate cuando el rango toca `SddIA/` material sin tocar `directories.evolution`; la puerta se abre solo cuando no hay material que cubrir |
| AEL-CA3 | Sobre un árbol limpio, el veredicto de `gate-evolution --range` local coincide con el del job `evolution gate (delta)` para el mismo `HEAD`, con y sin `--if-touched` |
| AEL-CA4 | `capsule_delivery_impact_assessment` calcula el diff real contra `origin/<target_branch>` y puebla `sddia_paths` |
| AEL-CA5 | El ciclo de cierre ejecuta `gate-evolution --range` antes de Publicación remota y bloquea el push si falla |
| AEL-CA6 | Tests unitarios: `--if-touched` con material sin evolution bloquea; con evolution presente pasa; sin material pasa |
| AEL-CA7 | Smoke: rama con mutación bajo `SddIA/` y sin registro no consigue abrir PR mediante `delivery-close-cycle` |

## 5. Notas de implementación

Sobre AEL-CA2, la semántica correcta parece ser la simétrica de la actual: saltar el gate
cuando el rango **no toca `SddIA/` material**, en lugar de cuando no toca `evolution`.
Conviene revisar si el flag debería desaparecer y dejar que el propio veredicto resuelva
el caso vacío, que ya contempla `L-SELF / sin material` en `lib.rs`.

Sobre AEL-CA5, valorar si la fase debe ser bloqueante o si basta con reutilizar
"Impacto SddIA condicional" una vez implementada, sacándola de `is_dcc_secondary_phase`
para que deje de degradarse a `fail_soft`.

Revisar también si conviene mover el paso `evolution gate (delta)` fuera de
`wasi-runtime-smoke` a un job propio, para que el nombre del check rojo señale la causa.
