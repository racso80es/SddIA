---
uuid: "c4a8f2e1-7b3d-4e9a-a1c6-5d8f0b2e4a71"
id: capability-bindings
name: capability-bindings
version: "1.7.0"
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
  - capability_id: "fs:persist"
    contract: "fs.persist"
    provider: "skill:filesystem-manager"
    provider_version: ">=1.0.0"
  - capability_id: "bus:route"
    contract: "bus.route"
    provider: "skill:bus-operator"
    provider_version: ">=1.0.0"
  - capability_id: "qa:probe"
    contract: "qa.probe"
    provider: "tool:event-bus-audit"
    provider_version: ">=1.0.0"
  - capability_id: "audit:compliance"
    contract: "audit.compliance"
    provider: "skill:compliance-auditor"
    provider_version: ">=1.0.0"
  - capability_id: "llm:interact"
    contract: "llm.interact"
    provider: "skill:mayeuta-llm"
    provider_version: ">=1.0.0"
  - capability_id: "gov:rbac"
    contract: "gov.rbac"
    provider: "skill:rbac-governor"
    provider_version: ">=1.0.0"
  - capability_id: "channel:ingest"
    contract: "channel.ingest"
    provider: "tool:telegram-gateway"
    provider_version: ">=1.0.0"
  - capability_id: "agenda:persist"
    contract: "agenda.persist"
    provider: "skill:agenda-manager"
    provider_version: ">=1.0.0"
  - capability_id: "memory:pref-write"
    contract: "memory.pref_write"
    provider: "skill:user-preference-store"
    provider_version: ">=1.0.0"
  - capability_id: "memory:pref-query"
    contract: "memory.pref_query"
    provider: "skill:user-preference-store"
    provider_version: ">=1.0.0"
  - capability_id: "asset:fetch"
    contract: "asset.fetch"
    provider: "tool:github-raw-fetcher"
    provider_version: ">=1.0.0"
  - capability_id: "tracker:lifecycle"
    contract: "tracker.lifecycle"
    provider: "tool:linear-tracker-adapter"
    provider_version: ">=1.0.0"
---

# Capability bindings (SSOT DI)

Mapa **capability → artefacto canónico** para resolución ciega (PBI-042 Hito 2).

- Una fila por `capability_id` (Q2 / L-CODEX-ROLE).
- Lectura runtime solo vía `capability_di.bindings` en Cúmulo.
- No es Library_Codex de normas; no actúa como taxonomía.
- H9: `qa:probe` canónico `tool:event-bus-audit`; procesos con otro tool que `provides` la misma cap resuelven por preferencia de `delegates_to`.
- H9-D: `audit:compliance` ≠ `qa:probe` (Gobernanza vs Caos).
- H11: `gov:rbac` → `skill:rbac-governor`; `channel:ingest` → `tool:telegram-gateway`.
- `tracker:lifecycle` → `tool:linear-tracker-adapter`. Una sola fila. El resolver la usa cuando el provider efectivo es `linear`; con `file` no hay `di_binding` (parser del manifiesto, PBI 14).
