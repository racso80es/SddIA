---
document_id: PBI-FIX-THERMAL-RESILIENCE-HEARTBEAT-MAYEUTA
title: "[FIX] Resiliencia Térmica en Heartbeat Audit y Poda Ontológica en Mayeuta"
format: markdown
version: "1.1.0"
created: "2026-09-07"
status: "abierto"
priority: alta
process: bug-fix
refined: "2026-09-07"
laudo: A
subsumes:
  - document_id: PBI-FIX-FRACTURE-3b71cec5263b
    process: email-watcher
    trace: "Centinela email-watcher lock huérfano: PID 13215 muerto. last_heartbeat=2026-09-06T06:12:04Z"
  - document_id: PBI-FIX-FRACTURE-a10b5364a1c7
    process: github-bridge-watcher
    trace: "Centinela github-bridge-watcher lock huérfano: PID 13281 muerto. last_heartbeat=2026-09-06T06:12:05Z"
  - document_id: PBI-FIX-FRACTURE-4543ba08b905
    process: iota-publish-relay
    trace: "Centinela iota-publish-relay lock huérfano: PID 13335 muerto. last_heartbeat=2026-09-06T06:12:06Z"
  - document_id: PBI-FIX-FRACTURE-ca6c00385158
    process: telegram-watcher
    trace: "Centinela telegram-watcher lock huérfano: PID 13243 muerto. last_heartbeat=2026-09-06T06:11:27Z"
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - SddIA/process/daemon-heartbeat-audit.md
  - SddIA/actions/enrich-fracture-pbi-kaizen.md
  - SddIA/engine/execute-process/src/engine/handlers/daemon_heartbeat.rs
  - SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs
  - SddIA/evolution/c6931c73-4cfc-4a11-b082-54099d420f59.md
  - docs/fixes/event-sweeper-heartbeat-fracture-f4befd66c513/
  - docs/fixes/mayeuta-heartbeat-kaizen-classifier/
  - docs/fixes/centinelas-fracture-ola-20260901/
  - docs/features/latido-ontologico-vitalidad-organos/
---

# [FIX] Resiliencia Térmica en Heartbeat Audit y Poda Ontológica en Mayeuta

## 0. Errata del planteamiento (Filtro A)

Hallazgos empíricos sobre el texto v1.0.0. Lo que sigue es mandato; lo tachado aquí no se implementa.

| Afirmación v1.0.0 | Veredicto | Evidencia |
|-------------------|-----------|-----------|
| «PR #118 introdujo `effective_heartbeat_baseline`» | **Alucinación de referencia.** No hay mención a `#118` fuera de este PBI. | Evolución real: `c6931c73-4cfc-4a11-b082-54099d420f59` (`docs/fixes/event-sweeper-heartbeat-fracture-f4befd66c513`). |
| Baseline «insuficiente ante cortes físicos por exigir latencias incompatibles» | **Inexacto.** El fallo no es un umbral de gracia. | `audit_running_daemon` llama `emit_orphan_lock_fracture` **antes** de baseline, `missed_cycles` y `host_suspend`. |
| Equivalencia `btime` ≡ `CLOCK_BOOTTIME` ≡ `libc::sysinfo` | **Incoherencia de reloj.** | `btime` (`/proc/stat`) es timestamp Unix de arranque (pared). `CLOCK_BOOTTIME` y `sysinfo.uptime` son **duraciones**. Solo el timestamp de pared se compara con `lock.started_at` ISO. |
| «Depurar o ceder el lock» como deber nuevo del auditor | **Duplicación.** | `DaemonRuntime::bootstrap` ya recupera lock huérfano (`pid` muerto → `remove_file`). `daemon-kill-switch` ya lista `stale_locks_removed`. El proceso `daemon-heartbeat-audit` declara «No arranca ni mata Centinelas». |
| «Red teaming del prompt de Mayeuta» | **Alucinación ontológica (ya corregida en v1.0).** | Handler nativo `analyze_fracture_kaizen`; cero LLM. |
| Eslogan nuevo de fallback («Incertidumbre Empírica:…») | **Ruido léxico.** | El fallback vigente ya dice: «Causa raíz no clasificada automáticamente… requiere laudo humano». No se sustituye el copy. |
| CA solo de inhibición pre-boot | **Incompleto.** | El test `orphan_lock_dead_pid_emits_fracture_once` **debe seguir verde**: PID muerto **después** del `btime` actual sigue siendo fractura (contrato `latido-ontologico-vitalidad-organos` L3). |
| «Argos emite APTO» como acto autónomo del centinela | **Inexacto de rol.** | APTO = fase Verificación del ciclo `bug-fix` (`validacion.md`), no un evento del daemon Argos. |

