---
uuid: "c4a8f2e1-7b3d-4e9a-a1c6-5d8f0b2e4a71"
id: capability-bindings
name: capability-bindings
version: "1.0.0"
nature: "runtime-ssot"
scope: "agnostic"
bindings:
  - capability_id: "doc:closure"
    contract: "doc.closure"
    provider: "skill:filesystem-manager"
    provider_version: ">=1.0.0"
  - capability_id: "proc:git-sync"
    contract: "proc.git_sync"
    provider: "skill:git-manager"
    provider_version: ">=1.0.0"
---

# Capability bindings (SSOT DI)

Mapa **capability → artefacto canónico** para resolución ciega (PBI-042 Hito 2).

- Una fila por `capability_id` (Q2 / L-CODEX-ROLE).
- Lectura runtime solo vía `capability_di.bindings` en Cúmulo.
- No es Library_Codex de normas; no actúa como taxonomía.
