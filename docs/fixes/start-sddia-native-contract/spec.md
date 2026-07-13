---
feature_name: start-sddia-native-contract
created: "2026-07-13"
process: bug-fix
base: main
scope: start-sddia-native-contract
---

# Especificación — contrato nativo de `start-sddia`

## Causa raíz

Los launchers de centinelas preferían `target/release`. El `event-watcher` release local, compilado antes de la poda de Python, aún intentaba invocar `execute-process.py`; el binario debug actual no contiene esa referencia. El uso documentado ejecuta `cargo build`, que actualiza el perfil debug.

## Cambios requeridos

1. Priorizar `target/debug` y usar `release` solo como fallback en launchers, bridge y resolución del orquestador.
2. Exigir ELF ejecutable para los binarios/overrides y mostrar el path nativo resuelto al arrancar.
3. Rechazar el arranque al fallar cualquiera de los centinelas obligatorios, sin que los opcionales compensen el fallo.
4. Corregir el uso de `start-sddia.sh` para ejecutarlo desde la raíz.

## Criterios de aceptación

| ID | Criterio |
|---|---|
| CA1 | Los cuatro centinelas resuelven `debug` antes de `release`. |
| CA2 | El fallo de `event-watcher` o `event-sweeper` termina la ignición con código 1. |
| CA3 | Un override script/no ELF para `execute-process` o Kalma2 se rechaza explícitamente. |
| CA4 | Kalma2 y el orquestador verifican un ELF nativo antes de ejecutar. |
| CA5 | Smoke de ignición informa paths debug y no imprime `execute-process.py`. |
