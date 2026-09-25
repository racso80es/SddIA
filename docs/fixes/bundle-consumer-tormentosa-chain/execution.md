---
feature_name: bundle-consumer-tormentosa-chain
created: "2026-09-25"
process: bug-fix
branch_name: fix/bundle-consumer-tormentosa-chain
persist_ref: docs/fixes/bundle-consumer-tormentosa-chain
pbi_document_id: PBI-FIX-BUNDLE-CONSUMER-TORMENTOSA-CHAIN
execution_id: "4480e892-4647-48c5-a1de-934800e768d7"
---

# Ejecución

## Init

`SDDIA_LAB_ALLOW_DIRTY=1 SDDIA_AGENT_RELAY_IDE=1 ./sddia-run.sh --process bug-fix` → `execution_id` `4480e892-4647-48c5-a1de-934800e768d7`. Diseño `simulated`. Commit planificación `43ec2ea`.

## Genoma

`./sddia-run.sh --process entity-manager --inputs-file .tmp/em-instance-creator-1.4.0.json` → `instance-creator` 1.4.0, hash `sha256:ada69320770bdfec9d488261be9090035aa66d645ef1f518b521c19ad07705db`.

## Tests locales

```text
cargo test -p execute-process --lib engine::handlers::instance_creator
# 8 passed (3 previos + 5 nuevos)

bash SddIA/scripts/qa/test-instance-root-resolver.sh          # OK
bash SddIA/scripts/qa/test-daemon-binary-resolver.sh          # OK
SDDIA_BUNDLE_SKIP_WITNESS=1 bash SddIA/scripts/qa/test-build-release-bundle-filtro-c.sh
# OK; MANIFEST binaries=13; 4 ELF Tormentosa presentes en stage

SDDIA_BUNDLE_DIGEST_ONLY=antigravity-cli-executor ./SddIA/scripts/build-release-bundle.sh
# sha256:8743bd5c0a094d63a04516fef8cfb1342dc64eae17e187e3c846627f10850ae2

sddia-qa evolution-rehash --id 806c9463-4c82-4216-a246-d5650a8553e9
# sha256:feb135919449fcd3f39f9c60a78114bf3ed370969aa0fbc745c1c892f647d4fa
```

## Entrega

DCC `351c1ac3-4bdf-49a7-be3c-b7daa7e1279c` → PR #298. CI run `36129200795` verde en `e0a60e8`. `validacion.md` `global: APTO`.
