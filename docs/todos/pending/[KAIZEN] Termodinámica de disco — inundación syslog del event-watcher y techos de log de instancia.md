---
document_id: PBI-KAIZEN-DISK-THERMODYNAMICS-SYSLOG
uuid: "<generar UUID v4 al materializar>"
title: "[KAIZEN] Termodinámica de disco — inundación syslog del event-watcher y techos de log de instancia"
format: markdown
version: "1.0.0"
created: "2026-09-11"
status: propuesta
refinement_status: unrefined
priority: alta
type: kaizen
process: feature
dispatch: false
suggested_branch: feat/kaizen-disk-thermodynamics-syslog
persist_ref_suggested: docs/features/kaizen-disk-thermodynamics-syslog
depends_on: []
related:
  - SddIA/daemons/event-watcher.md
  - SddIA/daemons/event-watcher/src/main.rs
  - SddIA/tools/ephemeral-cache-purger.md
  - docs/todos/done/[FEATURE] Tool: ephemeral-cache-purger (Saneamiento Termodinámico).md
  - docs/todos/done/PBI-FIX-BUCLE-FANTASMA-SISTEMA-NERVIOSO.md
spawned_by: auditoria-host-20260911-racso-mint-portatil
---

### [KAIZEN] Termodinámica de disco — inundación syslog del event-watcher y techos de log de instancia

#### 1. Origen

Host `racso-mint-portatil` (2026-09-11): filesystem raíz **58 G / 100 % / 0 B libres**. `/var/log` ≈ **32 G**.

| Artefacto | Tamaño | Hecho |
|-----------|--------|--------|
| `/var/log/syslog` | ~20 G | mtime hoy; crecimiento continuo |
| `/var/log/syslog.1` | ~7,8 G | `delaycompress` semanal, sin comprimir |
| journald | ~4 G | `SystemMaxUse` comentado (sin techo) |
| `ephemeral-cache-purger` | n/a | jail `/tmp/cursor-sandbox-cache`; **veto `/var`** |

Cola de `syslog`: `event-watcher.sh[pid]: [WATCHER] skip routed-ok pending file uuid=…` a cadencia de **varias líneas/ms**. Fuente en Core: `println!` incondicional en `SddIA/daemons/event-watcher/src/main.rs` (~L505–509 y ~L580–584) cuando `watcher_skip_reason` devuelve `routed-ok` / `in-flight` / `fractal-terminal`. systemd/rsyslog captura stdout del unit.

Esto **no** es deuda de la tool de cache Cursor. Ampliar su allowlist a `/var/log` es **error crítico de arquitectura** (atomicidad, Core vs instancia, Táctica del Refugio, escalada de privilegios). Laudo **L-NO-PURGER-VAR**.

Emergencia de truncado (`syslog`/`syslog.1`/`journalctl --vacuum-size`) = **acto del Vértice Biológico en el host**, fuera de este PBI. Sin Slice A el disco vuelve a 100 % en horas.

#### 2. Alcance

Tres slices. Un PR de Core para A; B/C no mutan jail del purger.

**Slice A — Core (genoma `event-watcher`, proceso `feature` o `bug-fix` según Dedalo):**  
Hot path de skip rutinario **no** escribe a stdout/stderr capturable por journal/syslog.

- `routed-ok`, `in-flight`, `fractal-terminal`: silencio por defecto, o contador agregado **acotado** (p.ej. 1 línea/N segundos o 1 línea/ciclo con `skipped_n`), nunca 1 línea por UUID de pending.
- `max attempts` / fallos reales: conservan traza (o pasan a `tracing` nivel `warn` con rate-limit).
- Tests: hot path de skip no emite `println` por evento; lote físico / detectado nuevo evento intactos.
- No reabrir el bucle fantasma (`routed_ok` + pending en disco): se silencia el **log**, no el **skip**.

**Slice B — Instancia (no Core):** techos de host documentados en runbook de instancia / starter-kit, **inyectables**, cero paths Linux Mint en el genoma:

- journald `SystemMaxUse` (p.ej. 200–500 M).
- logrotate de rsyslog: `size`/`maxsize` además de `weekly`; comprimir `syslog.1` sin esperar 7 días.
- Unit `sddia-daemon@event-watcher`: `LogRateLimitIntervalSec` / `LogRateLimitBurst` como red de seguridad, no como SSOT (SSOT = Slice A).

**Slice C — Fuera / kitchen distinto:** tool nueva de higiene (`truncate`/`vacuum`, allowlist, no `remove_dir_all`, no uid 0 implícito). **Prohibido** v1.1 de `ephemeral-cache-purger`. Si se desea, PBI hermano; no es gate de este Kaizen.

#### 3. Restricciones

- L-NO-PURGER-VAR: no quitar `/var` de `VETO_PREFIXES`; no ensanchar `ALLOW_RE`.
- Agnosticismo Core: cero `/var/log/syslog` cableado en tools/skills/daemons de librería.
- Refugio: no `rm -rf /var/log`; forense mínimo = rotación/truncado operador + vacuum journal.
- DA-2: crate `event-watcher` ya es nativo; el cambio es I/O de traza, no cápsula nueva.
- Fire-and-forget: no aplica a journald del host.

#### 4. Criterios de aceptación (borrador)

- [ ] **KA-DISK-1** Grep/test: `skip routed-ok` / `in-flight` / `fractal-terminal` no hacen `println!` por UUID en el bucle de poll.
- [ ] **KA-DISK-2** Skip semántico intacto: UUID en `routed_ok` con fichero pending sigue `continue` (regresión bucle fantasma = FAIL).
- [ ] **KA-DISK-3** `max attempts` y errores de ruta siguen observables.
- [ ] **KA-DISK-4** Spec del persist_ref declara L-NO-PURGER-VAR; diff **no** toca `ephemeral-cache-purger` ni `purge_sandbox_cache.rs`.
- [ ] **KA-DISK-5** (instancia, no gate merge Core): runbook con `SystemMaxUse` + logrotate `size`; verificación en host es laboratorio, no CI.
- [ ] **KA-DISK-6** Cierre documental en rama: `validacion.md` APTO + PBI en `docs/todos/done/` en el mismo PR.

#### 5. Actuación inmediata (fuera de sprint, Racso)

Disco a 0 B libres. Antes de compilar:

1. Truncar/rotar `syslog` + `syslog.1` (~28 G).
2. `journalctl --vacuum-size=200M`.
3. Tras Slice A, rebuild/restart `event-watcher`.

Sin (1)+(2) el ciclo `feature` puede colapsar (git/cargo no escriben).
