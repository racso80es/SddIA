---
document_id: PBI-NORMA-PARIDAD-DOCUMENTAL
title: "[ARQUITECTURA] Norma de Paridad Documental (DIA) y Auditoría Kaizen"
format: markdown
version: "1.1.0"
created: "2026-05-22"
closed: "2026-05-25"
status: cerrado
priority: media
feature_ref: docs/features/norma-paridad-documental
related:
  - SddIA/process/pull-request-review.md
  - SddIA/templates/spec-template/spec.md
  - docs/features/norma-paridad-documental/validacion.md
---

# [ARQUITECTURA] Norma de Paridad Documental (DIA) y Auditoría Kaizen

**Estatus:** Cerrado — entregado en `feat/norma-paridad-documental`  
**Jurisdicción:** Yunque Operativo (Tormentosa)

---

## 1. Declaración de Propósito de Dominio

La evolución del código SddIA sin su correspondiente reflejo en los artefactos de conocimiento (`README.md`, manuales operativos) genera un pasivo técnico crítico denominado **"Fuga de Conocimiento Entrópica"**. 

Este PBI define la **Ley de Paridad de Artefactos**, cuyo objetivo es erradicar el desacople entre la "Intención Ejecutada" (Código) y la "Intención Declarada" (Documentación). Se establece un mecanismo de **Censura y Alerta Automatizada** integrado directamente en la Aduana de Fricción, que no bloquea la productividad, pero garantiza que ninguna mutación estructural pase desapercibida para el Cúmulo de conocimiento.

---

## 2. La Norma: Declaración de Impacto de Artefactos (DIA)

Todo nuevo requerimiento o feature deberá declarar de forma consciente su impacto en el ecosistema documental antes de ser ejecutado.

1. **Modificación de Plantillas:** `SddIA/templates/spec-template/` incorpora bloque DIA:
   - Metadato frontmatter: `impacts_doc: true | false`
   - Sección Markdown: `### Impacto en Documentación`

---

## 3. Backlog Atómico — Estado de entrega

| Hito | Objetivo Técnico | Estado |
| :---: | :--- | :---: |
| **H1** | Plantillas Base (`spec.json` + `spec.md` DIA) | ✅ |
| **H2** | Reglas DIA en `pull-request-review.md` v2.1.0 | ✅ |
| **H3** | `audit-doc-parity.py` (alerta no fatal) | ✅ |
| **H4** | Kaizen `PENDING_AUDIT_DOC_[hash].md` vía Cosecha | ✅ |

---

## 4. Validación empírica

| Paso | Resultado |
|------|-----------|
| Diff monitorizado + `impacts_doc: false` | ✅ `alert_required: true`, exit 0 |
| Aduana lab | ✅ `verdict: aprobado`, TODO Kaizen generado |
| Sensor sin acoplamiento Cúmulo | ✅ ceguera espacial EDA |

Evidencia: `docs/features/norma-paridad-documental/validacion.md`

---

## 5. Deuda Kaizen posterior

- Suscripción bus `Kaizen_Alert_Required` (EDA v2) — documentada en spec feature; no implementada en v1.
