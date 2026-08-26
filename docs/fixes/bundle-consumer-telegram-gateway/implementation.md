---
feature_name: bundle-consumer-telegram-gateway
created: "2026-08-26"
process: bug-fix
items:
  - seed-telegram-gateway-consumer-bins
  - gate-f-bundle-06-fail-closed
  - cargo-p-telegram-gateway
  - norm-f-06-aferente
branch_name: fix/bundle-consumer-telegram-gateway
persist_ref: docs/fixes/bundle-consumer-telegram-gateway
friction_id: F-BUNDLE-06
---

# Implementation — bundle-consumer-telegram-gateway

| Cambio | Path | Notas |
|--------|------|-------|
| Semilla + CAPSULE_SET | `SddIA/scripts/build-release-bundle.sh` | `telegram-gateway` ∈ `CONSUMER_BINS` / `CAPSULE_SET` (testigo + copia + manifiesto) |
| Cargo hardcode | mismo | `-p telegram-gateway` en línea de pkgs locales |
| Gate F-BUNDLE-06 | mismo | Si ELF `telegram-watcher` en stage → exigir `.md` + ELF `telegram-gateway` |
| ONBOARDING §5 | mismo | Checks aferentes en verificación rápida |
| Norma F-06 | `SddIA/norms/sddia-distribution-protocol.md` | Gate mínimo eferente + aferente condicional; bump `1.2.2`→`1.2.3` |

Sin mutación de genoma `tool:`/`process:`/`daemon:` telegram-gateway (defecto solo de empaquetado).
