---
feature_name: p16-pyyaml-poda
created: "2026-07-10"
process: bug-fix
base: main
scope: requirements-pyyaml-audit
document_id: PBI-FIX-P16-PYYAML-PODA
version_spec: "1.0.0"
---

# Especificación — P16 poda PyYAML en `requirements.txt`

## Diagnóstico

| Síntoma | Evidencia |
|---------|-----------|
| `requirements.txt` raíz contiene solo `PyYAML` | Herencia post-migración orquestador Rust (CA-8 cumplido en entrypoint) |
| Gate P16 condicional (clarify D6) | Poda solo si `grep` no reporta importaciones activas en touchpoints productivos |
| Bridges residuales activos | `_execute_process_route_bridge.py`, `_execute_process_capsules_bridge.py` en `pending/` |

**Causa raíz:** PyYAML ya no es requerido por el **entrypoint** orquestador (`execute-process` binario Rust, `orchestrator_resolve.py`), pero persiste en el **subgrafo Python residual** (bridges internos + scripts QA de integridad/auditoría).

## Corrección (alcance P16)

1. Auditar consumidores directos e indirectos en `SddIA/scripts/qa/`.
2. Evaluar gate de poda: eliminar de `requirements.txt` **solo** si no hay importaciones activas.
3. Documentar justificación en `execution.md` si la poda no procede.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA1 | Inventario completo de consumidores PyYAML (directos + vía `execute_process_core`) |
| CA2 | `requirements.txt` sin PyYAML **o** justificación documentada en `execution.md` |
| CA3 | Golden orchestrator 14/14 verde |
| CA4 | Smoke `native-without-python` verde (orquestador sin PyYAML) |
| CA5 | PBI archivado en `docs/todos/done/` |

## Decisión de diseño

La poda total queda **diferida** hasta cierre de:

- `[FIX] Porte route-domain-event core a Rust`
- `[FIX] Porte procesos residuales capsules bridge a Rust`

Mientras tanto, `requirements.txt` se mantiene con `PyYAML` como dependencia lab/QA documentada.
