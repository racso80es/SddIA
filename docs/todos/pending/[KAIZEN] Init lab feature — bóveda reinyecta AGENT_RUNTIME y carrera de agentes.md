---
document_id: PBI-KAIZEN-FEATURE-LAB-INIT-FRICTIONS
uuid: "58e3c9f7-0e90-4e51-8b87-a9054a9b30fe"
title: "[KAIZEN] Init lab feature — bóveda reinyecta AGENT_RUNTIME y carrera de agentes"
format: markdown
version: "1.1.0"
created: "2026-08-27"
refined: "2026-08-27"
status: "refinado"
priority: "alta"
process: feature
type: kaizen
dispatch: false
suggested_branch: feat/kaizen-feature-lab-init-frictions
incident_ref: "Sesión Tekton 2026-08-27 19:59→20:14 — forja plan Aduana DLT: hang init + runtime huérfano + execution_id fabricado"
friction_ids:
  - F-VAULT-UNSET-REINJECT
  - F-VAULT-DUAL-POLICY
  - F-AGENT-RUNTIME-NO-TIMEOUT
  - F-AGENT-RUNTIME-ORPHAN
  - F-EXECUTION-ID-NO-PROPAGADO
  - F-DAEMON-FORGE-PORTE
  - F-DAEMON-INDEX-DESYNC
  - F-DIRTY-WT-CROSS-CHECKOUT
  - F-PBI-UNTRACKED-BARRIDO
depends_on: []
related:
  - SddIA/engine/execute-process/src/core/env.rs
  - SddIA/engine/execute-process/src/engine/agent_runtime.rs
  - SddIA/engine/execute-process/src/engine/workspace.rs
  - SddIA/engine/execute-process/src/engine/workspace_init.rs
  - SddIA/engine/execute-process/src/engine/residual_runner.rs
  - SddIA/engine/execute-process/src/engine/entity_manager.rs
  - SddIA/engine/execute-process/src/forges/factory.rs
  - SddIA/scripts/common/sddia_shell_lib.sh
  - SddIA/scripts/tools/kalma2-agent-runtime-cursor.py
  - SddIA/process/entity-manager.md
  - SddIA/process/daemon-creator.md
  - SddIA/daemons/index.md
  - docs/features/kaizen-aduana-dlt-relay-supervisado/
  - docs/todos/done/[KAIZEN] Aduana DLT — relay IOTA supervisado y causa real en anclaje batch.md
source_audit: "Auditoría forense del ciclo forja-plan Aduana DLT (stop-at-plan); código verificado en main tras merge ecd8438"
---

# [KAIZEN] Init lab feature — bóveda reinyecta AGENT_RUNTIME y carrera de agentes

## 1. Falla Estructural y Contexto

Auditoría del ciclo del 2026-08-27 cuyo mandato era implementar `PBI-KAIZEN-ADUANA-DLT-RELAY-SUPERVISADO` y **detenerse al forjar el plan**. El entregable documental existe, pero **no lo produjo el ciclo autorizado**: lo escribió un runtime de agentes huérfano del intento anterior, ya muerto su proceso padre.

### 1.1. Cronología verificada

Reconstruida cruzando `mtime` de `.SddIA/workspaces/feature/*` con `mtime` de `docs/features/kaizen-aduana-dlt-relay-supervisado/*` y las entradas de `_agent_handoff.md` (hora local +0200).

| Hora | Hecho | Evidencia física |
|------|-------|------------------|
| 19:59:26 | Init #1 arranca. Cuelga: stdout solo `[CONFIG] Jerarquía detectada…`, sin acuse JSON | Workspace `cf73995c-034d-4e71-8e16-6bf4e35bc0a1` |
| ~20:01 | Kill manual del PID del CLI | — |
| 20:01:53 | Runtime **anidado y ya huérfano** escribe handoff Mayeuta (`status: executed`) | `_agent_handoff.md` §1 |
| 20:02:05 | Init #2 con `SDDIA_AGENT_RUNTIME_COMMAND=""`. ~3.1s, `success: true`, fases agente `simulated` | Workspace `cdd000a0-75d3-4bf9-9a4b-c1d889860ed2` |
| 20:03:01 / 20:03:57 | El huérfano escribe `objectives.md` y `clarify.md` — **después** de que el ciclo válido cerrara | `mtime` posterior al init #2 |
| 20:04:58 | El huérfano escribe `spec.md` y `plan.md` | idem |
| 20:05:26 | Última entrada de handoff (Dedalo) | 3m21s tras el kill del padre |

