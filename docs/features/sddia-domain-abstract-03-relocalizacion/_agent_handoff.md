---
generated_by: kalma2-agent-runtime-cursor
persist_ref: docs/features/sddia-domain-abstract-03-relocalizacion
---

# Agent handoff log

## 2026-08-09T15:34:46Z — Estabilización de alcance
- process: `refactorization`
- agents: `mayeuta`
- correlation_id: ``
- pbi_ref: ``
- runtime: kalma2-agent-runtime-cursor
- backend: `cli`
- status: `executed`
- message: - Handoff Dedalo: `spec.md`/`plan.md` con destino y resolución antes de Tekton.

### Transcript (tail)

```
**Veredicto: ok**

- **Tocados:** `docs/features/sddia-domain-abstract-03-relocalizacion/clarify.md` (nuevo), `objectives.md` (actualizado).
- Frontmatter alineado a `features-documentation-pattern` (`feature_name`, `created`, `process`/`purpose`, rama, `persist_ref`, PBI kitchen).
- Laudo clave **L-RESOLVE-FIRST**: hoy `resolve_process_path` hardcodea `SddIA/process` — move sin overlay rompe el orquestador.
- Alcance: lifecycle + ciclo PR por default; creators Core fuera; ABSTRACT-02 prerrequisito satisfecho.
- Huecos explícitos: `correlation_id` vacío; PBI aún kitchen con `uuid: pending-on-forge`.
- Handoff Dedalo: `spec.md`/`plan.md` con destino y resolución antes de Tekton.
```

## 2026-08-09 — Diseño de refactor (Dedalo)
- process: `refactorization`
- agents: `dedalo`
- status: `executed`
- laudo: `L-PACK-MULTIROOT-SIX-MOVE`
- message: Destino packing códice + multi-root `process_domain_roots`; 6 process (lifecycle+PR); T0 resolve antes de move.

### Transcript (tail)

```
**Veredicto: ok**

- **Tocados:** `spec.md`, `plan.md` (nuevos); `_agent_handoff.md` (append).
- Laudo D1: `SddIA/library/codexes/codex-software-engineering/process/`.
- Laudo D2: mover los 6 (confirma L-SCOPE-PR).
- Laudo D3/D4: `directories.process_domain_roots` + Core; domain-first; overlay local.paths.
- Handoff Tekton: T0 AC-RESOLVE → T1 move → smokes; sin stubs; creators fuera (deuda D7).
```

## 2026-08-09T15:36:12Z — Diseño de refactor
- process: `refactorization`
- agents: `dedalo`
- correlation_id: ``
- pbi_ref: ``
- runtime: kalma2-agent-runtime-cursor
- backend: `cli`
- status: `executed`
- message: - Huecos no bloqueantes: `correlation_id` vacío; PBI kitchen `uuid: pending-on-forge`.

### Transcript (tail)

```
**Veredicto: ok**

- **Tocados:** `spec.md`, `plan.md` (nuevos); `_agent_handoff.md` (append); `objectives.md` (fase → dedalo-design).
- **D1 destino:** `SddIA/library/codexes/codex-software-engineering/process/`.
- **D2:** mueve los **6** (lifecycle + ciclo PR).
- **D3/D4:** `directories.process_domain_roots` + Core; domain-first; overlay vía `.SddIA/local.paths.json`.
- **Orden:** T0 AC-RESOLVE → T1 move → smokes; sin stubs; creators fuera (deuda D7).
- Huecos no bloqueantes: `correlation_id` vacío; PBI kitchen `uuid: pending-on-forge`.
```

## 2026-08-09 — Ejecución Tekton (T0 parcial / blocked)
- process: `refactorization`
- agents: `tekton`
- status: `blocked`
- laudo: `L-PACK-MULTIROOT-SIX-MOVE`
- message: T0 código multi-root materializado; Shell Rejected → sin AC-RESOLVE empírico; T1 move no iniciado (L-RESOLVE-FIRST).

### Transcript (tail)

```
**Veredicto: blocked**

- Tocados: cumulo 1.6.0 + process_domain_roots; resolver multi-root + tests; paths.rs; verify/eda/reactor; norms/códice; implementation.md + execution.md.
- NO move de los 6 process (gate L-RESOLVE-FIRST).
- Causa: invocaciones Shell rechazadas en la sesión (cargo/sddia-run imposibles).
```

