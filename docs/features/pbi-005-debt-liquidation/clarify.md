---
feature_name: pbi-005-debt-liquidation
created: "2026-05-19"
process: feature
purpose: Estabilización PBI-005 — purga destructiva y suscriptor DLT
---

# Clarificación — PBI-005 (Debt Liquidation)

Transcript de decisiones del Agente de Integración (Arquitecto), 2026-05-19.

---

## D1 — Inicio de feature

| Pregunta | Decisión |
|----------|----------|
| ¿Proceso? | **`feature`** (`SddIA/process/feature.md` v1.2.0). |
| ¿Puerta Física? | `execute-process.py` — fases **simuladas**; Git real vía **`git-manager`**. |
| Rama | `feat/pbi-005-debt-liquidation` desde `main` actualizado (`origin/main`). |
| `persist_ref` | `docs/features/pbi-005-debt-liquidation` |

---

## D2 — Contrato entity-manager (purga)

| Pregunta | Decisión |
|----------|----------|
| ¿Campo de operación? | **`lifecycle_operation: delete`** (no `operation`; colisión con git-manager documentada en `emit-domain-mutation.md`). |
| Entidad de laboratorio | `entity_class: skill`, `entity_name: test-cli-skill`. |
| Reconciliación de índice | Suscriptor async `action: sync-entity-index`; en validación Hito 1 se invoca la cápsula `sync-entity-index.py` con el payload del evento para confirmar purga determinista (equivalente al dispatch del watcher). |

---

## D3 — Expansión DLT en genoma

| Pregunta | Decisión |
|----------|----------|
| ¿Patrón de suscripción? | Duplicar el bloque de `PullRequest_Merged`: `agent: cumulo`, `tool: iota-immutable-publisher`, intent *Anclaje DLT IOTA Rebased.* |
| ¿Reemplazar sync-entity-index? | **No.** Mantener ambos suscriptores en `Domain_Entity_Deleted` (índice + DLT). |
| ¿Ancla de payload en delete? | `hash_signature_old` REQUIRED en clase ECST; `hash_signature_new` FORBIDDEN (`domain-entity-deleted.md`). |

---

## D4 — Deuda explícita post-Hito 1

| Ítem | Notas |
|------|-------|
| Watcher + IOTA en delete | El parche `event-watcher.py` debe enrutar `tool: iota-immutable-publisher` para instancias `Domain_Entity_Deleted` (hoy probado a nivel genoma; ejecución DLT E2E queda para Argos cuando el daemon procese la cola). |
| `execute-action.py` | PBI-005 Hito 2 — no bloquea validación de purga. |
| Git hooks | PBI-005 Hito 3 — automatización PR events. |

---

## D5 — Commits

| Regla | Valor |
|-------|-------|
| Hito 1 | Commit atómico: `event-subscriptions.json` + evidencia documental de feature + estado post-purga (skill eliminado, evento pending). |
| Prohibición | Sin atajos Git crudos fuera de `git-manager` salvo lectura/auditoría. |
