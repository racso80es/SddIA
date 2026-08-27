---
feature_name: kaizen-tekton-evolution-gate-no-poll
created: "2026-08-27"
process: feature
purpose: Estabilización Mayeuta — PBI-KAIZEN-TEKTON-EVOLUTION-GATE-NO-POLL v1.1.0
branch_name: feat/kaizen-tekton-evolution-gate-no-poll
persist_ref: docs/features/kaizen-tekton-evolution-gate-no-poll
pbi_ref: docs/todos/pending/[KAIZEN] Tekton — aduana local evolution y veto de polling CI.md
document_id: PBI-KAIZEN-TEKTON-EVOLUTION-GATE-NO-POLL
uuid: "07dc027a-fdb5-487a-9fea-1a5dd67d38ca"
execution_id: "96471044-003a-457a-bf59-041e94053b12"
source_pr_url: https://github.com/racso80es/SddIA/pull/206
incident_ref: "TEKTON_CI_POLL_EVOL_HASH — cierre PPR #203: hash_integrity placeholder + 3 commits ciegos + polling Actions"
mayeuta_verdict: ok
laudo: rehash-ssot-no-deferred-anchor
---

# Clarificación — kaizen-tekton-evolution-gate-no-poll

Transcript Mayeuta (2026-08-27). Semilla PBI v1.1.0. Filtro A contra genoma vigente. No se implementa a ciegas. Dedalo no arranca en esta sesión.

---

## D0 — Apertura formal

| Pregunta | Decisión |
|----------|----------|
| Proceso | `feature` v1.3.2 |
| `feature_name` | `kaizen-tekton-evolution-gate-no-poll` |
| Rama | `feat/kaizen-tekton-evolution-gate-no-poll` |
| `persist_ref` | `docs/features/kaizen-tekton-evolution-gate-no-poll` |
| `document_id` | `PBI-KAIZEN-TEKTON-EVOLUTION-GATE-NO-POLL` |
| Init lab | `./sddia-run.sh --process feature` + `SDDIA_LAB_SKIP_PBI_ARCHIVE=1` + `SDDIA_LAB_SKIP_DELIVERY_CLOSE=1` + `SDDIA_AGENT_RUNTIME_COMMAND=` |
| `execution_id` | `96471044-003a-457a-bf59-041e94053b12` |
| Fase | Estabilización Mayeuta (esta sesión). **Dedalo/Tekton no arrancan.** |
| Antecesor | PR #206 / PPR #203 (`b7e4a91c-…`); commits ciegos `4aced07`, `5ea8639`, `8963ec4` |

**Toll:** un `persist_ref`, un PR. Cierre documental en rama (PBI → `docs/todos/done/` + `validacion.md` APTO) en el mismo PR.

---

## D1 — R1: ¿quién emite `pending-merge` / `pending-anchor-on-merge`?

Grep en `directories.process`, `directories.skills`, `directories.tools` y motor `execute-process`: **cero emisores** de `hash_integrity: sha256:pending-merge` o `sha256:pending-anchor-on-merge`.

| Locus | Hecho |
|-------|-------|
| Cápsula `sddia-evolution-register` | `execute`: `verdict \| alta \| modificacion \| baja`. `render_detail` calcula `canonical_hash` y escribe hex real. No hay operación de anclaje diferido. |
| `factory.rs` | Inserta `hash_signature: sha256:pending` en **entidades de genoma**, no en registros evolution. |
| Eventos dominio | `hash_signature: "sha256:pending-anchor-on-merge"` es ritual de **otro contrato** (firma de entidad). Homología de etiqueta, no pipeline hacia `directories.evolution`. |

Los cuatro fósiles son **escritura humana/Tekton** de `{id_cambio}.md` fuera de la cápsula:

| UUID | Placeholder | Origen aparente |
|------|-------------|-----------------|
| `67110f2f-2be8-4fd3-b0a7-8dc400fe803f` | `sha256:pending` | F-BUNDLE-06 / PR #194 |
| `c2e8f4a1-7b3d-4e9c-a5f6-8d1e2f3a4b5c` | `sha256:pending` | Rehab PPR #202 |
| `c4a91e7b-2f68-4d3a-a8e1-5b7c9d0e2f14` | `sha256:pending-merge` | Rehab DCC PPR #187 |
| `a1c9e7f3-2b4d-5e6f-8a9b-0c1d2e3f4a5b` | `sha256:pending-anchor-on-merge` | Ola documental centinelas 2026-08-19 |

Contrato 1.1.1 §2 + §3: `hash_integrity` vacío/ausente = no conforme; **no rellenar**. Placeholder con sufijo no es semántica de proceso: es no-conforme disfrazado.

**Laudo:** re-anclar los cuatro con el comando de D3 en el mismo PR. No hay consumidor que espere el placeholder. Fuera de este ciclo: sellar `hash_signature` de eventos (contrato distinto).

---

## D2 — Fidelidad `build_registry` (A)

Semilla verificada: `frontmatter` = `load_frontmatter_yaml(&p)` (working tree); `raw` = `read_blob(..., staged)` con `staged = !range`. `--range` cae a WT. Pre-commit mezcla índice (raw) y WT (fm).

**Laudo:**