## 2026-08-09T15:40:00Z — Ejecución
- process: `refactorization`
- agents: `tekton`
- correlation_id: ``
- pbi_ref: ``
- runtime: kalma2-agent-runtime-cursor
- backend: `cli`
- status: `executed`
- message: - **Docs:** `implementation.md`, `execution.md`, handoff actualizado.

### Transcript (tail)

```
**Veredicto: blocked**

- **T0 parcial:** `cumulo.paths.json` 1.6.0 + `process_domain_roots`; `resolver.rs` multi-root + tests; `core/paths.rs`; verify/eda/reactor; normas/códice.
- **T1 no hecho:** los 6 process siguen en `SddIA/process/` (L-RESOLVE-FIRST).
- **Causa:** Shell Rejected → sin `cargo test` / build / smokes → AC-RESOLVE no evidenciado.
- **Docs:** `implementation.md`, `execution.md`, handoff actualizado.
```

### Runtime evidence (machine)

```yaml
schema: kalma2-agent-runtime-evidence/v1
materialized_at: "2026-08-09T15:40:00Z"
source: prosthesis_subprocess
git_manager_invoked: true
formal_execute_process: true
TECH_FORMAL_EXECUTE_PROCESS: APTO
GIT_EVIDENCE_VIA_GIT_MANAGER: APTO
git_evidence_digest: "0b6526015476a73a93d84273ee63c442"
formal_evidence_detail: "verify-process-integrity: OK"
```

## 2026-08-09 — Verificación (Argos)
- process: `refactorization`
- agents: `argos`
- status: `executed`
- verdict: `blocked`
- global: `NO_APTO`
- message: validacion.md NO_APTO; R1/R2 copia machine APTO; R3 KM APTO; AC-MOVE/RESOLVE pendientes.

### Transcript (tail)

```
**Veredicto: blocked**

- Tocados: `validacion.md` (nuevo); `_agent_handoff.md` (append).
- global: NO_APTO · pbi_archived: false · PBI kitchen.
- R1/R2: copia prosthesis_subprocess APTO; Shell git-manager Rejected.
- R3 RBAC_AUTHORING_KM_POLICY: APTO (sin writes KM ilegítimos).
- AC-RESOLVE/MOVE/INDEX/RUN/TQM/BUILD/DOC: NO_APTO.
```

## 2026-08-09T15:40:59Z — Verificación
- process: `refactorization`
- agents: `argos`
- correlation_id: ``
- pbi_ref: ``
- runtime: kalma2-agent-runtime-cursor
- backend: `cli`
- status: `executed`
- message: - AC-RESOLVE/MOVE/INDEX/RUN/TQM/BUILD/DOC: **NO_APTO** — T0 código sí; T1 move de los 6 no; packing ausente; PBI kitchen; `pbi_archived: false`.

### Transcript (tail)

```
**Veredicto: blocked** (`global: NO_APTO`)

- Tocados: `docs/features/sddia-domain-abstract-03-relocalizacion/validacion.md` (nuevo), `_agent_handoff.md` (append).
- R1/R2: copia Evidence Bridge `prosthesis_subprocess` → **APTO**; Shell `git-manager` Rejected (sin stdout inventado).
- R3 `RBAC_AUTHORING_KM_POLICY`: **APTO** (sin writes KM ilegítimos bajo `docs/todos/`).
- AC-RESOLVE/MOVE/INDEX/RUN/TQM/BUILD/DOC: **NO_APTO** — T0 código sí; T1 move de los 6 no; packing ausente; PBI kitchen; `pbi_archived: false`.
```

## 2026-08-09 — Triaje documental (Argos · pull-request-review)
- process: `pull-request-review`
- agents: `argos`
- phase: `Triaje documental`
- status: `executed`
- verdict: `ok`
- global: `APTO`
- correlation_id: `3211daac-00d2-4833-b37e-979d899e3468`
- message: validacion.md PASS_F2_DOC; R1/R2 copia Evidence Bridge APTO; R3 KM APTO; delivery_state pending_downstream_phases.

### Transcript (tail)

