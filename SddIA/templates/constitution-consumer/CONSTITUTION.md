---
id: constitution-consumer-linux
uuid: "a8e4c2b0-1d3f-4a7e-9c5b-6f8e0d2a4b1c"
version: "1.0.0"
profile: consumer
os_target: linux
---

# Constitución de instancia — perfil consumidor (Linux)

Plantilla F-09: **sin** L2 Windows/pwsh. Copiar a `{instancia}/.SddIA/constitution/` en despliegue.

## Identidad

- Producto / workspace: _(rellenar)_
- Perfil runtime: `SDDIA_RUNTIME_PROFILE=consumer`

## Directrices operativas

1. Filtro C: no forja de genoma; no `github-bridge-watcher`.
2. Sensorial: preferir systemd `@%f` (R-07); un watermark writer por instancia.
3. Secretos solo en `.SddIA/.dev/.env` (fuera de git).
4. Host Linux: shells `/bin/bash`; **prohibido** invocar `pwsh` / rutas Windows.

## Fuera de alcance de esta constitución

- Wizard UX onboarding.
- Procesos de ingeniería (`feature`, `entity-manager`, creators) en la instancia consumidor.
