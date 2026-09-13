---
document_id: PBI-KAIZEN-DISK-THERMODYNAMICS-SYSLOG
uuid: "9454e7eb-f397-4e36-abaf-91fd91cba802"
title: "[KAIZEN] Termodinámica de disco — inundación syslog del event-watcher y techos de log de instancia"
format: markdown
version: "1.2.0"
created: "2026-09-11"
updated: "2026-09-13"
status: en_pr
refinement_status: refinado
persist_ref: docs/features/kaizen-disk-thermodynamics-syslog
branch_name: feat/kaizen-disk-thermodynamics-syslog
priority: alta
type: kaizen
process: feature
dispatch: false
suggested_branch: feat/kaizen-disk-thermodynamics-syslog
persist_ref_suggested: docs/features/kaizen-disk-thermodynamics-syslog
depends_on: []
spawned_by: auditoria-host-20260911-racso-mint-portatil
refinement_notes: >-
  v1.2.0 Refinado (2026-09-13, Filtro A sobre v1.1.0). Correcciones:
  (6) `dead-letter kaizen` ya es silencioso en el skip del poll; no es vector de fuga.
  (7) `fractal-terminal` no se produce en el bucle batch (`process_name == route-domain-event`).
  (8) Números de línea: skip I/O en L505–512 (batch) y L580–587 (fractal), no L480–518 / L559–590.
  (9) `log_route_outcome` / `apply_route_outcome_fractal` son trazas one-shot de despacho, no del tick.
  (10) `LogRateLimit*` en la fábrica aplica a todos los daemons factory, no solo event-watcher;
  la unidad viva no hereda hasta regenerar o sincronizar `.SddIA/systemd/`.
  (11) Citar DA-2 como veto de `tracing` es inexacto: `directories.daemons` no está en la tabla DA-2;
  el veto es A-NO-TRACING-DEPENDENCY.
  (12) KA-DISK-8 faltaba en gates_this_wave. Laudo: cero println en cualquier skip del poll,
  incluida la traza opcional de max attempts.
  v1.1.0: unidad canónica sddia-event-watcher@; sin tracing; dos bucles; L-NO-PURGER-VAR.
architectural_constraints:
  - L-NO-PURGER-VAR
  - A-EVENT-WATCHER-NO-HOT-PATH-PRINTLN
  - A-UNIT-TEMPLATE-NAMING-SSOT
  - A-NO-TRACING-DEPENDENCY
  - A-CORE-HOST-PATH-AGNOSTIC
gates_this_wave:
  - KA-DISK-1
  - KA-DISK-2
  - KA-DISK-3
  - KA-DISK-4
  - KA-DISK-5
  - KA-DISK-6
  - KA-DISK-7
  - KA-DISK-8
related:
  - SddIA/daemons/event-watcher.md
  - SddIA/daemons/event-watcher/src/main.rs
  - SddIA/templates/systemd/sddia-daemon@.service.template
  - .SddIA/systemd/sddia-event-watcher@.service
  - SddIA/tools/ephemeral-cache-purger.md
  - SddIA/tools/ephemeral-cache-purger/src/jail.rs
  - SddIA/engine/execute-process/src/engine/purge_sandbox_cache.rs
  - docs/todos/done/[FEATURE] Tool: ephemeral-cache-purger (Saneamiento Termodinámico).md
  - docs/todos/done/PBI-FIX-BUCLE-FANTASMA-SISTEMA-NERVIOSO.md
  - docs/features/kaizen-ignicion-soberana-centinelas/clarify.md
  - docs/todos/pending/[DEUDA] Paciente 0 — prompt y proceso de despliegue.md
---

### [KAIZEN] Termodinámica de disco — inundación syslog del event-watcher y techos de log de instancia

#### 0. Errata, alucinaciones previas y clarificaciones técnicas (no reintroducir)