## 1. Contexto y diagnóstico

Cuatro `System_Fracture_Detected` simultáneos (2026-09-06) tras cese físico del host. Trazas verificadas en los PBI Cúmulo (hashes `3b71cec5263b`, `a10b5364a1c7`, `4543ba08b905`, `ca6c00385158`). Emisor `argos`, `attempted_action: daemon-heartbeat-audit`. Patrón:

```text
Centinela {daemon_id} lock huérfano: PID {pid} muerto. last_heartbeat={ISO}
```

Ola previa documental (`centinelas-fracture-ola-20260901`) ya archivó sellos isomorfos como laudo B y dejó explícito: cubo Mayeuta `huérfan` colisiona con esta traza; cubo `orphan_lock` quedó Kaizen aparte. Este PBI es ese Kaizen **más** la puerta de arranque del auditor.

### A. Falso positivo de lock huérfano post-reboot (`daemon-heartbeat-audit`)

Hechos en `daemon_heartbeat.rs`:

1. `effective_heartbeat_baseline = max(last_heartbeat_at, lock.started_at)` (evolución `c6931c73`) solo corre si `pid_alive(pid) == true`. Mitiga carrera de cold-start con PID vivo; **no** cubre lock de sesión OS anterior.
2. Orden actual de `audit_running_daemon`:

```rust
if !pid_alive(pid) {
    return emit_orphan_lock_fracture(...); // ← sin boot_time, sin host_suspend
}
if host_suspend { return Ok(None); }
// baseline / missed_cycles …
```

3. `host_suspend` (skew wall−mono) **ya existe**. Tras reboot, `last_audit_mono_ms` persistido + `saturating_sub` puede marcar `host_suspend=true` y aun así emitir fractura huérfana porque el `pid_alive` precede. No se «añade» host_suspend; no se usa como puerta de reboot.
4. El auditor no lee `btime`. Un lock en `.SddIA/daemons/status/*.lock` con `started_at` anterior al arranque actual y PID ausente del kernel vigente se trata como colapso en caliente.
5. Los launchers **no** necesitan cambio: `sddia-daemon-runtime` ya recupera lock huérfano al `bootstrap`. El defecto es la emisión de deuda ontológica **antes** de que el launcher reescriba el lock.

### B. Secuestro semántico en Mayeuta (`enrich-fracture-pbi-kaizen`)

Hechos en `enrich_fracture_pbi_kaizen.rs`:

1. Cubo `heartbeat_starvation` (`is_heartbeat_starvation_trace`, F-MAYEUTA-HB-BLIND) **no** cubre esta familia: exige `omitió` + `ciclos consecutivos de Daemon_Heartbeat`. La traza huérfana no los contiene.
2. Trampa léxica vigente (`has_any(&["orphan", "ruido de sistema", "eda genómica", "huérfan"])` sobre el blob concatenado). `"huérfan"` ⊂ `"huérfano"` de la traza Argos → causa raíz «Entidad genómica indexada sin correlato `Domain_Entity_Created`». Los cuatro PBI subsumidos lo demuestran.
3. Precedente de clase: F-MAYEUTA-HB-TOKEN-TRAP. Misma disciplina: matcher exclusivo sobre `error_trace`, anclas del `format!` emisor, no tokens unarios sobre el blob.

