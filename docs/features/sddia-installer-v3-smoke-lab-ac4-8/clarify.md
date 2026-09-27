---
feature_name: sddia-installer-v3-smoke-lab-ac4-8
pbi_uuid: "b93ca305-01e5-44ba-ab59-d4a70188d2e9"
---

# Clarify

- Rama origen: `main` (post merge stdin #308).
- `SDDIA_INSTALLER_LAB_SKIP_ENABLE=1`: no `systemctl enable --now` en CI; copia unidades y log a stderr.
- `SDDIA_INSTALLER_BUNDLE_PROFILE`: override solo para forzar fallo AC-5 en lab.
