---
document_id: PBI-SNAPSHOT-FRICCION-JULES
title: "[OPERATIVO] Snapshot de Fricción: Paradoja Raw Kernel y Colapso del Laboratorio"
format: markdown
version: "1.0.0"
created: "2026-06-01"
status: done
priority: alta
process: feature
closed: "2026-06-11"
branch_name: feat/snapshot-friccion-laboratorio-jules
feature_ref: docs/features/snapshot-friccion-laboratorio-jules
origin: docs/todos/pending/Snapshot_Friccion_Laboratorio_Jules.md
---

# [OPERATIVO] Snapshot de Fricción: Paradoja Raw Kernel y Colapso del Laboratorio

**Fecha de Registro:** 2026-06-01  
**Cierre:** 2026-06-11 — feature `docs/features/snapshot-friccion-laboratorio-jules/`  
**Nodos de Impacto:** [ Sistema Operativo | Gobernanza de IA Obrera | Arquitectura de Datos ]

---

## Resolución (cierre)

| Bloque | Estado | Evidencia |
|--------|--------|-----------|
| §3.1 PyYAML | ✅ (heredado) | `requirements.txt`, `sddia-run.sh` |
| §3.1 Git failsoft | ✅ | `git-manager.py`, `execute_process_capsules.py` |
| §3.2 Raw Kernel → feature | ✅ | `external-ai-constraints.md` v1.1.0 DA-4 |
| §4 WASI | ✅ (heredado) | `wasi-poc-ignition`, `migracion-rust-wasi` |
| §5.1 Transpilador | ✅ | `SddIA/skills/intent-transpiler.md` |
| §5.2 Aduana Husky | ✅ (heredado) | PR #73 |

---

## 1. El Evento Causal (La Ignición)

Se inyectó a la IA obrera (Jules) una secuencia de proyectiles tácticos para materializar la trazabilidad EDA en el agente Argos. Para evitar la alucinación y el sesgo de verbosidad, se aplicó la restricción máxima: `[EXECUTE AS RAW KERNEL. PROHIBIT VERBOSITY. PENALIZE CONJECTURE. NO EXPLANATIONS, ONLY CODE.]` junto con directrices de modificación física directa.

## 2. La Falla Estructural (El Colapso)

*(Registro histórico — ver feature `snapshot-friccion-laboratorio-jules` para análisis y remediación.)*

## 3–5. Acciones derivadas

Materializadas en feature `docs/features/snapshot-friccion-laboratorio-jules/` y entregas heredadas citadas en tabla de resolución.