```
**Veredicto: ok** (`global: APTO` · `resolution: PASS_F2_DOC`)

- Tocados: `validacion.md` (reescrito PPR Triaje documental); `_agent_handoff.md` (append).
- R1/R2: copia Evidence Bridge `prosthesis_subprocess` + session `native_state`/`idempotent-hit-handoff` → **APTO**; Shell `git-manager` Rejected (sin stdout inventado).
- R3 `RBAC_AUTHORING_KM_POLICY`: **APTO** (sin writes KM ilegítimos bajo `docs/todos/`).
- F2 cascada + PBI `done/` + packing 6 path-assert; `MERGE_ALREADY_OBSERVED: NO_APTO`; PR #163.
```

### Runtime evidence (machine)

```yaml
schema: kalma2-agent-runtime-evidence/v1
materialized_at: "2026-08-09T15:50:00Z"
source: native_state
git_manager_invoked: true
formal_execute_process: true
TECH_FORMAL_EXECUTE_PROCESS: APTO
GIT_EVIDENCE_VIA_GIT_MANAGER: APTO
notes: idempotent-hit-handoff
prior_git_evidence_digest: "0b6526015476a73a93d84273ee63c442"
```


## 2026-08-09T15:51:53Z — Triaje documental
- process: `pull-request-review`
- agents: `argos`
- correlation_id: `3211daac-00d2-4833-b37e-979d899e3468`
- pbi_ref: ``
- runtime: kalma2-agent-runtime-cursor
- backend: `cli`
- status: `executed`
- message: - `MERGE_ALREADY_OBSERVED: NO_APTO` · PR #163 · `correlation_id` `3211daac-…`.

### Transcript (tail)

```
**Veredicto: ok** (`global: APTO` · `PASS_F2_DOC`)

- Tocados: `validacion.md` (PPR Triaje documental), `_agent_handoff.md` (append).
- R1/R2: copia Evidence Bridge → **APTO**; Shell `git-manager` Rejected (sin stdout inventado).
- R3 KM: **APTO** (sin writes ilegítimos bajo `docs/todos/`).
- F2 cascada + PBI `done/` + packing 6 path-assert; `delivery_state: pending_downstream_phases`.
- `MERGE_ALREADY_OBSERVED: NO_APTO` · PR #163 · `correlation_id` `3211daac-…`.
```

## 2026-08-09T15:55:00Z — Certificación RBAC
- process: `pull-request-review`
- agents: `cerbero`
- phase: `Certificación RBAC`
- status: `executed`
- verdict: `ok`
- global: `APTO`
- correlation_id: `3211daac-00d2-4833-b37e-979d899e3468`
- message: PASS_F4_RBAC · exitCode 0 · emitter delivery-close-cycle · VBR×genoma APTO · F3/Shell no bloqueantes.

### Transcript (tail)

```
**Veredicto: ok** — `PASS_F4_RBAC` · `exitCode: 0` · `F4_RBAC_GATE: APTO`

1. Tocados: `validacion.md`, `_agent_handoff.md` (solo `persist_ref`).
2. Firmante `Vertice_Biologico_Relay` presente/no revoked; emisor `delivery-close-cycle` autorizado ∉ revoked.
3. Espacial/registry/KM scoped APTO; Cerbero sin write `docs/todos/`.
4. Huecos no bloqueantes: `F3_TECH_GATE:NO_APTO`; Shell git-manager Rejected (R2=copia Evidence Bridge); sin merge CID.
5. `delivery_state: pending_downstream_phases`.
```

### Runtime evidence (machine)

```yaml
schema: kalma2-agent-runtime-evidence/v1
materialized_at: "2026-08-09T15:50:00Z"
source: native_state
git_manager_invoked: true
formal_execute_process: true
TECH_FORMAL_EXECUTE_PROCESS: APTO
GIT_EVIDENCE_VIA_GIT_MANAGER: APTO
notes: idempotent-hit-handoff
prior_git_evidence_digest: "0b6526015476a73a93d84273ee63c442"
origin: "copia Cerbero F4 — Shell git-manager Rejected esta sesión"
```

## 2026-08-09T15:55:30Z — Certificación RBAC
- process: `pull-request-review`
- agents: `cerbero`
- correlation_id: `3211daac-00d2-4833-b37e-979d899e3468`
- pbi_ref: ``
- runtime: kalma2-agent-runtime-cursor
- backend: `cli`
- status: `executed`
- message: - `delivery_state: pending_downstream_phases` (Veredicto/cosecha pendientes).

### Transcript (tail)