- Una sola fuente: parsear YAML del `raw` capturado.
- `read_blob(repo, rel, rev)` con `rev ∈ {":", "HEAD"}` (pre-commit vs `--range`). Prohibido booleano `staged`.
- Invariante: árbol limpio → `gate-evolution --range` ≡ job `evolution gate (delta)`.
- Pre-commit hereda fidelidad; **no** reescribir `pre_commit_gate.sh` más allá de eso (PBI § Fuera).

R3: el algoritmo de hash no cambia; solo se elimina la mezcla. Test de fidelidad **antes** de cablear el hook.

---

## D3 — Re-anclaje canónico (B) — nombre fijado

**Laudo de nombre:** `sddia-qa evolution-rehash --id <uuid> [--json] [--dry-run]`.

SSOT del dígito: `canonical_hash` en la cápsula (`lib.rs`: strip líneas `hash_integrity:` + CRLF→LF + `lines().join("\n")` → **sin newline final**). Prohibido Python/`sha256sum`/`openssl` ad hoc.

`execute()` hoy no admite `rehash`. Dedalo elige el cableado (operación cápsula vs invocación del mismo símbolo en el crate); el CLI no reimplementa el strip.

Contrato `evolution_contract.md` **1.1.1 → 1.1.2**: documentar la semántica exacta. **Prohibido** cambiar el algoritmo.

---

## D4 — Formato de `hash_integrity` (C)

Hoy: vacío → mismatch; placeholder pasa hasta recompute en CI.

**Laudo:** `validate_record` exige `^sha256:[0-9a-f]{64}$` **antes** del recompute. Fallo → `EVOL_HASH_MISMATCH` con mensaje `placeholder/formato inválido` (accionable: invocar `evolution-rehash`). Hex en minúsculas (hex::encode vigente).

---

## D5 — `--all` y fósiles (D)

`correlators` filtra `in_diff == true`. Los cuatro fósiles son invisibles al delta.

**Laudo:**

1. Re-anclar (D3) en este PR.
2. `gate-evolution --all` audita todos los registros bajo `directories.evolution` (vía Cúmulo).
3. Job CI + invocación local: **bloqueante solo tras el saneamiento**, mismo PR. No dejar `main` rojo.

---

## D6 — DA-6 veto de vigilancia remota (E)

DA-5 cubre post-acuse del CLI Core; el patrón migró a GitHub Actions.

**Laudo:**

- `external-ai-constraints.md` **v1.5.0 → v1.6.0** (forja `entity-manager`, no bisturí). Directriz **DA-6**: tras el primer log de check fallido, prohibido `sleep` de espera, `gh pr checks` en bucle y `gh run rerun` del **mismo `headSha`**. Un finding → parche local verificado (`gate-evolution --json --range` exit 0 si el diff toca `directories.evolution`) → un push.
- Prohibido empujar docs de cierre con check rojo conocido.
- Touchpoint: extender `.cursor/rules/tekton-fire-and-forget.mdc`. **Prohibida** rule nueva.

---

## D7 — Aduana pre-push (F) y R4

`pre_push_gate.sh` hoy: skip hooks / `in_delivery_close_cycle` → `Local_QA_Requested` → `delivery-close-cycle`.

**Laudo:** si `origin/main...HEAD` toca `directories.evolution` (ruta desde Cúmulo, **cero listas ad hoc** — contrato §7/§99), ejecutar `gate-evolution --range` y bloquear si `success == false`. **Antes** de `route-domain-event`. Respetar `in_delivery_close_cycle` y `SDDIA_SKIP_HOOKS` (solo operador humano).

---

## D8 — Tests (G) — no negociables

| Fixture | Esperado |
|---------|----------|
| `hash_integrity: sha256:pending` | `EVOL_HASH_MISMATCH` por formato |
| Newline final y CRLF | Hash estable e idéntico (blinda D3) |
| Registro correcto en HEAD + WT corrupto (y viceversa) | `--range` juzga HEAD |

---

## D9 — Fuera de alcance

- Mutar `canonical_hash` (invalidaría el universo conforme).
- Rehabilitación `accept-pr` / umbrales Radamanto (PPR #203 cerrado).
- Dashboard CI / Espejo de Consciencia.
- Reescritura amplia de `pre_commit_gate.sh`.
- Sellar `hash_signature` de eventos / `pending-forge` de genoma.

---

## Criterios PBI → laudo

| ID | Laudo |
|----|-------|
| K-FIDEL | D2 + test fidelidad |
| K-REHASH | D3 `evolution-rehash` |
| K-FORMAT | D4 |
| K-FOSIL | D1 + D5 |
| K-LOCAL | D7 |
| K-NOPOLL | D6 |
| K-DOC | patrón v1.2.1; archivo en el mismo PR |

---

## Handoff Dedalo

Consumir `objectives.md` como `refined_requirements`. Fijar:

1. Firma de `read_blob` + parseo fm desde `raw`.
2. Cableado `evolution-rehash` → `canonical_hash` (sin duplicar strip).
3. Regex y mensaje de D4 en `validate_record`.
4. Flag `--all` + job workflow; orden saneamiento → bloqueo.
5. Punto de inserción en `pre_push_gate.sh` (antes de `invoke_process route-domain-event`).
6. Bump contrato 1.1.2 y DA-6 vía creators/`entity-manager`.