| Afirmación previa / Asunción | Veredicto | Evidencia y Hecho Técnico |
|---|---|---|
| Unidad `sddia-daemon@event-watcher` con `LogRateLimit*` | **Alucinación de nombre de unidad (rompe F-08).** | Prohibido en arquitectura (`docs/features/kaizen-ignicion-soberana-centinelas/clarify.md`). Usar `%i=event-watcher` deja `%f` vacío. Unidad template canónica: `sddia-event-watcher@.service` (fábrica `sddia-daemon@.service.template` vía `instance-creator`). En este host: `sddia-event-watcher@home-racso-Proyectos-SddIA.service`. |
| Migrar logs a `tracing` con nivel `warn` y rate-limit | **Alucinación de dependencia.** | `SddIA/daemons/event-watcher/Cargo.toml` solo enlaza `sddia-daemon-runtime` y `serde_json`. Los centinelas escriben a stdout/stderr. El veto es **A-NO-TRACING-DEPENDENCY**. `directories.daemons` **no** está en la tabla DA-2; no citar DA-2 como prohibición de `tracing`. |
| `max attempts` en el poll loop debe conservar traza libre | **Inexactitud / fuga latente.** | `POLL_SECONDS = 2`. Si el skip imprime en cada tick, cada fichero estancado emite 30 líneas/min. El fallo de subproceso ya va por `eprintln!` en `apply_route_outcome_*`. **Laudo:** cero I/O de skip en el hot path, incluida la traza «opcional» de max attempts. |
| Un único punto de fuga en `main.rs` | **Inexactitud de alcance.** | Dos bucles en `run_watcher`: (1) lote físico `route-domain-event` (skip I/O **L505–512**); (2) despacho unitario fractal (skip I/O **L580–587**). Ambos replican el `println!` de skip. |
| Rangos L480–518 / L559–590 como locus del skip I/O | **Inexactitud de línea (v1.1.0).** | Esos rangos cubren el bucle completo (cupo in-flight + elegibilidad + skip). El I/O térmico está en L505–512 y L580–587. |
| `dead-letter kaizen` inunda syslog en cada tick | **Inexactitud.** | `watcher_skip_reason` puede devolver `"dead-letter kaizen"` en el bucle batch, pero el `println!` del skip **no** cubre esa cadena (solo `in-flight`, `routed-ok`, `fractal-terminal`, `max attempts`). El skip ya es silencioso. Hay un `println!` **one-shot** en `log_route_outcome` (`testigo dead-letter`). No es el vector térmico. KA-DISK-1 lo trata como no-regresión. |
| `fractal-terminal` se emite en el bucle batch | **Inexactitud.** | `watcher_skip_reason` solo retorna `fractal-terminal` si `process_name != "route-domain-event"`. El lote batch nunca toma esa rama. El silencio de `fractal-terminal` aplica al bucle fractal (KA-DISK-2). |
| Cobertura de tests existente para skips | **Inexactitud.** | El crate no declara tests. Deben introducirse tests nativos. |
| `LogRateLimit*` en la fábrica se aplica solo a event-watcher y la unidad viva lo hereda al merge | **Inexactitud / omisión.** | La fábrica `sddia-daemon@.service.template` la consume `instance-creator` para **todos** los daemons factory (`event-watcher`, `event-sweeper`, `kalma2-bridge`, `telegram-watcher`, …). `sddia-email-watcher@.service.template` es plantilla aparte (fuera de KA-DISK-7). La unidad chequeada `.SddIA/systemd/sddia-event-watcher@.service` **no** se regenera con el merge del template; hay que sincronizarla en este PR o documentar drop-in + `daemon-reload`. |
| `log_route_outcome` / éxito fractal son el mismo vector que el skip del poll | **Inexactitud.** | Esas trazas se emiten **una vez por despacho**, no cada 2 s. Fuera de Slice A salvo que un laudo posterior las silencie. Prohibido «arreglarlas» en este ciclo. |

---

#### 1. Origen y Evidencia Forense

Host `racso-mint-portatil` (2026-09-11 a 2026-09-13): filesystem raíz `/dev/nvme0n1p2` (58 G) colapsó a **0 B libres** (100 % de uso). Tras contención de emergencia, `/var/log` permanece como foco térmico crítico (>20 G acumulados en rotaciones).

| Artefacto | Tamaño registrado | Hecho Técnico |
|---|---|---|
| `/var/log/syslog` | ~3,6 G – 20 G | Crecimiento ininterrumpido a varios MB/minuto mientras el centinela esté activo. |
| `/var/log/syslog.1` | ~7,8 G – 14 G | Retenido sin comprimir por la directiva `delaycompress` semanal de Debian/Ubuntu/Mint. |
| journald | ~4 G | `SystemMaxUse` no acotado en `/etc/systemd/journald.conf` (ocupa hasta el 10 % del disco por defecto). |
| `ephemeral-cache-purger` | n/a | Enjaulado en `/tmp/cursor-sandbox-cache`; **veto inmutable `/var`** (Laudo L-NO-PURGER-VAR). |

