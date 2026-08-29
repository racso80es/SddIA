---
feature_name: arch-immunological-system
created: "2026-08-29"
process: feature
branch_name: feat/arch-immunological-system
persist_ref: docs/features/arch-immunological-system
pbi_ref: docs/todos/pending/PBI-ARCH-IMMUNOLOGICAL-SYSTEM.md
document_id: PBI-ARCH-IMMUNOLOGICAL-SYSTEM
uuid: "056ac6a1-02fc-4988-a704-1f5b648d0e40"
execution_id: "987e1747-bd08-4c80-ad41-648f09cc4b12"
mayeuta_verdict: ok
laudo: suspend-skew-plus-phagocyte
---

# Objetivos — arch-immunological-system

## Misión

Dotar a `daemon-heartbeat-audit` (Argos) de **discriminación suspend/crash** y de **fagocitosis** de PBIs `PBI-FIX-FRACTURE-*` ya recuperados, de modo que un letargo de host no materialice deuda humana ni deje `paths.todos.pending` con fracturas históricas.

## Punto objetivo

> **O-INMUNO:** Tras un salto de reloj de host (Δwall − Δmono ≥ `suspend_skew_seconds`) con PID vivo, el sweep **reancla** baseline y `fractures_emitted` permanece vacío. Tras ignición sana (`missed_cycles=0`), los PBI-FIX-FRACTURE cuya traza sea anterior a `lock.started_at` quedan ledger-fagocitados y, en forja, archivados a `paths.todos.done` + ola B + evolution, sin laudo PBI-a-PBI del Vértice Biológico.

## Alcance

| Dentro | Fuera |
|--------|-------|
| Handler `daemon_heartbeat.rs`: skew wall/mono, reancla, SSOT umbrales | Nuevo evento `Anomaly_Detected` |
| Overlay instancia de umbrales | `radamanto.thresholds.json` / sondeo Radamanto |
| Ledger fagocitosis bajo `daemons_instance.state` | Mutar intervalos de centinelas |
| Proceso de archivo ola B (pending→done + evolution) | `delivery-close-cycle` automático |
| Tests unitarios del handler + predicado de poda | Watermark IMAP / familia B |
| Update documental `daemon-heartbeat-audit.md` vía `entity-manager` | Kill-switch / arranque de daemons |

## Ley aplicada

- Verificador empírico: **Argos** (`daemon-heartbeat-audit`). Radamanto: exclusión contractual (sin cronómetros ni PID).
- Ceguera espacial: rutas vía `cumulo.paths.json` (`daemons_instance.state`, `paths.todos.*`, `paths.fixPath`).
- Git vía `skill:git-manager`. Genoma `SddIA/process/` solo `entity-manager`.
- `directories.events`: no forjar clase nueva en esta feature.
- Cierre documental en rama (un PR) al **ejecutar** T0–Tn; esta sesión es stop-planning.
- Filtro C: no delegar ruido de suspend al Vértice Biológico.