**Consecuencia:** el ciclo con `execution_id` válido (`cdd000a0`) produjo **cero artefactos documentales** — sus fases fueron `simulated`. Todo el corpus Mayeuta/Dedalo pertenece a un proceso sin ciclo vivo que lo ampare.

### 1.2. El `execution_id` de los artefactos es ficción

| Artefacto | `execution_id` declarado | Procedencia real |
|-----------|--------------------------|------------------|
| `objectives.md`, `clarify.md` | `cdd000a0-75d3-4bf9-9a4b-c1d889860ed2` | **Parche manual posterior**: el ciclo `cdd000a0` nunca escribió estos archivos |
| `spec.md`, `plan.md` | `a7f3e291-6c4b-4d8e-9a1f-3b5e7c8d0e2f` | **Fabricado por el agente**: no existe `.SddIA/workspaces/feature/a7f3e291-…` |

Causa: el Core genera `execution_id` en `workspace.rs:56-67` y lo inyecta en `inputs`/`state`, pero **`build_prompt` no lo pasa al agente** (`kalma2-agent-runtime-cursor.py:460-498` entrega `persist_ref`, `branch_name`, `correlation_id`) y **el frontmatter del handoff tampoco lo escribe** (`:565-571`). El agente, obligado por `features-documentation-pattern` a rellenar el campo, lo inventa. `clarify.md` lo delata: guarda `mayeuta_session_id: a7f3e291-…`, el mismo UUID que luego reaparece como `execution_id` en `spec.md`/`plan.md`.

Esto **no es una carrera** de dos ejecuciones legítimas: es un identificador de trazabilidad que el motor no propaga y que la capa de agentes rellena con basura verosímil. La trazabilidad documental del ciclo es, hoy, decorativa.

### 1.3. Cadena causal

**1. `unset` es una trampa — confirmado en código.**

```rust
// SddIA/engine/execute-process/src/core/env.rs:68-78
fn apply_env(merged: &HashMap<String, String>) {
    for (key, value) in merged {
        if env::var(key).is_err() { env::set_var(key, value); }
    }
    for key in VAULT_PRECEDENCE_KEYS { /* pisa siempre */ }
}
```

| Acción del operador | Resultado | `is_configured()` (`agent_runtime.rs:12-18`) |
|---------------------|-----------|-----------------------------------------------|
| `unset` / `env -u` | Bóveda **reinyecta** | `true` → spawn del runtime |
| `export VAR=` | No reinyecta (var presente aunque vacía) | `false` → `simulated` |

`SDDIA_AGENT_RUNTIME_COMMAND` **no** está en `VAULT_PRECEDENCE_KEYS` (`env.rs:9`, hoy solo `SDDIA_LAB_SIMULATE_IOTA` y `SDDIA_IOTA_TIMEOUT_SECONDS`). El operador que hace lo natural (`unset`) obtiene lo contrario de lo que pide, y el CLI **no falla: se cuelga**.

**2. Dos políticas de bóveda divergentes.** El Rust hace `setdefault` (respeta el entorno). `_sddia_load_vault` (`sddia_shell_lib.sh:76-109`, línea 107) hace `export key=value` **incondicional**. El remedio `COMMAND=""` funciona al invocar `sddia-run.sh` directamente, pero **no** en rutas lanzadas por `start-sddia.sh`, daemons o smokes, que pasan por la política shell. Una misma variable tiene dos semánticas según la puerta de entrada.

**3. El hang no tiene techo.** `invoke_agent_phase` hace `child.wait_with_output()` (`agent_runtime.rs:240`) **sin timeout**. El límite de 600s (`SDDIA_AGENT_RUNTIME_TIMEOUT_SECS`) vive únicamente dentro de la prótesis Python. Si el comando configurado no es esa prótesis, o si cuelga antes de armar su propio watchdog, el CLI espera indefinidamente. El «≥95s» observado es el tiempo que el operador aguantó, no un techo del sistema.