**Mecanismo de la fuga térmica:**
1. En `.events/pending/` (y homólogos fractales), ciertos eventos permanecen en disco esperando consenso o permanecen tras `routed_ok`.
2. Cada ciclo (`POLL_SECONDS = 2`) el daemon `event-watcher` escanea los `.json` de `watch_targets`.
3. Si el UUID está en `book.routed_ok` y el fichero sigue en disco, `watcher_skip_reason` retorna `Some("routed-ok pending file uuid=...")`. Análogos: `in-flight`, `fractal-terminal` (solo no-batch), `max attempts`.
4. En L505–512 (batch) y L580–587 (fractal) el watcher ejecuta, según prefijo:
   ```rust
   println!("[WATCHER] skip {skip}");           // in-flight | routed-ok | fractal-terminal
   println!("[WATCHER] Skip {key}: {skip}");    // max attempts
   ```
   y `continue` (el archivo no entra en `eligible` / no se despacha).
5. Cota: N ficheros que **sí** entran en esas ramas de print × 0,5 Hz. Ejemplo: 42 ficheros `routed-ok` → 21 líneas/s (~1,8×10⁶/día). Ficheros solo `dead-letter kaizen` **no** suman a esa cota.
6. systemd captura stdout → journald; rsyslog replica a `/var/log/syslog`.

**Invariante de Refugio (Laudo L-NO-PURGER-VAR):**
Esto **no** es un fallo del purger de sandbox Cursor. Modificar `ephemeral-cache-purger` para que limpie `/var/log` sería transgresión de seguridad (trazas forenses del SO, `uid 0` implícito, ruptura de bounded context). Contención: fuente en Core (Slice A) + techos de absorción de instancia (Slice B).

---

#### 2. Alcance por Slices

Tres slices. El PR de Core cubre Slice A y la fábrica systemd de Slice B, más la unidad de instancia versionada en este repo. journald/rsyslog de host = runbook, no genoma.

```
+-------------------------------------------------------------------------+
| SLICE A: Core (event-watcher crate)                                     |
| - Silencio total de skips en hot path de sondeo (ambos bucles)          |
| - Cero I/O periódico de max attempts                                    |
| - Tests unitarios nativos                                               |
| - No mutar log_route_outcome / apply_route_outcome_* println one-shot   |
+-------------------------------------------------------------------------+
                                    |
+-------------------------------------------------------------------------+
| SLICE B: Templates Core + unidad de instancia versionada                |
| - LogRateLimit en sddia-daemon@.service.template (todos los factory)    |
| - Sincronizar .SddIA/systemd/sddia-event-watcher@.service en este repo  |
| - Runbook host: journald SystemMaxUse, logrotate size; drop-in live     |
+-------------------------------------------------------------------------+
                                    |
+-------------------------------------------------------------------------+
| SLICE C: Fuera de este PBI                                              |
| - Veto a mutar ephemeral-cache-purger / purge_sandbox_cache / VETO /var |
| - Plantilla email-watcher aparte                                        |
| - Silencio de log_route_outcome                                         |
+-------------------------------------------------------------------------+
```

##### Slice A — Core (`event-watcher`):
1. **Silenciamiento del hot path:**
   - Bucle batch (`process_name == "route-domain-event"`, skip L505–512) y bucle fractal (skip L580–587):
     - Ante cualquier `Some(skip)` de `watcher_skip_reason`: `continue` **sin** `println!`/`eprintln!`.
     - Cubre `routed-ok`, `in-flight`, `fractal-terminal`, `max attempts` y `dead-letter kaizen` (este último ya silencioso; no-regresión).
2. **Inmunidad al bucle fantasma:**
   - No cambiar el valor de retorno de `watcher_skip_reason` ni el `continue`.
   - El archivo sigue fuera de `eligible` / no se despacha.
3. **Tests unitarios nativos** en `src/main.rs` `#[cfg(test)]`:
   - `watcher_skip_reason`: `in-flight`, `routed-ok`, `fractal-terminal`, `max attempts`, `dead-letter kaizen`.
   - Función pura de política: skip del hot path **nunca** produce traza.
   - `RouteBook.routed_ok` retiene UUID si el fichero sigue en disco.
4. **Fuera:** no alterar mensajes one-shot de `log_route_outcome` ni de éxito/fallo en `apply_route_outcome_fractal`.

##### Slice B — Templates e instancia versionada:
1. En `SddIA/templates/systemd/sddia-daemon@.service.template` `[Service]`:
   ```ini
   LogRateLimitIntervalSec=30s
   LogRateLimitBurst=500
   ```
   Efecto: todos los daemons factory al regenerar con `instance-creator`. Defensa en profundidad (no sustituye Slice A).
2. Sincronizar `.SddIA/systemd/sddia-event-watcher@.service` con las mismas claves (unidad versionada de este repo; sin paths de host en Core).
3. Runbook (persist_ref + Paciente 0, no genoma):
   - journald: `SystemMaxUse=300M`, `SystemMaxFileSize=50M`.
   - logrotate rsyslog: `size 500M` / `maxsize 500M` para syslog.
   - Drop-in + `systemctl --user daemon-reload` + restart de la unidad **activa** del operador (el nombre `…@home-racso-…` es ejemplo de este host, no literal de Core).

