## Summary

- Corrige `start-sddia.sh` v1.2: carga bóveda antes de Kalma2, gate de heartbeats obligatorios y cleanup que retira locks.
- Absorbe 5 PBIs de fractura (4 centinelas heartbeat + kalma2 prótesis SSE).
- Validación empírica: ignición operativa, audit fresco, chat SSE con vault, locks NONE tras apagado.

## Test plan

- [x] `timeout 75 ./start-sddia.sh` → banner S+ Grade, 2/2+opcionales, Kalma2 HTTP
- [x] `heartbeat-audit.json` missed_cycles=0 post-gate
- [x] `POST /api/chat` streama tokens con CLI de bóveda
- [x] Post-SIGTERM: cero locks en `.SddIA/daemons/status/`