**4. El hijo sobrevive al padre.** El spawn (`agent_runtime.rs:215-246`) no crea grupo de proceso propio ni registra el PID para limpieza. Matar el CLI deja vivos al runtime y a `cursor-agent`, que siguieron escribiendo en `persist_ref` durante 3m21s tras el kill. No existe guarda de reentrada (`SDDIA_AGENT_RUNTIME_DEPTH` o equivalente) en el motor.

**5. Porte daemon a medio camino.** `run_daemon_forge` ya está en `main` (`factory.rs:1110`, rama `"daemon"` en `:1278`, fallback por `daemon_name` en `:1283`), pero el circuito sigue abierto: `entity-manager` no declara la clase (`entity-manager.md:6` y `entity_manager.rs` `PILOT_CLASSES:16-18`, `creator_name:36-48`, `dir_by_class:51-63` — 9 clases piloto, ninguna es daemon), y `residual_runner.rs:748` mantiene el fail-soft exclusivo que convierte un fallo de forja de `daemon-creator` en `status: simulated` + nota `forja pendiente porte`, mientras el resto de creators fallan con `failed`.

**6. La forja de daemon solo cubre la definición.** `run_daemon_forge` materializa `{name}.md` + fila de índice. El launcher `SddIA/daemons/{name}.sh`, el wrapper `SddIA/scripts/daemons/{name}.sh`, el crate Rust y el alta en `SYSTEMD_FACTORY_DAEMONS` (`instance_creator.rs:191+`) quedan fuera. En `iota-publish-relay` todo eso se creó a mano — mutación manual de genoma (DA-2) sin cadena autorizada que la ampare. Además quedan **dos launchers con semántica distinta**: `SddIA/daemons/iota-publish-relay.sh` (exec directo del binario) y `SddIA/scripts/daemons/iota-publish-relay.sh` (vía `_run_daemon.sh`, con lock).

**7. Índice desincronizado.** `SddIA/daemons/index.md` tiene seis filas (`:19-24`) y su nota de integridad sigue diciendo «cinco Centinelas catalogados» enumerando los cinco anteriores (`:32`). La forja escribe la fila y no toca el resumen: el índice miente sobre su propio censo, y ya lo hace en `main`.

**8. WT sucio atraviesa checkout.** `workspace_init.rs` ejecuta `fetch` → `checkout base` → `pull` → `checkout -b` (`:201-242`) **sin consultar `git status` en ningún punto**. No hay stash, ni lista de paths, ni abort. `SDDIA_LAB_ALLOW_DIRTY` **no existe en el código** ni en ningún otro punto del repositorio. El único gate «dirty» del motor está en el Snapshot final de `delivery-close` (`phase_capsules.rs:316-366`), es decir, al cerrar, no al abrir. Durante el incidente, `docs/todos/pending/[OPERATIVO] Latido Ontológico…` viajó modificado desde `main` a la rama de feature.

**9. Los PBI sin trackear son destruibles por el propio ciclo.** Este PBI fue barrido del disco por el commit `66aa7cd` («revertir PBIs ajenos del snapshot DCC») del ciclo Aduana DLT: al no estar bajo git, la limpieza de artefactos ajenos al snapshot lo eliminó sin dejar rastro en ninguna rama. Un PBI en `docs/todos/pending/` es un activo del sistema; que su supervivencia dependa de haber sido añadido al índice a tiempo es una trampa estructural, no un descuido del operador.

## 2. Objetivo Medible

Que un init lab de `feature` / `bug-fix` / `refactorization` con relevo IDE sea **determinista y acotado en el tiempo**, que ningún proceso agente sobreviva a su ciclo, y que la trazabilidad documental (`execution_id`) sea emitida por el motor y no redactada por el agente.

Éxito si:

