---
document_id: PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL-PPR-REVOKED-REGISTRY
title: "[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (kaizen-aduana-evolution-local)"
format: markdown
version: "1.2.0"
created: "2026-08-28"
updated: "2026-08-29T04:47:57Z"
status: done
refinement_status: implemented
pbi_archived: true
persist_ref: docs/features/ppr-revoked-registry-rehab-kaizen-aduana-evolution
refinement_date: "2026-08-29T04:43:31Z"
priority: alta
process: refactorization
executor_vehicle: feature
type: refactorization
dispatch: false
uuid: c4e8f1a2-9b3d-4f7e-a6c1-2d8e5f0b3a71
entity: pull-request-review
entity_type: process
olas:
  - A1
  - A2
suggested_branch: refactor/ppr-revoked-registry-rehab-kaizen-aduana-evolution
persist_ref_suggested: docs/features/ppr-revoked-registry-rehab-kaizen-aduana-evolution
branch_name: refactor/ppr-revoked-registry-rehab-kaizen-aduana-evolution
source_branch: fix/kaizen-aduana-evolution-local-ca12-ca14
source_correlation_id: "8ZjTzcBwfFAVFQujfjGCJwJeJcj5pbB4SMHAD5bn5ybE"
feature_ref: docs/fixes/kaizen-aduana-evolution-local
parent_pbi: docs/todos/done/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #190).md
incident_ref: "REVOKED_ENTITY_ALERT_PULL_REQUEST_REVIEW — pull-request-review re-revoked post-rehab #190 (success_rate_below_threshold since 2026-08-28T10:10:42Z; rehabilitated_at 2026-08-26T18:02:03Z; rehab_laudo residual PBI-PPR-190-REVOKED-REGISTRY; samples no podadas → success_rate 0.25)"
blocked_by:
  - "refactorization ∈ revoked since 2026-08-20T05:48:56Z (dedup done PPR #186) → vehículo DCC = feature"
related:
  - .SddIA/cerbero/revoked_entities.json
  - .SddIA/radamanto/stats.json
  - SddIA/agents/radamanto.thresholds.json
  - SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs
  - SddIA/engine/execute-process/src/engine/thermodynamic.rs
  - SddIA/engine/execute-process/src/engine/cerbero_di_rbac.rs
  - SddIA/engine/execute-process/src/engine/phase_terminal.rs
  - SddIA/library/codexes/codex-software-engineering/process/pull-request-review.md
  - docs/todos/done/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #190).md
  - docs/todos/done/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #174).md
  - docs/todos/done/PBI-PPR-208-ACCEPT-PR-REVOKED-REGISTRY.md
  - docs/features/accept-pr-revoked-registry-rehab-ppr208/execution.md
  - docs/features/accept-pr-anti-recurrence-ppr203/plan.md
  - docs/fixes/kaizen-aduana-evolution-local/validacion.md
source_audits:
  - docs/fixes/kaizen-aduana-evolution-local/validacion.md
  - .SddIA/cerbero/revoked_entities.json
  - .SddIA/radamanto/stats.json
---

# [ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (kaizen-aduana-evolution-local)

## Mandato

Rehabilitar el proceso `pull-request-review` en `.SddIA/cerbero/revoked_entities.json` **podando el estado Radamanto que causa la recidiva**, tras re-revocación post-cierre del ciclo #190 (rehab @ `2026-08-26T18:02:03Z` → `revoked` @ `2026-08-28T10:10:42Z`), observada en Cosecha Kaizen PPR `kaizen-aduana-evolution-local` (F4/F5 bloqueante `FAIL_F4_RBAC`).

