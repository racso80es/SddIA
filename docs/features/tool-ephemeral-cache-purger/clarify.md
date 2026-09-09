---
feature_name: tool-ephemeral-cache-purger
created: "2026-09-09"
process: feature
purpose: Estabilización Filtro A PBI v1.2.0; laudos de perímetro, I/O y DoD
version_clarify: "1.0.0"
execution_id: "7f37a724-5d10-41df-9cc9-cf22dc275951"
pbi_ref: docs/todos/pending/[FEATURE] Tool: ephemeral-cache-purger (Saneamiento Termodinámico).md
document_id: PBI-FEATURE-TOOL-CACHE-PURGER
pbi_uuid: "0987ac64-2c95-41c4-9ca7-d777446034cb"
pbi_version: "1.2.0"
---

# Clarificación — tool-ephemeral-cache-purger

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery + `SDDIA_LAB_ALLOW_DIRTY=1`. `execution_id` `7f37a724-5d10-41df-9cc9-cf22dc275951`. Rama `feat/tool-ephemeral-cache-purger`. Mayeuta…Argos: simulated / phase-barrier; relevo IDE.

Semilla: PBI v1.2.0 (Filtro A). Dirty del PBI v1.1.0→v1.2.0 viajó en el checkout de rama.

## Decisiones

| ID | Laudo |
|----|-------|
| L-ENTITY | Entidad = **Tool** Core (`scope: core`), no Skill. Acción orquestadora `purge-sandbox-cache` sin agentes. |
| L-CONTEXT | `filesystem-ops` (matriz `execution-contexts.md` §2.2). Cerbero = RBAC de contexto. Jail de paths = Rust. |
| L-REGEX | SSOT único: `^/tmp/cursor-sandbox-cache(/[a-f0-9]{16,64}(/.*)?)?$`. |
| L-DEFAULT | Candidatos = `…/<hash>/cargo-target`. `purge_sandbox_root: false`. |
| L-BLACKLIST | Prefijo de componente, no substring `/`. `$HOME` por env. Cero `/home/racso`. |
| L-SYMLINK | Walk `symlink_metadata` antes de cualquier `canonicalize`. |
| L-IO | Envelope = `sddia-io` (`success`, `exitCode`, `feedback`, `result`, `error`). Sin campo `name`/`message` inventados. |
| L-SUBSTRATE | `rust-native`. Excepción a tools-contract §8: harness WASI usa `--dir=.`. |
| L-TWO-PHASE | Acoplamiento dry-run→purga en la **acción** (misma invocación). Tool `simulate: false` sigue jail-only. |
| L-ARGOS | Argos = fase Verificación del proceso `feature`. Prohibido en runtime de la acción. |
| L-FORGE | Genoma `.md`+índice vía `entity-manager`. UUID autoridad = creator. Crate bajo `implementation_path_ref` post-forja. Handler de acción en engine (no DA-2). |
| L-CA7 | Fixture de test bajo el prefijo, uid del runner. Host `root:root` 8.4 GB = lab; `EACCES` no bloquea merge. |
| L-CI | `validacion.md` no `global: APTO` hasta `run_id` verde. `accept-pr` solo tras checks verdes del PR. |
| L-NFT | NFT-Ready (identidad). DLT/MoveVM fuera (Kitchen). |

## Fuera (este ciclo)

WASI preopen `/tmp`; tokenización on-chain; purga de árboles fuera del regex; sudo/escalada de privilegios; `max_depth` como recorte de `cargo-target`.
