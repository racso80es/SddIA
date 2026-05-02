# [SPEC-ID]: {Nombre de la Tarea o Característica}

## 1. Información General

| Campo | Detalle |
| :--- | :--- |
| **ID de Especificación** | SPEC-GAF-2026-{Correlativo} |
| **Rama Relacionada** | {tipo}/{descripcion-breve} |
| **Estado** | Draft / Validado / Implementado |
| **Responsable** | Spec Architect |
| **Token de Auditoría** | AUDITOR-PROCESS-OK |

## 2. Propósito y Contexto

### 2.1. Objetivo (Goal)
Describir de forma sucinta qué se quiere lograr y qué problema resuelve.

### 2.2. Alcance (Scope)
*   **Incluido:** Punto A, Punto B.
*   **Fuera de Alcance:** Elementos que no se tocarán en esta iteración para evitar el scope creep.

## 3. Arquitectura y Diseño Técnico

### 3.1. Componentes Afectados
Identificar qué partes del proyecto GesFer.Product.Front serán modificadas:
*   `src/app/`: Páginas, layouts o rutas.
*   `src/components/`: Componentes UI, shared o layout.
*   `src/lib/`: Clientes API, configuración, utilidades.
*   `src/contexts/`: Contextos de estado global.
*   `src/types/`: Definiciones de tipos TypeScript.

### 3.2. Modelo de Datos / Lógica
Si aplica, incluir definiciones de tipos, interfaces o cambios en contratos de API.

## 4. Requisitos de Seguridad

*   **Validación de Input:** Uso de Zod para validación de formularios y respuestas de API.
*   **Privacidad:** Identificar si se manejan datos sensibles (PII).
*   **Autorización:** Roles necesarios para acceder a la funcionalidad (NextAuth session).

## 5. Criterios de Aceptación

Para dar por cerrada esta especificación, se deben cumplir los siguientes puntos:

- [ ] `npm run build` pasa sin errores.
- [ ] `npm run lint` pasa sin errores.
- [ ] Se han pasado los tests correspondientes (`npm run test`).
- [ ] El log de auditoría en paths.auditsPath (Cúmulo) ha sido actualizado.
- [ ] El Audit Log de la rama refleja la evolución.

## 6. Notas de Implementación

Lista de puntos clave para la fase de implementación:
- Componentes: ¿Server Component o Client Component?
- Imports: usar alias `@/` para todos los imports cross-directory.
- Accesibilidad: cumplir WCAG 2.1 AA en elementos interactivos.

## 7. Trazabilidad de Auditoría

*   **Fecha de Creación:** YYYY-MM-DD hh:mm
*   **Evento:** Generación inicial mediante proceso documental (invoke-command, documentation; paths.skillCapsules, paths.skillsDefinitionPath).
*   **Referencia de Log:** paths.auditsPath (Cúmulo)
