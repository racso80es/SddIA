---
document_id: PBI-PPR-187-DCC-REVOKED-REGISTRY
title: "[ARQUITECTURA] delivery-close-cycle — rehabilitación revoked_entities (PPR #187)"
format: markdown
version: "1.2.0"
created: "2026-08-20"
updated: "2026-08-24T17:45:00Z"
refinement_status: implemented
persist_ref: docs/features/dcc-revoked-registry-rehab-ppr187
branch_name: refactor/dcc-revoked-registry-rehab-ppr187
status: done
priority: media
process: refactorization
type: refactorization
dispatch: false
uuid: c4a91e7b-2f68-4d3a-a8e1-5b7c9d0e2f14
suggested_branch: refactor/dcc-revoked-registry-rehab-ppr187
persist_ref_suggested: docs/features/dcc-revoked-registry-rehab-ppr187
source_correlation_id: 4gKBTRCyZzvEFQcbDWFnBmdC3ZjvqTJmauHiYgTWwj32
source_pr_url: https://github.com/racso80es/SddIA/pull/187
feature_ref: docs/features/kaizen-consumer-ignition-filtro-c
incident_ref: "REVOKED_ENTITY_ALERT_DELIVERY_CLOSE_CYCLE — delivery-close-cycle ∈ revoked since 2026-08-20T12:04:10Z (abrupt_success_rate_drop); re-revocación post-rehab #174+#177"
entity: delivery-close-cycle
parent_pbi: docs/todos/done/[ARQUITECTURA] umbrales Radamanto process — rehabilitación revoked_entities (PPR #174+#177).md
related:
  - .SddIA/cerbero/revoked_entities.json
  - .SddIA/radamanto/stats.json
  - SddIA/agents/radamanto.thresholds.json
  - SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs
  - SddIA/engine/execute-process/src/engine/delivery_close.rs
  - SddIA/engine/execute-process/src/engine/phase_capsules.rs
  - SddIA/engine/execute-process/src/engine/thermodynamic.rs
  - SddIA/engine/execute-process/src/engine/residual_runner.rs
  - SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md
  - docs/todos/done/[ARQUITECTURA] umbrales Radamanto process — rehabilitación revoked_entities (PPR #174+#177).md
  - docs/todos/done/[ARQUITECTURA] delivery-close-cycle — rehabilitación revoked_entities (PPR #177).md
  - docs/todos/done/[ARQUITECTURA] delivery-close-cycle — revoked_entities y ECST signer (PPR #136).md
  - docs/todos/done/[ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #185).md
  - docs/todos/done/[ARQUITECTURA] refactorization — rehabilitación revoked_entities (PPR #186).md
  - docs/features/radamanto-process-threshold-rehab/execution.md
  - docs/features/feature-revoked-registry-rehab/spec.md
  - docs/features/kaizen-consumer-ignition-filtro-c/validacion.md
source_audits:
  - docs/features/kaizen-consumer-ignition-filtro-c/validacion.md
  - docs/features/kaizen-consumer-ignition-filtro-c/finalize-process.md
olas:
  - id: A1
    name: saneamiento-instancia
    locus: ".SddIA/ (Yunque Rúnico · volátil)"
    git_diff: prohibido
  - id: A2
    name: evolucion-motor-rust
    locus: SddIA/engine/execute-process/
    git_diff: obligatorio
source_correlation_ids:
  - 4gKBTRCyZzvEFQcbDWFnBmdC3ZjvqTJmauHiYgTWwj32
  - 34736c88-34d3-46f8-a050-75e7775d005b
---

# [ARQUITECTURA] delivery-close-cycle — rehabilitación revoked_entities (PPR #187)

## Mandato

Rehabilitar el proceso `delivery-close-cycle` en `.SddIA/cerbero/revoked_entities.json` tras **re-revocación** post-cierre de ola 2 (#174+#177).

Rehab de registro **sin** A1 absoluto + laudo anti-recurrencia **reabre el vector** `abrupt_success_rate_drop` en el mismo peaje (jurisprudencia PBI-185 / PBI-174-177).

| Campo | Valor |
|-------|--------|
| Entidad | `delivery-close-cycle` |
| Registro | `.SddIA/cerbero/revoked_entities.json` → **`revoked.delivery-close-cycle`** |
| `entity_type` | `process` (correcto post-#174+#177; no regresionar a `tool`) |
| `reason` | `abrupt_success_rate_drop` |
| Since | `2026-08-20T12:04:10Z` |
| ≠ incidente #177 done | #177: `success_rate_below_threshold` since `2026-08-16T16:40:55Z` (cerrado) |
| ≠ incidente #136 done | #136: `abrupt_success_rate_drop` since `2026-07-13` (cerrado; signer ECST liquidado) |
| Check origen | `REVOKED_ENTITY_ALERT_DELIVERY_CLOSE_CYCLE` (F4/F5 · alerta no bloqueante) |
| Emisor ECST seed | `github-bridge-watcher` ∉ revoked → `RBAC_EMITTER_NOT_REVOKED: APTO` |
| Alerta | Registro DCC en Cerbero; no bloquea PPR #187 |

## Genealogía de revocaciones DCC

| Episodio | PR / ciclo | `reason` | `since` | Estado |
|----------|------------|----------|---------|--------|
| #136 | `delivery-close-cycle-revoked-signer` | `abrupt_success_rate_drop` | `2026-07-13` | **done** — rehab + signer `Vertice_Biologico_Relay` |
| #177 (ola 2) | `radamanto-process-threshold-rehab` | `success_rate_below_threshold` | `2026-08-16T16:40:55Z` | **done** — umbrales `process: 0.70` + fail-soft handoff |
| **#187 (este PBI)** | Kaizen consumidor Filtro C | `abrupt_success_rate_drop` | `2026-08-20T12:04:10Z` | **abierto** — episodio nuevo |

Vector activo ≠ `success_rate_below_threshold` (ya corregido en motor 1.1.0). Vector activo = caída abrupta con ventana corta (`abrupt_drop_min_samples: 3`).

## Sighting Cosecha

PPR #187 · CID `4gKBTRCyZzvEFQcbDWFnBmdC3ZjvqTJmauHiYgTWwj32` · emisor ECST `github-bridge-watcher` ∉ revoked · `PullRequest_Presented` @ `2026-08-20T12:04:09Z` · clave DCC revocada 1s después (`12:04:10Z`).

Materialización: Cosecha Kaizen (Cúmulo) @ `2026-08-20T14:15:00Z` · `KAIZEN_COSECHA_GATE: APTO` · `kaizen_seeds: 1`.

## Sightings adicionales

| Sighting | CID | Nota |
|----------|-----|------|
| Cosecha Kaizen gemelo | `34736c88-34d3-46f8-a050-75e7775d005b` | Dedup; seed ya materializada @ `4gKBTRCy…` |
| Argos F5 gemelo | `34736c88-34d3-46f8-a050-75e7775d005b` | `PASS_F5_VERDICT` · DCC∈revoked no bloqueante |
| Cerbero F4 | `4gKBTRCy…` | 8 loci / 0 bloqueos · emisor GBW APTO · alerta DCC |

**Prohibido deduplicar** contra satélite done #177 (`since 2026-08-16T16:40:55Z`).

## Contexto PR #187 (semilla)

| Campo | Valor |
|-------|--------|
| Título PR | feat: Kaizen perfil ignición consumidor Filtro C |
| Estado | **MERGED** |
| Rama | `feat/kaizen-consumer-ignition-filtro-c` |
| PBI feature | `PBI-KAIZEN-CONSUMER-IGNITION-FILTRO-C` → `docs/todos/done/` |
| `persist_ref` | `docs/features/kaizen-consumer-ignition-filtro-c` |
| Evolution | `SddIA/evolution/14f34c46-7683-4a2f-9042-69795d170d88.md` |
| Snapshot DCC | commit `8da2167` · mensaje `delivery-close: snapshot final consolidado` @ `2026-08-20T12:04:04Z` |
| ECST Presented (DCC) | CID `34736c88-34d3-46f8-a050-75e7775d005b` (emisor histórico DCC) |
| ECST Presented (GBW) | CID `4gKBTRCy…` (semilla Cosecha) |

### Ejecución DCC en cierre PR #187

- Fase **Aduana EDA genómica**: `orphan_count=2` preexistentes (`github-raw-fetcher`, `download-remote-asset`) → Argos `block` en esa fase; **no introducidos por este Kaizen** (`validacion.md` · `eda_audit_note`).
- A pesar del block EDA, el PR se abrió igualmente (`plan.md` T7 · `delivery-close-cycle` con huérfanos preexistentes).
- DCC terminó con `exit_code: 1` → sample KO en ventana Radamanto → degradación @ `12:04:10Z`.
- Residual documental Kaizen: huérfanos EDA fuera de alcance; enable systemd host pendiente operador.

## Estado empírico instancia (corte 2026-08-21)

### Cerbero — `.SddIA/cerbero/revoked_entities.json`

```json
"delivery-close-cycle": {
  "entity_type": "process",
  "reason": "abrupt_success_rate_drop",
  "since": "2026-08-20T12:04:10Z"
}
```

Otros revocados laterales (fuera de alcance salvo laudo agrupado): `bug-fix`, `refactorization`, `emit-pr-audited-event`.

### Radamanto — bucket raíz `delivery-close-cycle`

| Campo | Valor |
|-------|--------|
| `status` | `degraded` |
| `degraded_at` | `2026-08-20T12:04:10Z` |
| `recovery_attempts` | **2** |
| `rehab_laudo` (previo) | `PBI-PPR-174-177-REVOKED-PROCESS-THRESHOLDS` |
| `rehabilitated_at` (previo) | `2026-08-16T16:37:15Z` |
| `structure_valid` | `false` |
| Ventana | 5 samples · 3 OK / 2 KO · **rate 0,60** |
| Umbral `process` | `success_rate_min: 0.70` (`radamanto.thresholds.json` v1.1.0) |

Samples KO recientes (runtime ~4,5–5,3 s):

| `asset_id` (tail) | `duration_ms` | `exit_code` |
|-------------------|---------------|-------------|
| `d7310496-…` | 5270 | 1 |
| `19391b9f-…` | 4478 | 1 |

Samples OK previos (~4,3–4,5 s): `5ffede6c`, `431cb97b`, `8ccee19f`.

Bucket anidado `entities.delivery-close-cycle`: `healthy` · samples lab ~75–126 ms · **no es el revocado** (fósil ontología).

## Diagnóstico causal

### Regla Radamanto (`radamanto_batch_core.rs`)

Con `status == healthy` previo a la degradación:

```text
if samples.len >= batch_min_events (10) AND rate < rate_min → success_rate_below_threshold
else if samples.len >= abrupt_drop_min_samples (3) AND rate < rate_min → abrupt_success_rate_drop
```

Con 5 samples y rate 0,60 < 0,70 (`process`): aplica **`abrupt_success_rate_drop`**, no el vector de #177.

### Cadena causal (hipótesis operativa)

```text
2026-08-16  Rehab #174+#177: DCC → healthy, entity_type process, fail-soft handoff (L-FAILSOFT-OLA2)
        → ventana mezcla éxitos runtime + lab
2026-08-20T12:04:04Z  DCC en cierre PR #187 (feature Kaizen consumidor)
        → Aduana EDA: orphan_count=2 preexistentes → fase blocked/failed
        → exit_code global 1 (sin fail_soft suficiente en este peaje concreto)
2026-08-20T12:04:10Z  Radamanto batch: rate 3/5 < 0.70, n≥3
        → Domain_Entity_Degraded (abrupt_success_rate_drop)
        → Cerbero: revoked.delivery-close-cycle
        → recovery_attempts: 1 → 2
```

### Diferencia vs ola 2 (#177)

| Dimensión | #177 | #187 |
|-----------|------|------|
| Vector | `success_rate_below_threshold` | `abrupt_success_rate_drop` |
| Ontología al revocar | `tool` (entropía; corregida en rehab) | `process` (ya correcta) |
| Causa raíz | Macro-proceso evaluado con umbral `tool` 0.85 | Ventana corta post-rehab con 2 fallos EDA/runtime |
| Intervención previa | Umbrales por tipo + fail-soft handoff DCC | Insuficiente para block EDA con huérfanos preexistentes |

### Relación con ciclo hermano #185/#186

El ciclo `feature-revoked-registry-rehab` (#185) aplicó A2 fail-soft **padre** (`feature` → DCC hijo) + A3 poda `survival_hollow`. **No rehabilitó DCC** ni cortó re-muerte del propio peaje DCC cuando falla como proceso raíz (cierre de entrega directo).

PBI #186 (`refactorization`) comparte peaje lifecycle; rehab pendiente de verificar en instancia al cierre de #185/#186.

## Contrato de fases DCC (referencia)

Proceso `delivery-close-cycle.md` v1.1.1 — 7 fases:

1. Snapshot final (`git-manager`)
2. Impacto SddIA condicional (`agent:argos`)
3. **Aduana EDA genómica** (`agent:argos`) — block si `orphan_count > 0`
4. Publicación remota (`git-manager` · `proc:git-sync`)
5. Apertura en forja (`shell-executor` · `gh`)
6. Sello Presentación ECST (`emit-pr-presented-event`)
7. Higiene local (`git-manager`)

Kintsugi ola 2 (#174+#177): `mark_fail_soft_if_secondary` + cola post-`pr_url`/`delivery_push`. **No cubre** block Argos en fase 3 cuando el umbral físico (push/PR) aún no cruzó.

## Contexto heredado

Ola 2 (#174+#177) liquidó umbrales Radamanto + rehab instancia DCC (`rehabilitated_at: 2026-08-16T16:37:15Z`). Esta cicatriz es **episodio nuevo** (`since` distinto); no deduplicar contra satélite done #177.

Jurisprudencia aplicable:

| Ref | Origen | Aplicación a #187 |
|-----|--------|-------------------|
| **L-REHAB-INST** | #174+#177 · #185 | A1 en `.SddIA/`; evidencia `execution.md`; prohibido versionar `revoked_entities`/`stats` en PR |
| **L-FAILSOFT-OLA2** | #174+#177 | Extender con **adjudicación retroactiva** EDA post-umbral físico (§ Vector A2) |
| **L-HOLLOW** / **L-BATCH-SKIP** | #185 A3 | **No aplica a DCC** (`delivery-close-cycle` ∉ `LIFECYCLE_PROCESSES`) |
| **L-THRESH** | #174+#177 · #185 | `radamanto.thresholds.json` v1.1.0 intacto salvo laudo explícito |

## Refinamiento táctico S+ Grade (manifiesto de forja)

Impacto absoluto sobre el **Yunque Rúnico** (Cúmulo volátil `.SddIA/`). Separación dogmática:

| Vector | Locus | Git |
|--------|-------|-----|
| **A1** | Instancia `.SddIA/` | **Prohibido** incluir en commit |
| **A2** | Motor Rust `execute-process` | **Obligatorio** en el PR del ciclo |

Este bloque es SSOT para `clarify.md` / `spec.md` del `persist_ref` (`docs/features/dcc-revoked-registry-rehab-ppr187/`). Un ciclo `refactorization`. Prohibido despachar `bug-fix` satélite.

### Correcciones anti-alucinación (auditoría código 2026-08-21)

| Propuesta errónea | Hecho verificado |
|-------------------|------------------|
| Inyectar `is_secondary: true, fail_soft: true` en definición de fases YAML/Rust estática | **`fail_soft` es runtime** en el JSON del `phase_report`; lo escribe el motor en `delivery_close.rs`, no el YAML del proceso. |
| Ampliar solo `is_dcc_secondary_phase` con `"Aduana EDA genómica"` | **Insuficiente**: `mark_fail_soft_if_secondary` exige `(has_pr \|\| delivery_push)` **en el momento de la llamada**. EDA es fase **3**; `pr_url`/`delivery_push` aparecen en fases **4–5**. Al evaluar EDA aún no hay umbral físico. |
| Tocar `residual_runner.rs` para evitar `survival_hollow` en DCC | **Inexacto**: `survival_hollow` vive en `radamanto_batch_core.rs` y aplica a telemetría de `LIFECYCLE_PROCESSES` (`feature`, `bug-fix`, `refactorization`). **`delivery-close-cycle` ∉ esa lista**. No hay poda hollow que corregir en `residual_runner` para DCC. |
| A3 poda hollow (#185) como vector #187 | **Fuera de alcance**: el fallo #187 es `exit_code: 1` real en bucket DCC, no sample hueco lab. |

---

### Vector A1 — Saneamiento de instancia (Yunque Rúnico)

Intervención **exclusiva** sobre topología volátil. Evidencia en `execution.md`. **Dogmáticamente prohibido** versionar `.SddIA/cerbero/` ni `.SddIA/radamanto/` en el diff del PR.

#### Amnistía Cerbero — `.SddIA/cerbero/revoked_entities.json`

1. Localizar nodo `delivery-close-cycle` bajo clave `revoked`.
2. **Eliminar el nodo por completo** (Cerbero no tiene estado `healthy`; ausencia = derecho de ejecución).
3. Verificar `permanent.delivery-close-cycle` **ausente**.

#### Restauración Radamanto — `.SddIA/radamanto/stats.json`

1. Localizar bucket **raíz** `delivery-close-cycle` (**no** `entities.delivery-close-cycle`).
2. Mutación obligatoria:

| Campo | Desde (empírico) | Hacia |
|-------|------------------|-------|
| `status` | `degraded` | `healthy` |
| `recovery_attempts` | `2` | **`0`** (estricto) |
| `degraded_at` | `2026-08-20T12:04:10Z` | `null` |
| `consecutive_success_count` | — | `0` |
| `rehab_laudo` | `PBI-PPR-174-177-…` | `PBI-PPR-187-DCC-REVOKED-REGISTRY` |
| `rehabilitated_at` | `2026-08-16T16:37:15Z` | ISO UTC de la intervención A1 |
| `entity_type` | — | conservar coherencia `process` (instancia Cerbero) |

3. **Poda termodinámica de ventana (`samples`):** vaciar el array **o** conservar únicamente los **3** últimos eventos con `exit_code == 0` (hoy hay exactamente 3 OK runtime ~4,3–4,5 s). Eliminar los 2 KO (`d7310496…`, `19391b9f…`). Sin esto, un solo fallo futuro re-dispara `abrupt_success_rate_drop` (rate 1/4 = 0,25 < 0,70 con `abrupt_drop_min_samples: 3`).

---

### Vector A2 — Evolución genética del motor (anti-recurrencia Rust)

Objetivo: cuando la transacción física (push + `pr_url`) **sí** cruza, un block EDA por huérfanos **preexistentes** no debe colapsar `exit_code` global ni envenenar Radamanto.

#### Cuello de botella verificado

```text
Orden fases DCC (delivery-close-cycle.md v1.1.1):
  1 Snapshot → 2 Impacto → 3 Aduana EDA → 4 Push → 5 gh PR → 6 ECST → 7 Higiene
```

- `capsule_eda_genomic_audit_gate` (`phase_capsules.rs`): `orphan_count > 0` sin backfill activo → `status: blocked`, `argos_verdict: block`.
- El bucle en `delivery_close::run` **ejecuta todas las fases** aunque EDA bloquee; push/PR pueden completarse después.
- `aggregate_execution_terminal` (`phase_terminal.rs`): `blocked` **sin** `fail_soft: true` → `success: false`, `exit_code: 1`.
- Caso #187: EDA blocked **antes** de `pr_url`; `mark_fail_soft_if_secondary` en línea 81 **no** marca `fail_soft` (predicado post-umbral no cumplido aún).

#### Laudo técnico A2 (extensión L-FAILSOFT-OLA2)

**No** mutar YAML `{name}.md` del proceso salvo nota documental vía `entity-manager`. Cambios en Rust:

| Touchpoint | Intervención |
|------------|--------------|
| `delivery_close.rs` | Tras el bucle de fases y **antes** de `aggregate_execution_terminal`: función de **adjudicación retroactiva** (p. ej. `adjudicate_eda_fail_soft_post_physical`). Si `(pr_url \|\| delivery_push)` presentes en `state` y el report de `"Aduana EDA genómica"` tiene `status ∈ {blocked, failed}` con `orphan_count > 0` y `argos_verdict == "block"`, inyectar `"fail_soft": true` en ese report. |
| `delivery_close.rs` | Opcional coherente: ampliar `is_dcc_secondary_phase` con `"Aduana EDA genómica"` **solo** si la adjudicación retroactiva ya garantiza umbral físico (no sustituye el post-pass). |
| `phase_capsules.rs` | **No** debilitar `capsule_eda_genomic_audit_gate` a `pass` silencioso: Argos debe seguir registrando `argos_verdict: block` / ruido de sistema en el report. |
| `phase_terminal.rs` | **Prohibido mutar** agregador (jurisprudencia #185). |
| `residual_runner.rs` | Alinear path EDA DCC (L.606–613): tras `Ok(gate)` blocked, invocar la misma adjudicación retroactiva **o** delegar en helper compartido exportado desde `delivery_close.rs`. Hoy **no** llama `mark_fail_soft_if_secondary` en `Ok(gate)`. |

**Fuera de A2:** `radamanto_batch_core.rs` / `survival_hollow` — no aplican a bucket DCC directo.

#### Contrato de comportamiento (AC motor)

```text
PRE:  orphan_count > 0 preexistente (github-raw-fetcher, download-remote-asset)
      backfill_manifest_active == false
RUN:  fases 1–7 completan; fase 4–5 cruzan umbral físico
POST: report EDA conserva blocked + argos_verdict block + fail_soft true (retroactivo)
      aggregate_execution_terminal → success true · exit_code 0
      Radamanto sample exit_code 0 (telemetría termodinámica del run)
```

**Causal duro (sin fail_soft):** fallo snapshot, push, apertura PR, o block Argos **después** de umbral con deuda **introducida** por el diff del ciclo actual.

#### Tests obligatorios (unit + integración)

- `delivery_close.rs` `#[cfg(test)]`: EDA blocked + state con `pr_url` post-adjudicación → agregador `success`.
- EDA blocked **sin** `pr_url`/`delivery_push` → sigue causal (`exit_code: 1`).
- Regresión: tests existentes `dcc_hygiene_failed_is_fail_soft_when_pr_url_present` / `dcc_snapshot_failed_never_fail_soft` intactos.

---

### Criterios Argos (Protocolo de Acero)

- [ ] **A1 git-clean:** `.SddIA/cerbero/` y `.SddIA/radamanto/` **no** aparecen en `git status` / diff del PR.
- [ ] **A1 evidencia:** `execution.md` documenta timestamp A1, campos reseteados y recorte de `samples`.
- [ ] **A2 build:** `cargo build -p execute-process` (o workspace engine) sin errores de compilación.
- [ ] **A2 tests:** `cargo test -p execute-process delivery_close` (o filtros equivalentes) en verde, incluido caso EDA+huérfanos+umbral físico.
- [ ] **A2 smoke CLI:** invocación `./sddia-run.sh --process delivery-close-cycle` en lab con huérfanos preexistentes simulados/confirms instancia: fase EDA reporta block **pero** envelope `success: true` · `exitCode: 0` cuando push/PR cruzaron.
- [ ] **L-THRESH:** `SddIA/agents/radamanto.thresholds.json` v1.1.0 **intacto**.
- [ ] **RBAC posterior:** `RBAC_EMITTER_NOT_REVOKED: APTO` con emisor `delivery-close-cycle` en aduana PPR siguiente.

## Fuera de alcance

- Rehabilitación `bug-fix`, `refactorization`, `emit-pr-audited-event`, `feature` (ciclos laterales / hermanos).
- Backfill EDA de `github-raw-fetcher` / `download-remote-asset` (deuda sistémica preexistente; no causada por PR #187).
- Merge / handoff `accept-pr` de PR #187 (ya MERGED).
- Mutar `radamanto.thresholds.json` v1.1.0 sin laudo.
- Versionar `.SddIA/cerbero/` o `.SddIA/radamanto/` en el diff git del PR de cierre.

## Dedup explícito

| Finding | Tratamiento |
|---------|-------------|
| PBI done #177 (`since 2026-08-16`) | **No dedup** — episodio distinto |
| PBI done #136 (`since 2026-07-13`) | **No dedup** — cerrado; referencia histórica |
| Sighting gemelo `34736c88…` | **Dedup** — misma seed @ `4gKBTRCy…` |
| `RBAC_EMITTER_NOT_REVOKED` en PPR #187 | Emisor GBW APTO; alerta = registro DCC |
| `GIT_EVIDENCE_SESSION_SHELL` / F3 | Dedup done PPR #136 (Kaizen consumidor) |

## Criterio de cierre (Done)

- [x] **A1** instancia verificada (Yunque Rúnico): `delivery-close-cycle` ∉ `revoked` ni `permanent` · stats raíz `healthy` · `recovery_attempts: 0` · `rehab_laudo` + `rehabilitated_at` · ventana `samples` podada
- [x] **A2** motor desplegado: adjudicación retroactiva EDA + tests + smoke CLI (§ Criterios Argos)
- [x] `spec.md` / `clarify.md` del ciclo reflejan este refinamiento sin drift
- [x] `validacion.md` APTO · `pbi_archived: true`
- [x] Este TODO → `docs/todos/done/` en la misma rama del PR

## Despacho sugerido

```bash
./sddia-run.sh --process refactorization --inputs '{
  "persist_ref": "docs/features/dcc-revoked-registry-rehab-ppr187",
  "branch_name": "refactor/dcc-revoked-registry-rehab-ppr187",
  "source_process": "refactorization",
  "pbi_ref": "docs/todos/pending/[ARQUITECTURA] delivery-close-cycle — rehabilitación revoked_entities (PPR #187).md"
}'
```

Prohibido mutación manual de genoma (`SddIA/process/`, `SddIA/tools/`, etc.) fuera de cadena `entity-manager`.
