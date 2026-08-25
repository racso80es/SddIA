---
document_id: AUDIT-KAIZEN-PACIENTE0-REDEPLOY-20260825-RESIDUAL
uuid: "d4f13e9a-5d91-4ab8-a2f5-be2e6b8c4815"
title: "Auditoría empírica Paciente 0 — residual F-DEP-07/08/09 F-SMOKE-01"
created: "2026-08-25"
feature_name: kaizen-paciente0-redeploy-20260825
persist_ref: docs/features/kaizen-paciente0-redeploy-20260825
instance_path: /home/racso/Proyectos/SddIA_AP
bundle_manifest: "20260825T124331Z"
instance_creator_correlation_id: "37890eec-266f-468e-9eda-1d269dd3c77b"
---

# Auditoría — Paciente 0 residual (2026-08-25 T6)

No duplica `docs/audits/kaizen-paciente0-redeploy-20260825.md` (T6 antecesor / G5). Bitácora del redeploy **un** `instance-creator` tras absorber F-DEP-07/08/09 y F-SMOKE-01.

## 1. Canal

```text
forja feat/kaizen-paciente0-redeploy-20260825
  → build-release-bundle SIN --skip-build --out SddIA_AP
  → plant {} en .SddIA/local.paths.json (precondición F-DEP-08; no unlink)
  → instance-creator env -u SDDIA_EXECUTE_PROCESS_BIN (sin pin)
  → systemd %f copia operador (L-SYS)
  → start-sddia.sh con pin forja (F-DEP-09)
```

Vault: `SddIA_AP.deploy-vault` (`vault_files_copied=6`). Secretos no versionados. Sin G5.

## 2. Fricciones → estado

| ID | 12:01 (debug stale) | T6 este ciclo |
|----|---------------------|---------------|
| F-DEP-07 | Creator ELF debug pre-Kaizen | Sin pin: handler release/debug fresco; overlay+CORE_ROOT correctos |
| F-DEP-08 | `unlink` de `{}` | Stub `{}` sustituido por starter-kit en **un** creator |
| F-DEP-09 | Orquestador = ELF forja | Log: discard pin; `orquestador (bundle): …/SddIA_AP/SddIA/target/release/execute-process` |
| F-SMOKE-01 | `Local_QA_Requested` sin `branch` | Smoke `local_qa_emitted: false`. Padres `afc03462` / `7688b280` son **cicatriz previa** (sweeper alerta; no emisión nueva) |
| F-SYS-01 | enable operador | Sin cambio (L-SYS) |

## 3. Métricas

| Métrica | Valor |
|---------|-------|
| Bundle `created_at` | `20260825T124331Z` |
| `.rs` / PY_LEAK | 0 / no |
| Binarios / cápsulas | 7 / 7 |
| `instance-creator` | `success:true` cid `37890eec-…` vault 6 |
| Overlay post-creator | no `{}`; `local_tools` presente |
| ExecStart | `/home/racso/Proyectos/SddIA_AP/SddIA/daemons/email-watcher.sh` `active` |
| Ignición pin forja | F-DEP-09 CONFIG + ELF instancia |
| `cargo build` chunk T6 | 0 |
| WUI | HTTP 200 `:8766` |

## 4. Residual

- Padres `Local_QA_Requested` pre-Kaizen en `pending/` (dead-letter `payload.branch`). No reemitidos.
- `DT-SYSTEMD-USER-ENABLE` / wizard / G5 / F-TRIAGE-03.
- Primer `nohup` chocó puerto 8766 (instancia previa); segundo arranque: Kalma2 ACTIVO.