1. Existe **una** forma explícita y documentada de pedir relevo IDE que funcione por las dos puertas (Rust y shell); `unset` deja de ser un modo de fallo silencioso.
2. Ninguna invocación de runtime de agentes puede colgar el CLI sin límite: hay timeout en el motor con acuse `failed`/`timeout`, no espera infinita.
3. Matar el ciclo mata a su descendencia; ningún huérfano escribe en `persist_ref` después del cierre.
4. `execution_id` en el frontmatter de los artefactos es el del motor, verificable contra `.SddIA/workspaces/{process}/{id}/`.
5. `daemon-creator` materializa o falla ruidoso; nunca `simulated` opaco. El índice de daemons no se contradice.
6. `workspace-init` no arrastra WIP ajeno a la rama de trabajo sin consentimiento explícito.
7. La limpieza de un ciclo no puede destruir PBIs ajenos sin trackear.

## 3. Decisiones Arquitectónicas Obligatorias

### 3.1. Semántica única de relevo de agentes (F-VAULT-UNSET-REINJECT + F-VAULT-DUAL-POLICY)

La v1.0.0 ofrecía tres opciones. La auditoría las reduce: la opción «documentar que `unset` es incorrecto» es insuficiente porque no cubre la puerta shell, donde `export VAR=` **tampoco** funciona. Se exige **flag positivo + paridad de puertas**:

1. Flag explícito de relevo (p. ej. `SDDIA_AGENT_RELAY_IDE=1`) que fuerce `is_configured() == false` **independientemente** del valor de bóveda. La ausencia de comando deja de ser la forma de pedir relevo; el relevo se pide, no se simula por omisión.
2. `_sddia_load_vault` debe adoptar la misma política de precedencia que `apply_env`: `export` solo si la variable no está definida, salvo lista explícita de precedencia. Una variable, una semántica, dos puertas.
3. Log en stderr al activarse: `agent-runtime: lab-relay activo (bóveda ignorada)`. El estado debe ser legible sin leer código.

Prohibido dejar el conocimiento operativo únicamente en `clarify.md` de features sueltas.

### 3.2. El runtime de agentes se acota y se entierra (F-AGENT-RUNTIME-NO-TIMEOUT + F-AGENT-RUNTIME-ORPHAN)

1. Timeout en el motor (`agent_runtime.rs`), no delegado a la prótesis: superado el límite, `status: failed` con `error: agent-runtime-timeout` y acuse JSON. El default debe ser configurable y estar por encima del de la prótesis para no enmascararlo.
2. Spawn en grupo de proceso propio y kill del grupo al expirar el timeout o al abortar el ciclo. Ningún descendiente debe sobrevivir al `execution_id` que lo engendró.
3. Guarda de reentrada: el motor marca el entorno del hijo (profundidad o flag) de modo que un `execute-process` anidado no vuelva a disparar fases de agente.

### 3.3. `execution_id` lo emite el motor, no el agente (F-EXECUTION-ID-NO-PROPAGADO)

1. `execution_id` entra en el payload y en el prompt del agente, junto a `persist_ref` y `branch_name`.
2. El frontmatter de `_agent_handoff.md` lo declara.
3. Guard de escritura: si el `persist_ref` ya contiene artefactos con un `execution_id` distinto del vivo → `conflict` explícito, no sobrescritura silenciosa.
4. Un `execution_id` sin workspace correspondiente en `.SddIA/workspaces/{process}/` es inválido por definición; debe ser detectable por auditoría documental.

### 3.4. Porte de forja daemon — consolidación (F-DAEMON-FORGE-PORTE)

`run_daemon_forge` ya vive en `main`. Falta cerrar el circuito:

1. Añadir `daemon` a las clases piloto de `entity-manager` (`.md` + `PILOT_CLASSES` + `creator_name` + `dir_by_class`) **o** canonizar la invocación directa de `daemon-creator` en la norma obrera. Hoy la semilla del PBI Aduana DLT dice «entity-manager» y es incorrecta: esa vía devuelve `entity_class fuera del piloto`.
2. Eliminar el fail-soft de `residual_runner.rs:748`. Un fallo de forja es `failed`.
3. Decidir el alcance de la forja: o `run_daemon_forge` materializa también launcher, wrapper y alta systemd, o el contrato declara explícitamente qué queda como delivery post-forja y bajo qué autorización — hoy ese tramo se cubre con mutación manual del genoma.
4. Resolver el doble launcher de `iota-publish-relay`: una sola vía de arranque.