| Campo | Valor |
|-------|--------|
| Entidad | `pull-request-review` |
| Registro | `.SddIA/cerbero/revoked_entities.json` → **`revoked.pull-request-review`** |
| `entity_type` | `process` |
| `reason` | `success_rate_below_threshold` |
| Since | `2026-08-28T10:10:42Z` |
| ≠ incidente #190 done | #190: permanent+revoked simultáneos · cerrado (`done/` @ `2026-08-26T18:02:03Z`) |
| Check origen | empírico Cerbero F4/F5 en PPR `kaizen-aduana-evolution-local` + `REVOKED_PROCESS_PULL_REQUEST_REVIEW` |
| Emisor ECST | default contractual `delivery-close-cycle` ∉ revoked |

## Estado empírico verificado (`2026-08-29T04:33:37Z`)

`.SddIA/cerbero/revoked_entities.json`:

| Bucket | Contenido |
|--------|-----------|
| `permanent` | `{}` (vacío) |
| `revoked` | `bug-fix` (`abrupt_success_rate_drop` @ `2026-08-28T16:18:17Z`) · **`pull-request-review`** (`success_rate_below_threshold` @ `2026-08-28T10:10:42Z`) · `refactorization` (`abrupt_success_rate_drop` @ `2026-08-20T05:48:56Z`) |

`.SddIA/radamanto/stats.json` → `pull-request-review`:

| Campo | Valor | Lectura |
|-------|-------|---------|
| `status` | `degraded` | no redimible sin `structure_valid: true` |
| `structure_valid` | `false` | bloquea transición `degraded → pending_redemption` |
| `consecutive_success_count` | `0` | redención exige `redemption_success_count: 3` |
| `recovery_attempts` | `1` | margen `max_recovery_attempts: 3` → 2 intentos antes de `deprecated` |
| `samples` | **20** (5 exit 0 / 15 exit≠0) | `success_rate` **0.25** vs umbral `process` **0.70** |
| `rehab_laudo` | `PBI-PPR-190-REVOKED-REGISTRY` | residual del ciclo cerrado |
| `rehabilitated_at` | `2026-08-26T18:02:03Z` | residual; `degraded_at` posterior (`2026-08-28T10:10:42Z`) |

Umbrales vigentes (`SddIA/agents/radamanto.thresholds.json` v1.1.0): `success_rate_min_by_entity_type.process: 0.70` · `redemption_success_count: 3` · `max_recovery_attempts: 3` · `abrupt_drop_min_samples: 3`.

## Diagnóstico — causa raíz de la recidiva (no es "otra revocación más")

**El rehab #190 limpió el bucket Cerbero pero no podó `samples` en Radamanto.** Con 15/20 muestras `exit_code ≠ 0` supervivientes, `success_rate` quedó fijado en `0.25`; `success_rate()` (`radamanto_batch_core.rs`) evalúa la ventana completa, por lo que **la primera telemetría posterior al rehab re-revoca de forma determinista** contra el umbral `0.70`. La recidiva no es aleatoria: era inevitable.

Contraste con las rehabs que **sí** aguantaron: `accept-pr` #208 y `bug-fix`/`feature` #210 fijaron `samples: []` + `structure_valid: true` + `recovery_attempts: 0` (ver `docs/features/accept-pr-revoked-registry-rehab-ppr208/execution.md`, ley **L-SAMPLES** del plan A2 #203). Este PBI debe replicar esa receta, no la de #190.

Segundo bloqueo, independiente del ratio: con `status: degraded` y `structure_valid: false` la máquina de redención nunca arranca (la transición a `pending_redemption` solo ocurre en `set_structure_valid(valid=true)`), así que la entidad no puede auto-sanar por acumulación de éxitos.

### Hipótesis secundaria — bucle de refuerzo denegación → muestra KO (NO verificada; a confirmar empíricamente en T0)

Doce de las quince muestras KO tienen `duration_ms` en el rango `636–1301 ms`, incompatible con una ejecución real de PPR (las muestras exitosas están en `258 000–412 000 ms`). El perfil sugiere abortos tempranos de gobernanza. Si una denegación por revocación se registrara como muestra KO de la propia entidad, el sistema sería autoconfirmante: revocado → deniega → muestra KO → ratio peor → re-revocación.

