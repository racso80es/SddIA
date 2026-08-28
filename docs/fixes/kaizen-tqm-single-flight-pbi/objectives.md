---
feature_name: kaizen-tqm-single-flight-pbi
created: "2026-08-28"
process: bug-fix
branch_name: fix/kaizen-tqm-single-flight-pbi
persist_ref: docs/fixes/kaizen-tqm-single-flight-pbi
pbi_ref: docs/todos/pending/[KAIZEN] TQM sin single-flight por PBI — cadenas bug-fix duplicadas y agentes en carrera.md
execution_id: "25e1072f-3ba1-4b64-8e24-b9513ab702e3"
---

# Objetivos — kaizen-tqm-single-flight-pbi

## Misión

---
document_id: PBI-KAIZEN-TQM-SINGLE-FLIGHT-PBI
uuid: "86eabaf5-3fa2-4321-96ad-88d1b5485aa2"
title: "[KAIZEN] TQM sin single-flight por PBI — cadenas bug-fix duplicadas y agentes en carrera"
format: markdown
version: "1.1.0"
created: "2026-08-28"
updated: "2026-08-28"
status: "pendiente"
priority: "alta"
process: bug-fix
type: kaizen
dispatch: false
suggested_branch: fix/kaizen-tqm-single-flight-pbi
incident_ref: "Sesión Tekton 2026-08-28 07:31 — dos cadenas route-domain→TQM→bug-fix simultáneas sobre docs/todos/pending/[FIX] x.md"
friction_ids:
  - F-TQM-NO-DEDUP-PBI
  - F-AGENT-RACE-SAME-PERSIST-REF
  - F-WORKTREE-CROSS-WRITE-DURANTE-CICLO
  - F-TQM-LOCK-TOCTOU-VACIO
  - F-TQM-LIVENESS-PROC-LINUX-ONLY
depends_on:
  - PBI-KAIZEN-FEATURE-LAB-INIT-FRICTIONS
related:
  - SddIA/engine/execute-process/src/engine/handlers/task_queue_manager.rs
  - SddIA/engine/execute-process/src/engine/agent_runtime.rs
  - SddIA/engine/execute-process/src/engine/thermodynamic.rs
  - SddIA/engine/execute-process/src/engine/route_fractal_core.rs
  - SddIA/events/orchestration/index.md
  - SddIA/core/event-orchestration-subscriptions.json
  - SddIA/scripts/tools/kalma2-agent-runtime-cursor.py
  - docs/features/kaizen-feature-lab-init-frictions/execution.md
source_audit: "Tabla de procesos capturada en vivo durante el cierre del PR #209; dos cursor-agent concurrentes escribiendo docs/fixes/x/"
---

# [KAIZEN] TQM sin single-flight por PBI — cadenas bug-fix duplicadas y agentes en carrera

## 1. Falla Estructural y Contexto

Durante el cierre de `PBI-KAIZEN-FEATURE-LAB-INIT-FRICTIONS` se observó que
`docs/fixes/x/` se ensuciaba de forma repetida pese a revertirlo varias veces. La
causa no era residuo: había un productor activo.

El `event-watcher` despachó **dos cadenas completas e independientes** para el mismo
PBI, con ocho segundos de separación y `correlation_id` distintos. Cada cadena levantó
su propio runtime de agente y su propio `cursor-agent`, ambos escribiendo sobre el
mismo `persist_ref`.

### 1.1. Cronología verificada

Capturada con `ps -eo pid,ppid,etime,lstart,cmd` a las 07:32 (hora local +0200).

| Hora | PID | Hecho |
|------|-----|-------|
| 07:31:04 | 752859 | `route-domain` sobre `.events/domain/eaa5fb5b-….json` |
| 07:31:04 | 752860 | `task-queue-manager` — `pbi_ref: docs/todos/pending/[FIX] x.md` |
| 07:31:04 | 752861 | `bug-fix` — `correlation_id: eaa5fb5b-fdc6-4911-9782-8518c6bf0801` |
| 07:31:12 | 753455 | `route-domain` sobre `.events/domain/cc6d6e2c-….json` |
| 07:31:12 | 753456 | `task-queue-manager` — **mismo** `pbi_ref` |
| 07:31:12 | 753457 | `bug-fix` — `correlation_id: cc6d6e2c-b84b-40f9-ac01-acff25ed252e` |
| 07:31:56 | 757948 | `kalma2-agent-runtime-cursor.py` de la cadena 1 → `cursor-agent --print --trust` |
| 07:32:21 | 758489 | `kalma2-agent-runtime-cursor.py` de la cadena 2 → `cursor-agent --print --trust` |

Ambas cadenas materializaron artefactos en `docs/fixes/x/`. El `_agent_handoff.md`
resultante quedó sellado con el `execution_id` de la última en escribir
(`92716387-568c-42c9-895d-2bf2aa186659`), perdiendo la traza de la otra.

## 2. Impacto

- **Coste económico directo**: dos invocaciones de `cursor-agent` de pago por cada PBI
  despachado por duplicado.
