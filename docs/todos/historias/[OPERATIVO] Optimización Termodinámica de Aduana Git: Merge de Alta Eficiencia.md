---
document_id: HU-MERGE-THERMODYNAMICS
title: "[OPERATIVO] Optimización Termodinámica de Aduana Git: Merge de Alta Eficiencia"
format: markdown
version: "1.3.0"
created: "2026-10-03"
refined: "2026-10-03"
decisions:
  - id: D-1
    verdict: "aduana ligera síncrona suficiente para qa_profile docs-only; sin doble aduana"
    by: "Vértice Biológico"
    date: "2026-10-03"
  - id: D-2
    verdict: "la atestación se invalida en accept-pr Fase 4; no hay suscriptor nuevo de PullRequest_Merged"
    by: "Tekton"
    date: "2026-10-03"
status: "refinada"
priority: "alta"
process: "feature"
related:
  - SddIA/scripts/qa/git-hooks/hook_common.sh
  - SddIA/scripts/qa/git-hooks/pre_commit_gate.sh
  - SddIA/scripts/qa/git-hooks/pre_push_gate.sh
  - SddIA/scripts/qa/git-hooks/post_merge_gate.sh
  - SddIA/scripts/common/sddia_shell_lib.sh
  - SddIA/tools/sddia-qa/src/gate_evolution.rs
  - SddIA/engine/execute-process/src/engine/pull_request_review.rs
  - SddIA/engine/execute-process/src/engine/accept_pr.rs
  - SddIA/engine/execute-process/src/engine/executor.rs
  - SddIA/engine/execute-process/src/engine/agent_runtime.rs
  - SddIA/library/codexes/codex-software-engineering/process/pull-request-review.md
  - SddIA/library/codexes/codex-software-engineering/process/accept-pr.md
  - SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md
  - SddIA/events/orchestration/local-qa-requested.md
  - SddIA/events/domain/pull-request-presented.md
  - SddIA/events/domain/pull-request-merged.md
  - SddIA/core/event-domain-subscriptions.json
  - SddIA/norms/pull-request-orchestration.md
  - SddIA/norms/obediencia-procesos.md
  - SddIA/CONSTITUTION_CORE.md
antecedentes:
  - docs/features/pbi-005-hito3-git-hooks
  - docs/features/husky-pre-push-blocking-route
  - docs/features/prepush-argos-qa-witness
  - docs/features/kaizen-tekton-evolution-gate-no-poll
  - docs/features/evolution-registry-gate
---

### [OPERATIVO] Optimización Termodinámica de Aduana Git: Merge de Alta Eficiencia

> **Nota de refinamiento v1.1.0.** La v1.0.0 contenía afirmaciones no verificables contra el genoma (ver §0). Esta versión reescribe alcance, fases y criterios sobre la topología real de los hooks medida el 2026-10-03. Se eliminan los marcadores `[cite: N]` (residuo de generación con contexto externo no presente en el repo).

#### 0. Correcciones sobre v1.0.0 (Filtro A — alucinaciones e inexactitudes)

