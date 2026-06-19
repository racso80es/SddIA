---
feature_name: kaizen-start-sddia-ignicion
created: "2026-06-19"
process: feature
branch_name: feat/kaizen-start-sddia-ignicion
uuid: aad51b93-198b-4cf7-b6a8-195d7f988fb5
---

# Ejecución — Kaizen start-sddia ignición ecosistema

## Comandos de verificación

```bash
cd /home/racso/Proyectos/SddIA
chmod +x start-sddia.sh
./start-sddia.sh
# otra terminal:
pgrep -x event-watcher && pgrep -x event-sweeper
curl -sf -o /dev/null -w '%{http_code}\n' http://127.0.0.1:8765/
```

## Resultado esperado

```
[SddIA] Centinelas activos: 4/4
[SddIA] Kalma2 disponible en: http://127.0.0.1:8765
```

Apagado: `Ctrl+C` o `kill -TERM <pid-start-sddia>` → mensaje `Ecosistema detenido de forma segura.`

## Evidencia

Validación en caliente 2026-06-19: arranque completo y cleanup verificado en sesión Tekton.