**Corrección de nomenclatura (evita alucinación previa):** `FAIL_F4_RBAC` es la etiqueta de veredicto de la **aduana Cosecha** (gate F4 en `validacion.md`/`_agent_handoff.md`), **no** un `failed_phase_code` del motor. Grep en `SddIA/engine/` lo confirma: no existe en el crate. El motor emite en `failed_phase_code` uno de tres códigos nativos (`cerbero_di_rbac.rs::CerberoDiCode`):

| Código motor | Origen | ¿Poda A2? |
|--------------|--------|-----------|
| `CERBERO_ENTITY_REVOKED` | provider ∈ `revoked_entities` (`is_entity_revoked`) | **candidato único** |
| `CERBERO_RBAC_DENIED` | `context ∉ requester_policies` (violación real de política) | **NUNCA** — anomalía legítima |
| `CERBERO_CONFIG_ERROR` | políticas vacías / provider no resoluble | **NUNCA** — fallo real |

**Fricción estructural (laudo Dedalo, motiva A2 quirúrgica):** podar categóricamente todo aborto RBAC crearía un punto ciego que silenciaría violaciones legítimas de permisos. El discriminante debe aislar **exclusivamente** `CERBERO_ENTITY_REVOKED`.

**Inexactitud de mecanismo (a resolver en T0):** el gate `validate_di_rbac` evalúa revocación **del provider** (`skill:`/`action:`/`tool:`), no del proceso puntuado. Un proceso `pull-request-review` (id sin prefijo de provider) **no** matchea ese gate por sí mismo; solo generaría `CERBERO_ENTITY_REVOKED` si una de sus fases delega en un **provider** revocado. Por tanto:

- El lazo "PPR revocado → aborta sus propios runs vía este gate" **no está sustentado** por el código leído. Queda como hipótesis abierta.
- La poda A2 debe ser **auto-referencial**: solo hollow si el provider revocado (parseable de `failed_phase_error`) **coincide** con la entidad puntuada (`target_entity_from_payload`). De lo contrario se silenciaría un fallo legítimo tipo "mi dependencia está revocada".

Estado de la evidencia: **no concluyente**. `is_survival_hollow()` poda `lab_hollow`, `detach`, `detached_child` y `cycle_phase`, pero **no** abortos de gobernanza. Los eventos `Raw_Execution_Finished` de esos `asset_id` ya fueron consumidos/purgados. **T0 debe primero reproducir empíricamente cómo se generan las muestras KO de `pull-request-review`** (¿provider revocado? ¿fallo real?) antes de escribir cualquier poda; sin esa confirmación, A2 no se ejecuta.

## Genealogía

| Episodio | PR / ciclo | `since` | Estado |
|----------|------------|---------|--------|
| #124/#125/#174 | olas rehab PPR | varios | **done** |
| #190 | kaizen-paciente0-redeploy | `2026-08-25T16:25:55Z` / `17:24:18Z` | **done** (rehab @ `18:02:03Z`, **sin poda de samples**) |
| **este PBI** | Cosecha PPR kaizen-aduana-evolution-local | `2026-08-28T10:10:42Z` | **done** (rehab @ `2026-08-29T04:47:57Z`; A2 diferida) |

## Sighting Cosecha

PPR kaizen-aduana-evolution-local · CID `8ZjTzcBwfFAVFQujfjGCJwJeJcj5pbB4SMHAD5bn5ybE` · `persist_ref` `docs/fixes/kaizen-aduana-evolution-local` · `source_branch` `fix/kaizen-aduana-evolution-local-ca12-ca14` · F4 `FAIL_F4_RBAC` · F5 `delivery_state: failed` · Handoff **prohibido**.

Materialización: Cosecha Kaizen (Cúmulo) · `KAIZEN_COSECHA_GATE: APTO` · seed nueva (esta; path id).

## Alcance

### Ola A1 — rehab de instancia (fuera del diff del PR)

