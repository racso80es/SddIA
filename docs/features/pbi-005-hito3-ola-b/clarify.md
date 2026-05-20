---
feature_name: pbi-005-hito3-ola-b
created: "2026-05-20"
process: feature
purpose: Clarificación Ola B — hooks pre-push/post-merge y cierre CA-3 PBI-005
---

# Clarificación — PBI-005 Hito 3 Ola B (Hooks ciclo PR)

Transcript de decisiones (2026-05-20). Complementa `objectives.md` y alimentará `spec.md` (fase Dedalo).

---

## D1 — Inicio formal

| Pregunta | Decisión |
|----------|----------|
| ¿Proceso? | **`feature`** v1.2.0 |
| Rama | `feat/pbi-005-hito3-ola-b` |
| `persist_ref` | `docs/features/pbi-005-hito3-ola-b` |
| Feature hermana | `pbi-005-hito3-git-hooks` — Ola A cerrada PR #12; **no duplicar** ADN D1–D8 de pre-commit |
| Manifiestos | PBI-005 v1.4.0 § CA-3; backlog post-PR11 § P1 Ola B |
| Alcance PBI | **Solo** pendientes que bloquean cierre PBI-005 (H3.1–H3.5 + actualización manifiesto) |

---

## D2 — Partición respecto a la feature hermana

| Ola | Feature | Hooks | Estado |
|-----|---------|-------|--------|
| **A** | `pbi-005-hito3-git-hooks` | `pre-commit` | ✅ `main` @ `12119f7` |
| **B** | **`pbi-005-hito3-ola-b`** (esta) | `pre-push`, `post-merge` | ⏳ |

**Motivo de feature separada:** Ola A ya fusionada y validada; Ola B tiene ciclo de vida, PR y `validacion.md` propios. El roadmap § 7 de la hermana se **promueve** a spec de esta feature sin reescribir Ola A.

---

## D3 — Inventario de pendientes PBI-005 (alcance acotado)

| Pendiente | ¿Incluido? | Notas |
|-----------|------------|-------|
| H3.1 Contrato hooks PR | ✅ | Entregable documental + referencia en hooks |
| H3.2 `pre-push` → `delivery-close-cycle` | ✅ | Núcleo CA-3.2 |
| H3.3 `post-merge` → `accept-pr` | ✅ | Núcleo CA-3.2 |
| H3.4 Sin `gh pr merge` en hooks | ✅ | Aduana Argos |
| H3.5 Smoke + `event_ids` | ✅ | Cierre feature |
| Actualizar PBI operativo a «completado» | ✅ | v1.5.0 tras APTO |
| OC.1–OC.5 shims CLI Ola C | ❌ | Backlog P2 — no bloquea DoD PBI |
| L.1 handler lab `accept-pr` | ❌ | Backlog P3 — hooks pueden invocar proceso aunque lab sea parcial |
| Reexport PDF (D.3) | ❌ | Higiene P6 opcional |

---

## D4 — ¿Nueva acción monolítica para hooks PR?

| Opción descartada | Motivo |
|-------------------|--------|
| Acción `git-pre-push-present-pr` combinando push + gh + bus | Viola SRP; duplica `delivery-close-cycle` (mismo impasse que `request-change-incorporation`) |
| Acción `git-post-merge-accept` | Duplica `accept-pr` |

| Opción adoptada | Motivo |
|-----------------|--------|
| Scripts en `SddIA/scripts/qa/git-hooks/` que invocan **`execute-process.py --process … --inputs-file …`** | Fractalidad idéntica a `pr-presented-orchestration`; hooks = adaptadores del host, no genoma |
| Payload JSON efímero en `tmp/` generado por el hook | Evita hardcodear inputs en shell; trazabilidad en logs del hook |

---

## D5 — Hook `pre-push` (H3.2)

