---
document_id: PBI-NORMA-PARIDAD-DOCUMENTAL
title: "[ARQUITECTURA] Norma de Paridad Documental (DIA) y Auditoría Kaizen"
format: markdown
version: "1.0.0"
created: "2026-05-22"
status: "abierto"
priority: media
feature_ref: docs/features/norma-paridad-documental
related:
  - SddIA/process/pull-request-review.md
  - SddIA/templates/spec-template.md
  - docs/todos/pending/[OPERATIVO] Backlog pendiente post-PR11 — Hito 3, Ola C y laboratorio.md
---

# [ARQUITECTURA] Norma de Paridad Documental (DIA) y Auditoría Kaizen

**Estatus:** Pendiente / Teórico (en espera de laudo empírico)  
**Jurisdicción:** Yunque Operativo (Tormentosa)
---

## 1. Declaración de Propósito de Dominio

La evolución del código SddIA sin su correspondiente reflejo en los artefactos de conocimiento (`README.md`, manuales operativos) genera un pasivo técnico crítico denominado **"Fuga de Conocimiento Entrópica"**. 

Este PBI define la **Ley de Paridad de Artefactos**, cuyo objetivo es erradicar el desacople entre la "Intención Ejecutada" (Código) y la "Intención Declarada" (Documentación). Se establece un mecanismo de **Censura y Alerta Automatizada** integrado directamente en la Aduana de Fricción, que no bloquea la productividad, pero garantiza que ninguna mutación estructural pase desapercibida para el Cúmulo de conocimiento.

---

## 2. La Norma: Declaración de Impacto de Artefactos (DIA)

Todo nuevo requerimiento o feature deberá declarar de forma consciente su impacto en el ecosistema documental antes de ser ejecutado.

1. **Modificación de Plantillas:** El archivo `SddIA/templates/spec-template.md` (y relacionados) deberá incorporar un bloque obligatorio:
   - Metadato frontmatter: `impacts_doc: true | false`
   - Sección Markdown: `### Impacto en Documentación` (donde se enumeran explícitamente los archivos manuales/README que requieren actualización por este cambio).

---

## 3. Especificación Técnica del Backlog Atómico (TODO)

| Hito | Objetivo Técnico | Criterio de Validación Estricta (Filtro A) |
| :---: | :--- | :--- |
| **H1** | **Actualización de Plantillas Base** | Modificar los archivos `spec.json` y `spec.md` dentro de `SddIA/templates/` para inyectar el bloque DIA (`impacts_doc`). |
| **H2** | **Inyección de Auditoría en la Aduana** | En `SddIA/process/pull-request-review.md`, añadir en la **Fase 1 (Triaje Técnico)** una regla para Argos: "Cruzar el diff del PR contra el spec.md". |
| **H3** | **Script Analizador (Cross-Audit)** | Crear un script ligero `audit-doc-parity.py` invocado por la Aduana. Si el PR toca `SddIA/core/` o rutas base y `impacts_doc` es false/inexistente, el script no devuelve error fatal (exit 1), sino una bandera de alerta. |
| **H4** | **Generación de Alerta Kaizen (Cúmulo)** | Si la bandera de alerta se levanta, la Aduana instruirá al agente **Cúmulo** para que persista automáticamente un archivo en `docs/todos/PENDING_AUDIT_DOC_[hash].md` alertando de la posible fuga de conocimiento. |

---

## 4. Matriz de Contención de Riesgos Operativos

| Vector de Riesgo | Impacto Estructural | Contramedida Rúnica (Filtro B) |
| :--- | :--- | :--- |
| **Falsos Positivos de Argos** | Bloqueo de PRs válidos por modificaciones menores en el código que no requieren cambio documental. | **Regla de Fricción Suave:** La auditoría documental **nunca** abortará el merge (no emite un Status Code 1). Su único efecto es generar deuda técnica explícita (TODO) para revisión humana posterior. |
| **Omisión Humana/IA** | Tekton olvida rellenar la sección DIA en el `spec.md`. | **Detección Diferencial:** Argos detectará el cambio de código; al ver que la sección DIA está vacía o en `false`, disparará la alerta Kaizen automáticamente. El sistema asume que la IA miente por omisión. |

---

## 5. Protocolo de Validación Empírica

1. Alterar un archivo core simulando una feature, dejando `impacts_doc: false` en su especificación.
2. Presentar el PR en el entorno local (Aduana).
3. Confirmar que la Aduana aprueba el PR pero **Cúmulo** deposita físicamente el archivo de advertencia en `docs/todos/`.