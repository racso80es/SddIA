---
feature_name: p16-pyyaml-poda
created: "2026-07-10"
process: bug-fix
items_applied:
  - grep-audit-pyyaml-consumers
  - requirements-txt-justification
  - golden-orchestrator-verification
  - native-without-python-smoke
---

# Ejecución — P16 poda PyYAML

## Auditoría ejecutada

```bash
rg -l 'import yaml|from yaml|PyYAML' SddIA/scripts/qa/ --glob '*.py'
```

**Resultado:** 7 archivos con importación directa; 8 adicionales vía `parse_frontmatter` / bridges.

## Verificación orquestador (CA-8)

| Prueba | Resultado |
|--------|-----------|
| `golden_orchestrator_parity.py` | ✅ 14/14 |
| `orchestrator_touchpoint_e2e_smoke.py` → `native-without-python` | ✅ OK |

El entrypoint productivo **no requiere PyYAML**. Confirmado.

## Decisión sobre `requirements.txt`

**Poda NO aplicada.** Motivos:

1. **Gate P16:** `grep` **no** está limpio en paths productivos del subgrafo QA/bridge.
2. **Bridges activos:** `_execute_process_route_bridge.py` y `_execute_process_capsules_bridge.py` siguen invocando `execute_process_core.py` (PyYAML obligatorio).
3. **CI:** `.github/workflows/sddia-index-qa.yml` instala `pyyaml` para `verify-process-integrity.py`, `verify-task-closure.py`, `audit-doc-parity.py`.
4. **FIX hijos pendientes:** porte route bridge + capsules bridge deben cerrarse antes de la poda total.

## Justificación de mantener `PyYAML` en `requirements.txt`

```text
requirements.txt (raíz) = dependencia lab mínima para scripts QA y bridges Python residuales.
El orquestador binario Rust (CA-8) no la consume.
Eliminación condicionada a cierre P17-bridges + grep limpio en SddIA/scripts/qa/.
```

## Próximo gate de poda

Ejecutar eliminación de `PyYAML` cuando:

- `[FIX] Porte route-domain-event core a Rust` → cerrado
- `[FIX] Porte procesos residuales capsules bridge a Rust` → cerrado
- `rg 'import yaml|PyYAML' SddIA/scripts/qa/` → vacío
- CI migrado a parser Rust o scripts retirados
