---
feature_name: telemetria-reactiva-eda-fase6
created: "2026-05-28"
purpose: Decisiones Fase 6 y herencia Fases 1–5
---

# Clarificación — Fase 6

## Precondición (gate Fase 5)

- **Fase 5** cerrada con `validacion.md` APTO (PR #56 mergeado en `main`): contratos ED `telemetry_provided`, Peaje fail-soft con `telemetry_receipt`, proceso `telemetry-compliance-audit`, evento `Telemetry_Compliance_Breached`, inmunidad Fan-Out T5.6.
- **Fases 1–4** en `main`: genoma fractal, workspaces, Peaje + enrutadores, Radamanto + Self-Healing.
- **README actual** describe predominantemente pipeline V3+ monolítico (`pending/` → `route-domain-event`) y orquestación vía `persist_ref` sin workspaces ni Radamanto.
- No se reabren Fases 1–5 salvo enlace roto bloqueante detectado durante auditoría F6.

## Decisiones heredadas (aplican en Fase 6)

| ID | Resolución | Uso en Fase 6 |
|----|------------|---------------|
| D0.2 | Coexistencia V3+ + bus fractal | README debe explicitar **ambos** modelos y cuándo aplica cada uno |
| D0.6 | PBI maestro archiva al Done global | **Única** fase con `pbi_archived: true` |
| D2.4–D2.7 | Workspaces dinámicos | Sustituir sesgo feature/fix en § Orquestación |
| D3.13 | Fail-soft Peaje | Documentar tolerancia recibo omitido |
| D5.11 | Sin suscripción post-breach | Mencionar `Telemetry_Compliance_Breached` sin prometer gobernanza reactiva |
| Axioma §0.3 | Simetría fractal genoma/runtime | Tabla rutas alineada a `eda_fractal` en `cumulo.paths.json` |

## Decisiones cerradas — Fase 6

| ID | Pregunta | Resolución |
|----|----------|------------|
| D6.1 | ¿Alcance del README? | **Solo** `README.md` raíz del repositorio. Prohibido `README.md` en subcarpetas genoma events (Códice = `index.md`) |
| D6.2 | ¿Profundidad vs features de fase? | Narrativa de entrada (1–2 pantallas por tema); enlaces a `docs/features/telemetria-reactiva-eda-fase*` y Códices `index.md` para detalle |
| D6.3 | ¿Tratamiento pipeline V3+? | **Conservar** diagrama y tabla existentes bajo subsección «Pipeline dominio legacy (V3+)»; no eliminar — marcar coexistencia con bus fractal |
| D6.4 | ¿Nombre «Peaje» en README? | **Peaje Termodinámico (CLI)** para interceptación física; **Cerbero** permanece «Peaje RBAC» — dos peajes distintos, sin ambigüedad |
| D6.5 | ¿Radamanto en tabla agentes? | Fila nueva con rol actuario; enlace a `SddIA/agents/radamanto.md` |
| D6.6 | ¿Self-Healing en README? | Diagrama secuencia simplificado o lista numerada; eventos `Tool_Degraded` / `Status_Restored` / `Tool_Deprecated` mencionados |
| D6.7 | ¿Fan-Out telemetría? | Mencionar brevemente: múltiples suscriptores (`radamanto-batch`, `telemetry-compliance-audit`); purga solo infraestructura (T5.6) |
| D6.8 | ¿Ontología Event? | Fila actualizada: genoma fractal, `event_family`, instancia en bus fractal o V3+ según familia |
| D6.9 | ¿Ontología Process? | Añadir `workspace_template` obligatorio (process-contract v1.4.0) |
| D6.10 | ¿Verificación Argos? | Checklist manual AC6.x + grep coherencia + validación enlaces relativos en diff README |
| D6.11 | ¿Código en diff? | **Prohibido** salvo corrección de enlace roto confirmado por auditoría; scope principal doc-only (T6.1) |
| D6.12 | ¿Mover PBI? | En fase «Cierre documental en rama», **antes** del merge: `pending/` → `done/` mismo `document_id` |

## Gaps detectados README vs `main` (inventario Mayeuta)

| Sección README actual | Gap | Acción planificada |
|-----------------------|-----|-------------------|
| § Eventos — solo V3+ | Falta bus fractal `./.events/{telemetry,orchestration,domain}/` | §6.A plan |
| § Eventos — `SddIA/events/index.md` | Genoma ahora usa Códices por familia | Corregir referencia |
| § Agentes — 6 filas | Falta Radamanto | §6.B plan |
| § Orquestación — `persist_ref` | No menciona workspaces ni `filesystem-manager` | §6.C plan |
| (ausente) | Sin Aduana Universal / Peaje | §6.D plan — sección nueva |
| Tabla ontología Event/Process | Desactualizada | §6.E plan |
| Diagrama mermaid V3+ único | Puede inducir a error si no se contextualiza | §6.F — coexistencia explícita |

## Jurisdicciones (Panteón — narrativa README)

| Actor | Rol en documentación pública | README F6 |
|-------|------------------------------|-----------|
| **CLI (Peaje)** | Interceptación física; emisor único telemetría | § Aduana Universal |
| **Argos** | Inspector materia/código/artefactos | Mantener; contrastar con Radamanto |
| **Radamanto** | Actuario confianza; batch telemetría; DLT Tool_* | Nueva fila agentes |
| **Cerbero** | RBAC reactivo Self-Healing | Mencionar en bucle |
| **Cúmulo** | SSOT + DLT dominio legacy PR/ECST | Coexistencia D0.1 |
| **telemetry-compliance-audit** | Juez contratos tokens | Mencionar en § Aduana (no confundir con Argos) |

## Referencias

- PBI maestro § Fase 6 (6.A–6.F, AC6.1–AC6.5)
- Gate: [`validacion.md`](../telemetria-reactiva-eda-fase5/validacion.md) Fase 5 APTO
- SSOT rutas: [`cumulo.paths.json`](../../../SddIA/core/cumulo.paths.json) v1.4.0
- Códices familia: [`telemetry/index.md`](../../../SddIA/events/telemetry/index.md), [`orchestration/index.md`](../../../SddIA/events/orchestration/index.md), [`domain/index.md`](../../../SddIA/events/domain/index.md)
