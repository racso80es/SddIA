# `@sddia/core`

Shared Kernel npm del Core SddIA (producto F1 — Fractura Core).

## Jurisdicción

Este paquete declara la **frontera de consumo** del Core:

- Nodos de control (Cúmulo, Cerbero, Tekton, Mayeuta, Dédalo, Argos) permanecen en `directories.agents`.
- Física de bus/eventos según `cumulo.paths.json` (`eda_*`).
- Ley de E/S: `SddIA/norms/capsule-json-io.md` (schema 2.0).

El consumidor es **ciego al dominio** de cualquier instancia (incluido Paciente 0).

## Runtime

El runtime ejecutable sigue siendo el workspace Cargo (`SddIA/sddia-core`, `sddia-io`) y `execute-process`. Esta fachada npm es dependencia inerte para cáscaras JS (`apps/sddia-forge`, `apps/sddia-portal`).

## Versión

`0.1.0` — `private: true` en F1 (publish registry = feature hija).
