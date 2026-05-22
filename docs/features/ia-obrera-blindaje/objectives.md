---
feature_name: ia-obrera-blindaje
process: feature
created: "2026-05-22"
persist_ref: docs/features/ia-obrera-blindaje
branch_name: feat/ia-obrera-blindaje
related_todo: docs/todos/TODO-BLINDAJE-IA-OBRERA.md
---

# Objetivos — Blindaje Ontológico IA Obrera

## Meta

Erradicar la **Entropía Táctica** de IAs externas (Cursor, Jules, etc.) forzando obediencia al genoma SddIA: prohibición de forja manual en directorios protegidos y enrutamiento obligatorio vía `execute-process.py` / procesos canónicos.

## Objetivos medibles

| ID | Objetivo | Criterio |
|----|----------|----------|
| **O1** | **Norma motor** `external-ai-constraints.md` | Archivo en `SddIA/norms/` con Dogma de Soberanía, Prohibición de Forja Manual y Única Vía de Acción |
| **O2** | **Inyección touchpoint** | `.cursorrules` incluye bloque § referenciando la norma SSOT; `touchpoints-ia.md` actualizado |
| **O3** | **Prefijo creator (Fase B)** | Los 8 procesos `*-creator` declaran `execution_directive_prefix` canónico alineado a la norma |
| **O4** | **Correlato EDA** | `Domain_Entity_Created` para la norma vía `entity-manager` (backfill o forja en cadena) |
| **O5** | **Evolución registrada** | Entrada en `SddIA/evolution/` documentando la transmutación |
| **O6** | **Smoke Argos** | `validacion.md` confirma texto normativo, touchpoints y creators actualizados |

## Alcance por fases del TODO

| Fase TODO | Contenido | Esta feature |
|-----------|-----------|--------------|
| **A** — Manifiesto (.cursorrules) | Norma + inyección IDE | ✅ |
| **B** — Contexto en creators | Prefijo letal en `*-creator` | ✅ |
| **C** — Aduana física (Argos Gatekeeper) | pre-commit, pre-push, post-merge | ❌ Cerrada (`pbi-005-hito3-git-hooks`, Ola B) |

## No objetivos

- Reimplementar hooks Git (Fase C ✅).
- Automatizar `process:sddia-difusion` completo (deuda aparte); esta entrega sincroniza `.cursorrules` y documentación touchpoint manualmente.
- Crear `.windsurfrules` físico si el gestor no está desplegado — solo documentar en `touchpoints-ia.md`.
- Retirada de shims CLI en laboratorios `SddIA_1`…`SddIA_4`.

## Ley aplicada

- `docs/todos/TODO-BLINDAJE-IA-OBRERA.md`
- `SddIA/norms/touchpoints-ia.md` (SSOT difusión; no duplicar normas en touchpoints)
- `SddIA/norms/obediencia-procesos.md`
- Precedencia aduana: `docs/features/pbi-005-hito3-git-hooks/` (Fase C cerrada)
- Proceso `feature` v1.2.0

## Estado

| Fase feature | Estado |
|--------------|--------|
| Inicialización | ✅ rama `feat/ia-obrera-blindaje` |
| Clarificación | ✅ `clarify.md` |
| Especificación | ✅ `spec.md` |
| Planificación | ✅ `plan.md` |
| Implementación | ✅ genoma + touchpoints + creators |
| Validación | ✅ `validacion.md` — pendiente PR |
