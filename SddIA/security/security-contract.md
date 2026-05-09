---
contract_version: 1.0.0
nature: motor
description: Contrato que cada item de seguridad del motor debe cumplir en SddIA/security/.
scope: SddIA/security/
security_model:
  description: Contexto Karma2Token obligatorio para operaciones sensibles del motor.
  required_token: Karma2Token
  token_ref: SddIA/tokens/karma2-token.md
---

# Contrato de seguridad del motor (`SddIA/security/`)

**Alcance:** Políticas universales del runtime SddIA: límites de ejecución, prevención de inyección en orquestación, validación de entradas a cápsulas y hardening del Core.

**No cubre:** Reglas OWASP, linters o escaneo de dependencias del repositorio cliente; esas viven en `.sddia/security/` bajo `local-security-contract.json`.

**Estructura:** Un item por carpeta `SddIA/security/<uuid>/` con `spec.md` y `spec.json` conforme a `security-contract.json`.