| # | Afirmación v1.0.0 | Realidad verificada | Efecto en la HU |
| :--- | :--- | :--- | :--- |
| 0.1 | Touchpoint `SddIA/scripts/qa/git-hooks/hook_common.py` | No existe. La aduana es **bash** (`hook_common.sh`, `pre_commit_gate.sh`, `pre_push_gate.sh`, `post_merge_gate.sh`; cabecera "Ola 5 — sin Python"). El cómputo pesado vive en Rust (`execute-process`, `sddia-qa`). | Touchpoints reescritos en §5. |
| 0.2 | Los hooks disparan `cargo test`, `cargo build --release` o linting estricto de Python | **Falso.** Ningún hook compila ni ejecuta tests. `cargo test` solo corre en CI GitHub (`.github/workflows`). Pre-commit medido: `verify-process-integrity` 45 ms + `audit-eda-coverage --scan` 273 ms ≈ **0,3 s**. | La "Fase 1" original atacaba un coste inexistente. Se redirige el triaje por delta al punto donde sí hay latencia (pre-push, §2.1). |
| 0.3 | `telemetry-compliance-audit` forma parte de la cadena síncrona de hooks y debe moverse al bus | **Falso.** Ya es asíncrono: suscriptor de `Raw_Execution_Finished` en `event-telemetry-subscriptions.json`. Audita recibos de telemetría por ejecución, no PRs; reenrutarlo a `PullRequest_Merged` sería semánticamente incorrecto. | "Fase 3" original eliminada. |
| 0.4 | `PullRequest_Merged` se suscribe en `SddIA/core/event-orchestration-subscriptions.json` | Es familia **domain** (`SddIA/events/domain/pull-request-merged.md`); SSOT `event-domain-subscriptions.json`, con 3 suscriptores vigentes (`tracker-stamp`, `iota-immutable-publisher`, `notify-humanized-pr-merged`). | Referencia corregida. |
| 0.5 | Touchpoint `SddIA/process/accept-pr.md` | Reside en `SddIA/library/codexes/codex-software-engineering/process/accept-pr.md` (packing códice post-ABSTRACT-03). Igual para `pull-request-review.md`. | Ruta corregida. La misma deriva existe en `pull-request-orchestration.md` §4/§6: incorporada al alcance como **2.5-A**. |
| 0.6 | "Filtro C (Eficiencia / Necesidad)" | `CONSTITUTION_CORE.md` §4: **Filtro C (Necesidad)**. La eficiencia no es un filtro constitucional; es consecuencia de descartar ruido. | Terminología alineada. |
| 0.7 | "Validación ZKP" | No hay conocimiento cero: es una **atestación** (testigo firmado/anclado a hash) que la aduana verifica. Usar "ZKP" induce a error de diseño. | Renombrado a **atestación de QA** (§2.2). |
| 0.8 | Las auditorías de Argos se desplazan "a los estados `in_progress` / `in_review`" | Son estados del tracker Linear (otra HU), no del ciclo Git. Mezcla de dominios. | Sustituido por "fuera del hilo síncrono del hook" (§2.2). |
| 0.9 | Directorio `.SddIA/proofs/audits/` | No existe. SSOT de proofs: `cumulo.paths.json` → `eda_instance.proofs` (`.SddIA/proofs`), con namespaces ya en uso (`dlt-telemetry`, `pec-correlation`, `tqm-single-flight`) y resolver Rust `resolve_eda_proofs_dir`. | Se instancia namespace nuevo vía resolver existente, no ruta cableada. |
| 0.10 | AC-5: "fallos reales de código (tests rotos en Rust) **siguen** bloqueando" | Hoy **no bloquean localmente** (ver 0.2). Exigirlo sería scope creep (introducir `cargo test` en hook) y contradice el objetivo de la HU. | AC reformulado: no degradar los gates que **sí** existen. |
| 0.11 | Lista de exclusión `docs/`, `historias/`, `README.md` | `README.md` es parámetro de restricción (`.cursorrules` §7) y prefijo monitorizado DIA en `pull-request-review`. `historias/` ya está bajo `docs/`. | Conjunto pasivo redefinido en §2.1 (README excluido del bypass). |

#### 1. Propósito y Alcance

Reducir la latencia de la aduana Git local aplicando el **Filtro C (Necesidad)**: cada verificación síncrona debe justificar su presencia en el hilo del desarrollador; lo que no proteja la integridad del genoma en ese instante se desplaza a atestación previa o al bus EDA.

**Dónde está realmente el coste (topología medida):**

| Hook | Cadena actual | Coste dominante |
| :--- | :--- | :--- |
| `pre-commit` | `verify-process-integrity` + `audit-eda-coverage --scan` (bloquea solo si `orphan_count>0` **y** staged toca `GENOME_PREFIXES`) | ≈0,3 s. **No es el cuello de botella.** Mejora menor: no ejecutar el scan si staged no toca genoma. |
| `pre-push` (rama ya presentada / sin ramas nuevas) | `gh pr view` por rama (red) + `gate-evolution --range --if-touched --sync-base` (`git fetch` con presupuesto 3 s) | 1–4 s de red. |
| `pre-push` (rama nueva) | `route-domain-event Local_QA_Requested` **bloqueante** → `pull-request-review` (fetch + checkout + Argos *Triaje documental* [LLM tier medium] + Cerbero + Argos *Veredicto* [LLM] + Cúmulo + handoff `accept-pr` → Argos *Auditoría Genómica* [LLM] + merge + push + delete) → después `delivery-close-cycle` (push + `gh pr create` + sello ECST) | **Hasta 3 fases LLM síncronas** con `DEFAULT_TIMEOUT_SECS = 660` cada una, más 3–4 operaciones de red. Minutos. **Este es el objetivo.** |
| `post-merge` (en `main`) | `accept-pr` con `merge_already_done: true` → Argos *Auditoría Genómica* [LLM] de nuevo | Fase LLM redundante si la rama ya fue atestada. |

