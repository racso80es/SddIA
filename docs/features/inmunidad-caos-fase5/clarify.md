---
feature_name: inmunidad-caos-fase5
created: "2026-05-29"
purpose: Decisiones Fase 5 y herencia del gate Fase 4
---

# Clarificación — Fase 5 (Documentación y Done global)

## Precondición (gate Fase 4)

- **Fase 4** con `validacion.md` APTO (AC4.1–AC4.3): clases ECST `Suite_Execution_Requested` y `System_Immunity_Certified`, acción `emit-suite-execution-requested`, suscripciones domain, certificación post-manifiesto en `execute-suite`, witness DLT Radamanto documentado en `dlt-immunity-acta.md`.
- **Fases 1–3** en Core: tools ofensivas, procesos audit atómicos, genoma ED `Suite` + orquestador.
- **README raíz** sin mención Suite / Caos / Inmunidad (H28).
- **`touchpoints-ia.md`** ya incluye principio Inocuidad del Caos (`chaos-engineering` + `assert_workspace_bound`); **`paths-via-cumulo.md`** no lista aún claves `directories.suites` / `contracts.suites`.
- No se reabren Fases 1–4 salvo enlace roto bloqueante detectado durante auditoría F5.

## Decisiones heredadas (aplican en Fase 5)

| ID | Resolución | Uso en Fase 5 |
|----|------------|---------------|
| D0.1 | Contexto RBAC `chaos-engineering` | Documentar en README y touchpoints |
| D0.4 | DLT inmunidad vía **Radamanto** (no Cúmulo) | Narrativa certificación; enlace acta Fase 4 |
| D0.7 | `survival-manifest.md` evidencia Argos | Mencionar en flujo EDA |
| D0.8 | Fase 4 = ECST; Fase 5 = README | Alcance acotado a documentación |
| D0.9 | PBI en `pending/` hasta Fase 5 | **Única** fase con `pbi_archived: true` |
| H28 | README sin Caos | Gap principal §5.A |

## Decisiones cerradas — Fase 5

| ID | Pregunta | Resolución |
|----|----------|------------|
| **D5.1** | ¿Alcance del README? | **Solo** `README.md` raíz. Nueva sección **Ingeniería del Caos**; no duplicar Códices `SddIA/suites/index.md` ni `events/domain/index.md` |
| **D5.2** | ¿Profundidad vs features de fase? | Narrativa de entrada (1–2 pantallas); enlaces a `docs/features/inmunidad-caos-fase*` y acta `dlt-immunity-acta.md` |
| **D5.3** | ¿Axiomas en README? | Tres axiomas PBI §0: Inocuidad del Caos, Identidad Ontológica (Suite como ED), Atomicidad Diagnóstica |
| **D5.4** | ¿Flujo EDA a documentar? | Secuencia: `emit-suite-execution-requested` → `Suite_Execution_Requested` → `execute-suite` → `survival-manifest.md` → `System_Immunity_Certified` → Radamanto DLT |
| **D5.5** | ¿Ontología Suite? | Añadir fila **Suite** en tabla «Ontología de Activos» con `paths.directories.suites` y contrato `suites-contract.md` |
| **D5.6** | ¿Arsenal y nodos en README? | Mención breve tools ofensivas (3) y procesos audit (3); enlace a catálogos `tools/index.md` y `process/index.md` — sin duplicar vectores |
| **D5.7** | ¿Radamanto vs Cúmulo DLT? | Explicitar cuarto bucket `System_Immunity_Certified` en Radamanto; Cúmulo mantiene PR/ECST (ventana dual intacta) |
| **D5.8** | ¿`paths-via-cumulo.md`? | Añadir `directories.suites`, `contracts.suites` en § Claves de paths |
| **D5.9** | ¿`touchpoints-ia.md`? | Ampliar principio §3 con referencia ED Suite y `paths.directories.suites`; mantener regla Inocuidad existente |
| **D5.10** | ¿Verificación Argos? | Checklist AC5.x + coherencia README vs genoma Suite/ECST + validación enlaces relativos |
| **D5.11** | ¿Código en diff? | **Prohibido** salvo corrección de enlace roto confirmada por auditoría; scope principal doc-only (T5.1) |
| **D5.12** | ¿Mover PBI? | En fase «Cierre documental en rama», **antes** del merge: `pending/` → `done/` mismo `document_id` |

## Gaps detectados README vs `main` (inventario Mayeuta)

| Área | Gap | Acción planificada |
|------|-----|-------------------|
| README completo | Sin Suite / Caos / Inmunidad (H28) | §5.A plan — sección nueva |
| Tabla ontología | 8 familias; falta **Suite** | §5.A.2 plan |
| § Agentes Radamanto | Solo Tool_* / Self-Healing | Ampliar nota DLT con bucket Immunity (sin reescribir § agentes entero) |
| § Eventos domain | 13 clases; sin citar estímulo/certificación Caos | Mencionar en § Caos con enlace `domain/index.md` |
| `paths-via-cumulo.md` | Sin claves suites | §5.B plan |
| `touchpoints-ia.md` | Inocuidad sí; Suite no | §5.B plan |

## Jurisdicciones (Panteón — narrativa README F5)

| Actor | Rol en documentación Caos | README F5 |
|-------|---------------------------|-----------|
| **Acción `emit-suite-execution-requested`** | Emisor estímulo ECST | Flujo EDA |
| **Tekton / `execute-suite`** | Orquestador campaña | Flujo EDA |
| **Argos** | Compila `survival-manifest.md` | Evidencia pre-certificación |
| **Radamanto** | DLT `System_Immunity_Certified` | Certificación inmunidad |
| **Cúmulo** | SSOT suites; DLT PR/ECST | Sin cambio jurisdicción |
| **Cerbero** | RBAC `chaos-engineering` | Axioma Inocuidad |

## Referencias

- PBI maestro § Fase 5 (5.A–5.C, AC5.1–AC5.2)
- Gate: [`validacion.md`](../inmunidad-caos-fase4/validacion.md) Fase 4 APTO
- Acta DLT: [`dlt-immunity-acta.md`](../inmunidad-caos-fase4/dlt-immunity-acta.md)
- Análisis impacto: [`impact-analysis.md`](../inmunidad-caos-fase0/impact-analysis.md) H28
- SSOT rutas: [`cumulo.paths.json`](../../../SddIA/core/cumulo.paths.json)
