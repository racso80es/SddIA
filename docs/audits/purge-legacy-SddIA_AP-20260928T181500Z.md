---
title: "Purga legado Paciente 0 — SddIA_AP"
created: "2026-09-28T18:15:00+02:00"
verdict: APTO
instance_root_purged: "/home/racso/Proyectos/SddIA_AP"
paciente0_ssot: "/home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA"
pbi_ref: "PBI-OPERATIVO-APLICACIONES-FORGE-REDEPLOY AC-OP-6"
procedure_ref: "docs/todos/pending/[DEUDA] Paciente 0 — prompt de teardown.md v1.1.0"
---

# Purga `/home/racso/Proyectos/SddIA_AP` (descatalogado)

**Veredicto:** APTO (teardown legado)

| Check | Resultado |
|-------|-----------|
| `INSTANCE_ROOT` ausente | OK (`rm -rf` tras guards) |
| Unidades `@home-racso-Proyectos-SddIA_AP` | stop/disable/reset-failed (ya inactive) |
| `pgrep` bajo legado | vacío |
| Forja `/home/racso/Proyectos/SddIA` | intacta |
| Bóveda `/home/racso/Proyectos/.dev/.env` | intacta |
| Paciente 0 Aplicaciones | intacto; bridge `:8766` sigue en PID de Aplicaciones (no forja) |
| `:8765` forja | sin cambio (bridge forja) |

**Nota:** vaults `*.deploy-vault` / staging no tocados (fuera de alcance teardown §2).
