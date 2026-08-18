---
feature_name: kalma2-mvp-sensorial-email
created: "2026-08-18"
process: feature
base: main
scope: kalma2-mvp-sensorial-email
version_spec: "1.0.0"
document_id: PBI-KALMA2-MVP-01A
uuid: "c209c150-8ab4-4f0d-bcf7-8fa7a6101de0"
persist_ref: docs/features/kalma2-mvp-sensorial-email
branch_name: feat/kalma2-mvp-sensorial-email
dossier_ref: docs/features/kalma2-mvp-paciente-0
genome_ssot: docs/features/kalma2-mvp-paciente-0/spec.md
execution_id: "fa4dde03-a0ec-426f-ade7-850246ba7575"
---

# Especificación de ola — Circuito sensorial (01A)

Genoma de entidades: `docs/features/kalma2-mvp-paciente-0/spec.md` (SSOT). Este documento fija perímetro, mutaciones de esta ola y gates. UUIDs del dossier = reservas; prevalece `entity-manager` / `daemon-creator`.

## 1. Circuito (alcance 01A)

```
IMAP (RO) → email-watcher → Email_Received (domain)
                         → Daemon_Heartbeat (telemetry)
event-watcher → email-triage-gateway
  Triaje-C (determinista) → noise: salida temprana
  Clasificacion (llm:interact) solo si Triaje-C no concluye
  Asiento-Agenda (agenda:persist) solo actionable
  Emision → Email_Triaged
WUI: GET /api/status (proyección existente; 01A no añade rutas)
```

## 2. Entidades (punteros)

| Entidad | Clase | Forja | SSOT padre | UUID emitido |
|---------|-------|-------|------------|--------------|
| `email-triage-matrix` | norm | entity-manager | §5 | `3d8c7e09-6d98-422d-909f-5b233ba7fcf2` |
| `codex-kalma2-assistant` | codex | entity-manager | §6 | `c43544f3-c557-4cc3-8a03-7175282f2c88` |
| `email-received` | event domain | entity-manager | §3.1 | `574fe330-137f-4f3a-b72d-dba189c6c406` |
| `email-triaged` | event domain | entity-manager | §3.2 | `6a4b0e9a-42e1-425c-8a16-9344eae4f246` |
| `email-watcher` | daemon | in-ciclo (F-01) | §4 | `773a11e7-3a42-4eba-a383-79dd6ef8c263` |
| `email-triage-gateway` | process (domain root) | entity-manager + `process_jurisdiction` códice | §7 | `9cb9a63a-bb86-4b97-8a75-4dac2f2cb5ce` |
| `agenda-manager` | skill | entity-manager | §8.4 | `feb7314d-b86d-4653-a876-507c824ec9e2` |
| `sddia-email-watcher@.service.template` | template | in-ciclo (`directories.templates`) | §4 / L-02 | — |

## 3. Mutaciones SSOT de esta ola

| ID | Artefacto | Cambio |
|----|-----------|--------|
| R-01 | `SddIA/library/codexes/codex-contract.md` | v1.2.0; bloque `dlt` **opcional** (padre §9.4, §6.1) |
| R-02 | `SddIA/core/cumulo.paths.json` | `process_domain_roots` += `SddIA/library/codexes/codex-kalma2-assistant/process` [PATTERN-b6a9ed14-3a0d-4f5b-8444-d1b867335daf] |
| — | `SddIA/core/event-domain-subscriptions.json` | `Email_Received` → `email-triage-gateway` (padre §9.2) |
| — | `SddIA/core/capability-bindings.md` | solo `agenda:persist` (padre §9.1; `asset:fetch` = 01B) |

## 4. Invariantes (no relajables)

- G4: cápsula centinela sin `execute-process`, sin lectura `SddIA/`, sin IMAP write, sin ruta absoluta de host.
- G5: list-headers → `decision_path: deterministic`, coste en ceros, fase `Clasificacion` ausente del `execution_report` (status `skipped`).
- Payload `Email_Received`: `body_ref`, nunca cuerpo íntegro.
- IMAP read-only. Idempotencia `uid > last_uid`. Primer sondeo: ventana `SINCE` (defecto 60 días), no `ALL`.
- Códice: `canonical_hash == hash_signature`; `mint_status: pre-mint`; `token_id: null`.
- `Email_Triaged` sobrevive `purge_after` vía `{eda_instance.proofs}/email-triaged/{event_id}.json`; `GET /api/status?event_id=` proyecta domain o testigo.

## 5. Fuera de esta spec

Padre §§8.1–8.3, §9.1 `asset:fetch`, §10 (bridge/WUI sync). Ola B.
