---
feature_name: kaizen-delivery-close-snapshot-pr-body
created: "2026-07-22"
process: bug-fix
branch_name: fix/kaizen-delivery-close-snapshot-pr-body
persist_ref: docs/fixes/kaizen-delivery-close-snapshot-pr-body
pbi_ref: docs/todos/pending/[Kaizen] delivery-close — snapshot vacío y pr_body newlines en shell-executor.md
---

# Objetivos — kaizen-delivery-close-snapshot-pr-body

## Misión

Kaizen: delivery-close-cycle debe (1) consolidar o abortar si hay WIP dirty en Snapshot final; (2) abrir PR con --body-file sin pasar pr_body multilinea por argv de shell-executor; (3) tipar errores PR_BODY_METACHAR / SNAPSHOT_DIRTY_SKIPPED.

Semilla: docs/todos/pending/[Kaizen] delivery-close — snapshot vacío y pr_body newlines en shell-executor.md.
Origen: PR #127 / execution 067337ee-4ed1-44f5-b5be-40e8d7f6deb5.

K1 Snapshot: Con cambios unstaged/untracked, delivery-snapshot-final hace commit (git-manager) o aborta con error explícito; nunca reporta executed con snapshot_commit_hash = HEAD previo y working tree sucio.
K2 Apertura PR: delivery-gh-pr no pasa pr_body multilínea como argumento de shell-executor; usa --body-file (path bajo persist_ref / .tmp) o equivalente sin metacaracteres prohibidos.
K3 Diagnóstico: Si falla Apertura en forja, envelope/fase incluye causa tipada (PR_BODY_METACHAR / SNAPSHOT_DIRTY_SKIPPED).
K4 Paridad lab: Smoke o test body multilínea + working tree con untracked → close exit 0 o fail determinista.

Fuera de alcance: residual Hito 3 PBI-042; relajar allowlist metacaracteres de shell-executor; reescribir producto DI PR #127.
Relacionado cerrado: PBI-KAIZEN-DELIVERY-CLOSE-SHELL-EXECUTOR-WASM-FALLBACK (wasm; no este alcance).

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