**Fuera de alcance:** introducir `cargo test`/linters en hooks; modificar la semántica de `accept-pr` como SSOT de fusión (`pull-request-orchestration.md` §4); levantar el veto de push a `main`; tocar `telemetry-compliance-audit`.

#### 2. Vectores de Optimización (Especificación)

**2.1 Triaje por delta en `pre-push` (ejecución condicional)**

* **Cálculo sin red:** `pre_push_gate.sh` ya recibe `local_sha`/`remote_sha` por stdin. El delta se obtiene con `git diff --name-only <remote_sha>..<local_sha>` (ref nueva → `origin/main...<local_sha>` si `origin/main` existe; si no, `main`). Prohibido `git diff --name-only HEAD` sin rango (0,9 s medidos por refresco de índice en este repo).
* **Conjunto pasivo (`PASSIVE_PREFIXES`):** `docs/` íntegro (incluye `docs/todos/`, `docs/features/`, `docs/todos/historias/`) y `*.md` fuera de `SddIA/`, `.SddIA/`, `README.md`. Todo lo demás es **activo**. `GENOME_PREFIXES` de `pre_commit_gate.sh` se extrae a `hook_common.sh` como SSOT compartida y se amplía con `SddIA/engine/`, `SddIA/tools/`, `SddIA/scripts/`, `SddIA/core/`, `SddIA/norms/`.
* **Decisión:**
  * Delta ⊆ pasivo → emitir `Local_QA_Requested` con `payload.qa_profile: "docs-only"` y `blocking: true` **solo para el triaje documental determinista** (frontmatter + artefactos de `persist_ref`); las fases LLM de Argos y el handoff `accept-pr` se omiten (`SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF` ya existe como mecanismo; formalizarlo como input del proceso, no como variable de laboratorio). **Laudo D-1 (Vértice Biológico, 2026-10-03): la aduana ligera síncrona es suficiente.** No se dispara `pull-request-review` completo asíncrono tras el push para este perfil; `PullRequest_Presented` sigue emitiéndose por `delivery-close-cycle`, y su suscriptor `argos.pull-request-review` debe **honrar `qa_profile: docs-only` heredado en el payload** (mismo salto de fases LLM) para no reintroducir el coste por la puerta de atrás. La atestación resultante se escribe con `qa_profile: docs-only` y habilita el salto de *Auditoría Genómica* en `post-merge` (§2.2) siempre que `tree_sha` coincida.
  * Delta ∩ activo ≠ ∅ → cadena completa, salvo atestación válida (§2.2).
* **Pre-commit (menor):** ejecutar `audit-eda-coverage --scan` solo si `staged_touches_genome`; `verify-process-integrity` se mantiene siempre (45 ms).

**2.2 Atestación de QA (evidencia física local, reemplaza "ZKP")**

* **Emisor:** `pull-request-review`, al cerrar con `verdict: aprobado`, escribe `{proofs}/qa-attestations/{branch_slug}.json` resolviendo `{proofs}` con `resolve_eda_proofs_dir` (nunca ruta cableada).
* **Esquema mínimo:**

  ```json
  {
    "schema": "qa-attestation/1.0",
    "branch": "feat/x",
    "head_sha": "<40 hex>",
    "tree_sha": "<git rev-parse HEAD^{tree}>",
    "verdict": "aprobado",
    "qa_profile": "full | docs-only",
    "issued_at": "<RFC3339>",
    "ttl_secs": 86400,
    "correlation_id": "<uuid v4 del Local_QA_Requested>",
    "execution_id": "<uuid v4 de pull-request-review>",
    "hmac": "<HMAC-SHA256 del cuerpo canónico con clave local de instancia; opcional en v1>"
  }
  ```

  Anclar a `tree_sha` además de `head_sha`: un `commit --amend` o rebase sin cambio de contenido no invalida la atestación; un cambio de contenido sí.