### 3.5. El índice de daemons no puede contradecirse (F-DAEMON-INDEX-DESYNC)

La forja que inserta la fila actualiza también el censo del pie, o el censo deja de ser texto libre y pasa a derivarse. Un índice que dice «cinco» con seis filas es entropía documental.

### 3.6. Aislamiento del worktree en init (F-DIRTY-WT-CROSS-CHECKOUT)

`workspace_init` consulta `git status --porcelain` **antes** de la secuencia git. Si hay cambios fuera de `persist_ref` / `pbi_ref`: abortar con la lista de paths, salvo `SDDIA_LAB_ALLOW_DIRTY=1` explícito. Prohibido el checkout silencioso que arrastre WIP ajeno.

### 3.7. Los PBI no se barren (F-PBI-UNTRACKED-BARRIDO)

La limpieza de snapshot de `delivery-close` no puede eliminar archivos sin trackear bajo `docs/todos/`. O los preserva, o los reporta y aborta. Complementariamente, el alta de un PBI debe dejarlo bajo control de versiones desde su creación, no al primer ciclo que lo consuma.

## 4. Alcance

### Dentro

- Flag de relevo IDE + unificación de la política de bóveda entre Rust y shell.
- Timeout, kill de grupo y guarda de reentrada en `agent_runtime.rs`.
- Propagación de `execution_id` al prompt, al handoff y guard de conflicto en `persist_ref`.
- Consolidación del porte `daemon-creator` (clase piloto, fin del fail-soft, alcance de forja declarado).
- Sincronización del censo de `SddIA/daemons/index.md`.
- Gate de worktree sucio en init con escape explícito.
- Protección de `docs/todos/` frente a la limpieza de snapshot.

### Fuera

- Sustituir Kalma2 agent runtime.
- El PBI Aduana DLT en sí; es consumidor de este porte, no parte de él.
- Auto-review / Smart Mode de Cursor: aduana externa al Core.
- Saneamiento retroactivo del corpus `docs/features/kaizen-aduana-dlt-relay-supervisado/`, ya mergeado con `execution_id` falsos.
- GC de `.SddIA/workspaces/` (140 entradas acumuladas en `feature`). Deuda anotada, no de este ciclo.

## 5. Criterios de Aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| LAB-CA1 | Con bóveda que define `SDDIA_AGENT_RUNTIME_COMMAND`, el init con flag de relevo acusa JSON en <15s y las fases agente quedan `simulated` | Smoke con bóveda poblada + flag |
| LAB-CA2 | `unset SDDIA_AGENT_RUNTIME_COMMAND` deja de ser un modo de fallo silencioso: o se neutraliza, o el CLI lo señala | Test sobre `apply_env` / `is_configured` |
| LAB-CA3 | La política de bóveda es idéntica por ambas puertas | Test paridad `apply_env` ↔ `_sddia_load_vault` |
| LAB-CA4 | Un runtime que no responde produce `agent-runtime-timeout` y acuse JSON; el CLI no espera indefinidamente | Smoke con comando `sleep infinity` |
| LAB-CA5 | Tras abortar el ciclo, cero descendientes vivos y cero escrituras posteriores en `persist_ref` | `ps` del grupo + `mtime` de artefactos tras el kill |
| LAB-CA6 | Todo `execution_id` de frontmatter resuelve a `.SddIA/workspaces/{process}/{id}/` existente | Auditoría documental sobre un ciclo completo |
| LAB-CA7 | Un segundo writer con otro `execution_id` obtiene `conflict`, no sobrescritura | Smoke de conflicto |
| LAB-CA8 | `daemon-creator` materializa `.md` + fila de índice con exit 0 real, o falla con `failed`; nunca `simulated` | `./sddia-run.sh --process daemon-creator` en lab |
| LAB-CA9 | `index.md` no se contradice: censo del pie = número de filas | Verificación tras forja de un daemon de prueba |
| LAB-CA10 | Init con archivo dirty ajeno → aborta con lista de paths; solo `SDDIA_LAB_ALLOW_DIRTY=1` lo permite | Smoke de worktree sucio |
| LAB-CA11 | Un ciclo de entrega no elimina archivos sin trackear bajo `docs/todos/` | Smoke: PBI untracked + `delivery-close` |