| Pregunta | Decisión |
|----------|----------|
| ¿Cuándo dispara? | En **cada** `git push` donde el hook esté instalado |
| ¿Qué proceso? | `delivery-close-cycle` vía `execute-process.py --process delivery-close-cycle --inputs-file …` |
| `source_process` en payload | `git-hook-pre-push` (trazabilidad; distinto de `feature`) |
| `branch_name` | Rama que se empuja (`@{u}` o argumentos del hook — resolver en spec) |
| `persist_ref` | Inferir `docs/features/<nombre>/` si existe carpeta para rama; si no, omitir o usar heurística documentada en spec |
| `pr_title` / `pr_body` | Valores por defecto mínimos generados por hook si no hay manifiesto feature |
| ¿Bypass? | `SDDIA_SKIP_HOOKS=1` — misma política que Ola A; solo operador humano |

**Sub-secuencia interna (no reimplementar en hook):** Snapshot → Argos (simulated en lab) → EDA gate → push → `gh pr create/view` → `emit-pr-presented-event`.

**Idempotencia (laudo O1):** si PR ya existe (`gh pr view` OPEN) **o** hay `PullRequest_Presented` en `eda_bus` para la rama, el hook **no invoca** `delivery-close-cycle` y retorna **exit 0 silencioso**.

**Cláusula Guarda (laudo O3):** push a `main` → Hard Fail antes de cualquier otra comprobación.

**Fail-fast:** exit ≠ 0 del hook **bloquea el push** si `delivery-close-cycle` falla en fases físicas (push, gh, sello).

---

## D6 — Hook `post-merge` (H3.3)

| Pregunta | Decisión |
|----------|----------|
| ¿Cuándo dispara? | Hook **`post-merge`** (Git ≥ 2.8) tras merge **local** a `main` |
| ¿Condición de rama? | Solo si `HEAD` refiere `main` / `refs/heads/main` tras la operación |
| ¿Qué proceso? | `accept-pr` vía `execute-process.py --process accept-pr --inputs-file …` |
| Inputs mínimos | `source_branch` (rama fusionada), `author`, `correlation_id` (UUID v4 generado en hook) |
| Entrada evento | JSON en `docs/events/pending/` según contrato `accept-pr.md` (`target_path`) |
| Prohibido | `gh pr merge`, `git merge` manual fuera del flujo, `--action emit-pr-merged-event` suelto |

**Resiliencia (laudo O4):** `post-merge` **siempre** invoca `accept-pr`. Si merge físico sin `PullRequest_Presented` previo (Merge Huérfano), `accept-pr` emite `PullRequest_Merged` con `traceability_anomaly: merge_huérfano` — no colapsa el sistema local.

**Alternativa descartada:** `post-checkout` en `main` — demasiado ruidoso (cada checkout dispararía).

---

## D7 — Contrato normativo H3.1

| Pregunta | Decisión |
|----------|----------|
| Ubicación preferida | **`SddIA/evolution/git-hooks-ca3-ola-b-contract.md`** (evolución táctica; UUID en frontmatter) |
| Alternativa | Norma en `SddIA/norms/` si Argos exige jurisdiction `dedalo` |
| Contenido mínimo | Tabla hook → trigger → proceso → evento; inputs JSON; prohibiciones; referencias cruzadas |
| Alineación | `pull-request-orchestration.md` v1.0.0 § 3–4; `accept-pr.md`; `delivery-close-cycle.md` v1.1 |

La hermana ya proponía `git-hooks-ca3-contract.md` en spec § 7.1 — **renombrar** a `git-hooks-ca3-ola-b-contract.md` para evitar colisión con borrador Ola A.

---

## D8 — H3.4 — Prohibiciones en hooks

| Prohibido en scripts hook | Motivo |
|---------------------------|--------|
| `gh pr merge` | SSOT fusión = `accept-pr` |
| `gh pr create` directo | SSOT presentación = `delivery-close-cycle` |
| `emit-pr-presented-event` / `emit-pr-merged-event` sueltos | Solo vía procesos |
| `git merge` / `git push` directos | Solo vía `git-manager` dentro de procesos (hook llama proceso, no git suelto) |

**Excepción:** el hook puede invocar `git rev-parse`, `git symbolic-ref` para **lectura** de contexto al construir JSON.

---

## D9 — Instalación y convivencia con Ola A