* **Verificación en `pre-push`** (operación de E/S, sin red, sin LLM): existe atestación para la rama ∧ `tree_sha == git rev-parse <local_sha>^{tree}` ∧ `verdict == aprobado` ∧ `now - issued_at < ttl_secs` ∧ (`qa_profile == full` ∨ delta ⊆ pasivo) → **omitir** `Local_QA_Requested` y pasar directo a `delivery-close-cycle`. Cualquier condición falsa → cadena completa (fail-closed).
* **Verificación en `post-merge`:** `accept-pr` con `merge_already_done` consulta la atestación de `source_branch`; si válida, la fase *Auditoría Genómica* se registra como `executed` con `note: attested-by:<execution_id>` sin invocar LLM.
* **Invalidación (D-2):** la borra la Fase 4 de `accept-pr` al sellar `PullRequest_Merged`. No se añade suscriptor. Así `PullRequest_Merged` conserva sus 3 suscriptores y el PBI 05 solo iguala la tabla documental.

**2.3 Poda de red y binarios en el hilo síncrono**

* `should_skip_pre_push_present`: invertir el orden — primero `scan_presented_for_branch` (bus local, ms), después `gh pr view` (red) solo si el bus no responde.
* `gate-evolution --sync-base`: omitir el `git fetch` si `ref_tracking_age_seconds(origin/main) ≤ STALE_REF_AGE_SECS` (hoy 3600 s) — la lógica de "modo synced por edad" ya existe en `resolve_base`; basta condicionar el fetch a ella.
* `resolve_sddia_qa` (`hook_common.sh`) prefiere `target/debug/sddia-qa` (1,2 GB) sobre `target/release/` (22 MB). Alinear con la política F-DEP-07 de `_sddia_resolve_orchestrator` (release salvo debug estrictamente más nuevo).
* Presupuesto LLM en contexto hook: `invoke_process` exporta `SDDIA_AGENT_RUNTIME_TIMEOUT_SECS` acotado (propuesta 180 s) cuando la invocación proviene de un hook; el timeout de 660 s queda para ejecución desacoplada. Al expirar: fase `failed` → hook bloquea con mensaje accionable (no fail-soft silencioso).

**2.4 Observabilidad (prerequisito de la HU)**

Sin baseline no hay optimización verificable. Dos capas complementarias:

* **Capa hook (shell):** `pre_push_gate.sh` y `post_merge_gate.sh` registran `{hook, branch, delta_class, attestation_hit, invoked_processes[], total_ms}` en `{proofs}/hook-timings/` (append-only, JSONL). Mide el coste total percibido por el desarrollador, incluido overhead de shell, `gh` y `git`.
* **Capa motor (Rust):** ver §2.5-C — `execution_report.json` por ejecución con `elapsed_ms` por fase. Permite atribuir el coste a fases concretas (LLM vs git vs determinista).

Primer entregable del plan: 10 muestras de baseline sobre la cadena actual con ambas capas activas.

**2.5 Deuda colateral incorporada al alcance (laudo 2026-10-03)**

