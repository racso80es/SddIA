---
feature_name: diff-digest-ci
pbi_ref: AUT-40
---

# Especificación

- Verificación en `diff_seal::verify_diff_seal_ci` leyendo `validacion.md` vía `git show` en HEAD del PR.
- Gate cuando `global: APTO` o cualquier campo de sello presente.
- Faltan cuatro campos → rojo; digest divergente → rojo.
