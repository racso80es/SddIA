---
feature_name: dcc-gh-api-connect-49ce2db7152d
created: "2026-09-08"
process: bug-fix
branch_name: fix/dcc-gh-api-connect-49ce2db7152d
persist_ref: docs/fixes/dcc-gh-api-connect-49ce2db7152d
pbi_ref: docs/todos/pending/[FIX] delivery-close-cycle — fractura sistémica (49ce2db7152d).md
execution_id: "0165ad09-f245-4465-aa03-280770a4ac93"
---

# Objetivos — dcc-gh-api-connect-49ce2db7152d

## Misión

DCC Apertura en forja colapsa a System_Fracture_Detected (Kintsugi) cuando gh falla con 'error connecting to api.github.com'. F4c ya clasifica DNS/timeout/unreachable; este token de gh CLI no está en dcc_transient_network_trace. Hueco: traza 'no se pudo resolver pr_url desde gh' + gh_stderr de conectividad se trata como fallo ontológico. Mandato: extender predicado F4c; sin retry; sin bypass raw;

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