| ID | Deuda | Implementación | Jurisdicción |
| :--- | :--- | :--- | :--- |
| **2.5-A** | `SddIA/norms/pull-request-orchestration.md` §4 y §6 citan `SddIA/process/accept-pr.md` (ruta pre-packing ABSTRACT-03). | Sustituir por resolución multi-root: "proceso `accept-pr` resuelto vía Cúmulo (`process_domain_roots`; ubicación física actual `SddIA/library/codexes/codex-software-engineering/process/accept-pr.md`)". Mismo tratamiento para la mención de `pull-request-review`. Bump `version` 1.1.0 → 1.2.0. | Norma = genoma → `entity-manager` vía `execute-process`. Registro en `SddIA/evolution/`. |
| **2.5-B** | `SddIA/events/domain/pull-request-merged.md` documenta 2 suscriptores; `event-domain-subscriptions.json` tiene 3 (`tekton.tracker-stamp` ausente en el `.md`). | Añadir fila `tracker-stamp · tekton · Sello Linear done + comentario merge (tracker_ref)`. D-2 excluye un suscriptor de invalidación. Bump 1.0.0 → 1.1.0; recalcular `hash_signature`. | Evento = genoma → `entity-manager`. Test `sddia-qa` de paridad tabla «Suscripciones» ↔ clave JSON para **todos** los eventos domain. PBI 05. |
| **2.5-C** | `.SddIA/workspaces/pull-request-review/`: 812 de 817 directorios vacíos. `bootstrap_workspace` crea el directorio por `workspace_template`, pero el `execution_report` solo viaja en el envelope stdout; `thermodynamic::run` recibe únicamente `duration_ms` total. Resultado: entropía física y latencia histórica no atribuible por fase. | (1) En `executor.rs`, cronometrar cada fase (`Instant::now()` alrededor de `execute_phase`) e inyectar `elapsed_ms` en cada entrada de `phase_reports`. (2) Al cerrar `run`, persistir `execution_report.json` (`process_name`, `execution_id`, `correlation_id`, `status_code`, `duration_ms`, `phases[]`) en el workspace ya materializado; escritura best-effort (fail-soft: error de E/S no altera `status_code`). (3) Purga única de directorios vacíos existentes bajo `.SddIA/workspaces/*/` mediante subcomando `sddia-qa workspace-prune --empty --json` (idempotente; informa `pruned_count`); prohibido `rm -rf` manual. (4) Exención: si el proceso está en `thermodynamic::is_exempt`, no se persiste informe (coherencia con peaje). | Motor Rust (`execute-process`, `sddia-qa`): código, no genoma; PR normal con tests. Purga: ejecución documentada en `validacion.md`. |

#### 3. Criterios de Aceptación (Protocolo de Acero)

| ID | Criterio | Verificación |
| :--- | :--- | :--- |
| **AC-1** | `pre-push` de rama **nueva** cuyo delta ⊆ pasivo completa la aduana local (`Local_QA_Requested` perfil `docs-only` + `delivery-close-cycle`) **sin invocar ninguna fase LLM** y con `total_ms` ≤ coste de red de push + `gh pr create` + 2 s. | Test E2E con repo temporal y `SDDIA_EXECUTE_PROCESS_BIN` instrumentado; `hook-timings` muestra `llm_phases: 0`. |
| **AC-2** | Con atestación válida (`tree_sha` coincidente, no caducada, `verdict: aprobado`, perfil `full`), `pre-push` omite `Local_QA_Requested` y `post-merge` omite la fase LLM de *Auditoría Genómica*. | Test de integración: proof sembrada + aserción sobre `execution_report` (`note: attested-by`). |
| **AC-3** | Atestación con `tree_sha` distinto, `verdict ≠ aprobado`, caducada, HMAC inválido (si activo) o ausente ⇒ cadena completa (fail-closed). Nunca se degrada a "omitir por error de lectura". | Red Teaming local: 5 mutaciones del JSON, 5 bloqueos. |
| **AC-4** | Emitido `PullRequest_Merged` para la rama, la atestación deja de existir; un push posterior sobre el mismo nombre de rama no la reutiliza. | Test bus EDA + aserción filesystem. |
| **AC-5** | Los gates existentes **no se degradan**: `verify-process-integrity` sigue bloqueando pre-commit; `audit-eda-coverage` sigue bloqueando cuando staged toca genoma y hay huérfanos; `gate-evolution` sigue bloqueando con material en rango; veto de push a `main` intacto. | Suite `sddia-qa` + tests shell existentes de `pbi-005-hito3-git-hooks`; sin regresión. |
| **AC-6** | `pre-push` de rama **ya presentada** (PR OPEN/MERGED o `PullRequest_Presented` en bus) no ejecuta `gh pr view` si el bus local resuelve; `gate-evolution` no ejecuta `git fetch` con `origin/main` fresco. | Test con `gh` ausente del PATH + ref joven: hook termina exit 0 sin red. |
| **AC-7** | Baseline y post-implementación documentados en `validacion.md` con ≥10 muestras cada uno; reducción de mediana `total_ms` en rama nueva con delta activo ≥ 50 % cuando hay atestación, y ≥ 90 % en delta pasivo. | `hook-timings` JSONL + tabla en `validacion.md`. |
| **AC-8** | Ningún nuevo bypass global: `SDDIA_SKIP_HOOKS` y `SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF` no amplían su alcance; el perfil `docs-only` es un input declarado del proceso `pull-request-review`, no una variable de entorno. | Revisión de diff + `verify-process-integrity` sobre el proceso actualizado. |
| **AC-9** | Rama con delta pasivo: el `pull-request-review` disparado por `PullRequest_Presented` tras el push hereda `qa_profile: docs-only` y **tampoco** ejecuta fases LLM (laudo D-1: aduana ligera única). | Test bus EDA: evento sembrado con `qa_profile` → `execution_report.json` con fases Argos `note: skipped-by-profile`, `llm_phases: 0`. |
| **AC-10** | `pull-request-orchestration.md` no contiene la cadena `SddIA/process/accept-pr.md`; `pull-request-merged.md` lista exactamente los suscriptores de `event-domain-subscriptions.json["PullRequest_Merged"]`; existe comprobación automática de paridad `.md` ↔ JSON para eventos domain. | `rg` sobre la norma; test `sddia-qa` de paridad en verde para todos los eventos domain. |
| **AC-11** | Toda ejecución no exenta de `execute-process` deja `execution_report.json` en su workspace con `elapsed_ms` por fase; `sddia-qa workspace-prune --empty` deja 0 directorios vacíos bajo `.SddIA/workspaces/` y es idempotente (segunda pasada `pruned_count: 0`). | Test de integración motor + ejecución documentada en `validacion.md`. |

