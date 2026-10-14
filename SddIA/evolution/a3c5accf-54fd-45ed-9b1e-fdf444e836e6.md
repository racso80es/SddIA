---
contrato_version: "1.1.2"
id_cambio: "a3c5accf-54fd-45ed-9b1e-fdf444e836e6"
fecha: "2026-10-14"
tipo_operacion: modificacion
descripcion_breve: "Kalma2: checkout de la rama de trabajo antes del gate dirty, barrier si init falla, y estado que no regresa a la espera."
hash_integrity: "sha256:6c0fcefbeef8d8d386bb724e7578880d015005416d3360e00f24479f10ce85e7"
relacionado:
  - "PBI-FIX-KALMA2-BRANCH-AND-STATUS"
  - "SddIA/engine/execute-process/src/engine/workspace_init.rs"
  - "SddIA/engine/execute-process/src/engine/executor.rs"
  - "SddIA/engine/execute-process/src/engine/progress_trace.rs"
  - "SddIA/engine/execute-process/src/engine/handlers/sync_client_assets.rs"
  - "SddIA/interfaces/kalma2-bridge/src/main.rs"
document_id: PBI-FIX-KALMA2-BRANCH-AND-STATUS
---

# PBI-FIX-KALMA2-BRANCH-AND-STATUS — rama antes de mutar y aviso de avance

BR-CA1–CA5: el checkout de la rama de trabajo es el primer paso git; si falla, el ciclo termina sin mutar la base. Inicialización `failed` deja Diseño, Ejecución y Verificación en `skipped`. `GET /api/status` nombra el fin de fase `executed` o `blocked` y no restaura la espera de un PEC `awaiting_agents` posterior. El mensaje lleva el veredicto corto del agente.