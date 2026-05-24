---
feature_name: vanguardia-soberania-local
created: "2026-05-24"
process: feature
purpose: Sellar puerta de entrada local — accept-pr higiene auditable + aduana ECST pre-pending
---

# Clarificación — Vanguardia Soberanía Local (L.1 + E.2)

Transcript de decisiones (2026-05-24).

---

## D1 — Inicio formal

| Pregunta | Decisión |
|----------|----------|
| ¿Proceso de inicio? | **`feature`** v1.3.0 |
| Rama | `feat/vanguardia-soberania-local` |
| `persist_ref` | `docs/features/vanguardia-soberania-local` |
| Manifiesto operativo | `docs/todos/pending/[OPERATIVO] Backlog pendiente post-PR11 — Hito 3, Ola C y laboratorio.md` § Prioridad 1 |
| FIX absorbido | `docs/todos/pending/[FIX] accept-pr — higiene silenciosa delete_branch tras merge.md` → track L.1 |

---

## D2 — Triaje L.1: causa raíz empírica Fase 4

| Hallazgo | Evidencia | Decisión |
|----------|-----------|----------|
| Anti-patrón `except RuntimeError: closed = None` | `capsule_accept_sync_cleanup` L437–445 | **Erradicar** |
| Payload `delete_branch` inválido | Cápsula envía `"remote": "origin"` (string); `git-manager` frozen exige `remote: boolean` + `force: boolean` | **Corregir** a dos invocaciones: local (`remote: false`) + remoto (`remote: true`) |
| Incidente PR #36 | Rama `feat/pull-request-automation-dlt` sobrevivió post-merge | Smoke regresión obligatorio |
| Homólogo `capsule_delivery_local_hygiene` | Mismo anti-patrón L282–287 | **Mismo tratamiento** en este PR |

---

## D3 — Semántica `hygiene_failure` vs éxito del proceso

| Pregunta | Decisión |
|----------|----------|
| ¿Merge + push OK pero delete falla aborta `accept-pr`? | **No** — merge ya materializado; `verdict: aprobado`, `status_code: 0`, con `hygiene_failure` explícito |
| ¿Push falla? | **Sí aborta** fase 4 — excepción no enmascarada; delete no ejecutar |
| Nodo de error | **`hygiene_failure`** (canónico); reservar `errors[]` solo si múltiples capas futuras |
| `closed_branch: null` sin intento delete | Permitido solo si `source_branch` ausente/vacío (sin `hygiene_failure`) |
| `closed_branch: null` tras intento delete | **Obligatorio** `hygiene_failure.survived_branch` |

---

## D4 — Contrato git-manager (frozen)

| Operación | Payload |
|-----------|---------|
| Delete local | `{ "branch_name": "<branch>", "remote": false, "force": false }` → `git branch -d` |
| Delete remoto | `{ "branch_name": "<branch>", "remote": true, "force": false }` → `git push origin --delete` |
| `closed_branch` output proceso | Nombre rama **solo** si ambas ops `success: true` |

---

## D5 — Triaje E.2: aduana pre-`pending/`

| Hallazgo | Decisión |
|----------|----------|
| Validación ECST solo en router | `route_domain_event_core.load_event_class_schemas` + `validate_ecst_instance` — **extraer** a módulo compartido |
| Emisores sin aduana | `_run_emit_domain_mutation` (`execute-action.py`) y `emit_domain_mutation` / `capsule_emit_domain_mutation` (`execute_process_capsules.py`) — **invocar aduana antes de write** |
| Deuda Ola C V3 §2 | Cerrar aquí; no delegar al sweeper |
| `payload_schema_hash` REQUIRED | **Fuera de alcance** — permanece OPTIONAL |

---

## D6 — Módulo compartido ECST

| Pregunta | Decisión |
|----------|----------|
| Nombre | `SddIA/scripts/qa/ecst_validation.py` |
| API mínima | `load_event_class_schemas(repo) -> dict`, `validate_ecst_instance(event, schema) -> tuple[bool, list[str]]`, `validate_domain_mutation_event(repo, event) -> tuple[bool, list[str]]` |
| Router | `route_domain_event_core.py` importa desde módulo (sin duplicar lógica) |
| Fuente schemas | Tablas REQUIRED/OPTIONAL/FORBIDDEN en `SddIA/events/*.md` vía `index.md` |

---

## D7 — Orquestación tracks

| Pregunta | Decisión |
|----------|----------|
| ¿Paralelo o secuencial? | **Paralelo** — L.1 y E.2 independientes |
| ¿Un PR o dos? | **Un PR** — revisión unificada, commits atómicos por track |
| Orden plan | Hito 0 (módulo ECST) → tracks L.1 / E.2 en paralelo → smoke conjunto → validación |

---

## D8 — Referencias cruzadas

| Artefacto | Relación |
|-----------|----------|
| `pull-request-review-redesign` | Upstream handoff → `accept-pr` |
| `pbi-005-hito3-ola-b` | Hook `post-merge` invoca `accept-pr`; no modificar hooks |
| `ola-c-event-entity` | Contrato bus + clases ECST |
| `emit-domain-mutation.md` | Genoma acción — añadir Paso 1b |
| `accept-pr.md` | Genoma proceso — § Fase 4 higiene |

---

## D9 — Smoke y lab

| Escenario | Track | Perfil |
|-----------|-------|--------|
| Post-merge delete OK | L.1 | Rama efímera lab + `accept-pr` |
| Post-merge delete fallo forzado | L.1 | Payload git-manager inválido **o** rama inexistente remoto → `hygiene_failure` |
| Emisión válida | E.2 | `--action emit-domain-mutation` create smoke |
| Emisión REQUIRED ausente | E.2 | Abort sin archivo en `pending/` |
| Router regresión | E.2 | Evento válido sigue enrutando post-aduana emisor |

Variables lab existentes: `SDDIA_LAB_SKIP_GIT_PUSH`, `SDDIA_SKIP_HOOKS` — sin cambio semántico.
