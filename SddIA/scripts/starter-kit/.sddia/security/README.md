# Seguridad del producto (`.sddia/security/`)

Reglas específicas del repositorio cliente: OWASP, SAST/DAST, linters, políticas de dependencias, gates de CI.

- **Contrato:** `.sddia/local-security-contract.json`
- **Estructura:** una carpeta por regla bajo `.sddia/security/<rule-id>/` con `spec.json` (obligatorio) y `spec.md` (opcional).
- **Argos:** evalúa entregas usando **motor** (`SddIA/security/`) + **producto** (este árbol).