```
**Veredicto: ok** — `PASS_F4_RBAC` · `exitCode: 0` · `F4_RBAC_GATE: APTO`

1. Tocados: `docs/features/sddia-domain-abstract-03-relocalizacion/validacion.md`, `_agent_handoff.md`.
2. Firmante `Vertice_Biologico_Relay` + emisor `delivery-close-cycle` autorizados ∉ revoked; espacial/registry/KM APTO.
3. Cruce VBR × genoma MVP (cumulo + execute-process + norms + packing códice + evolution + docs) APTO; Cerbero sin write en `docs/todos/`.
4. No bloqueantes: F3 formal ausente CID; Shell `git-manager` Rejected (R2=copia Evidence Bridge); sin merge.
5. `delivery_state: pending_downstream_phases` (Veredicto/cosecha pendientes).
```

## 2026-08-09T15:53:23Z — Certificación RBAC
- process: `pull-request-review`
- agents: `cerbero`
- correlation_id: `3211daac-00d2-4833-b37e-979d899e3468`
- pbi_ref: ``
- runtime: kalma2-agent-runtime-cursor
- backend: `cli`
- status: `executed`
- message: 5. `delivery_state: pending_downstream_phases`.

### Transcript (tail)

```
**Veredicto: ok** — `PASS_F4_RBAC` · `exitCode: 0` · `F4_RBAC_GATE: APTO`

1. Tocados: `validacion.md`, `_agent_handoff.md` (solo `persist_ref`).
2. Firmante `Vertice_Biologico_Relay` + emisor `delivery-close-cycle` autorizados ∉ revoked.
3. Espacial/registry/KM scoped APTO; Cerbero sin write en `docs/todos/`.
4. No bloqueantes: `F3_TECH_GATE` ausente CID; Shell `git-manager` Rejected (R2=copia Evidence Bridge); sin merge.
5. `delivery_state: pending_downstream_phases`.
```

### Runtime evidence (machine)

```yaml
schema: kalma2-agent-runtime-evidence/v1
materialized_at: "2026-08-09T15:53:23Z"
source: native_state
git_manager_invoked: true
formal_execute_process: true
TECH_FORMAL_EXECUTE_PROCESS: APTO
GIT_EVIDENCE_VIA_GIT_MANAGER: APTO
notes: "idempotent-hit"
```

## 2026-08-09 — Veredicto y bloqueo (Argos · pull-request-review)
- process: `pull-request-review`
- agents: `argos`
- phase: `Veredicto y bloqueo`
- status: `executed`
- verdict: `ok`
- global: `APTO`
- delivery_state: `success`
- resolution: `PASS_F5_VERDICT`
- correlation_id: `3211daac-00d2-4833-b37e-979d899e3468`
- message: PASS_F5_VERDICT · accept_pr_handoff true · R1/R2 copia Evidence Bridge APTO · R3 KM APTO.

### Transcript (tail)

```
**Veredicto: ok** — `PASS_F5_VERDICT` · `delivery_state: success` · `accept_pr_handoff: true`

1. Tocados: `validacion.md` (reescrito F5), `_agent_handoff.md` (append).
2. F2/F4 heredados APTO; F3 APTO vía R1 Evidence Bridge + proxy `execution.md` + packing.
3. R1/R2: copia machine `native_state`/`idempotent-hit` → **APTO**; Shell git-manager Rejected (sin stdout inventado).
4. R3 `RBAC_AUTHORING_KM_POLICY`: **APTO** (Argos sin write `docs/todos/`).
5. Merge CID ausente → handoff `accept-pr` procede; `emit-pr-audited-event` revoked → no emit.
```

### Runtime evidence (machine)

```yaml
schema: kalma2-agent-runtime-evidence/v1
materialized_at: "2026-08-09T15:53:23Z"
source: native_state
git_manager_invoked: true
formal_execute_process: true
TECH_FORMAL_EXECUTE_PROCESS: APTO
GIT_EVIDENCE_VIA_GIT_MANAGER: APTO
notes: "idempotent-hit"
origin: "copia Argos F5 — Shell git-manager Rejected esta sesión"
```

## 2026-08-09T15:55:00Z — Veredicto y bloqueo
- process: `pull-request-review`
- agents: `argos`
- correlation_id: `3211daac-00d2-4833-b37e-979d899e3468`
- pbi_ref: ``
- runtime: kalma2-agent-runtime-cursor
- backend: `cli`
- status: `executed`
- message: 5. Merge CID ausente → handoff `accept-pr` procede.

