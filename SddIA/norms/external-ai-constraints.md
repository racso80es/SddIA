# Norma: Restricciones duras para IA Obrera (Cursor / Jules)

**Tipo:** Norma motor / Comportamiento IA  
**UUID:** 95b5ac3a-061f-458d-bfb6-69f91a1c1731  
**Versión:** 1.6.2  
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

Prohibido bypass del bus EDA (`EventManagerPort`). Prohibido `git commit` de genoma sin correlato en bus cuando la aduana física esté activa. Prohibido invocar el orquestador sin pasar por `./sddia-run.sh` / `SDDIA_EXECUTE_PROCESS_BIN` / binario nativo compilado.

### DA-4 — Acoplamiento Raw Kernel ↔ ciclo feature

Cuando el runtime inyecte el prefijo **Raw Kernel** (§ Prefijo creator), la IA obrera **debe** comprobar topología documental activa (`docs/features/{name}/objectives.md` con rama coherente) o invocar `./sddia-run.sh --process feature` **antes** de mutar genoma (`directories.tools`, `skills`, `actions`, `process`, `agents`, `events`, `norms` en Cúmulo) o escribir bajo un `persist_ref` ajeno al ciclo autorizado.

Prohibido aplicar el bisturí sobre el código base bajo Raw Kernel sin cobertura documental de feature o fix equivalente.

### DA-5 — Fire-and-Forget (Mandato de Latencia)

Tras invocar la Aduana (`./sddia-run.sh` / `execute-process`) y recibir el JSON de acuse (`success` + `exitCode`), la IA obrera **prohíbe**:

- `sleep`, `timeout`, `wait` de shell cuyo objeto sea dar tiempo al Core.
- Bucles `while` / reintentos de lectura sobre colas/store EDA, status HTTP o artefactos de `persist_ref`.
- `AwaitShell` / `notify_on_output` / esperas extra **después** del acuse para vigilar `event-watcher` o materialización.

Éxito de Tekton = **inyección acusada**, no = trabajo remoto terminado. El siguiente estímulo lo dictan el Vértice Biológico o Kalma2.

**Fuera del veto:** ticks internos de daemons; backoff de cápsulas; bloqueo del invocador **hasta** el stdout JSON del CLI (el acuse); relevo IDE de fases `simulated` del ciclo `feature`/`bug-fix`/`refactorization` activo.

**Relevo IDE (lab):** con `SDDIA_AGENT_RELAY_IDE=1` (o `true`/`yes`/`on`), las fases `agent:` quedan `simulated` aunque la bóveda defina `SDDIA_AGENT_RUNTIME_COMMAND`. El motor emite en stderr: `agent-runtime: lab-relay activo (bóveda ignorada)`. `unset` o ausencia de `COMMAND` **no** es contrato de relevo; usar el flag explícito. Paridad bóveda: `_sddia_load_vault` aplica `setdefault` (misma semántica que `apply_env` en Rust); precedencia incondicional solo para `SDDIA_LAB_SIMULATE_IOTA` y `SDDIA_IOTA_TIMEOUT_SECONDS`.

Procesos largos (`pull-request-review` y los listados en `SDDIA_CLI_DETACH_PROCESSES`) el CLI desprende el hijo y acusa con `data.detached: true` al depositar `Process_Execution_Completed` (`cycle_phase: awaiting_agents`) vía `EventManagerPort` (`eda://orchestration/<event_id>`). Prohibido tratar `.SddIA/events/` como cola.

### DA-6 — Veto de Vigilancia Remota (CI)

Tras el **primer** log de check GitHub Actions fallido en un push dado:

- Prohibido `sleep` / espera activa de CI.
- Prohibido `gh pr checks` en bucle.
- Prohibido `gh run rerun` del **mismo `headSha`**.

Un finding → parche local verificado → un push. Si el diff toca `directories.evolution`, ejecutar `sddia-qa gate-evolution --json --range` con `exitCode: 0` **antes** del push.

Prohibido empujar documentación de cierre (`validacion.md`, PBI a `done/`) mientras haya un check rojo **conocido** del mismo `headSha`.

Complementa DA-5 (post-acuse CLI Core); no sustituye la aduana física `pre_push_gate.sh` cuando el rango toca evolution.

### DA-7 — Veto post-DCC `failed`/`blocked`

Tras acuse de `delivery-close-cycle` con fase Publicación remota `failed` o `blocked` (incl. `F-DCC-WORKFLOW-SCOPE`):

- Prohibido `git push`, `gh pr create` / `gh pr merge` raw, y URL `x-access-token`.
- El rechazo GitHub `without workflow scope` es colapso de **credencial** (PAT de `git-manager` ≠ token `gh auth`), no de Core.
- Operador: `gh auth setup-git` (unificar helper). `gh auth refresh -s workflow` solo si git ya delega en `gh`.
- Siguiente estímulo = laudo del Vértice Biológico o re-inyección DCC con credencial alineada.

### DA-8 — Invariantes de Handoff, Anti-Evasión Léxica y Paridad de Host (`KZ-PRM-01` .. `KZ-PRM-06`)

- **Oráculo primero en Ola 0 (`KZ-PRM-01`):** Cada `AC-1..AC-N` del plan debe tener un oráculo mecánico ejecutable capturado en rojo (`oracle-red.log`) antes de autorizar cambios productivos en Ola 1. Prohibido `grep` manual ad-hoc como único método de verificación.
- **Anti-evasión léxica (`KZ-PRM-02`):** Ante un gate o test de higiene rojo, prohibido ofuscar literales (`concat!`, interpolación dinámica), cambiar solo prefijos de ruta manteniendo `std::fs`, o ampliar `VIRTUAL_PATH_ALLOW` / `SCAN_EXCLUDE` / `INTEGRATION_SRC` / `allowlists/*.toml` sin laudo `RATCHET_LAUDO` previo en `clarify.md`.
- **Baseline hermético Ola 0 (`KZ-PRM-03`):** Registrar en `execution.md` antes de codificar: `git status --porcelain` vacío, ausencia física de `.events`/`orchestration`/`telemetry`/`_lab` en raíz, `systemctl --user list-units 'sddia-*'`, y baseline de tests con `env -u SDDIA_EVENT_STORE -u SDDIA_EXECUTE_PROCESS_BIN`.
- **Protocolo estricto `entity-manager` (`KZ-PRM-04`):** Verificar siempre `handoff_hash_signature_new != handoff_hash_signature_old`. Si devuelve `idempotent` o firmas iguales, la mutación no se aplicó: corregir el payload y nunca editar `.md` de genoma a mano. Inspeccionar frontmatter, tablas y diagramas `mermaid`.
- **Paridad build/host (`KZ-PRM-05`):** Si el diff toca crates Rust que producen o alimentan binarios residentes `sddia-*`: compilar `cargo build --workspace` y `cargo build --workspace --release`, reiniciar `systemctl --user restart 'sddia-*@*.service'` y re-ejecutar `cargo test -p execute-process --lib eda_event_token_hygiene` antes de `delivery-close-cycle`.
- **Separación Tekton/Argos (`KZ-PRM-06`):** Prohibido recortar ACs o diferir hijos del plan sin `refine-item` previo; prohibido degradar `F3_TECH_GATE` o filas `PARCIAL` a `non_blocking_findings`. Tekton jamás escribe `global: APTO` en `validacion.md`.

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
