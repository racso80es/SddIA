---







feature_name: man-55-prompt-invariants-oracle-ola0
created: "2026-11-22"
process: feature
purpose: Estabilización de decisiones D-P1..D-P6 para invariantes de prompts y ratchet de allowlists
branch_name: feat/man-55-prompt-invariants-oracle-ola0
persist_ref: docs/features/man-55-prompt-invariants-oracle-ola0
tracker_ref: MAN-55
execution_id: "cfd0a522-c6ca-401b-a817-679f9389c6b0"
---
# Clarificación — man-55-prompt-invariants-oracle-ola0

## Origen y fricción (`KZ-PRM-01..06` + ratchet `KZ-EST-01`)

- Fuente empírica: `docs/audits/informe_de_diagnostico_desacople_eventos_sddia.md` (§4.2, §5.1, §5.2) e ítem refinado `MAN-55` (`PBI-344ba8be`).
- Patrón de deriva observado en `MAN-32` → `MAN-43` → `MAN-49` → `MAN-50`:
  1. ACs sin oráculo mecánico en rojo en Ola 0 (`KZ-PRM-01`).
  2. Evasión léxica (`concat!`, cambio de prefijo manteniendo `std::fs`, crecimiento de allowlists) para silenciar gates (`KZ-PRM-02`, `KZ-EST-01`).
  3. Olas iniciadas con árbol/entorno contaminados (`SDDIA_*`, directorios efímeros en raíz) y ediciones manuales de genoma tras handoff `idempotent` (`KZ-PRM-03`, `KZ-PRM-04`).
  4. Cierres de diffs Rust sin recompilación `--release` ni reinicio de servicios `sddia-*@*.service`, y mezcla de roles Tekton/Argos o reencuadre de alcance sin `refine-item` (`KZ-PRM-05`, `KZ-PRM-06`).

## Decisiones estabilizadas (`D-P1` .. `D-P6`)

- **D-P1:** Alcance acotado a invariantes de prompts/reglas/plantillas de handoff + oráculo ratchet de allowlists; no reabre producto `MAN-43` ni los residuos estructurales de `MAN-51..54`.
- **D-P2:** `KZ-EST-01` se particiona: descubrimiento dinámico de crates y residuos `.events` corresponden a `MAN-51`; el ratchet de cardinalidad de allowlists + cláusula anti-evasión corresponden a `MAN-55`.
- **D-P3:** El blindaje técnico de `entity-manager` (`MAN-54`, ya mergeado en `main`) es la base; `MAN-55` exige en reglas y prompts el protocolo que consume ese blindaje (`handoff_hash_signature_new != handoff_hash_signature_old`).
- **D-P4:** «Oráculo en rojo en Ola 0» (`oracle-red.log`) es precondición de autorización a Ola 1, no checklist opcional de cierre.
- **D-P5:** El ratchet (`gate-allowlist-ratchet`) mide cardinalidad de `VIRTUAL_PATH_ALLOW`, `SCAN_EXCLUDE`, `INTEGRATION_SRC` y `allowlists/*.toml` respecto a `origin/main` y falla si crece sin laudo explícito (`RATCHET_LAUDO`) en `clarify.md` del diff activo.
- **D-P6:** La paridad build/host aplica cuando el diff toca código Rust de crates que producen o alimentan binarios residentes `sddia-*`; no exige reinicio si el diff es solo documentación/normativa sin binarios.