## 2. Vectores de intervención

### Vector A — Puerta `host_boot` en el auditor (motor, no DA-2)

Handler: `SddIA/engine/execute-process/src/engine/handlers/daemon_heartbeat.rs`. No es genoma.

1. **SSOT de arranque:** leer `btime` de `/proc/stat` → `DateTime<Utc>`. Prohibido tratar `CLOCK_BOOTTIME`/`sysinfo.uptime` como timestamp comparable. Si no hay `/proc/stat` (tests, no-Linux): fail-open configurable — inyectar `boot_time` en tests; en runtime sin `btime`, **no** inhibir (conservar L3).
2. **Predicado `lock_predates_host_boot`:** `parse_iso(lock.started_at)` (fallback: `last_heartbeat_at` del estado o del lock) **<** `boot_time`. Margen de gracia NTP **no** se inventa salvo que un test lo exija con evidencia; default comparación estricta.
3. **Orden:** evaluar `lock_predates_host_boot` **antes** de `emit_orphan_lock_fracture`. Si true:
   - `classification: host_reboot_stale_lock`
   - **prohibido** emitir `System_Fracture_Detected`
   - higiene de fichero: `remove_lock` solo si `!pid_alive` (no SIGKILL; no duplicar `daemon-kill-switch` sobre PIDs vivos). Objetivo: el sweep siguiente no reentra el ramal muerto. El bootstrap del daemon sigue siendo el dueño del ciclo de vida.
4. **PID reuse post-reboot:** si `started_at < boot_time` y `pid_alive` (PID reciclado), el lock **miente**. No emitir fractura; no borrar el lock si el PID vive (evitar matar un proceso ajeno). Clasificar `host_reboot_stale_lock` y dejar que el launcher aborte duplicado o recupere según su contrato.
5. **Invariante L3:** lock + PID muerto + `started_at >= boot_time` → `emit_orphan_lock_fracture` intacto. Test `orphan_lock_dead_pid_emits_fracture_once` verde.

Genoma (`entity-manager`, `entity_class: process`, `daemon-heartbeat-audit`): documentar `classification` real (`orphan_lock` | `host_reboot_stale_lock` además de `healthy` | `host_suspend` | `stale` | `recovered` | `degraded` | `shutting_down`); documentar que la emisión huérfana **no** aplica a locks anteriores a `btime`. Bump SemVer. DA-2.

### Vector B — Cubo `orphan_lock` en Mayeuta (motor + acción)

Handler: `enrich_fracture_pbi_kaizen.rs` (no genoma). Acción: bump vía `entity-manager`.

1. **`is_orphan_lock_trace(error_trace)`** — match **solo** `error_trace`, anclas del `format!` de `emit_orphan_lock_fracture`: `Centinela `, `lock huérfano`, `PID `, `muerto`, `last_heartbeat=`.
2. Si true: causa = lock de centinela con PID muerto / ciclo de vida de daemon (no EDA genómica). Veredicto `refactor_tool` o `process_fix` centrado en supervisión de procesos. Evaluar **antes** del cubo `orphan|huérfan` genómico y del catch-all `timeout|block|abort|failed|colaps`. No usar tokens `orphan`/`huérfan` sobre el blob concatenado para este cubo.
3. **Cubo EDA genómica acotado:** el `has_any(["orphan","huérfan",…])` global se sustituye por firma que exija contexto genómico (`audit-entity-eda-coverage` y/o `entity-manager` y/o `Domain_Entity_Created` / `eda genómica`) **y** no sea `is_orphan_lock_trace`.
4. Cubo `heartbeat_starvation` intacto (inanición ≠ lock huérfano).
5. Fallback sin firma exacta: texto vigente de no-clasificada + laudo humano. Sin eslogan nuevo.

