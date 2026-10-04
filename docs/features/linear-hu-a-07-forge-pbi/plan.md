---


feature_name: linear-hu-a-07-forge-pbi
---
# Plan — HU-A 07

1. Bump proceso `forge-pbi` (fases Recepción → Registro en tracker → Sellado).
2. Handler: `create_issue` con `parent_ref` desde `tracker_ref` de `historia_ref`; fail-soft sin HU padre.
3. Tests lab mock + gate Cerbero.
4. Cierre documental y PR único.
