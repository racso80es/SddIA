---
feature_name: email-triage-heuristic-cold-start
created: "2026-09-06"
process: feature
base: main
scope: email-triage-heuristic-cold-start
version_spec: "1.0.0"
document_id: PBI-EMAIL-TRIAGE-HEURISTIC
uuid: "2d939386-db39-44f0-804f-1d5ab6ed78c2"
persist_ref: docs/features/email-triage-heuristic-cold-start
branch_name: feat/email-triage-heuristic-cold-start
execution_id: "5b530130-8225-4904-98f0-a894523f9c7e"
dedalo_verdict: ok
laudos:
  - L-SLICE
  - L-HANDLER
  - L-ORDER
  - L-EXEMPT
  - L-MUTE
  - L-HASH
  - L-FAIL
  - L-PROMPT
  - L-PATH
  - L-FORGE
  - L-EVENT-VER
  - L-IMAP
  - L-CI
---

# Especificación — email-triage-heuristic-cold-start

## 1. Decisiones Dédalo

| ID | Decisión | Rationale |
|----|----------|-----------|
| **L-SLICE** | Solo Slice 1. | Gate botonera; spawned digest/LanceDB. |
| **L-HANDLER** | Mutar `email_triage.rs` + crate `user-preference-core` (hash). | Proceso ya es handler nativo; no hay cápsula de triaje. |
| **L-ORDER** | Query P → exención C → muro C → mute P → Clasificacion. | PBI §3 / D2. |
| **L-EXEMPT** | Solo `explicit_user`+priority max\|high activo. | D2; `inferred` no mueve el muro. |
| **L-MUTE** | Cierra `noise`/`preference` si C no concluyó. | CA2; G5 ampliado. |
| **L-HASH** | SHA-256 hex del addr normalizado. | CA9; spec memoria §3 `subject_key` = hash. |
| **L-FAIL** | Bloque versionado vacío. | L-FAIL-POLICY; CA10. |
| **L-PROMPT** | Inyectar `user_preference_context` en prompt existente. | CA3; cold-start = prompt idéntico si `preferences: []`. |
| **L-PATH** | Enum de cierre, no de consulta. | Matriz §5 + CA6. |
| **L-FORGE** | EM update; packed process domain root. | DA-2. |
| **L-EVENT-VER** | Replacements; SemVer evento 1.1.0 intacto. | Forge no bumpa versión en replacements. |
| **L-IMAP** | Cero STORE. | Matriz § restricciones; CA7. |
| **L-CI** | APTO solo con run verde. | features-documentation-pattern v1.2.1. |

## 2. Circuito (Slice 1)

```
Email_Received
  → email-triage-gateway
       1. Triaje-P  memory:pref-query  (barata, siempre)
       2. P-EXEMPT-C  (opcional; no cierra)
       3. Triaje-C    (muro; skip si exención)
       4. P-MUTE-SENDER (si C no concluyó)
       5. Clasificacion  (si nadie cerró; prompt + contexto P)
       6. Asiento-Agenda / Emision
  → Email_Triaged
  → send-telegram-notification  (solo verdict=actionable; sin cambio Slice 1)
```

## 3. Hash canónico del remitente

```
normalize_email_addr(from):
  decode_rfc2047(from)
  si hay '<'…'>' extraer interior; si no, string completo
  trim + ASCII lowercase

canonical_subject_key_from_addr(from) = hex(SHA-256(UTF-8(normalize_email_addr(from))))
```

Vive en `user-preference-core`. QuerySpec.subject_key = ese hex. Tests: `noreply@shop.tld` y `Shop <noreply@shop.tld>` colapsan al mismo key; el key no contiene `@`.

## 4. Matching P

Consulta: `QuerySpec { subject_key, include_proposed: false, max_results: 8 }`. Fail-open vía `query_context_block_with_capsule_fallback`.

| Regla | Condición | Efecto |
|-------|-----------|--------|
| `P-EXEMPT-C` | `status=active` ∧ `authority=explicit_user` ∧ `predicate=priority` ∧ `value.level` ∈ {`max`,`high`} | No aplicar C-LIST / C-NOREPLY / C-SUBJECT-NOISE. `matched_rule` se anota en extras si Clasificacion cierra; no es veredicto. |
| `P-MUTE-SENDER` | C no concluyó ∧ `status=active` ∧ `predicate=mute` ∧ `value.muted=true` ∧ (`until` ausente/null o ISO futuro) | `verdict=noise`, `decision_path=preference`, `matched_rule=P-MUTE-SENDER`, Clasificacion skipped. |

Preferencias parciales (`attention_window`, `priority:normal|low`, mute inactivo) no cierran; viajan al prompt.

## 5. Prompt Clasificacion

Prompt vigente **más** (solo si `preferences` no vacío):

```
user_preference_context=<JSON {schema_version, preferences}>
```

El JSON es el bloque de `query_context_block` (incluye `value`; no incluye `sensitivity`). Cold-start: no se concatena el bloque → CA1.

## 6. Contratos a forjar

| Artefacto | Operación EM | Cambio |
|-----------|--------------|--------|
| `email-triage-gateway` | process update domain | Fase Triaje-P; G5: C **o** mute skippean Clasificacion; output `decision_path` documenta `preference`; versión **1.1.0** |
| `email-triage-matrix` | norm update | `P-EXEMPT-C` / `P-MUTE-SENDER`; `decision_path` += `preference`; C no concluye bajo exención; versión **1.1.0** |
| `email-triaged` | event update replacements | Cuerpo: enum `deterministic \| llm \| preference`. UUID `6a4b0e9a-…` inmutable. Versión frontmatter **1.1.0** (L-EVENT-VER) |

Skill `user-preference-store`: sin cambio de contrato.

## 7. Tests (handler / crate)

| Caso | Aserción |
|------|----------|
| Store vacío + noreply | `C-NOREPLY`, `deterministic`, Clasificacion skipped |
| Fail-open (sin store) | igual CA1 |
| Mute activo + from humano | `P-MUTE-SENDER`, `preference`, sin LLM |
| Mute + List-Id | gana C-LIST (`deterministic`); mute no se evalúa |
| Exempt C + noreply | C skipped; Clasificacion corre (o elevación estructural) |
| Inferred priority max + noreply | muro C intacto |
| Proposed priority max + noreply | muro C intacto |
| Hash | addr en claro ≠ subject_key; colapso Name\<addr\> |
| Prompt | bloque ausente si preferences vacío; presente si parcial |
| IMAP | fuente no contiene STORE/expunge |
| Tres vías | verdict ∈ {noise,passive,actionable} |

## 8. Fuera de alcance

Slice 2; LanceDB; digest; réplica; `telegram-fallback-responder` como FSM; mutar IMAP; ampliar `actionable`.