##### Slice C — Fuera / veto:
- Prohibido mutar `SddIA/tools/ephemeral-cache-purger/src/jail.rs` y `SddIA/engine/execute-process/src/engine/purge_sandbox_cache.rs`.
- Prohibido quitar `/var` de `VETO_PREFIXES`.
- Herramienta futura de higiene de logs de SO = componente independiente, nunca extensión del purger.

---

#### 3. Restricciones Arquitectónicas

- **L-NO-PURGER-VAR:** Cero relajación de `ALLOW_RE` o `VETO_PREFIXES` en herramientas de sandbox.
- **A-EVENT-WATCHER-NO-HOT-PATH-PRINTLN:** El bucle continuo no debe hacer I/O de consola `O(N)` por tick sobre elementos pasivos.
- **A-UNIT-TEMPLATE-NAMING-SSOT:** Unidad canónica `sddia-event-watcher@.service` (`%f` = root de instancia). Prohibido `sddia-daemon@event-watcher`.
- **A-NO-TRACING-DEPENDENCY:** Cero dependencias nuevas en `event-watcher/Cargo.toml`.
- **A-CORE-HOST-PATH-AGNOSTIC:** Código bajo `SddIA/` sin rutas absolutas de host. Rutas de SO solo en runbook de instancia.

---

#### 4. Criterios de Aceptación

- [ ] **KA-DISK-1 (Silencio bucle batch):** Rama `process_name == "route-domain-event"`: cero stdout/stderr por skip (`routed-ok`, `in-flight`, `max attempts`, `dead-letter kaizen`). `fractal-terminal` no aplica a esta rama.
- [ ] **KA-DISK-2 (Silencio bucle fractal):** Rama unitaria fractal: cero stdout/stderr por skip (`routed-ok`, `in-flight`, `fractal-terminal`, `max attempts`).
- [ ] **KA-DISK-3 (Contención max attempts):** Alcanzar `MAX_ROUTE_ATTEMPTS` no emite `[WATCHER] Skip …` ni `[WATCHER] skip …` en cada tick. Errores de subproceso siguen en `eprintln!` de `apply_route_outcome_*` (one-shot por intento fallido de despacho).
- [ ] **KA-DISK-4 (Skip semántico intacto):** UUID en `book.routed_ok` con fichero en disco no se re-despacha. Sin regresión del bucle fantasma.
- [ ] **KA-DISK-5 (Tests):** `cargo test -p event-watcher` ejecuta y pasa tests de `watcher_skip_reason` y de política de silencio del skip.
- [ ] **KA-DISK-6 (Purger intacto):** Diff del PR no toca `SddIA/tools/ephemeral-cache-purger/` ni `purge_sandbox_cache.rs`. `VETO_PREFIXES` retiene `/var`.
- [ ] **KA-DISK-7 (Template + unidad instancia):** `sddia-daemon@.service.template` y `.SddIA/systemd/sddia-event-watcher@.service` incorporan `LogRateLimitIntervalSec` y `LogRateLimitBurst`.
- [ ] **KA-DISK-8 (Cierre documental en rama):** Cascada bajo `docs/features/kaizen-disk-thermodynamics-syslog/` (`clarify`, `objectives`, `spec`, `plan`, `implementation`, `execution`, `validacion` APTO con CI verde / `run_id`) y PBI en `docs/todos/done/` en el mismo PR. `global: APTO` prohibido sin check CI verde.

---

#### 5. Guía de Actuación Inmediata en el Host (Operador Racso)

Runbook de instancia. No forma parte del genoma Core. El nombre de unidad es el de **este** host.

Host con espacio comprometido y unidad activa `sddia-event-watcher@home-racso-Proyectos-SddIA.service`:

1. Truncar logs activos (no `rm` de `syslog` con el descriptor abierto):
   ```bash
   sudo truncate -s 0 /var/log/syslog
   sudo rm -f /var/log/syslog.1
   sudo systemctl restart rsyslog
   ```
2. Vaciar journald:
   ```bash
   sudo journalctl --vacuum-size=200M
   ```
3. Comprobar espacio:
   ```bash
   df -h /
   ```
4. Tras merge + `cargo build --release -p event-watcher` y unidad sincronizada:
   ```bash
   systemctl --user daemon-reload
   systemctl --user restart sddia-event-watcher@home-racso-Proyectos-SddIA.service
   ```
5. Verificar cese de inundación:
   ```bash
   journalctl --user -u sddia-event-watcher@home-racso-Proyectos-SddIA.service -n 50 -f
   ```