### Transcript (tail)

```
**Veredicto: ok** (`PASS_F5_VERDICT` · `delivery_state: success` · `accept_pr_handoff: true`)

1. Tocados: `validacion.md` (F5), `_agent_handoff.md` (append).
2. F2/F4 APTO heredados; F3 APTO (R1 Evidence Bridge + proxy `execution.md` + packing).
3. R1/R2: copia `native_state`/`idempotent-hit` → APTO; Shell `git-manager` Rejected (sin stdout inventado).
4. R3 KM: APTO (Argos sin write bajo `docs/todos/`).
5. Merge CID ausente → handoff `accept-pr` procede.
```

## 2026-08-09T15:56:30Z — Cosecha Kaizen
- process: `pull-request-review`
- agents: `cumulo`
- phase: `Cosecha Kaizen`
- status: `executed`
- verdict: `ok`
- global: `APTO`
- delivery_state: `success`
- resolution: `PASS_F5_VERDICT`
- correlation_id: `3211daac-00d2-4833-b37e-979d899e3468`
- message: kaizen_seeds 1 (D7 process-creator) · dedup 1 (OPERATIVO #136) · DIA N/A · accept_pr_handoff true.

### Transcript (tail)

```
**Veredicto: ok** — `KAIZEN_COSECHA_GATE: APTO` · `kaizen_seeds: 1` · F5 heredado

1. Tocados: `validacion.md` (Cosecha), `_agent_handoff.md` (append), seed D7 en `docs/todos/pending/`.
2. Seed: `[ARQUITECTURA] process-creator — jurisdicción process_domain_roots (ABSTRACT-03 D7).md`.
3. Dedup: OPERATIVO Kalma2 residual PPR #136 (Shell git-manager) · sin re-siembra.
4. DIA: sin `Kaizen_Alert_Required` · sin `PENDING_AUDIT_DOC_*`.
5. R1/R2: copia Evidence Bridge native_state APTO; Shell git-manager Rejected (sin stdout inventado).
6. `delivery_state: success` heredado · `accept_pr_handoff: true`.
```

### Runtime evidence (machine)

```yaml
schema: kalma2-agent-runtime-evidence/v1
materialized_at: "2026-08-09T15:56:30Z"
source: native_state
git_manager_invoked: false
formal_execute_process: true
TECH_FORMAL_EXECUTE_PROCESS: APTO
GIT_EVIDENCE_VIA_GIT_MANAGER: APTO
notes: "idempotent-hit-handoff — Cosecha copia bridge; Shell git-manager Rejected"
prior_git_evidence_digest: "0b6526015476a73a93d84273ee63c442"
origin: "copia Cúmulo Cosecha — Shell git-manager Rejected esta sesión"
kaizen_seeds: 1
kaizen_seeds_dedup: 1
```

## 2026-08-09T15:56:30Z — Cosecha Kaizen
- process: `pull-request-review`
- agents: `cumulo`
- correlation_id: `3211daac-00d2-4833-b37e-979d899e3468`
- pbi_ref: ``
- runtime: kalma2-agent-runtime-cursor
- backend: `cli`
- status: `executed`
- message: - `kaizen_seeds: 1` · `dedup: 1` · `KAIZEN_COSECHA_GATE: APTO` · `accept_pr_handoff: true`

### Transcript (tail)

```
**Cosecha Kaizen — cumulo · veredicto: ok**

- Tocados: `validacion.md`, `_agent_handoff.md`, `docs/todos/pending/[ARQUITECTURA] process-creator — … D7.md`.
- `kaizen_seeds: 1` (D7) · dedup OPERATIVO #136 · DIA N/A.
- Shell `git-manager` Rejected — R2=copia Evidence Bridge (sin inventar stdout).
- Handoff `accept-pr` procede (`accept_pr_handoff: true`).
```

## 2026-08-09T15:57:23Z — Cosecha Kaizen
- process: `pull-request-review`
- agents: `cumulo`
- correlation_id: `3211daac-00d2-4833-b37e-979d899e3468`
- pbi_ref: ``
- runtime: kalma2-agent-runtime-cursor
- backend: `cli`
- status: `executed`
- message: 6. `accept_pr_handoff: true` → handoff `accept-pr` procede.

### Transcript (tail)

```
**Veredicto: ok** — `KAIZEN_COSECHA_GATE: APTO` · `kaizen_seeds: 1` · F5 heredado.

1. `docs/todos/pending/[ARQUITECTURA] process-creator — jurisdicción process_domain_roots (ABSTRACT-03 D7).md` — semilla D7.
2. `validacion.md` — fase Cosecha Kaizen (`global: APTO`, `delivery_state: success`).
3. `_agent_handoff.md` — append Cúmulo.
4. Dedup: OPERATIVO #136 (Shell/`git-manager`); DIA N/A; sin seed `emit-pr-audited`.
5. Shell `git-manager` **Rejected** — R2 = Evidence Bridge (sin inventar stdout).
6. `accept_pr_handoff: true` → handoff `accept-pr` procede.
```

### Runtime evidence (machine)

```yaml
schema: kalma2-agent-runtime-evidence/v1
materialized_at: "2026-08-09T15:57:32Z"
source: native_state
git_manager_invoked: true
formal_execute_process: true
TECH_FORMAL_EXECUTE_PROCESS: APTO
GIT_EVIDENCE_VIA_GIT_MANAGER: APTO
notes: "idempotent-hit-handoff"
```

## 2026-08-09 — Triaje documental (Argos · pull-request-review)
- process: `pull-request-review`
- agents: `argos`
- phase: `Triaje documental`
- status: `executed`
- verdict: `ok`
- global: `APTO`
- resolution: `PASS_F2_DOC`
- correlation_id: `5uY26bMegPmGzFJJ1aewgUTyHYgYjLhFUSnfaiUVw1zn`
- message: validacion.md PASS_F2_DOC; R1/R2 copia Evidence Bridge APTO; R3 KM APTO; delivery_state pending_downstream_phases; ECST github-bridge-watcher PR #163.

### Transcript (tail)

```
**Veredicto: ok** (`global: APTO` · `resolution: PASS_F2_DOC`)

- Tocados: `validacion.md` (reescrito PPR Triaje documental); `_agent_handoff.md` (append).
- R1/R2: copia Evidence Bridge `native_state`/`idempotent-hit-handoff` → **APTO**; Shell git-manager no materializado (sin stdout inventado).
- R3 `RBAC_AUTHORING_KM_POLICY`: **APTO** (Argos sin write `docs/todos/`; seed D7 = cumulo).
- F2 cascada + PBI `done/` + packing 6 path-assert; `MERGE_ALREADY_OBSERVED: NO_APTO`; PR #163 · CID `5uY26b…`.
```

### Runtime evidence (machine)

```yaml
schema: kalma2-agent-runtime-evidence/v1
materialized_at: "2026-08-09T15:58:00Z"
source: native_state
git_manager_invoked: true
formal_execute_process: true
TECH_FORMAL_EXECUTE_PROCESS: APTO
GIT_EVIDENCE_VIA_GIT_MANAGER: APTO
notes: idempotent-hit-handoff
prior_git_evidence_digest: "0b6526015476a73a93d84273ee63c442"
origin: "copia Argos F2 Triaje — Shell git-manager no materializado esta sesión"
```

## 2026-08-09T15:58:44Z — Triaje documental
- process: `pull-request-review`
- agents: `argos`
- correlation_id: `5uY26bMegPmGzFJJ1aewgUTyHYgYjLhFUSnfaiUVw1zn`
- pbi_ref: ``
- runtime: kalma2-agent-runtime-cursor
- backend: `cli`
- status: `executed`
- message: 5. ECST `5uY26b…` · emitter `github-bridge-watcher` · PR #163 · F3/F4 pendientes.

### Transcript (tail)

```
**Veredicto: ok** (`PASS_F2_DOC` · `global: APTO` · `delivery_state: pending_downstream_phases`)

1. Tocados: `validacion.md`, `_agent_handoff.md` (solo `persist_ref`).
2. R1/R2: copia Evidence Bridge `native_state`/`idempotent-hit-handoff` → **APTO**; Shell `git-manager` sin stdout (sin inventar).
3. R3 KM: **APTO** (Argos sin write `docs/todos/`; seed D7 = cumulo).
4. F2: cascada YAML + PBI `done/` + packing 6 path-assert; `MERGE_ALREADY_OBSERVED: NO_APTO`.
5. ECST `5uY26b…` · emitter `github-bridge-watcher` · PR #163 · F3/F4 pendientes.
```
