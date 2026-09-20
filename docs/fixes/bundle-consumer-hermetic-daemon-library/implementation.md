---
feature_name: bundle-consumer-hermetic-daemon-library
created: "2026-09-20"
process: bug-fix
branch_name: fix/bundle-consumer-hermetic-daemon-library
persist_ref: docs/fixes/bundle-consumer-hermetic-daemon-library
---

# Implementación

| Archivo | Cambio |
|---------|--------|
| `SddIA/scripts/common/sddia_shell_lib.sh` | `_sddia_instance_is_hermetic_bundle`; rama MANIFEST en `_sddia_resolve_daemon_binary` |
| `SddIA/scripts/qa/test-daemon-binary-resolver.sh` | Caso bundle sin crate |
| `SddIA/scripts/build-release-bundle.sh` | Podado codexes consumer + gate Filtro C |
| `SddIA/scripts/qa/test-build-release-bundle-filtro-c.sh` | Smoke nuevo |
