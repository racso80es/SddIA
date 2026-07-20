---
feature_name: kalma2-process-dispatch
created: "2026-07-20"
process: feature
base: main
scope: kalma2-process-dispatch
version_spec: "1.0.0"
uuid: 0f5191df-927b-4da0-acf3-bb715766b5fa
status: dedalo_locked
laudo: B-prime-tqm-native-handler
---

# Especificación — kalma2-process-dispatch

## 1. Topología de responsabilidades

```text
kalma2-interact                    ← matiz A′ (extract_pbi_ref robusto)
   └─ Kalma2_Process_Requested     ← ECST sin cambio de REQUIRED
event-watcher → route-domain-event
   └─ dispatch_subscriber (rama Kalma2)  ← sin cambio de contrato
        └─ execute-process --process task-queue-manager
             └─ handlers::task_queue_manager (NUEVO)
                  └─ invoke hijo: bug-fix | feature | refactorization
```

Invariantes: bridge sin write al bus; C2 async (emisión ya desacoplada); suscriptor fijo TQM **no** derogado.

## 2. Laudos Dedalo

| Ref | Pregunta | Laudo | Justificación |
|-----|----------|-------|---------------|
| **L1** | B′ vs C | **B′** — handler nativo `task-queue-manager` | Respeta laudo O14/P1 (suscriptor fijo); C contradice allowlist-de-inyección |
| **L2** | ¿Ciclo hijo completo hasta PR? | **Arranque del ciclo**, no auto-PR | Con `correlation_id` (paquete Kalma2), el hijo hereda/ fuerza skip de archive+delivery salvo `SDDIA_TQM_FULL_CYCLE=1`; evita nuevo dead-letter por delivery sin agentes IDE |
| **L3** | ¿Inyectar solo `tasks_path`? | **Rechazado** como remedio único | Con `tasks_path` TQM «success» simula fases y **no** despacha el hijo (hueco O3) |
| **L4** | A′ `pbi_ref` | **Obligatorio** | Extraer `docs/todos/.../*.md` por anclas, no por tokens whitespace |
| **L5** | IOTA co-fail | **Fuera** | Q1 Racso |

### Causa raíz empírica (a7725b42)

```text
INPUT_VALIDATION missing: tasks_path
```

El dispatcher entrega `{correlation_id, process, task_text}` a TQM; el residual exige `tasks_path` → `failed` → dead-letter.

## 3. Contrato handler `task-queue-manager`

### 3.1 Entrada Kalma2 (paquete)

| Campo | Obligatorio | Notas |
|-------|:-----------:|-------|
| `process` | sí* | `bug-fix` \| `feature` \| `refactorization` |
| `task_text` | sí* | semilla; alias semántico de `raw_text` ya mapeado |
| `correlation_id` | rec. | `≡ event_id` del dominio |
| `pbi_ref` | no | si ausente, intentar extraer de `task_text` (misma regla A′) |

\*Si `process` ausente o es `task-queue-manager`: modo legado → exigir/default `tasks_path`=`docs/todos` y delegar residual simulado (compat).

### 3.2 Mapeo a hijo

| `process` | Inputs hijo mínimos |
|-----------|---------------------|
| `bug-fix` | `bug_summary←task_text`, `fix_name`, `branch_name=fix/{slug}`, `pbi_ref?`, `correlation_id?` |
| `feature` | `refined_requirements←task_text`, `feature_name`, `branch_name=feat/{slug}`, … |
| `refactorization` | `refactor_goal←task_text`, `refined_constraints` default documentado, `refactor_name`, `branch_name=feat/{slug}`, … |

`slug`: preferir hash `(hex)` del `pbi_ref`/texto; si no, stem sanitizado; fallback `kalma2-{8 hex}`.

### 3.3 Salida TQM

Envelope orquestador `success` iff hijo `success`. `data` incluye:

```json
{
  "dispatched_process": "bug-fix",
  "correlation_id": "<uuid>",
  "child": { "success": true, "execution_id": "...", "branch_name": "..." },
  "handler": "task-queue-manager-kalma2"
}
```

## 4. Matiz A′ — `extract_pbi_ref`

Algoritmo: localizar ancla `docs/todos/pending/` o `docs/todos/done/`; tomar hasta el primer `.md` inclusive. No tokenizar por whitespace.

## 5. Genoma

| Artefacto | Acción |
|-----------|--------|
| `SddIA/engine/execute-process` (handler + kalma2 extract) | Mutación código (no genoma indexado) |
| `SddIA/process/task-queue-manager.md` | Diff documental del contrato Kalma2 vía `entity-manager` (si se toca frontmatter/inputs); si solo cuerpo aclaratorio, mismo canal |
| Evento / subscriptions | **Sin cambio** |

## 6. Criterios de aceptación (Argos)

| ID | Criterio |
|----|----------|
| AC1 | TQM con paquete Kalma2 tipo a7725b42 → `success` (no `INPUT_VALIDATION`) |
| AC2 | Hijo `bug-fix`/`feature` arranca (`workspace-init` executed o simulado con SKIP_GIT) |
| AC3 | Prompt con path espaciado → payload/`pbi_ref` poblado en emisión |
| AC4 | Bridge/app.js sin cambios de emisión EDA |
| AC5 | IOTA no tocado |