#### 4. Riesgos y decisiones abiertas

* **R-1 Falsa pasividad.** Un cambio solo en `docs/features/*/validacion.md` puede alterar el gate documental de Done. Mitigación: el perfil `docs-only` **sí** ejecuta el triaje documental determinista; solo omite LLM y handoff.
* **R-2 Atestación robada entre ramas.** Clave = `branch_slug` + `tree_sha`; dos ramas con árboles idénticos comparten evidencia legítimamente (mismo contenido). Aceptable.
* **R-3 Reloj local.** TTL depende del reloj del host; `hmac` opcional en v1, obligatorio si la instancia es multiusuario (`instances.json`).
* **R-4 Jurisdicción.** Mutar `pull-request-review.md` y `accept-pr.md` (códice) exige `entity-manager` vía `execute-process`, nunca edición manual (`external-ai-constraints.md`). Bump de `version` + `hash_signature` + registro en `SddIA/evolution/`.
* **D-1 — RESUELTA (laudo Vértice Biológico, 2026-10-03): basta la aduana ligera.** Sin doble aduana para `docs-only`. Consecuencia normativa: el suscriptor `argos.pull-request-review` de `PullRequest_Presented` debe respetar `qa_profile` heredado (AC-9); la atestación `docs-only` es evidencia suficiente para `post-merge` mientras `tree_sha` coincida. Riesgo asumido: un cambio documental nunca recibe síntesis LLM de Argos; la cobertura de `README.md` y `SddIA/**/*.md` queda garantizada porque están excluidos del conjunto pasivo (§2.1).

#### 5. Touchpoints Físicos a Modificar

