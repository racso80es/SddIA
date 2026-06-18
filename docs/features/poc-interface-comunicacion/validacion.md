---
feature_name: poc-interface-comunicacion
branch: feat/poc-interface-comunicacion
global: APTO
pbi_archived: true
pr_url: https://github.com/racso80es/SddIA/pull/94
created: "2026-06-18"
process: feature
checks:
  AC1_ui_minima: "APTO — interfaces/kalma2/index.html + GET / HTTP 200"
  AC2_api_interact: "APTO — POST /api/interact JSON smoke success:true"
  AC3_bovedas: "APTO — load_hierarchical_env al arranque del puente"
  AC4_doble_envio: "APTO — app.js deshabilita botón durante fetch"
  AC5_smoke_termico: "APTO — eco visible; servidor estable tras petición"
  AC6_cierre_documental: "APTO — PBI en done/ + validacion.md en rama"
  O7_motor_real: "APTO — proceso kalma2-interact vía execute-process; puente integrado"
git_changes:
  - .SddIA/client/sddia-client-bridge.py
  - interfaces/kalma2/index.html
  - interfaces/kalma2/app.js
  - interfaces/kalma2/style.css
  - SddIA/process/kalma2-interact.md
  - SddIA/process/index.md
  - SddIA/core/eda-coverage.json
  - SddIA/scripts/qa/kalma2_interact_core.py
  - SddIA/scripts/qa/test_kalma2_interact.py
  - SddIA/scripts/qa/execute_process_capsules.py
  - docs/features/poc-interface-comunicacion/
  - docs/todos/done/PBI_PoC_Interface_Comuniccion.md
---

# Validación — PoC Interface Comunicación (Kalma2)

Argos laboratorio: entrega **APTO** para **Ola W1 PoC** en rama `feat/poc-interface-comunicacion`.

## Evidencia

| Check | Resultado |
|-------|-----------|
| Smoke API | `{"success": true, "response": "[eco PoC] ping de prueba kalma2"}` |
| UI estática | `GET http://127.0.0.1:8765/` → 200 |
| Bóvedas | `[CONFIG] Jerarquía detectada` en stderr al arrancar puente |

## Deuda aceptada (PoC)

- Sin Cerbero/RBAC — bind `127.0.0.1` exclusivo.
- Síntesis Mayeuta **lab determinista** (sin LLM externo); evolución conversacional real queda para feature posterior.

## Operador post-merge

```bash
python3 .SddIA/client/sddia-client-bridge.py
# Navegador → http://127.0.0.1:8765/
```
