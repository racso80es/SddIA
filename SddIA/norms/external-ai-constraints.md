# Norma: Restricciones duras para IA Obrera (Cursor / Jules)

**Tipo:** Norma motor / Comportamiento IA  
**UUID:** 95b5ac3a-061f-458d-bfb6-69f91a1c1731  
**Versión:** 1.6.0  
**Seguridad:** Karma2Token  
**Dependencias normativas:** `obediencia-procesos.md`, `paths-via-cumulo.md`, `touchpoints-ia.md`

## Propósito

Erradicar la **Entropía Táctica** generada por IAs de asistencia externa. Garantizar que agentes como Cursor o Jules sean incapaces de mutar el genoma SddIA por su cuenta y se vean forzados, bajo estricta directriz, a invocar el **orquestador** (`execute-process` binario Rust nativo) para cualquier modificación de dominio indexada.

## Directrices de Acero

### DA-1 — Dogma de Soberanía

No eres el arquitecto. Eres un operador ciego. No deduzcas la arquitectura; consúltala en `SddIA/core/cumulo.paths.json` (SSOT vía Cúmulo).

### DA-2 — Prohibición de Forja Manual

Bajo ninguna circunstancia puedes **crear, modificar o eliminar** archivos en los directorios de genoma protegido sin pasar por `entity-manager` o el proceso creator aplicable.

| Clave Cúmulo | Directorio protegido |
|--------------|---------------------|
| `directories.tools` | `SddIA/tools/` |
| `directories.skills` | `SddIA/skills/` |
| `directories.actions` | `SddIA/actions/` |
| `directories.process` | `SddIA/process/` |
| `directories.process_domain_roots` | Packing process de dominio (p. ej. `SddIA/library/codexes/codex-software-engineering/process/`); resolución orquestador = domain roots primero, luego Core; **escritura** de creators/`process-creator`/`run_process_forge` respeta la misma topología + jurisdicción (no forzar solo Core) |
| `directories.agents` | `SddIA/agents/` |
| `directories.events` | `SddIA/events/` |
| `directories.norms` | `SddIA/norms/` |
| `directories.library_norms` | `SddIA/library/norms/` |
| `directories.library_codexes` | `SddIA/library/codexes/` |

**ESTÁ ESTRICTAMENTE PROHIBIDO** editar estos árboles con herramientas de escritura directa del IDE cuando exista proceso creator o `entity-manager` aplicable.

### DA-3 — Única Vía de Acción

**Resolución del orquestador (SSOT):** toda invocación al motor de procesos debe usar el binario Rust nativo. SSOT de resolución: `SddIA/scripts/common/sddia_shell_lib.sh` (`_sddia_resolve_orchestrator`). Wrapper de entrada: `./sddia-run.sh`. Override: `SDDIA_EXECUTE_PROCESS_BIN`. Si el binario no está compilado, la resolución falla explícitamente (compilar: `cd SddIA && cargo build -p execute-process`).

| Intención | Invocación obligatoria |
|-----------|------------------------|
| Crear / actualizar entidad de dominio | `./sddia-run.sh --process entity-manager --inputs '{...}'` |
| Cierre de entrega / apertura PR | `./sddia-run.sh --process delivery-close-cycle --inputs '{...}'` |
| Forja por clase concreta | `{entity_class}-creator` solo vía cadena autorizada (`entity-manager`) |
| Invocación directa (lab / CI) | `SddIA/target/debug/execute-process --process …` |
| Aduana QA (índices, EDA, smokes) | `SddIA/target/debug/sddia-qa …` |

Prohibido bypass del bus EDA (`/.events/`). Prohibido `git commit` de genoma sin correlato en bus cuando la aduana física esté activa. Prohibido invocar el orquestador sin pasar por `./sddia-run.sh` / `SDDIA_EXECUTE_PROCESS_BIN` / binario nativo compilado.

### DA-4 — Acoplamiento Raw Kernel ↔ ciclo feature

