---
feature_name: vanguardia-soberania-local
process: feature
created: "2026-05-24"
persist_ref: docs/features/vanguardia-soberania-local
branch_name: feat/vanguardia-soberania-local
related_todo: docs/todos/pending/[OPERATIVO] Backlog pendiente post-PR11 — Hito 3, Ola C y laboratorio.md
related_fix: docs/todos/done/[FIX] accept-pr — higiene silenciosa delete_branch tras merge.md
tracks:
  - L.1
  - E.2
status: entregado
updated: "2026-05-24"
feature_ref: docs/features/vanguardia-soberania-local
---

# Objetivos — Vanguardia de Fricción: Soberanía Local (L.1 + E.2)

## Meta

Sellar la **puerta de entrada** del sistema local: fusión PR determinista e higiénica vía `accept-pr` (L.1) y aduana ECST en `emit-domain-mutation` antes de contaminar `pending/` (E.2). Hasta erradicar el uso suelto del gestor de repositorios y asegurar la higiene total de ramas tras la fusión, el ciclo de vida del código opera sobre bases inestables. En paralelo, el bus de eventos no puede seguir expuesto a mutaciones malformadas.

## Contexto operativo

| Hecho | Implicación |
|-------|-------------|
| Cápsulas `accept-pr` 4 fases en lab (PR #13) | Fases 1–3 operativas; Fase 4 traga fallos de `delete_branch` |
| Incidente PR #36 (`pull-request-automation-dlt`) | Rama feature no borrada; FIX documentado |
| `pull-request-review` → handoff `accept-pr` (PR #15) | Cadena Presented→Review→Accept probada |
| `emit-domain-mutation` v1.0.0 forjada | Validación de inputs; **sin** aduana ECST pre-disco |
| `route_domain_event_core.validate_ecst_instance` | Existe en router; emisores no la invocan aún |
| Deuda Ola C V3 §2 línea 50 | Validación en emisor pendiente — se cierra aquí |

## Objetivos medibles

### Track L.1 — `accept-pr` cápsula estricta

| ID | Objetivo | Criterio |
|----|----------|----------|
| **L1-O1** | **Anti-supresión Fase 4** | Cero `except RuntimeError: closed = None` en `capsule_accept_sync_cleanup` |
| **L1-O2** | **Payload empírico** | Fallo delete → nodo `hygiene_failure` (o `errors[]`) en retorno de fase + `execution_report` |
| **L1-O3** | **Trazabilidad higiene** | `closed_branch` solo si local **y** remoto OK; si no → `closed_branch: null` + `hygiene_failure.survived_branch` |
| **L1-O4** | **Smoke post-merge** | Rama eliminada **o** `hygiene_failure` explícito (nunca silencio) |
| **L1-O5** | **Genoma + runbook** | `accept-pr.md` § Fase 4 + guías sin `git-manager` suelto |

#### L1-SPEC — `capsule_accept_sync_cleanup` (kernel)

**Target:** `SddIA/scripts/qa/execute_process_capsules.py`

```python
# FORBIDDEN — erradicar
try:
    invoke_git_manager(repo, "delete_branch", {...})
    closed = source.strip()
except RuntimeError:
    closed = None
```

```python
# REQUIRED — secuencia física (contrato git-manager frozen)
invoke_git_manager(repo, "delete_branch", {
    "branch_name": branch,
    "remote": False,
    "force": False,
})  # git branch -d

invoke_git_manager(repo, "delete_branch", {
    "branch_name": branch,
    "remote": True,
    "force": False,
})  # git push origin --delete
```

```python
# REQUIRED — propagación de fallo (no enmascarar)
# RuntimeError / exitCode != 0 → capturar mensaje, NO tragar
# closed_branch = branch IFF ambas ops success
# else closed_branch = None + hygiene_failure poblado
```

**Retorno fase — éxito:**

```json
{
  "status": "executed",
  "handler": "accept-sync-cleanup",
  "push": { "gitStdout": "...", "gitStderr": null },
  "closed_branch": "feat/example"
}
```

**Retorno fase — higiene fallida (rama sobrevivió):**

```json
{
  "status": "executed",
  "handler": "accept-sync-cleanup",
  "push": { "gitStdout": "...", "gitStderr": null },
  "closed_branch": null,
  "hygiene_failure": {
    "survived_branch": "feat/example",
    "branch_deleted_local": false,
    "branch_deleted_remote": false,
    "operations": [
      {
        "op": "delete_branch_local",
        "command": "git branch -d",
        "success": false,
        "error": "<stderr|RuntimeError message>"
      },
      {
        "op": "delete_branch_remote",
        "command": "git push origin --delete",
        "success": false,
        "error": "<stderr|RuntimeError message>"
      }
    ]
  }
}
```

**Retorno fase — higiene parcial:**

```json
{
  "closed_branch": null,
  "hygiene_failure": {
    "survived_branch": "feat/example",
    "branch_deleted_local": true,
    "branch_deleted_remote": false,
    "operations": [
      { "op": "delete_branch_local", "success": true },
      { "op": "delete_branch_remote", "success": false, "error": "..." }
    ]
  }
}
```

**`execution_report.phases[]` — Fase 4 con fallo:**

```json
{
  "phase_name": "Sincronización y Limpieza",
  "status": "executed",
  "handler": "accept-sync-cleanup",
  "closed_branch": null,
  "hygiene_failure": { "...": "..." }
}
```

**`execute-process` stdout — contrato cápsula (JSON puro, una emisión):**

```json
{
  "success": true,
  "status_code": 0,
  "data": {
    "process_name": "accept-pr",
    "verdict": "aprobado",
    "merge_commit_hash": "...",
    "event_id": "...",
    "closed_branch": null,
    "hygiene_failure": { "...": "..." }
  },
  "execution_report": {
    "process_name": "accept-pr",
    "phases": [ "... incluye hygiene_failure en fase 4 ..." ]
  },
  "error": null
}
```

**Reglas:**

| Regla | Valor |
|-------|-------|
| Suprimir `RuntimeError` en delete | **PROHIBIDO** |
| `closed_branch: null` sin `hygiene_failure` tras intento delete | **PROHIBIDO** |
| Texto humano fuera de JSON en stdout | **PROHIBIDO** |
| `state["hygiene_failure"]` | propagar a `data` si presente |
| Push fallido | abortar fase (excepción no enmascarada); delete no ejecutar |

**Alcance mismo patrón:** `capsule_delivery_local_hygiene` (mismo anti-patrón L435–445 homólogo).

### Track E.2 — Aduana ECST en `emit-domain-mutation`

| ID | Objetivo | Criterio |
|----|----------|----------|
| **E2-O1** | **Aduana pre-pending** | Instancia ECST validada contra Clase catalogada **antes** de `WRITE_FILE` |
| **E2-O2** | **Aborto determinista** | Payload malformado → `success: false`, `exitCode: 1`, sin archivo en `pending/` |
| **E2-O3** | **Reutilización SSOT** | Módulo compartido con lógica de `route_domain_event_core` (schemas desde `SddIA/events/`) |
| **E2-O4** | **Smoke emisor** | Evento válido persiste; REQUIRED ausente aborta; `event_type` no catalogado aborta |
| **E2-O5** | **Spec actualizada** | `emit-domain-mutation.md` documenta Paso 1b (aduana ECST) |

## Orquestación

- **L.1** y **E.2** se implementan **en paralelo** (tracks independientes, revisión unificada en un PR si conviene).
- **Precedencia conceptual:** E.2 protege el bus aguas arriba; L.1 protege `main` y ramas aguas abajo del merge.
- Absorber criterios del FIX [`accept-pr — higiene silenciosa delete_branch`](../../todos/pending/[FIX]%20accept-pr%20%E2%80%94%20higiene%20silenciosa%20delete_branch%20tras%20merge.md) en L1-O1–O3.

## No objetivos (esta feature)

- Hooks Git Hito 3 (`pre-push` / `post-merge`) — ya entregados Ola B.
- Rediseño `pull-request-review` o Oráculo DLT — ya en `main`.
- Ola C V3 coreografía (`event-sweeper`, recibos atómicos) — P4 backlog.
- Endurecer `payload_schema_hash` a REQUIRED — deuda `eda-domain-entities-splus`.
- IOTA físico en CI (E.1) — backlog P3 posterior a vanguardia.

## Artefactos previstos

| Track | Rutas principales |
|-------|-------------------|
| L.1 | `SddIA/scripts/qa/execute_process_capsules.py`, `SddIA/process/accept-pr.md`, `SddIA/norms/git-operations.md` |
| E.2 | `SddIA/scripts/qa/execute-action.py`, `execute_process_capsules.py`, módulo ECST compartido, `SddIA/actions/emit-domain-mutation.md` |
| Feature | `clarify.md`, `spec.md`, `plan.md`, smoke JSON, `validacion.md` |

## Estado

| Fase feature | Estado |
|--------------|--------|
| Objetivos | ✅ |
| Clarificación | ✅ `clarify.md` |
| Especificación | ✅ `spec.md` |
| Plan | ✅ `plan.md` |
| Implementación | ✅ `implementation.md` |
| Validación | ✅ `validacion.md` APTO |