- **Corrupción de traza**: el handoff conserva un solo `execution_id`; la escritura
  perdedora queda sin evidencia y sin forma de auditarla.
- **Escritura cruzada en el worktree**: cualquier ciclo humano o de IA que trabaje en
  la misma copia ve aparecer cambios ajenos a mitad de operación, lo que obliga a
  aislarlos a mano antes de cerrar un PR (se hizo vía `git stash` acotado en el PR #209).
- **Riesgo de sobrescritura**: dos agentes sobre el mismo `persist_ref` sin bloqueo
  pueden pisarse en cualquier orden.

## 3. Causa raíz y estado real del guard

`task_queue_manager.rs` implementa `SingleFlightGuard` sobre
`.SddIA/daemons/state/tqm-single-flight`. Auditado el código, la falla se descompone
en un defecto primario y tres defectos latentes. **Distinguirlos importa: parte de lo
que parece faltar ya está implementado y no debe reescribirse.**

### 3.1. D1 — Clave de exclusión errónea (causa raíz del incidente)

```rust
let path = dir.join(format!("{correlation_id}.lock"));
```

La clave es el `correlation_id`, que es único por evento de dominio. Dos eventos
distintos sobre el mismo PBI obtienen cada uno su cerrojo y avanzan en paralelo. El
guard nunca pudo evitar el incidente: por diseño solo protege contra el reprocesado
del *mismo* evento.

Corolario: el guard **solo se adquiere si existe `correlation_id`**
(`if let Some(cid) = correlation_id`). Un despacho sin `correlation_id` hoy corre
sin exclusión alguna.

### 3.2. D2 — Liveness ya existe; lo que falla es su precisión

`lock_pid_alive()` **ya lee el PID del interior del cerrojo y verifica
`/proc/{pid}`**, y `try_acquire_single_flight` ya purga y reintenta (3 vueltas) ante
cerrojo muerto. La propuesta de "registrar el PID y verificar si sigue activo" está
implementada y no constituye trabajo pendiente. Los huecos reales son otros:

- **D2a — Ventana TOCTOU con cerrojo vacío.** Entre `create_new` y el `writeln!` del
  PID hay un intervalo en el que el fichero existe con contenido vacío. Un segundo
  proceso que lo lea en ese instante falla el `parse::<u32>()`, concluye "muerto",
  **borra el cerrojo del propietario vivo** y adquiere el suyo. Es la única ruta por
  la que la corrección de D1 podría seguir permitiendo doble despacho.
- **D2b — Reutilización de PID.** La existencia de `/proc/{pid}` no prueba identidad:
  tras reciclado del espacio de PIDs, un proceso ajeno mantiene vivo un cerrojo
  huérfano indefinidamente (falso positivo → despacho legítimo bloqueado para siempre).
- **D2c — `/proc` es dependencia Linux.** Fuera de Linux `lock_pid_alive` devuelve
  siempre `false`, degradando el single-flight a no-op **en silencio**. Contradice el
  agnosticismo del Core (`.cursorrules` §5).

`Drop` no se ejecuta ante `SIGKILL` ni `abort`, pero eso lo cubre el liveness: el
huérfano se detecta y purga en la siguiente adquisición. **No** es necesario un
mecanismo adicional de limpieza si D2a–D2c quedan resueltos.

### 3.3. D3 — El descarte no deja rastro auditable

`single_flight_hit_envelope` devuelve `single_flight_hit: true` en el envelope y
**no emite ningún evento**. El envelope muere con el proceso: no hay artefacto
persistente que permita auditar cuántos despachos se descartaron ni contra quién.

### 3.4. Alcance temporal del cerrojo (restricción, no defecto)

`bug-fix`, `feature` y `refactorization` no están en la allowlist de
`cli_detach`, así que `dispatch_child` bloquea de forma síncrona y el guard vive
tanto como el ciclo hijo. Consistente con el `ps` del incidente (ambos TQM vivos a
los 60 s). **Si en el futuro se detachan estos procesos (o se fuerza
`SDDIA_CLI_DETACH=1`), el `Drop` liberará el cerrojo mientras el agente sigue
trabajando** y la exclusión se evaporará. Debe quedar cubierto por test de regresión.

## 4. Criterios de aceptación

| ID | Criterio |
|----|----------|
| TQM-CA1 | Dos eventos con distinto `correlation_id` y el mismo `pbi_ref` producen una sola cadena; la segunda retorna descartada con causa explícita y `success: true` |
| TQM-CA2 | El guard se adquiere siempre que `pbi_ref` resuelva, **con independencia de que exista `correlation_id`** |
| TQM-CA3 | La clave es estable ante variaciones sintácticas del mismo PBI (`./docs/…`, separadores, espacio final) y ante el tránsito `pending/` → `done/` durante el ciclo |
| TQM-CA4 | Cerrar la ventana TOCTOU de D2a: un cerrojo existente pero ilegible/vacío **no** se trata como muerto sin evidencia positiva de defunción |
| TQM-CA5 | El liveness verifica identidad además de existencia de PID (D2b): un PID reciclado no perpetúa un cerrojo huérfano |
| TQM-CA6 | En plataformas sin `/proc` el liveness no degrada en silencio (D2c): o bien mecanismo equivalente, o bien fallo explícito/aviso registrado |
| TQM-CA7 | El descarte emite un evento en familia `orchestration` con **ambos** `correlation_id` (vigente y descartado) y el `pbi_ref` normalizado |
| TQM-CA8 | La evidencia del descarte es **durable**: sobrevive al purgado del router (ver §5.3); no basta con emitir |
| TQM-CA9 | Test unitario: dos adquisiciones sobre el mismo `pbi_ref` con `correlation_id` distintos — la segunda es descarte |
| TQM-CA10 | Test unitario: cerrojo con contenido vacío / PID no parseable no es purgado por un competidor (regresión de D2a) |
| TQM-CA11 | Test de regresión de alcance (§3.4): si el hijo se detacha, el diseño no libera el cerrojo antes de tiempo o el test falla ruidosamente |
| TQM-CA12 | Smoke: dos eventos de dominio consecutivos sobre el mismo PBI levantan un único `cursor-agent` |

## 5. Notas de implementación

### 5.1. Clave del guard — corrección de la premisa

No existe en el Core ninguna "función de resolución de rutas canónicas" a la que
delegar la normalización de un `pbi_ref`. `resolve_documentation_features_path` y
`resolve_documentation_fixes_path` resuelven **directorios de configuración**
(`paths.featurePath`, `paths.fixPath`) desde `cumulo.paths.json`; no normalizan la
referencia a un PBI concreto. Lo que sí existe es el helper `normalize_rel`
(`trim` + `\`→`/` + strip `./`), **duplicado en cuatro módulos**
(`engine/eda_bus.rs`, `sddia-daemon-runtime/eda_bus.rs`, `eda_bus_topology.rs`,
`kalma2-bridge`). La delegación correcta es a esa convención, preferiblemente
extrayéndola a un único punto en lugar de crear la quinta copia.

`fs::canonicalize` queda **descartado**: falla si el fichero no existe, resuelve
symlinks y devuelve rutas absolutas del host, lo que rompe la portabilidad entre
worktrees y hace que el mismo PBI produzca claves distintas en cada copia.

El `pbi_ref` no es utilizable como nombre de fichero (contiene `/`, espacios, `[`,
`]`, `—`). La clave debe ser un digest hexadecimal estable —hay precedente con
`canonical_hash` (SHA-256) en `sddia-evolution-register`— sobre la ruta ya
normalizada y con rechazo previo de `..`.

Para TQM-CA3, la ruta es un identificador frágil: el PBI migra de
`docs/todos/pending/` a `docs/todos/done/` dentro del propio ciclo. Preferir el
`document_id` (o el `uuid`) del frontmatter cuando el cuerpo sea legible —
`load_pbi_body` + `extract_fm_string` ya lo permiten sin código nuevo— y usar el
hash de la ruta normalizada solo como fallback.

### 5.2. Liveness

Trabajar sobre `lock_pid_alive`, no reimplementarlo. Para D2a, escribir el contenido
del cerrojo de forma atómica (fichero temporal + `rename`) o exigir evidencia positiva
de defunción antes de purgar. Para D2b, sellar junto al PID un discriminante estable
del proceso (p. ej. `starttime` de `/proc/{pid}/stat`, o el propio `execution_id`
verificable). Para D2c, aislar la comprobación tras una función con implementación
por plataforma y registrar explícitamente la degradación cuando no exista mecanismo.

### 5.3. Taxonomía del evento de descarte

La familia `orchestration` es la correcta y hay precedente directo:
`thermodynamic.rs` ya emite en esa familia con `emitter_agent: "task-queue-manager"`.
El descarte es una decisión de la línea de montaje táctica, no un fallo de sistema
(`domain`) ni una métrica cruda (`telemetry`).

**Advertencia que invalida la solución ingenua**: `route_orchestration_event` invoca
`route_fractal_event` con `purge_after: true`. Si el `event_type` no tiene
suscriptores en `event-orchestration-subscriptions.json`, el router retorna
`success: true` y **borra el evento** — el descarte quedaría sin rastro, cumpliendo
TQM-CA7 pero incumpliendo TQM-CA8. No hay riesgo de dead-letter (la rama sin
suscriptores retorna antes), pero sí de evaporación silenciosa. Se requiere, por
tanto, o bien un suscriptor que persista la proyección (patrón
`persist-pec-correlation-proof` de Cúmulo), o bien registro durable fuera del bus.

Toda clase ECST nueva debe declararse como entidad `{name}.md` con `uuid` en
`SddIA/events/orchestration/` y quedar catalogada en su `index.md` (mantenido por
Cúmulo) conforme a `events-contract`. Evaluar antes si `Process_Execution_Completed`
con payload discriminado cubre el caso sin ampliar el catálogo.

### 5.4. Defensa en profundidad

El guard `persist-execution-id-conflict` de `agent_runtime` cubre el conflicto a nivel
de fase, pero actúa demasiado tarde: el `cursor-agent` de pago ya se invocó. Se
mantiene como segunda barrera, no como sustituto.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
