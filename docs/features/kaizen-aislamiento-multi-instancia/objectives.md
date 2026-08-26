---
feature_name: kaizen-aislamiento-multi-instancia
created: "2026-08-26"
process: feature
branch_name: feat/kaizen-aislamiento-multi-instancia
persist_ref: docs/features/kaizen-aislamiento-multi-instancia
pbi_ref: docs/todos/pending/[KAIZEN] aislamiento multi-instancia centinelas.md
document_id: PBI-KAIZEN-AISLAMIENTO-MULTI-INSTANCIA
uuid: "b5d19318-a0fd-440b-9aac-8c6d93f775ed"
execution_id: "3b40b62c-d048-4896-b8c1-1ee267ca7704"
mayeuta_verdict: ok
laudo: execstart-percent-f-launcher-cwd-no-pkill
---

# Objetivos — kaizen-aislamiento-multi-instancia

## Misión

Independencia de centinelas entre raíces (forja, Paciente 0, N clientes) en el mismo `user.slice`: molde systemd user universal (`%f`), lanzadores ciegos a `SCRIPT_DIR` cuando hay instancia, cero `pkill -x` global.

## Punto objetivo

> **O-AISLAMIENTO:** `ExecStart=%f/SddIA/…` (familia distinta para email vs fábrica); `instance-creator` y `start-sddia.sh` no hornean path absoluto de host en ExecStart; `REPO_ROOT` = `SDDIA_INSTANCE_ROOT` \| cwd con `.SddIA` \| fallback lab; parada = PID del lock de esa raíz.

## Alcance

| Dentro | Fuera |
|--------|-------|
| F-SYS-02 plantillas `%f` + creator + materialize | F-TRIAGE, G5 IMAP |
| F-DEP-10 resolución compartida en `sddia_shell_lib` | lock-session / linger |
| F-CEN-PKILL lock PID-only | forjar `paciente0-deploy` |
| Preferencia release>debug en wrappers email/kalma2 | Namespacing de nombres de unidad |

## Ley aplicada

- Ceguera espacial: `%f` = raíz de instancia en runtime systemd; no `@@SDDIA_CORE_ROOT@@` en ExecStart.
- Dualidad de lanzadores: email = `SddIA/daemons/`; resto = `SddIA/scripts/daemons/`.
- R-07 es criterio de ensayo, no de plantilla.
