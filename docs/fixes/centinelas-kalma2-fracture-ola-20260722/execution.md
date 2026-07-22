---
feature_name: centinelas-kalma2-fracture-ola-20260722
created: "2026-07-22"
process: bug-fix
branch: fix/centinelas-kalma2-fracture-ola-20260722
---

# Execution

## Pasos

1. `./sddia-run.sh --process bug-fix` → workspace-init (`objectives.md`, rama `fix/centinelas-kalma2-fracture-ola-20260722`).
2. Diagnóstico empírico: locks DEAD, bridge sin vault, heartbeats OK solo con ecosistema vivo.
3. Parche `start-sddia.sh` / `start-sddia.md`.
4. Smoke: `timeout 75 ./start-sddia.sh`
   - operativo ~6s
   - bóveda LLM exportada
   - heartbeats OK
   - `POST /api/chat` → tokens mock stream
   - cleanup → `locks NONE`
5. Cascada documental + archivo PBI + evolution.

## Notas

- Agent-runtime CLI falló (SSL EPROTO); fases Dedalo/Tekton/Argos ejecutadas en runtime IDE.
- Fractura pendiente previa `9f6b681d` es testigo pre-fix (repro sin vault); no regenerada por el smoke post-fix.