| Pregunta | Decisión |
|----------|----------|
| Ubicación scripts | `SddIA/scripts/qa/git-hooks/pre-push`, `post-merge`, módulos Python compartidos si aplica |
| Instalador | **Laudo O5:** `install-hooks.ps1` + `install-hooks.sh` iteran `git-hooks/`; symlink (Unix) o copia (Windows); excluyen `.py`/instaladores |
| Instalación | Manual post-laudo operador; `.git/hooks/` local |
| Windows | Git sh + `python`; rutas relativas a `git rev-parse --show-toplevel` |
| Orden de rollout | Documentar en `implementation.md`: (1) Ola A ya instalada, (2) añadir Ola B, (3) smoke H3.5 |

---

## D10 — Perfil laboratorio vs runtime IDE

| Aspecto | Laboratorio | Runtime IDE |
|---------|-------------|-------------|
| Fases 2–5 feature | Mayeuta/Dedalo/Tekton/Argos vía IDE | Agentes V5 |
| `delivery-close-cycle` en hook | Fases 1–3 pueden `simulated`; fases 4–6 físicas si `SDDIA_LAB_*` no bloquean | Completo |
| `accept-pr` en hook | Puede requerir pasos manuales documentados hasta L.1 | Proceso completo |
| `validacion.md` | Debe declarar qué fases fueron físicas vs simulated | APTO pleno |

---

## D11 — Criterios de cierre PBI-005

| Pregunta | Decisión |
|----------|----------|
| ¿CA-3 cerrado solo con spec? | **No** — requiere hooks operativos + `validacion.md` H3.5 |
| ¿PBI operativo a v1.5.0? | Tras merge de esta feature y APTO Argos |
| ¿Marcar matriz faenas 3b ✅? | En mismo PR o commit documental de cierre |
| Eventos DLT | Mismos criterios que Ola A: watcher + `delivery_state.cumulo` en Presented/Merged |

---

## D12 — Commits y merge de esta feature

| Orden | Contenido |
|-------|-----------|
| 1 | `objectives.md` + inicio feature (workspace-init) |
| 2 | `clarify.md` (esta entrega) |
| 3 | `spec.md` + `plan.md` (Dedalo) |
| 4 | Evolution H3.1 + hooks H3.2–H3.4 + instalador |
| 5 | `implementation.md`, `execution.md`, `validacion.md` |
| 6 | Actualización PBI operativo v1.5.0 |

Merge vía **`accept-pr`** cuando Argos emita **APTO** (no `gh pr merge`).

---

## D13 — Resoluciones de Acero (cerradas en spec.md § 4)

| ID | Laudo operador | Estado |
|----|----------------|--------|
| **O1** | Idempotencia absoluta `pre-push`: si PR abierta (`gh pr view`) o evento Presented en bus → **no** invocar `delivery-close-cycle`; exit 0 silencioso | ✅ |
| **O2** | Heurística estricta: `feat/{slug}` → `docs/features/{slug}/` si existe; si no, `persist_ref: null` (modo degradado) | ✅ |
| **O3** | Hard Fail push a `main`: *«Violación de Soberanía: main solo muta mediante el proceso accept-pr (PR merge). Push bloqueado.»* | ✅ |
| **O4** | `post-merge` siempre invoca `accept-pr`; Merge Huérfano → `PullRequest_Merged` con `traceability_anomaly: merge_huérfano` | ✅ |
| **O5** | Instalador dinámico: iterar `git-hooks/`, symlinks/copias en `.git/hooks/`, asimilación automática de hooks futuros | ✅ |

Detalle técnico canónico: `spec.md` § 4–9.

---

## Referencias cruzadas

| Artefacto | Ruta |
|-----------|------|
| Ola A (cerrada) | `docs/features/pbi-005-hito3-git-hooks/` |
| Orquestación PR presentado | `docs/features/pr-presented-orchestration/` |
| PBI operativo | `docs/todos/done/[OPERATIVO] Planificación de Backlog… (Ola A).md` v1.5.1 |
| Backlog consolidado | `docs/todos/[OPERATIVO] Backlog pendiente post-PR11…` |
| Norma PR | `SddIA/norms/pull-request-orchestration.md` |
| Hooks existentes | `SddIA/scripts/qa/git-hooks/` |
