---
feature_name: kalma2-mvp-sensorial-email
created: "2026-08-18"
process: feature
branch_name: feat/kalma2-mvp-sensorial-email
persist_ref: docs/features/kalma2-mvp-sensorial-email
uuid: "c209c150-8ab4-4f0d-bcf7-8fa7a6101de0"
status: executed
agent: tekton
document_id: PBI-KALMA2-MVP-01A
execution_id: "fa4dde03-a0ec-426f-ade7-850246ba7575"
items_applied:
  - T0-topologia
  - T1-ratificacion-ssot
  - T2-ley-y-codice
  - T3-eventos
  - T4-centinela
  - T5-triaje
  - T9a-aduana-parcial
  - T10-post-auditoria
---

# Ejecución — kalma2-mvp-sensorial-email

## Gates T0–T5

| Gate | Criterio | Evidencia | Resultado |
|------|----------|-----------|-----------|
| **G0** | Topología 01A | `docs/features/kalma2-mvp-sensorial-email/{objectives,clarify,spec,plan}.md` + rama `feat/kalma2-mvp-sensorial-email` | APTO |
| **G1** | R-01/R-02 + retrocompat | `codex-contract` v1.2.0; `cumulo.paths.json` v1.6.3; 4 códices sin `dlt` | APTO |
| **G2** | Identidad de activo | `canonical_hash == hash_signature` `sha256:01738b6c938b72eacd18435a62780b074b1949e5db48f99a4af8a2cdd73b5f6b`; `token_id: null`; `pre-mint` | APTO |
| **G3** | Clases + suscripción | `event-bus-audit` `d266329c-…` `exitCode:0`; `Email_Received` sin hallazgo; `Email_Triaged` EMPTY+PURGE (F-05); 0 `ORPHAN` Email_* | APTO |
| **G4** | Ceguera lógica de la cápsula | `rg` `SddIA/daemons/email-watcher/src`: 0 `execute-process`, 0 `SddIA/`, 0 IMAP write, 0 `/home/` | APTO |
| **G5** | Peaje termodinámico | CLI 2026-08-18: `verdict=noise` `decision_path=deterministic` `classification_ran=false`; fase `Clasificacion` `skipped`; coste `{0,0,0}`; `event_id` `2abd8420-396c-44e0-8852-c657b3d8071f` | APTO |

## G5 — acuse CLI

```text
./sddia-run.sh --process email-triage-gateway --inputs '{"event_file_path":".tmp/g5-email-received-list.json"}'
```

`success:true` · `verdict:noise` · `decision_path:deterministic` · `emitted:true` · `Clasificacion.status:skipped` (`triaje-c-concluded`) · `matched_rule:C-LIST`.

Testigo: `.SddIA/proofs/email-triaged/2abd8420-396c-44e0-8852-c657b3d8071f.json` (`thermodynamic_cost` en ceros).

Units: `cargo test -p execute-process email_triage` (3) · `cargo test -p email-watcher` (4, incl. watermark) · `cargo test -p kalma2-bridge build_status_body_projects_email_triaged` (2, domain + proof post-purge).

## T9a

| Ítem | Resultado |
|------|-----------|
| Idempotencia UID | APTO (unit `watermark_skips_uids_already_seen` + roundtrip) |
| WUI proyección `Email_Triaged` | APTO (unit bridge; testigo durable F-05) |
| `daemon-heartbeat-audit` sweep | APTO · `fractures_emitted: []` · `status:sweep` |
| `event-bus-audit` | APTO · `exitCode:0` · `emit_kaizen_alert:false` · workspace `d266329c-ec1d-42d8-8847-bb9589d93a88` · Email_*: 0 huérfanos; EMPTY+PURGE solo `Email_Triaged` (spec) |
| Heartbeat vivo `email-watcher` | DEFER · reiniciar `./start-sddia.sh` para loop continuo |
| SIGKILL &lt;5 s | DEFER lab (template systemd presente) |
| Correo IMAP real → WUI | APTO · UID 104385 «Kalma2 validación 01A» · proof `5e7e24e0-8121-4911-bafa-f9e39924d384` |

## T10 · Post-auditoría IMAP

| Ítem | Resultado |
|------|-----------|
| A-01 ventana inicial 60 d (`SINCE`) | APTO |
| A-02 lock huérfano | APTO |
| A-03/A-05 `start-sddia` + launcher | APTO |
| A-04 bóveda instancia | APTO |
| A-06 UNSEEN + cap + watermark contiguo | APTO · units `plan_poll_prioritizes_unseen` · `contiguous_watermark_skips_high_uid_gap` |
| F-06 UTF-8 cabeceras IMAP | APTO |
| F-07 enrutamiento E2E triaje | APTO |
| Lab E2E Racso | APTO · `verdict:passive` · WUI status OK |

## Comandos

```bash
cd SddIA && cargo test -p execute-process email_triage
cd SddIA && cargo test -p email-watcher
cd SddIA && cargo test -p kalma2-bridge build_status_body_projects_email_triaged
cd SddIA && cargo test -p event-bus-audit utf8_prefix_does_not_split_emdash
./sddia-run.sh --process email-triage-gateway --inputs '{"event_file_path":".tmp/g5-email-received-list.json"}'
./sddia-run.sh --process daemon-heartbeat-audit --inputs '{"sweep":true}'
./sddia-run.sh --process event-bus-audit --inputs '{"emit_kaizen_alert":false}'
```

## Notas

- Forja nativa de códice/skill es esqueleto: enriquecimiento post-acuse conservando uuid+hash.
- `process_jurisdiction` + `process_domain_root` obligatorios al forjar procesos empacados.
- F-04: corte UTF-8 en informe de dead-letter; cápsula parcheada, `event-bus-audit.md` no tocado.
- IMAP: `SDDIA_EMAIL_IMAP_HOST` vacío → T9a lab (latido, SIGKILL, correo real) no ejecutable en este host.
- Evolution y `delivery-close-cycle`: G9b / cierre documental; no en este estímulo.
- Fixture G5 en `.tmp/` (one-shot; no versionar).
