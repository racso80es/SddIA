---
contrato_version: "1.1.2"
id_cambio: "a3c5accf-54fd-45ed-9b1e-fdf444e836e6"
fecha: "2026-10-14"
tipo_operacion: modificacion
descripcion_breve: "Kalma2: checkout de la rama de trabajo antes del gate dirty, barrier si init falla, y estado que no regresa a la espera."
hash_integrity: "sha256:41fa565014a422a0810ca502495c38e4421f9b6bc8e50488979fb199f4350eb6"
relacionado:
  - "PBI-FIX-KALMA2-BRANCH-AND-STATUS"
  - "SddIA/engine/execute-process/src/engine/workspace_init.rs"
  - "SddIA/engine/execute-process/src/engine/executor.rs"
  - "SddIA/engine/execute-process/src/engine/progress_trace.rs"
  - "SddIA/interfaces/kalma2-bridge/src/main.rs"
document_id: PBI-FIX-KALMA2-BRANCH-AND-STATUS
---

# PBI-FIX-KALMA2-BRANCH-AND-STATUS — rama antes de mutar y aviso de avance

BR-CA1–CA5: el checkout de la rama de trabajo es el primer paso git; si falla, el ciclo termina sin mutar la base. Inicialización `failed` deja Diseño, Ejecución y Verificación en `skipped`. `GET /api/status` nombra el fin de fase `executed` o `blocked` y no restaura la espera de un PEC `awaiting_agents` posterior. El mensaje lleva el veredicto corto del agente.