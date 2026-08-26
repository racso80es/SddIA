---
document_id: PBI-FIX-ACCEPT-PR-DELETE-BRANCH-PAYLOAD
uuid: "94f74fa6-a063-4d94-96a1-2f4d92ffb692"
title: "[FIX] accept-pr — payload delete_branch vs contrato git-manager"
format: markdown
version: "1.0.0"
status: pending
type: bug-fix
priority: alta
process: bug-fix
persist_ref: docs/fixes/accept-pr-delete-branch-payload
created: "2026-08-26"
updated: "2026-08-26"
pbi_archived: false
derived_from:
  - FIX-ACCEPT-PR-SILENT-DELETE-BRANCH
  - PBI-KAIZEN-AISLAMIENTO-MULTI-INSTANCIA
incident_ref: "PR #193 accept-pr — hygiene_failure delete_branch payload"
antecesor_fix_ref: docs/todos/done/[FIX] accept-pr — higiene silenciosa delete_branch tras merge.md
antecesor_process_ref: SddIA/library/codexes/codex-software-engineering/process/accept-pr.md
friction_ids:
  - F-GIT-DELETE-BRANCH-PAYLOAD
  - F-GIT-FROZEN-IO-GAP
tech_debt_ids:
  - DT-ACCEPT-PR-DELETE-BRANCH-RUST
  - DT-GIT-MANAGER-FROZEN-DELETE-BRANCH
blocks_on: []
---

# [FIX] accept-pr — payload `delete_branch` vs contrato git-manager

## 0. Contexto

