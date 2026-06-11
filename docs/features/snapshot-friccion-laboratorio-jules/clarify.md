---
feature_name: snapshot-friccion-laboratorio-jules
process: feature
created: "2026-06-11"
purpose: Estabilización de requisitos — cierre PBI Snapshot Fricción Jules
---

# Clarificación — snapshot-friccion-laboratorio-jules

Transcript de decisiones (2026-06-11). Resuelve alcance tras auditoría del PBI padre y entregas ya mergeadas en `main`.

---

## D1 — Naturaleza de la feature

| Pregunta | Decisión |
|----------|----------|
| ¿Feature nueva o contenedor de cierre? | **Cierre táctico** del PBI Snapshot; no re-forja incidente Argos EDA |
| ¿Proceso? | `feature` v1.3.0 |
| Rama | `feat/snapshot-friccion-laboratorio-jules` |
| `persist_ref` | `docs/features/snapshot-friccion-laboratorio-jules` |
| Manifiesto | `docs/todos/pending/Snapshot_Friccion_Laboratorio_Jules.md` |

---

## D2 — Alcance heredado cerrado (fuera de esta feature)

| Bloque PBI | Entrega existente | Decisión |
|------------|-------------------|----------|
| §3.1 PyYAML | `requirements.txt`, `sddia-run.sh` | **No reabrir** — documentar como prerequisito lab en `execution.md` |
| §4 WASI dogma + PoC | `wasi-poc-ignition`, `migracion-rust-wasi` | **No reabrir** — referencia cruzada en `validacion.md` |
| §5.2 Aduana Husky | PR #73, `husky-pre-push-blocking-route` | **No reabrir** — marcar cumplido en cierre PBI |
| §5 parcial blindaje | `ia-obrera-blindaje` | **Extender** solo en O2 (acoplamiento RAW → feature) |

---

## D3 — Git failsoft (O1)

| Opción | Decisión |
|--------|----------|
| Skip global `SDDIA_LAB_SKIP_GIT_*` como solución | **Rechazada** — parche de laboratorio, no failsoft de producción |
| Propagar error fatal de git sin envelope | **Rechazada** — reproduce el colapso del incidente Jules |
| Degradación en `git-manager` + orquestador tolerante | **Adoptada** |

**Comportamiento esperado:**

1. Si `git fetch`/`pull` falla por red o autenticación → JSON `{ success: false, offline: true, errorSummary: "..." }` sin excepción no capturada en `invoke_git_manager`.
2. Fases que requieren remoto (`workspace-init` con red) pueden continuar en modo local si `base_branch` ya está presente.
3. Documentar flags lab existentes como override explícito del operador, no como comportamiento por defecto.

**Touchpoints:** `scripts/skills/git-manager.py` (fallback nativo), cápsula Rust `git-manager` si aplica, `execute_process_capsules.py` (`invoke_git_manager`, `run_workspace_init`).

---

## D4 — Raw Kernel acoplado a feature (O2)

| Opción | Decisión |
|--------|----------|
| Nueva norma separada | **Rechazada** — fragmenta `external-ai-constraints.md` |
| Ampliar norma motor + prefijo creator | **Adoptada** |
| Gate solo en hooks | **Insuficiente** — el incidente ocurrió antes del commit |

**Texto normativo mínimo:** cuando el runtime inyecte el prefijo `[EXECUTE AS RAW KERNEL…]`, la IA obrera **debe** comprobar existencia de `persist_ref` activo (`docs/features/{name}/objectives.md`) o invocar `execute-process.py --process feature` antes de mutar genoma o `docs/features/`.

---

## D5 — Transpilador de Intenciones (O3)

| Pregunta | Decisión |
|----------|----------|
| ¿Skill o Tool? | **Skill** — orquestación de contexto, sin lógica de dominio pesada |
| ¿Forja? | `entity-manager` / `skill-creator` — prohibida edición manual en `SddIA/skills/` |
| Nombre canónico | `intent-transpiler` |
| I/O | stdin: `{ "raw_instruction": string, "intent_hints": string[] }` → stdout: envelope JSON con `structured_directive`, `target_paths[]`, `required_process`, `persist_ref` |

**Ceguera espacial:** la skill no interpreta negocio del cliente; solo estructura y enruta según SSOT (`cumulo.paths.json`, `interaction-triggers.json`).

---

## D6 — Cierre documental (O4)

| Ítem | Decisión |
|------|----------|
| Mover PBI | `docs/todos/pending/Snapshot_Friccion_Laboratorio_Jules.md` → `docs/todos/done/` en **esta rama** |
| `validacion.md` | `pbi_archived: true` solo tras O1–O3 verificados o explícitamente diferidos con laudo en `clarify.md` |
| PR único | Código + documentación + PBI archivado — sin PR documental post-merge |

---

## D7 — Incertidumbres abiertas

| ID | Tema | Resolución pendiente |
|----|------|----------------------|
| U1 | Failsoft en WASM vs fallback Python | Auditar si `git-manager.wasm` puede detectar offline; si no, failsoft en fallback nativo documentado en D8 migración |
| U2 | Transpilador como skill Cursor vs cápsula | Fase 1: definición `.md` + contrato; implementación cápsula Rust en iteración posterior si no hay crate listo |
