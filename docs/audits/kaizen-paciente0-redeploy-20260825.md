---
document_id: AUDIT-KAIZEN-PACIENTE0-REDEPLOY-20260825
uuid: "56aff1d3-d5f6-4502-9b5b-e5a57dc718e3"
title: "Auditoría empírica Paciente 0 — Kaizen redeploy 2026-08-25"
created: "2026-08-25"
feature_name: kaizen-paciente0-redeploy-fricciones
persist_ref: docs/features/kaizen-paciente0-redeploy-fricciones
instance_path: /home/racso/Proyectos/SddIA_AP
bundle_manifest: "20260825T111733Z"
instance_creator_correlation_id: "9528fb5f-9d72-4990-adce-44bca7dc734d"
g5_event_id: "413e6edf-e19c-4ec4-8b5c-e9ccd6a4d13f"
---

# Auditoría — Paciente 0 redeploy Kaizen (2026-08-25)

Consulta futura: bitácora del redeploy **sin parches de instancia** tras absorber F-DEP-* / F-TRIAGE-01 en Core. Complementa el PBI 2026-08-24 (fricciones) y el ensayo 2026-08-20.

## 1. Canal canónico usado

```text
forja feat/kaizen-paciente0-redeploy-fricciones
  → build-release-bundle (SIN --skip-build) dist/sddia-release-consumer-t6
  → --skip-build replay OK (cicatriz SHA-256)
  → rsync overlay → /home/racso/Proyectos/SddIA_AP (preserva .SddIA / .events)
  → instance-creator (vault deploy-vault, skip_ignition)
  → systemd %f CORE_ROOT=instancia (sin re-render manual)
  → ./start-sddia.sh (MANIFEST presente; 0 cargo)
```

Vault: `SddIA_AP.deploy-vault` (6 ficheros). Secretos no versionados.

## 2. Fricciones 2026-08-24 → estado 2026-08-25

| ID | 2026-08-24 | 2026-08-25 |
|----|------------|------------|
| F-DEP-01 CORE_ROOT | Parche operador; ExecStart lab | `ExecStart=/home/racso/Proyectos/SddIA_AP/SddIA/daemons/email-watcher.sh` vía creator |
| F-DEP-02 cargo en bundle | Parche `_ensure_orchestrator` instancia | Log ignición: 0 `cargo build`; ELF resuelto |
| F-DEP-03 ELF `.py` | `strings` con `execute-process.py` | `PY_LEAK=no`; EDA `route-telemetry` / `route-domain` purgado |
| F-DEP-04 `local.paths` `{}` | Copia manual starter-kit | Creator materializa overlay no vacío |
| F-DEP-06 cola domain | Skip max attempts | Watcher enruta; smoke creator `route_domain skipped` (skip_ignition); probe clase inventada = ECST-gate fail (esperado) |
| F-TRIAGE-01 reunión→passive | Proof `6e552199-…` passive | G5 lab `t6-g5-20260825` → `actionable` + `subject_elevation: true` |
| F-TRIAGE-02 tokens 0 | Inferencia no medible | G5: tokens 0 + elevación estructural (guard, no LLM) |
| F-TRIAGE-03 inbox passive | Fuera de ciclo | Inbox WUI muestra `actionable` |

## 3. Métricas T6

| Métrica | Valor |
|---------|-------|
| Bundle `created_at` | `20260825T111733Z` |
| `.rs` en bundle | 0 |
| Binarios / cápsulas | 7 / 7 |
| Testigos SHA-256 | 7 (CONSUMER_BINS) |
| `--skip-build` post-build | exit 0 |
| `--skip-build` pre-testigo (T1) | exit 1 |
| WUI | HTTP 200 `:8766` |
| `instance-creator` | `success:true` `vault_files_copied=6` |
| Systemd email-watcher@AP | `active` |
| G5 verdict | `actionable` |
| G5 proof | `413e6edf-e19c-4ec4-8b5c-e9ccd6a4d13f` |
| Agenda | `6ec66cd2-…` título reunión 26/08/2026 10:00 |
| WUI `/api/email-inbox` | 1 item `t6-g5-20260825` |
| Telegram poke | `send-telegram-notification` instancia: `success:true` `message_id=9` |

## 4. G5 — método

No se esperó IMAP real (DA-5). Estímulo `Email_Received` sintético en bus de instancia + `email-triage-gateway` con ELF de la instancia. Asunto: `Reunión con Racso el 26/08/2026 a las 10:00`. Reproduce el patrón UID 104579 (2026-08-24) que salió `passive`.

Duplicado de proof/agenda: el watcher también consumió el `Email_Received` (esperado en instancia viva).

## 5. Residual

- G5 tokens 0: guard de asunto; T-INFER unitario cubre `classification-degraded` sin env.
- `rustc` drift fuera de cicatriz (spec §1.1).
- F-TRIAGE-03 / wizard UX: PBI distintos.
- `AGENT_RUNTIME_*` en `Proyectos/.dev` raíz: no podado (D11).
- IMAP real: fuera (spec §5).
- Cierre de entrega: `delivery-close-cycle` este estímulo.
