# Informe de Auditoría Post-Merge y Remediación Operativa — `MAN-50`, `MAN-49` y `MAN-43`

- **PBIs auditados:**
  - `MAN-50` (`PBI-91b7ec84`, PR #456, cierre definitivo de `MAN-43` / `MAN-49`)
  - `MAN-49` (`PBI-7e574af7` / `PBI-2e55b337`, PR #455, padre `MAN-43`)
  - `MAN-43` (`PBI-0f4f0f5f`, PR #454, épica `MAN-40` / `MAN-32`)
- **Rama / Commit auditado:** `main` @ `b1376f215188352fc74b3db31fb6cf52e85eb29f`
- **Fecha de auditoría y remediación:** `2026-11-20`
- **Dictamen de auditoría (previo a remediación operativa):**
  - **`MAN-50`:** **CUMPLIDO EN CÓDIGO / INCUMPLIDO EN RUNTIME (`AC-2`, `AC-5`)** — `.events/` fue recreado en raíz 1 minuto después del `delivery-close-cycle` y rompió `eda_event_token_hygiene_repo_root_no_bus_topology_dirs` en local.
  - **`MAN-49`:** **CUMPLIDO EN CÓDIGO / PENDIENTE DESPLIEGUE** — el código de `sddia-daemon-runtime` estaba corregido en `main`, pero los 5 servicios `systemd --user` seguían ejecutando binarios pre-`MAN-49`.
  - **`MAN-43`:** **CUMPLIDO EN CORE / ABIERTO EN RUNTIME + 3 FOCOS PERIFÉRICOS**.
- **Dictamen tras remediación operativa inmediata en el host (`2026-11-20 11:30 CET`):**
  - **`MAN-50` y `MAN-49`:** **CERRADOS OPERATIVAMENTE EN EL HOST** (`cargo build --workspace --release`, reinicio de los 5 servicios `systemd --user`, purga de `.events/`, `9/9` tests de higiene en verde y `0` violaciones en gates QA).
  - **`MAN-43`:** **CUMPLIDO EN CORE Y RUNTIME DEL HOST**, con **3 focos periféricos** fuera del alcance de `MAN-49`/`MAN-50` documentados en la § 4.

---

## 1. Forense de la Recreación de `.events/` (`2026-11-20 10:42:51.620`)

### 1.1 Cronología exacta
1. **Antes del merge (`10:41:23`):** la raíz del repositorio estaba limpia (`ROOT_CLEAN`) tras el build y la suite de pruebas de `MAN-50`.
2. **Apertura de PR #456 (`10:42:00`–`10:42:34`):** `delivery-close-cycle` (`feat/man-50-cierre-man-43`) publica la rama y emite `PullRequest_Presented`.
3. **Detección por daemon obsoleto (`10:42:48`–`10:42:51`):** según `journalctl --user`, el servicio `sddia-github-bridge-watcher@home-racso-Proyectos-SddIA.service` (**PID `26733`**, en ejecución desde `2026-11-16 19:04:20 CET`) intercepta la PR:
   - `10:42:48`: `[GITHUB-BRIDGE] Procesando PR remoto: https://github.com/racso80es/SddIA/pull/456`
   - `10:42:51`: ejecuta `operation=github_materialize` desde el binario `SddIA/target/release/github-bridge-watcher` (compilado el **`2026-11-16 18:53:25`**, previo a `MAN-49` y `MAN-50`).
4. **Materialización física (`10:42:51.620459525 +0100`):** dicho binario antiguo invoca la versión pre-`MAN-49` de `ensure_eda_bus` (`fs::create_dir_all`) y recrea en la raíz del workspace los 7 subdirectorios vacíos:
   - `.events/pending`
   - `.events/processing` y `.events/processing/subscribers`
   - `.events/processed` y `.events/processed/subscribers`
   - `.events/dead-letter` y `.events/dead-letter/subscribers`

### 1.2 Causa raíz de la regresión operativa
En `MAN-50` (`B-02`) solo se detuvo `event-watcher` (PID `26551`) y únicamente se recompilaron 4 binarios (`sddia-workspace-server`, `event-watcher`, `execute-process`, `sddia-qa` debug). Permanecían activos **5 servicios `systemd --user`** arrancados el `2026-11-16 19:04:20 CET` con binarios `--release` obsoletos y contención sobre `events.redb`:

| Servicio `systemd --user` | PID previo | En memoria desde | Timestamp binario previo | Impacto observado |
| :--- | :--- | :--- | :--- | :--- |
| `sddia-github-bridge-watcher@home-racso-Proyectos-SddIA.service` | `26733` | `2026-11-16 19:04:20` | `2026-11-16 18:53:25` | Recreador directo de `.events/` al procesar PR #456 |
| `sddia-email-watcher@home-racso-Proyectos-SddIA.service` | `26634` | `2026-11-16 19:04:20` | `2026-11-17 16:20:11` | Binario pre-`MAN-49` + contención `events.redb` (`outcome=locked`) |
| `sddia-telegram-watcher@home-racso-Proyectos-SddIA.service` | `26676` | `2026-11-16 19:04:20` | `2026-11-17 16:20:12` | Binario pre-`MAN-49` + contención `events.redb` (`outcome=locked`) |
| `sddia-kalma2-bridge@home-racso-Proyectos-SddIA.service` | `26598` | `2026-11-16 19:04:20` | `2026-11-17 16:20:23` | Binario pre-`MAN-49` + contención `events.redb` (`outcome=locked`) |
| `sddia-iota-publish-relay@home-racso-Proyectos-SddIA.service` | activo | `2026-11-16 19:04:20` | `2026-11-16 18:53:18` | Binario pre-`MAN-49` |

Además permanecían sin recompilar en `SddIA/target/release/`: `sddia-qa` (`2026-11-16`), `event-bus-audit` (`2026-11-08`), `event-sweeper` (`2026-10-21`) y `transit-event-payload` (`2026-09-28`).

### 1.3 Fallo reproducido en `main` antes de remediar
Al ejecutar el test de regresión introducido por `MAN-50` sobre `main @ b1376f21`:
- `cargo test -p execute-process --lib eda_event_token_hygiene` → **8 passed; 1 failed**
- Fallo: `engine::eda_event_token_hygiene::tests::eda_event_token_hygiene_repo_root_no_bus_topology_dirs` en `SddIA/engine/execute-process/src/engine/eda_event_token_hygiene.rs:229`:
  `raíz contaminada: existe directorio de topología \`.events/\``
- (CI en GitHub pasó porque el runner es efímero y no ejecuta los servicios `systemd --user` del host.)

---

## 2. Contraste Histórico contra Todas las Auditorías Previas

| Auditoría / Sello | Veredicto declarado entonces | Contraste real frente a `main @ b1376f21` |
| :--- | :--- | :--- |
| `docs/audits/man-32-lineage-man-43-remediation-20261111.md` | `NO_APTO_TOTAL` (`F-01`..`F-12`) en H1; re-etiquetado a `APTO_TOTAL` en O15 | **Cerrado en código:** el último rastro de `F-03` (6 writers de fallback dead-letter) quedó migrado a `EventManagerPort` en `MAN-50`. |
| `docs/audits/man-43-o6-remediation-20261113.md` | Inconsistencia interna (`NO_APTO_TOTAL` en cuerpo vs `APTO_TOTAL` en frontmatter) | **Cerrado:** `cumulo.paths.json` sin rutas FS `./.events/*` y referencia documental limpiada en `MAN-50` (solo persiste un comando `rg` histórico en tabla de evidencia L37). |
| `docs/audits/man-43-o14c-reaudit-ac13-20261116.md` / `man-43-o14d-reaudit-ac13-20261116.md` | `APTO_TOTAL_PROPUESTO` (`19/19 PASS`) | **Cerrado en código / violado en host antes de esta remediación:** `O14d` tuvo falso negativo sobre `".events"` sin barra; tras `MAN-50` el código cumple, pero los daemons del host recrearon `.events/` hasta su reinicio. |
| `docs/audits/man-43-post-merge-audit-20261117.md` | `CUMPLIMIENTO PARCIAL — NO APTO_TOTAL` (`A-01`..`A-04`) | **Cerrado en código** vía `MAN-49` (`R-01`..`R-04`) + `MAN-50` (`B-01`, `B-03a`, `B-03b`). |
| `docs/audits/man-49-man-43-post-merge-audit-20261120.md` | Brechas `B-01`, `B-02`, `B-03a`, `B-03b` | **`B-01`, `B-03a`, `B-03b`: CERRADOS.** **`B-02`: REABIERTO** por daemons/binarios stale en host; **CERRADO** tras la remediación de la § 5. |
| `docs/features/man-49-remediacion-man-43/validacion.md` | `global: APTO` (Argos, PR #455) | Gobernanza correcta (`L-ARGOS-SEAL`), pero no cubrió `B-01`..`B-03` ni el reinicio completo de daemons `systemd`. |
| `docs/features/man-50-cierre-man-43/validacion.md` | `global: APTO`, `ROOT_TOPOLOGY_CLEAN: APTO` (Argos, PR #456) | **Desmentido en tiempo real:** el evento `PullRequest_Presented` de PR #456 disparó `github-bridge-watcher` (PID `26733`) y recontaminó la raíz en el mismo minuto. Además Argos dejó `F3_TECH_GATE: NO_APTO` y `DOC_HONESTY_AC5_MATRIX: NO_APTO` como `non_blocking_findings`. |

---

## 3. Cumplimiento Detallado por PBI en `main @ b1376f21`

### 3.1 `MAN-50` (`docs/features/man-50-cierre-man-43/`)
- **`AC-1` (`B-01` — 6 escritores `write_*_dead_letter` + eliminación fallback `".events"`):** **CUMPLIDO.**
  - `SddIA/engine/execute-process/src/engine/eda_bus_topology.rs:65-110` usa `bus_defaults_empty()` por defecto; eliminado `bus_defaults_from_root(".events")`.
  - Los 6 `write_*_dead_letter` (`capability_di_gate.rs`, `capability_di_resolver.rs`, `capability_di_output_validator.rs`, `cerbero_di_rbac.rs`, `cerbero_di_envelope.rs`, `route_domain_core.rs`) enrutan vía `EventManagerPort`.
- **`AC-2` (`B-02` — purga `.events/`, recompilación binarios/daemons y blindaje):** **INCUMPLIDO inicialmente en host → CERRADO tras remediación (§ 5).**
- **`AC-3` (`B-03a` — `integration_src = []`):** **CUMPLIDO.**
  - `SddIA/tools/sddia-qa/allowlists/eda-unit-backend.toml:33` tiene `integration_src = []`.
- **`AC-4` (`B-03b` — 6 referencias documentales secundarias):** **CUMPLIDO.**
  - Limpiadas en `.antigravity/rules/[ARQUITECTURA] constitution_core.md:32`, `SddIA/events/events-contract.md:19`, los 4 procesos códex (`accept-pr.md`, `delivery-close-cycle.md`, `pull-request-review.md`, `refine-item.md`) y `docs/audits/man-43-o6-remediation-20261113.md`.
- **`AC-5` (suite workspace + gates + honestidad):** **CUMPLIDO en gates (`0/216` y `0/203`); test local restaurado a `9/9` tras remediación.**
  - Observación de alcance: `MAN-50` incluyó modificación en `.SddIA/project.md` (`mcp.allowed_executables`), aceptada explícitamente antes del merge.

### 3.2 `MAN-49` (`docs/features/man-49-remediacion-man-43/`)
- **`R-01`..`R-04` (`AC-1`..`AC-5`):** **CUMPLIDOS en código** (`SddIA/sddia-daemon-runtime/src/eda_bus.rs:153-155` y `eda_bus_topology.rs:179` sin `fs::create_dir_all`), y efectivos en runtime tras reiniciar los 5 servicios `systemd --user`.

---

## 4. Focos Periféricos Residuales de `MAN-43` (Fuera de `MAN-49`/`MAN-50`)

Detectados durante la inspección de código en `main`:
1. **`SddIA/engine/execute-process/src/engine/handlers/instance_creator.rs:603-606`:** al aprovisionar una instancia hija sigue creando físicamente `instance_root.join(".events")` con `["pending", "domain", "orchestration", "telemetry", "dead-letter"]`.
2. **`SddIA/tools/transit-event-payload/src/main.rs:26-57`:** herramienta CLI legacy con defaults `.events/{pending,processing,processed,dead_letter}` y `fs::create_dir_all`.
3. **`SddIA/scripts/qa/git-hooks/hook_common.sh:312-332`:** `eda_bus_dirs()` conserva defaults de lectura `.events/{pending,processing,processed}` (solo inspecciona si los directorios existen; no los crea).

---

## 5. Remediación Operativa Ejecutada en el Host (`2026-11-20 11:30 CET`)

1. **Compilación completa `--release` del workspace:**
   - `cd SddIA && cargo build --workspace --release` → `Finished 'release' profile [optimized] target(s) in 19m 09s` (exit `0`). Todos los binarios de `SddIA/target/release/` quedaron actualizados con el código de `main @ b1376f21`.
2. **Reinicio de los 5 servicios `systemd --user` y purga de `.events/`:**
   - Reiniciados vía `systemctl --user restart`:
     - `sddia-github-bridge-watcher@home-racso-Proyectos-SddIA.service` → `active (running)` (PID `1955879`)
     - `sddia-email-watcher@home-racso-Proyectos-SddIA.service` → `active (running)` (PID `1955866`)
     - `sddia-telegram-watcher@home-racso-Proyectos-SddIA.service` → `active (running)` (PID `1955893`)
     - `sddia-kalma2-bridge@home-racso-Proyectos-SddIA.service` → `active (running)` (PID `1955885`)
     - `sddia-iota-publish-relay@home-racso-Proyectos-SddIA.service` → `active (running)` (PID `1955897`)
   - Purgado `/home/racso/Proyectos/SddIA/.events` y verificado que no reaparece (`ls: no se puede acceder a '.events': No existe el archivo o el directorio`).
3. **Verificación post-remediación en verde:**
   - `cargo test -p execute-process --lib eda_event_token_hygiene` → **`ok. 9 passed; 0 failed; 0 ignored`** (incluyendo `eda_event_token_hygiene_repo_root_no_bus_topology_dirs`).
   - `./target/release/sddia-qa gate-event-manager-port --json` → `{"ok":true,"scanned_files":216,"violation_count":0,"violations":[]}`.
   - `./target/release/sddia-qa gate-eda-unit-no-real-backend --json` → `{"ok":true,"scanned_files":203,"violation_count":0,"violations":[]}`.
