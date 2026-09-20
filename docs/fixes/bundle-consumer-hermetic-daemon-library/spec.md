---
feature_name: bundle-consumer-hermetic-daemon-library
created: "2026-09-20"
process: bug-fix
branch_name: fix/bundle-consumer-hermetic-daemon-library
persist_ref: docs/fixes/bundle-consumer-hermetic-daemon-library
pbi_document_id: PBI-FIX-BUNDLE-CONSUMER-HERMETIC-LIBRARY
execution_id: "42efc1b8-3a23-4dc4-920c-5e2e4ff21f6b"
---

# Especificación — bundle consumer hermetic + Filtro C

## Diagnóstico

| ID | Síntoma | Causa |
|----|---------|-------|
| F-BUNDLE-HERMETIC-DAEMON-RESOLVER | `kalma2-bridge.sh` / centinelas exit 1 pese a ELF en bundle | `_sddia_resolve_daemon_binary` exige `Cargo.toml` del crate (post aduana ELF↔fuente) |
| F-BUNDLE-LIBRARY-FILTRO-C | G4 KO: `codex-software-engineering` en instancia | `build-release-bundle` rsync de `SddIA/library` completo |

## Diseño

1. **`sddia_shell_lib.sh`:** si `MANIFEST.json` en raíz → resolver release/debug ELF sin aduana de fuente.
2. **`build-release-bundle.sh`:** en consumer, tras rsync, podar `library/codexes` al `--codex` + `index.md` + `codex-contract.md`; gate fail-closed si queda `codex-software-engineering`.

## CA

- Smoke `test-daemon-binary-resolver.sh` (caso bundle).
- Smoke `test-build-release-bundle-filtro-c.sh`.