`.SddIA/cerbero/` y `.SddIA/radamanto/` están en `.gitignore` (líneas 24/27): la mutación es de instancia y **no** viaja en el PR.

1. `revoked_entities.json`: eliminar `revoked.pull-request-review`. **No tocar** `bug-fix` ni `refactorization` (seeds/ciclos ajenos).
2. `stats.json` → `pull-request-review`: `status: healthy` · `samples: []` · `consecutive_success_count: 0` · `recovery_attempts: 0` · `degraded_at: null` · `structure_valid: true` · `entity_type: process` · `rehab_laudo: PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL-PPR-REVOKED-REGISTRY` · `rehabilitated_at` = timestamp del laudo (sustituye los residuales de #190).
3. Smoke post-rehab: una ejecución PPR real (o handoff) sin re-revocación inmediata; registrar `execution_id` en `execution.md`.

### Ola A2 — anti-recurrencia de motor (condicionada a laudo + confirmación T0)

Objetivo: que una **denegación por revocación previa no cuente como fallo de la entidad**, cerrando el bucle autoconfirmante — **sin** crear un punto ciego para violaciones legítimas de política.

0. **T0 — confirmación empírica (bloqueante de A2):** reproducir la generación de muestras KO de `pull-request-review` e identificar el `failed_phase_code` real. Si no es `CERBERO_ENTITY_REVOKED` auto-referencial, A2 no procede (el problema es otro y A1 basta).
1. Discriminante: reutilizar `failed_phase_code` (ya propagado por `thermodynamic.rs` líneas 174/248 desde `phase_terminal.rs::apply_failed_phase_fields`). Ningún código nuevo de aduana.
2. Extender `is_survival_hollow()` en `radamanto_batch_core.rs` para podar **exclusivamente** cuando:
   - `failed_phase_code == "CERBERO_ENTITY_REVOKED"`, **y**
   - el provider revocado (de `failed_phase_error`) coincide con la entidad puntuada (`target_entity_from_payload`) — poda **auto-referencial**.
   - Patrón idéntico a `lab_hollow` / `detached_child`.
3. **Prohibido podar** `CERBERO_RBAC_DENIED` y `CERBERO_CONFIG_ERROR`: son señales legítimas y deben degradar `success_rate` como hasta ahora.
4. Tests unitarios: `t_a2_hollow_entity_revoked_self` (poda), `t_a2_hollow_rbac_denied_not_podado` (violación real NO podada), `t_a2_hollow_revoked_other_provider_not_podado` (dependencia ≠ self NO podada). Aserción de que `lab_hollow`, `detach`, `detached_child`, `cycle_phase` siguen intactos.
5. **Prohibido** en A2: mutar `radamanto.thresholds.json`, `phase_terminal.rs`, el agregador terminal, o el YAML de `pull-request-review`.

Si el laudo limita el ciclo a A1, o si T0 no confirma el mecanismo, A2 se materializa/queda como PBI hijo con esta evidencia; A1 no depende de A2.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| `AC-A1-CERBERO` | `pull-request-review` ausente de `revoked` y de `permanent`; `bug-fix` y `refactorization` intactos |
| `AC-A1-SAMPLES` | `samples: []` y `success_rate` no lastrado por la ventana de #190 (**L-SAMPLES**) |
| `AC-A1-LAUDO` | `rehab_laudo` / `rehabilitated_at` apuntan a este `document_id`; residuales #190 eliminados |
| `AC-A1-REDEEM` | `status: healthy` · `structure_valid: true` · `recovery_attempts: 0` · `degraded_at: null` |
| `AC-A1-SMOKE` | ejecución PPR posterior al rehab sin re-revocación inmediata; `execution_id` registrado |
| `AC-A2-DISCRIM` | poda solo `CERBERO_ENTITY_REVOKED` auto-referencial; `CERBERO_RBAC_DENIED` y `CERBERO_CONFIG_ERROR` siguen degradando `success_rate` (o PBI hijo abierto) |
| `AC-A2-TESTS` | `cargo test -p execute-process --lib` verde; nuevos `t_a2_hollow_*` (poda self, no-poda rbac_denied, no-poda otro provider); podas preexistentes sin regresión |
| `AC-GIT-CLEAN` | diff del PR sin `.SddIA/cerbero/**` ni `.SddIA/radamanto/**` |
| `AC-NO-THRESH` | `radamanto.thresholds.json` sin modificar |
| `AC-DOC` | cascada Tekton en `persist_ref` + entrada `evolution` con `uuid c4e8f1a2-9b3d-4f7e-a6c1-2d8e5f0b3a71` (una por ciclo) |

## Restricciones operativas

- **L-VEHICLE:** `refactorization` ∈ revoked → DCC con `source_process: feature` como vehículo y nota `process_label: refactorization` (precedente A2 #203). No se rehabilita `refactorization` en este ciclo.
- **L-DEDUP:** un solo PBI canónico (este `document_id` / `uuid`); prohibido archivar alias duplicado.
- **Cierre en rama (un PR):** PBI a `docs/todos/done/` + `validacion.md` `global: APTO` con `pbi_archived: true` dentro del **mismo** PR que el código. Prohibido segundo PR documental.
- **DA-5:** tras el acuse JSON del CLI (`detached: true` en PPR), prohibido `sleep`/polling/`AwaitShell`.
- **Kintsugi:** si un proceso oficial colapsa durante el ciclo, detener y escalar por fractura; prohibido bypass raw.

## Criterio de cierre

- [x] Laudo rehabilitación Cerbero / Radamanto con poda de `samples` (anti-recurrencia post-rehab #190)
- [x] `pull-request-review` ausente de `revoked` (y `permanent` si aplica)
- [x] Smoke PPR post-rehab sin re-revocación inmediata (`execution_id` `ff62b08c-9f6f-4740-9664-3060bea114d8`)
- [x] Laudo explícito sobre ola A2: **diferida** — PBI hijo (`L-A2-T0`; mecanismo no confirmado)
- [x] Cascada feature/fix + `validacion.md` APTO + PBI en `done/` en el mismo PR

## Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Repetir la receta #190 (sin poda) | `AC-A1-SAMPLES` bloqueante; recidiva determinista demostrada |
| `structure_valid` queda `false` | `AC-A1-REDEEM`: sin él la redención automática nunca arranca |
| `recovery_attempts` acumulando hacia 3 | reset a `0` en A1; a la tercera, `Domain_Entity_Deprecated` |
| Ejecutar con vehículo revocado | **L-VEHICLE** (`feature`) |
| Instancia versionada en el PR | `AC-GIT-CLEAN` |
| Ajustar umbrales para "aprobar" | `AC-NO-THRESH`: falsear el termómetro, no curar la entidad |
| **Poda A2 ciega silencia RBAC real** | `AC-A2-DISCRIM`: solo `CERBERO_ENTITY_REVOKED` auto-referencial; nunca `CERBERO_RBAC_DENIED`/`CERBERO_CONFIG_ERROR` |
| A2 sobre mecanismo no verificado | T0 bloqueante: confirmar `failed_phase_code` real antes de escribir poda |

## Fuera de alcance

- Rehab `refactorization` (dedup done PPR #186).
- **`bug-fix` re-revocado @ `2026-08-28T16:18:17Z`**, posterior al rehab #210 (`rehabilitated_at 2026-08-28T06:13:50Z`): episodio nuevo **sin PBI abierto** → requiere seed propia. No se resuelve aquí.
- Residual Kalma2 Shell/`git-manager` (dedup OPERATIVO PPR #136).
- Merge/Handoff soberano del ciclo `kaizen-aduana-evolution-local` (desbloqueado para PPR ∉ revoked; handoff downstream fuera de alcance).
- Revisión de umbrales Radamanto (dedup done PPR #174+#177).
