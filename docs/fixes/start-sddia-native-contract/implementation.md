---
feature_name: start-sddia-native-contract
created: "2026-07-13"
process: bug-fix
items:
  - required-daemon-gate
  - native-elf-validation
  - debug-profile-priority
  - start-documentation-alignment
---

# Implementación — contrato nativo de `start-sddia`

| Touchpoint | Cambio |
|---|---|
| `start-sddia.sh` | Separa centinelas obligatorios de opcionales, exige éxito individual, verifica ELF y expone el binario resuelto. |
| `SddIA/daemons/*.sh` | Exige bit ejecutable y prioriza debug sobre release. |
| `sddia_shell_lib.sh` | Rechaza overrides no ELF y resuelve `execute-process` debug antes de release. |
| `kalma2-bridge` | Verifica magic ELF para el orquestador y devuelve un error explícito si no es nativo. |
| `start-sddia.md` | Alinea construcción/ejecución desde raíz y formaliza la garantía nativa. |

La prioridad debug conserva coherencia con el prerequisito documentado `cargo build`; release se conserva como fallback para instalaciones que solo publiquen dicho perfil.
