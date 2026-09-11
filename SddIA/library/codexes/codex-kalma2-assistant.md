---
uuid: "c43544f3-c557-4cc3-8a03-7175282f2c88"
name: "SddIA Codex Kalma2 Personal Assistant"
version: "1.0.1"
nature: "domain-codex"
author: "codex-creator"
target_environment: ["personal-assistant", "email", "kalma2"]
certification_grade: "Pendiente"
process_membership:
  - email-triage-gateway
  - email-noise-digest
composition:
  - norm: "3d8c7e09-6d98-422d-909f-5b233ba7fcf2"
    path: "../norms/email-triage-matrix.md"
  - norm: "4c448c82-de41-460f-b24f-82a84fa5ed69"
    path: "../norms/features-documentation-pattern.md"
dlt:
  asset_class: "domain-codex"
  mint_status: "pre-mint"
  ledger: "iota-rebased-testnet"
  canonical_hash: "sha256:5e7ab8d9ab578785ac1101ac89e34b86fdbccfcb4a81760d96dcea06f47328a7"
  token_id: null
  owner_vertex: "biological-vertex"
hash_signature: "sha256:5e7ab8d9ab578785ac1101ac89e34b86fdbccfcb4a81760d96dcea06f47328a7"
---

# SddIA Codex Kalma2 Personal Assistant

## Estrategia de Dominio

Empaqueta la ley del triaje de correo (`email-triage-matrix`) y los procesos `email-triage-gateway` y `email-noise-digest` como activo tokenizable de asistente personal. El Core permanece ciego: solo transporta ECST y resuelve capacidades. La semántica de ruido/pasivo/accionable vive aquí.

## Instrucciones de Prioridad

1. **`email-triage-matrix`**: prevalece sobre cualquier heurística del agente ejecutor o del LLM. Triaje-C determinista cierra antes que Clasificacion.
2. **`features-documentation-pattern`**: rige la cascada documental de tareas del propio Core; no altera el veredicto de un correo.
3. Prohibido reintroducir la matriz en `SddIA/process/` o en la cápsula del Centinela.