## 6. Orden de ejecución sugerido

1. **F-AGENT-RUNTIME-NO-TIMEOUT + F-AGENT-RUNTIME-ORPHAN** — barato y detiene la hemorragia: acota el hang y elimina los huérfanos que corrompen el corpus. Precede al resto porque sin esto cada ciclo lab puede repetir el incidente.
2. **F-VAULT-UNSET-REINJECT + F-VAULT-DUAL-POLICY** — desbloquea el init lab por ambas puertas.
3. **F-EXECUTION-ID-NO-PROPAGADO** — restituye la trazabilidad documental.
4. **F-DAEMON-FORGE-PORTE + F-DAEMON-INDEX-DESYNC** — consolida lo ya aterrizado en `main`.
5. **F-DIRTY-WT-CROSS-CHECKOUT + F-PBI-UNTRACKED-BARRIDO** — higiene de rama y preservación de activos.

## 7. Estado del territorio al refinar

El ciclo Aduana DLT cerró y mergeó en `main` (`ecd8438`). Verificado sobre `main` tras el merge:

| Hueco | Estado |
|-------|--------|
| `run_daemon_forge` + rama `"daemon"` | **Aterrizado** (`factory.rs:1110`, `:1278`, `:1283`) |
| `entity-manager` con clase `daemon` | **Abierto** (`entity-manager.md:6`: 9 clases, ninguna daemon) |
| Fail-soft `simulated` en `daemon-creator` | **Abierto** (`residual_runner.rs:748`) |
| Censo de `daemons/index.md` | **Abierto y ya en main** (6 filas, «cinco Centinelas») |
| Timeout en `agent_runtime` | **Abierto** (`:240` sigue siendo `wait_with_output()` desnudo) |
| `SDDIA_LAB_ALLOW_DIRTY` | **Inexistente** en todo el repositorio |

**F-DAEMON-FORGE-PORTE deja de ser bloqueante y pasa a ser consolidación.** El resto de fricciones permanece intacto en el código de `main`.

## 8. Correcciones respecto a v1.0.0

| Afirmación v1.0.0 | Veredicto | Corrección |
|-------------------|-----------|------------|
| «Artefactos Mayeuta con `execution_id` distinto `a7f3e291-…`» | **Impreciso** | Mayeuta escribió `cdd000a0` (parcheado a mano); `a7f3e291` está en `spec.md`/`plan.md` y **no existe como workspace**: es un UUID fabricado |
| «Carrera documental entre dos ejecuciones» | **Reencuadrado** | No hubo dos writers legítimos: hubo un huérfano escribiendo tras el cierre del ciclo válido, y un `execution_id` que el motor nunca propaga |
| «`factory.rs` no tiene rama daemon» | **Superado** | `run_daemon_forge` aterrizó en `main` con el merge del ciclo Aduana DLT |
| «Hang ≥95s» | **Subestimado** | No hay timeout en `agent_runtime.rs:240`; el techo lo puso el operador |
| «`unset` es una trampa» | **Confirmado** | `env.rs:68-78`, corroborado por el test `not_configured_returns_simulated` |
| «`entity-manager` no declara `daemon`» | **Confirmado** | `entity-manager.md:6`, `entity_manager.rs:16-18` |
| «`workspace-init` no aísla WT sucio» | **Confirmado y agravado** | No consulta `git status` en absoluto; `SDDIA_LAB_ALLOW_DIRTY` no existe en código |
| «Mandato stop-at-plan cumplido» | **Superado por los hechos** | El ciclo continuó hasta implementación, entrega y merge en `main` |

**Conclusión:** el peaje no estuvo en el diseño Dedalo sino en tres huecos del Core que se refuerzan entre sí: **una bóveda con dos políticas**, **un runtime de agentes sin límite ni entierro**, y **una trazabilidad que el motor delega en la redacción del agente**. Cuando el proceso no emite el identificador, el agente lo inventa; cuando el hijo no muere con el padre, escribe el expediente de un ciclo que ya cerró; y cuando el PBI no está bajo git, el propio ciclo que lo motivó puede borrarlo.