## 3. Deuda satélite (falsos positivos)

Unificar y archivar, **sin fusionar `document_id`**:

1. `docs/todos/pending/[FIX] iota-publish-relay — fractura sistémica (4543ba08b905).md`
2. `docs/todos/pending/[FIX] email-watcher — fractura sistémica (3b71cec5263b).md`
3. `docs/todos/pending/[FIX] github-bridge-watcher — fractura sistémica (a10b5364a1c7).md`
4. `docs/todos/pending/[FIX] telegram-watcher — fractura sistémica (ca6c00385158).md`

En `docs/todos/done/`, mismo `document_id`, `status: invalidado_por_falso_positivo`, `subsumed_by: PBI-FIX-THERMAL-RESILIENCE-HEARTBEAT-MAYEUTA`, `fix_ref` al `persist_ref` de este ciclo. Prohibido backfill EDA sobre estos hashes.

El PBI madre se archiva en el cierre documental del `bug-fix` (mismo PR).

## 4. Criterios de aceptación

- [ ] **CA-1:** `audit_running_daemon` lee `btime` (`/proc/stat`) y **no** llama `emit_orphan_lock_fracture` si `lock.started_at` (o fallback de latido) es anterior al boot actual.
- [ ] **CA-2:** Test: lock con `started_at` anterior a `boot_time` inyectado + PID muerto → `fractures_emitted` vacío y `classification=host_reboot_stale_lock`.
- [ ] **CA-3:** Test existente `orphan_lock_dead_pid_emits_fracture_once` verde (PID muerto post-boot sigue emitiendo una vez).
- [ ] **CA-4:** `is_orphan_lock_trace` clasifica la traza canónica de los cuatro sellos como ciclo de vida de daemon; sección **sin** `Domain_Entity_Created` ni backfill `audit-entity-eda-coverage`.
- [ ] **CA-5:** Traza `orphan`/`huérfano` **sin** anclas Argos de lock + contexto genómico verdadero sigue pudiendo caer al cubo EDA; traza `timeout in worker` + `attempted_action=daemon-heartbeat-audit` **no** entra a `orphan_lock` ni a `heartbeat_starvation`.
- [ ] **CA-6:** `cargo test -p execute-process -- daemon_heartbeat enrich_fracture_pbi_kaizen` verde.
- [ ] **CA-7:** Bump `daemon-heartbeat-audit.md` y `enrich-fracture-pbi-kaizen.md` vía `entity-manager` (DA-2). Handler nativo no se edita a mano en genoma.
- [ ] **CA-8:** Los 4 PBI satélite en `docs/todos/done/` con metadatos de §3; PBI madre en `done/` en el mismo PR; `validacion.md` `pbi_archived: true`.
- [ ] **CA-9:** Checks GitHub del PR verdes **antes** de `accept-pr` (mandato de cierre de este ciclo; no marcar ese check APTO sin `run_id`/URL).

## 5. Fuera de alcance

| Prohibido | Motivo |
|-----------|--------|
| Mutar `missed_cycles_threshold` / `suspend_skew_seconds` para silenciar sellos | No es un problema de umbral |
| Red teaming / prompts / LLM sobre Mayeuta | No hay prompt |
| Cambiar keepalive de centinelas | Los PIDs murieron con el host; no hay inanición en caliente |
| Reescribir `start-sddia.sh` / systemd | Bootstrap ya recupera lock huérfano |
| Fusionar los cuatro `document_id` en uno | Identidad de fractura inmutable |
| Tratar `CLOCK_BOOTTIME` como `started_at` | Tipos de reloj distintos |
| `SDDIA_PHAGOCYTE_APPLY=1` como único cierre | El fagocito exige lock+PID vivo; no cubre reboot |
| Editar `SddIA/process/` o `SddIA/actions/` a mano | DA-2 → `entity-manager` |