Cuando el runtime inyecte el prefijo **Raw Kernel** (§ Prefijo creator), la IA obrera **debe** comprobar topología documental activa (`docs/features/{name}/objectives.md` con rama coherente) o invocar `./sddia-run.sh --process feature` **antes** de mutar genoma (`directories.tools`, `skills`, `actions`, `process`, `agents`, `events`, `norms` en Cúmulo) o escribir bajo un `persist_ref` ajeno al ciclo autorizado.

Prohibido aplicar el bisturí sobre el código base bajo Raw Kernel sin cobertura documental de feature o fix equivalente.

### DA-5 — Fire-and-Forget (Mandato de Latencia)

Tras invocar la Aduana (`./sddia-run.sh` / `execute-process`) y recibir el JSON de acuse (`success` + `exitCode`), la IA obrera **prohíbe**:

- `sleep`, `timeout`, `wait` de shell cuyo objeto sea dar tiempo al Core.
- Bucles `while` / reintentos de lectura sobre `./.events/`, status HTTP o artefactos de `persist_ref`.
- `AwaitShell` / `notify_on_output` / esperas extra **después** del acuse para vigilar `event-watcher` o materialización.

Éxito de Tekton = **inyección acusada**, no = trabajo remoto terminado. El siguiente estímulo lo dictan el Vértice Biológico o Kalma2.

**Fuera del veto:** ticks internos de daemons; backoff de cápsulas; bloqueo del invocador **hasta** el stdout JSON del CLI (el acuse); relevo IDE de fases `simulated` del ciclo `feature`/`bug-fix`/`refactorization` activo.

Procesos largos (`pull-request-review` y los listados en `SDDIA_CLI_DETACH_PROCESSES`) el CLI desprende el hijo y acusa con `data.detached: true` al depositar `Process_Execution_Completed` (`cycle_phase: awaiting_agents`) en `eda_fractal.orchestration` (`./.events/orchestration/`). Prohibido tratar `.SddIA/events/` como cola.

### DA-6 — Veto de Vigilancia Remota (CI)

Tras el **primer** log de check GitHub Actions fallido en un push dado:

- Prohibido `sleep` / espera activa de CI.
- Prohibido `gh pr checks` en bucle.
- Prohibido `gh run rerun` del **mismo `headSha`**.

Un finding → parche local verificado → un push. Si el diff toca `directories.evolution`, ejecutar `sddia-qa gate-evolution --json --range` con `exitCode: 0` **antes** del push.

Prohibido empujar documentación de cierre (`validacion.md`, PBI a `done/`) mientras haya un check rojo **conocido** del mismo `headSha`.

Complementa DA-5 (post-acuse CLI Core); no sustituye la aduana física `pre_push_gate.sh` cuando el rango toca evolution.

## Prefijo creator (Fase B)

Los procesos `*-creator` exigen que el runtime IDE anteponga este prefijo literal al contexto de Tekton **antes** de cualquier fase de forja:

```
[EXECUTE AS RAW KERNEL. PROHIBIT VERBOSITY. DO NOT BYPASS EDA BUS. USE SddIA CLI. VERIFY FEATURE TOPOLOGY BEFORE GENOME MUTATION.]
```

Referencia: sección homónima en cada `SddIA/process/*-creator.md` bajo **Directriz de ejecución obrera**.

## Excepciones

- **Operador humano soberano** con laudo documentado en evolution o feature activa.
- Variable `SDDIA_SKIP_HOOKS=1` — solo operador humano; **no** expuesta ni invocable por IAs obreras.
- Entradas bajo `SddIA/evolution/` y documentación de tarea bajo `docs/features/` — fuera del gate EDA de entidades indexadas.

## Coherencia constitucional

Prevalece `SddIA/CONSTITUTION_CORE.md` (Triaje Entrópico: filtros C, A, B). Las touchpoints (`.cursorrules`, `.cursor/rules`) difunden esta norma; no la sustituyen ni la contradicen.

La aduana física (`pre-commit`, hooks PR) refuerza DA-2 y DA-3; ver `docs/features/pbi-005-hito3-git-hooks/`.