| Artefacto | Cambio |
| :--- | :--- |
| `SddIA/scripts/qa/git-hooks/hook_common.sh` | `GENOME_PREFIXES`/`PASSIVE_PREFIXES` compartidos; `delta_paths <from> <to>`; `delta_class`; `read_qa_attestation`; `resolve_sddia_qa` alineado a F-DEP-07; orden bus→`gh` en `should_skip_pre_push_present`; export de timeout acotado en `invoke_process`; emisión `hook-timings`. |
| `SddIA/scripts/qa/git-hooks/pre_push_gate.sh` | Triaje por delta + verificación de atestación antes de `Local_QA_Requested`; `qa_profile` en payload. |
| `SddIA/scripts/qa/git-hooks/pre_commit_gate.sh` | `audit-eda-coverage` condicionado a `staged_touches_genome`. |
| `SddIA/scripts/qa/git-hooks/post_merge_gate.sh` | Pasar `attestation_ref` a `accept-pr`. |
| `SddIA/tools/sddia-qa/src/gate_evolution.rs` | Fetch condicionado a edad de `origin/main` bajo `--sync-base`. |
| `SddIA/engine/execute-process/src/engine/pull_request_review.rs` | Input `qa_profile` (desde `Local_QA_Requested` o heredado de `PullRequest_Presented`); escritura de atestación al cierre aprobado; salto de fases LLM en `docs-only` con `note: skipped-by-profile`. |
| `SddIA/engine/execute-process/src/engine/accept_pr.rs` | Lectura de atestación en *Auditoría Genómica*; borrado al sellar `PullRequest_Merged`. |
| `SddIA/engine/execute-process/src/engine/executor.rs` | `elapsed_ms` por fase; persistencia best-effort de `execution_report.json` en workspace (2.5-C). |
| `SddIA/tools/sddia-qa/src/` | Subcomando `workspace-prune --empty --json` (2.5-C); test de paridad suscriptores `.md` ↔ `event-domain-subscriptions.json` (2.5-B). |
| `SddIA/library/codexes/codex-software-engineering/process/pull-request-review.md` | v2.4.0: input `qa_profile`, output `attestation_path`. (vía `entity-manager`) |
| `SddIA/library/codexes/codex-software-engineering/process/accept-pr.md` | v1.1.0: input opcional `attestation_ref`. (vía `entity-manager`) |
| `SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md` | Propagar `qa_profile` al payload de `PullRequest_Presented` (laudo D-1). (vía `entity-manager`) |
| `SddIA/events/orchestration/local-qa-requested.md` | v1.1.0: `qa_profile` en OPTIONAL. (vía `entity-manager`) |
| `SddIA/events/domain/pull-request-presented.md` | `qa_profile` en OPTIONAL. (vía `entity-manager`) |
| `SddIA/events/domain/pull-request-merged.md` | v1.1.0: tabla de suscriptores en paridad con JSON (2.5-B). (vía `entity-manager`) |
| `SddIA/norms/pull-request-orchestration.md` | v1.2.0: rutas de `accept-pr` / `pull-request-review` por resolución multi-root (2.5-A). (vía `entity-manager`) |
| `SddIA/core/cumulo.paths.json` | Sin cambio de rutas; namespaces `qa-attestations/` y `hook-timings/` cuelgan de `eda_instance.proofs`. |
| `SddIA/evolution/` | Registro del hito con UUIDs de las entidades mutadas (procesos, eventos, norma). |
| `.SddIA/workspaces/` | Purga única de directorios vacíos vía `sddia-qa workspace-prune` (ejecución registrada en `validacion.md`). |

#### 6. PBI

Cada PBI cierra en su propia rama (`docs/todos/done/` + `validacion.md`). El 05 no bloquea a 01–04.

| Orden | PBI | Entrega | AC |
|------:|-----|---------|----|
| 01 | `PBI-MERGE-THERMO-01-OBSERVABILITY` | `elapsed_ms` por fase, `execution_report.json`, `workspace-prune`, JSONL de hooks, baseline ≥10. | AC-11, AC-7a |
| 02 | `PBI-MERGE-THERMO-02-NET-PRUNE` | Bus antes que `gh`; fetch condicionado; `sddia-qa` según F-DEP-07; timeout de hook 180 s. | AC-6, AC-5a |
| 03 | `PBI-MERGE-THERMO-03-DELTA-PROFILE` | Delta por rango, `qa_profile`, salto LLM en hook y en `PullRequest_Presented` (D-1), pre-commit condicional. | AC-1, AC-8, AC-9, AC-5b |
| 04 | `PBI-MERGE-THERMO-04-ATTESTATION` | Testigo `tree_sha` en `qa-attestations/`; fail-closed; salto de *Auditoría Genómica*; borrado en Fase 4 (D-2). | AC-2, AC-3, AC-4 |
| 05 | `PBI-MERGE-THERMO-05-DOC-PARITY` | Rutas multi-root en la norma; tabla de `PullRequest_Merged` = JSON; test de paridad domain. | AC-10 |
| 06 | `PBI-MERGE-THERMO-06-E2E-MEASURE` | 20 muestras post contra el baseline del 01 y pasada de regresión. | AC-7, AC-5 |
