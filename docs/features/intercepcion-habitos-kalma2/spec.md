---
feature_name: intercepcion-habitos-kalma2
created: "2026-09-12"
process: feature
base: main
scope: core
branch_name: feat/intercepcion-habitos-kalma2
persist_ref: docs/features/intercepcion-habitos-kalma2
execution_id: "54b02c30-b579-4d0b-a7e0-272d3c3c6af0"
document_id: PBI-NUCLEO-INTERCEPCION-HABITOS-KALMA2
---

# Spec — intercepcion-habitos-kalma2

## 1. Genoma conscience

`SddIA/conscience/aiua_core.md` §6: fila `delegar_habito` | Motor Sí | Destino `User_Preference_Change_Requested` vía `emit-user-preference-change-requested`.

Args: `subject_hint` obligatorio. Defaults: `subject_kind=person`, `predicate_hint=mute`, `operation=activate`, `scope_type=channel`, `scope_id=email`. `raw_utterance` solo en el fence; no en ECST.

## 2. Parser y despacho (`aiua_intent.rs`)

- `HABIT_TENDON = "delegar_habito"` incluido en `is_motor_tendon`.
- `run`: si `name == HABIT_TENDON`, no exigir `goal`/`target_component`.
- Matriz defensiva sobre `predicate_hint` + `raw_utterance` + `operation`:
  - borra|elimina|limpia|delete|expunge|destroy|ignora|silencia → mute + `{muted:true}` + activate
  - reactiva|vuelve a avisarme → mute + `{muted:false}` + revoke
  - prioriza|urgente|máxima → priority + `{level:max}`
  - importante|relevante → priority + `{level:high}`
- Payload emit: `operation`, `channel: kalma2`, `subject_kind`, `subject_hint`, `predicate` (= hint ya coercido), `value`, `scope_type`, `scope_id`, `priority_level` si aplica, `utterance_ref` opcional.
- Llamada: `user_preference_change_requested::run(repo, emit_inputs)`.
- `validate_ecst_event` sobre el JSON escrito si el path de sellado es accesible; si el emit no valida, fallar el despacho.
- Output: `success`, `event_id`, `target_path`, `event_type: User_Preference_Change_Requested`, `intent_dispatched: delegar_habito`.

## 3. Acción `dispatch-aiua-intent`

`entity-manager` `update` (UUID `a1086194-e19c-49b1-88e1-bd828211b3a8`, bump 1.0.0 → 1.1.0).

Cuerpo: SDLC → `Aiua_Process_Requested`; Suite → `emit-suite-execution-requested`; hábito → `emit-user-preference-change-requested`. Restaurar el artefacto truncado.

## 4. Destilación (`user-preference-core`)

```text
normalize_hint(s) = lowercase(collapse_ws(trim(s)))
canonical_subject_key_from_hint(s) = hex(SHA-256(UTF-8(normalize_hint(s))))
```

`preference_from_event_payload`:

1. `subject_kind` ← `payload.subject_kind` else `"person"`.
2. `subject_key` ← `payload.subject_key` si no vacío; else `canonical_subject_key_from_hint(subject_hint)`.
3. `predicate` ← `predicate` else `predicate_hint` else `"priority"`.
4. Si `predicate == "mute"` y `value.muted` ausente → `value = {"muted": true}` (no pisar `muted` explícito ni `until`).
5. `activate` → Active + ExplicitUser (sin leer `authority`/`status` del ECST).

## 5. Triaje — candidatos

Tras `from_decoded`:

```text
keys = { canonical_subject_key_from_addr(from) }
          ∪ { canonical_subject_key_from_hint(tok) | tok ∈ candidates(from, subject) }
```

`candidates`: display-name tokens `[A-Za-z0-9]{3,}`; local-part; labels de dominio excepto el último si len∈{2,3} (TLD). Asunto: mismos tokens ≥3, sin stopwords no hace falta (conjunto pequeño).

Para cada key: `query(QuerySpec { subject_key, include_proposed: false, max_results: 8 })`. Unión. `p_mute_sender` y `p_exempt_c` sobre la unión. `pref_ctx` del primer spec (addr) basta para el prompt LLM.

## 6. Tests

- `aiua_intent`: extract, is_motor, borra→mute, raw_utterance ausente del payload, dispatch escribe domain.
- `user-preference-core`: hint hasheado; kind no hereda hint; mute default value; `subject_key` preexistente intacto (`hash-juan-smoke`).
- `user_preference` ingest: evento con solo `subject_hint` persiste key de 64 hex.
- `email_triage`: seed por hint hasheado `computrabajo` + From `Jobs <alertas@computrabajo.com>` → P-MUTE-SENDER; `noreply@` sigue C-NOREPLY.
- `aiua_stimulus`: overlay `delegar_habito` despacha; no hay llamada a ingest (asserción: éxito del latido sin `user-preference-ingest` en fases). Copiar clase `user-preference-change-requested.md` al tmp como el test SDLC copia `aiua-process-requested.md`.

## 7. Evolution

Registro `sddia-qa evolution-register` + fila `Evolution_log.md`. UUID nuevo. `gate-evolution --range` antes de push si el diff toca `directories.evolution`.
