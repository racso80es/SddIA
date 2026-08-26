---
feature_name: bundle-consumer-telegram-gateway
created: "2026-08-26"
updated: "2026-08-26T13:45:00Z"
process: bug-fix
agent: argos
branch: fix/bundle-consumer-telegram-gateway
branch_name: fix/bundle-consumer-telegram-gateway
persist_ref: docs/fixes/bundle-consumer-telegram-gateway
pbi_ref: docs/todos/done/[FIX] bundle consumidor — telegram-gateway ausente en grafo telegram-watcher.md
pbi_document_id: PBI-FIX-BUNDLE-TELEGRAM-GATEWAY
document_id: PBI-FIX-BUNDLE-TELEGRAM-GATEWAY
uuid: "67110f2f-2be8-4fd3-b0a7-8dc400fe803f"
friction_id: F-BUNDLE-06
pr_url: https://github.com/racso80es/SddIA/pull/194
global: APTO
pbi_archived: true
verdict: aprobado
checks:
  CA1_BUNDLE_ELF_MANIFEST: APTO
  CA2_GATE_FAIL_CLOSED: APTO
  CA3_WITNESS_SHA256: APTO
  CA4_TELEGRAM_GATEWAY_PROCESS: APTO
  CA5_PACIENTE0_G_TELEGRAM: PENDIENTE
  CA6_PBI_DONE: APTO
  branch: APTO
  git_changes: APTO
git_changes:
  - SddIA/scripts/build-release-bundle.sh
  - SddIA/norms/sddia-distribution-protocol.md
  - docs/fixes/bundle-consumer-telegram-gateway/
  - docs/todos/done/[FIX] bundle consumidor — telegram-gateway ausente en grafo telegram-watcher.md
  - docs/audits/paciente0-deploy-20260826T110203Z.md
---

# Validación — bundle-consumer-telegram-gateway (bug-fix)

## Veredicto

**APTO** — `global: APTO` · `pbi_archived: true` · PBI en `docs/todos/done/`.

| ID | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| CA1 | Bundle consumer incluye `telegram-gateway` en ELF + `MANIFEST.json` | **APTO** | `build-release-bundle.sh --profile consumer` → 8 bins / 8 capsules; `telegram-gateway` ∈ ambos |
| CA2 | Gate falla si `telegram-watcher` sin `telegram-gateway` | **APTO** | Revisión estática gate L380–387 `build-release-bundle.sh` |
| CA3 | Testigo `.sha256` para `telegram-gateway` | **APTO** | Escrito en ciclo build smoke |
| CA4 | `./sddia-run.sh --process telegram-gateway --inputs '{"text":"sigues?"}'` | **APTO** | `success:true`, `emitted:true`, `Manual_Task_Requested` |
| CA5 | Mensaje bot Paciente 0 / G-telegram | **PENDIENTE** | Requiere redeploy instancia con bundle post-merge (fuera forja) |
| CA6 | Cierre documental en rama | **APTO** | PBI archivado; `validacion.md` coherente |

## Smoke bundle (forja)

```text
[bundle] bins= 8 capsules= 8
test -x …/telegram-gateway → OK
MANIFEST capsules_resolved incluye telegram-gateway → OK
```

## Residuo operativo

Redeploy `SddIA_AP` con bundle regenerado tras merge PR #194 para cerrar CA5 empírico en Paciente 0.
