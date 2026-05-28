---
feature_name: telemetria-reactiva-eda-fase6
created: "2026-05-28"
process: feature
items:
  - id: "6.A"
    touchpoint: "README.md § Eventos"
    proposal: "Trinidad, genoma fractal, bus eda_fractal, V3+ legacy"
  - id: "6.B"
    touchpoint: "README.md § Agentes"
    proposal: "Radamanto, Argos vs Radamanto, Self-Healing"
  - id: "6.C"
    touchpoint: "README.md § Orquestación"
    proposal: "workspaces, persist_ref ortogonal, filesystem-manager"
  - id: "6.D"
    touchpoint: "README.md § Aduana Universal"
    proposal: "Peaje, telemetry_receipt, compliance audit"
  - id: "6.E"
    touchpoint: "README.md tabla ontología"
    proposal: "filas Event/Process + workspacesRoot"
  - id: "6.F"
    touchpoint: "README.md enlaces + SddIA/agents/index.md"
    proposal: "coherencia SSOT; fix conteo agentes"
  - id: "6.G"
    touchpoint: "docs/todos/done/ PBI maestro"
    proposal: "Done global PBI-TELEMETRIA-REACTIVA-EDA-UNIFICADO"
---

# Implementación — Fase 6

| Paso | Archivos | Cambio |
|------|----------|--------|
| 6.A | `README.md` | § Eventos: Trinidad, genoma fractal, `eda_fractal`, pipeline V3+ legacy |
| 6.B | `README.md` | § Agentes: Radamanto, delimitación Argos, Self-Healing resumido |
| 6.C | `README.md` | § Orquestación: workspaces dinámicos, Ceguera Espacial, `filesystem-manager` |
| 6.D | `README.md` | Nueva § Aduana Universal: Peaje Termodinámico, recibos, compliance |
| 6.E | `README.md` | Tabla ontología Event/Process; `Desacoplamiento Core/instancia` + SSOT |
| 6.F | `README.md`, `SddIA/agents/index.md` | Enlaces features fase 0–5; fix integridad «siete agentes» |
| 6.G | `docs/todos/done/…PBI….md` | Archivo PBI maestro; frontmatter `status: done` |

Excepción D6.11: corrección mínima en `agents/index.md` (contradicción conteo vs fila Radamanto).

Sin mutaciones en genoma `SddIA/events/` ni código runtime (T6.2).
