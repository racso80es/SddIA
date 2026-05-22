# Personalización Vía C — Eventos de instancia (`.SddIA/events/`)

Este directorio **no** es cola del bus EDA. Almacena overrides locales de suscripción y configuración de instancia, resueltos vía `cumulo.paths.json` → `eda_instance.customization`.

## Topología (tres planos)

| Plano | Ruta SSOT | Naturaleza |
|-------|-----------|------------|
| **Clase (genoma)** | `SddIA/events/{name}.md` | Contrato ECST versionado en Git |
| **Instancia (runtime)** | `.events/{pending,processing,processed,dead-letter}/` + `{estado}/subscribers/` (testigos) | JSON volátil del bus V3+ |
| **Personalización (Vía C)** | `.SddIA/events/` | Overrides locales no versionados |

## Overrides de suscripción

Copie `event-subscriptions.local.json` en este directorio para fusionar suscriptores adicionales o desactivar fan-out en laboratorio. El watcher resuelve primero el SSOT (`SddIA/core/event-subscriptions.json`); la fusión con overrides locales queda documentada como deuda si no está cableada.

Ejemplo mínimo (`event-subscriptions.local.json`):

```json
{
  "PullRequest_Presented": [],
  "Domain_Entity_Created": [
    {
      "agent": "cumulo",
      "action": "sync-entity-index",
      "intent": "Reconciliación local idempotente."
    }
  ]
}
```

## `local.paths.json` (opcional)

Redirige rutas del bus en entornos de desarrollo sin mutar el genoma:

```json
{
  "event_bus": "./.events",
  "eda_bus": {
    "pending": "./.events/pending",
    "processing": "./.events/processing",
    "processed": "./.events/processed",
    "dead_letter": "./.events/dead-letter"
  }
}
```

## Instalación

1. Crear `.SddIA/events/` en la raíz del workspace (gitignored).
2. Copiar esta plantilla desde `SddIA/templates/eda-instance-events/README.md`.
3. Añadir archivos locales según necesidad; **no** commitear `.SddIA/`.

## Referencias

- `SddIA/events/events-contract.md` — contrato ECST y aseguramiento forense
- `SddIA/core/cumulo.paths.json` — `event_bus`, `eda_bus`, `eda_instance.customization`
- `SddIA/actions/route-domain-event.md` — validación instancia ↔ Clase (Paso 2b)
- `SddIA/scripts/daemons/event-sweeper.py` — recolector post-consenso