Empiría **2026-08-26** (`accept-pr` sobre PR #193, merge `fb12e076`). Fase **Sincronización y Limpieza**: push `main` OK; `closed_branch: null`; `hygiene_failure`:

```text
delete_branch payload keys must exactly match ["branch_name", "remote", "force"]
```

El merge soberano **no** falló. La higiene de rama origen **sí** (cero invocación git válida). GitHub ya había borrado el ref remoto al MERGED; la rama local se eliminó **a mano** con `git-manager` (`remote: false`, `force: false`).

No reabre el FIX histórico `FIX-ACCEPT-PR-SILENT-DELETE-BRANCH` (PR #37, cápsulas Python: tragar excepción). Aquel síntoma era silencio. Este es **contrato de payload** en el handler **Rust** `accept_pr.rs` post-migración orquestador.

SSOT de proceso ya correcto: `accept-pr.md` § Fase 4 exige **dos** llamadas `delete_branch` con `remote`/`force` booleanos. El handler nativo no lo cumple.

---

## 0bis. Adecuación del estímulo (anti-alucinación)

| Afirmación | Veredicto | Corrección |
|------------|-----------|------------|
| «`remote` debe ser el nombre del remote (`origin`)» | **Falso para `delete_branch`** | En `push`/`pull`/`fetch`, `remote` es **string**. En `delete_branch` (cápsula Rust) `remote` es **boolean** (local vs `git push origin --delete`). Homónimo; no unificar. |
| Una sola invocación con `remote: "origin"` borra local y remoto | **Falso** | Claves extra/faltantes → abort cápsula **antes** de `git`. Falta `force`. |
| El FIX #37 ya curó esto | **Incompleto** | Curó Python + visibilidad `hygiene_failure`. El puerto Rust (`SddIA/engine/execute-process/src/engine/accept_pr.rs` `delete_branch_hygiene`) reintrodujo payload ilegítimo. |
| `skill-io-git-manager-frozen.md` documenta `delete_branch` | **Falso** | Enum §2: `status`…`branch_list` **sin** `delete_branch`. La cápsula **sí** implementa la op (líneas 178–192). Dualidad norma congelada vs genoma físico. |
| Fallo remoto «ref no existe» debe abortar `accept-pr` | **Fuera / peligroso** | Post-merge GitHub a menudo ya no hay ref. Fail-soft **por operación**; `closed_branch` solo si local `-d` OK. |

---

## 1. Fricción

| ID | Síntoma | Causa raíz | Ad-hoc | Acción |
|----|---------|------------|--------|--------|
| **F-GIT-DELETE-BRANCH-PAYLOAD** | `hygiene_failure` en cada `accept-pr`; rama local/remota no las borra el proceso | `delete_branch_hygiene` envía `{"branch_name", "remote": "origin"}` | `git-manager` delete_branch a mano | **DT-ACCEPT-PR-DELETE-BRANCH-RUST:** dos invocaciones alineadas a `accept-pr.md` |
| **F-GIT-FROZEN-IO-GAP** | Cerbero/Argos no pueden validar la op contra la norma congelada | `skill-io-git-manager-frozen.md` v1.0.0 omite `delete_branch` / `merge` / ops extra de la cápsula | Operar contra el `.rs` | **DT-GIT-MANAGER-FROZEN-DELETE-BRANCH:** evolucionar frozen vía `entity-manager` → `norm-creator` (o proceso de norma Core si el laudo lo sitúa en `directories.norms`) |

Código vigente (ilegítimo):

```rust
invoke_git_manager(
    repo,
    "delete_branch",
    &json!({"branch_name": branch, "remote": "origin"}),
)
```

Cápsula (`git-manager` `src/main.rs`):

```text
payload_exact(..., ["branch_name", "remote", "force"])
remote: bool  → true  => git push origin --delete <branch>
force:  bool  → false => git branch -d (local)
```

Proceso (SSOT):

```text
local:  { "branch_name", "remote": false, "force": false }
remoto: { "branch_name", "remote": true,  "force": false }
```

---

## 2. Qué **no** reabre este ciclo

- Bypass `gh` / `git` crudo en Tekton para higiene (sigue `git-manager`).
- `SDDIA_SKIP_HOOKS` en el push de `main` (ya acotado al hijo en Fase 4).
- Silenciar de nuevo `hygiene_failure`.
- Forzar `-D` (`force: true`) como default.
- Namespacing de remotes distintos de `origin` (la cápsula hardcodea `origin` en delete remoto; fuera salvo que se forje op nueva).

---

## 3. Objetivos

1. `delete_branch_hygiene` (Rust) = dos llamadas contractuales; `operations[]` en `hygiene_failure` si alguna falla.
2. `closed_branch` = nombre de rama **solo** si delete **local** `success`.
3. Delete remoto `ref no existe` → fail-soft (no `status_code: 1` del proceso si el merge+push ya OK).
4. Norma frozen declara `delete_branch` (y, si el laudo lo exige, las demás ops que la cápsula ya ejecuta: `merge`, `get_last_commit`, `diff_name_only`) **o** la cápsula se recorta al enum — **una** verdad, no las dos.
5. Unit: payload ilegítimo no se emite; payload canónico sí.

---

## 4. Criterios de cierre

- [ ] `accept_pr.rs`: **0** `"remote": "origin"` en `delete_branch`. Dos `invoke_git_manager` con booleanos + `force`.
- [ ] Test unitario del helper (o del handler Fase 4) que falle si el payload no coincide con `payload_exact`.
- [ ] Smoke/`accept-pr` lab: rama dummy local se borra; remoto ausente no tumba `success: true` del proceso.
- [ ] `skill-io-git-manager-frozen.md` (SemVer) incluye `delete_branch` **o** documenta exclusión consciente + recorte de cápsula (laudo Mayeuta). Mutación de norma/skill vía `entity-manager`.
- [ ] `validacion.md` APTO + PBI → `docs/todos/done/` en el **mismo** PR (`bug-fix`).

---

## 5. Orden de forja

```text
(1) Test que reproduce hygiene_failure con payload actual
(2) Alinear delete_branch_hygiene a accept-pr.md (local luego remoto; fail-soft por op)
(3) Frozen I/O + cápsula (entity-manager); tests git-manager
(4) validacion.md + archivo PBI
```

UUID PBI: `94f74fa6-a063-4d94-96a1-2f4d92ffb692`. Init lab: `./sddia-run.sh --process bug-fix` + skips archive/delivery según perfil; `fix_name`: `accept-pr-delete-branch-payload`. Rama: `fix/accept-pr-delete-branch-payload`.

---

## 6. Referencias

| Ref | Uso |
|-----|-----|
| `SddIA/engine/execute-process/src/engine/accept_pr.rs` | `delete_branch_hygiene` |
| `SddIA/skills/git-manager/src/main.rs` | `payload_exact` + bools |
| `SddIA/library/codexes/codex-software-engineering/process/accept-pr.md` | SSOT Fase 4 |
| `SddIA/norms/skill-io-git-manager-frozen.md` | hueco enum |
| PR #193 | empíria `hygiene_failure` |
| `docs/todos/done/[FIX] accept-pr — higiene silenciosa delete_branch tras merge.md` | antecesor Python; no duplicar |
