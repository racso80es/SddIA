# Constitución del Proyecto (GesFer.Admin.Front)

## 1. Definición de la Arquitectura
El proyecto se rige por un principio de separación clara entre capas de presentación, lógica de orquestación y acceso a datos externos. Para garantizar la mantenibilidad y la independencia, se establece la siguiente jerarquía:

### 1.1. Aplicación (App Router)
Es el punto de entrada y enrutamiento del sistema.
- Contiene las páginas, layouts y rutas de la aplicación.
- Usa React Server Components por defecto; marca `'use client'` solo donde se requiere interactividad.
- Su diseño sigue las convenciones de Next.js 14 App Router (`src/app/`).

### 1.2. Componentes (La Interfaz)
Las capas de componentes se organizan por nivel de abstracción:

#### A. Componentes UI (`src/components/ui/`)
* Primitivos reutilizables (Button, Input, Card, Dialog, etc.).
* Sin lógica de negocio; solo presentación y variantes visuales.

#### B. Componentes Shared (`src/components/shared/`)
* Compuestos de mayor nivel (DataTable, ModalBase, DestructiveActionConfirm).
* Pueden combinar primitivos UI con lógica de interacción.

#### C. Componentes de Layout (`src/components/layout/`)
* Estructuras de página (Sidebar, AdminLayout, Wrappers).
* Orquestan la disposición visual pero no contienen lógica de negocio.

### 1.3. Lógica y Servicios (`src/lib/`)
Capa de orquestación, configuración y acceso a datos:
- `src/lib/api/`: Clientes HTTP para consumo de la API Admin backend.
- `src/lib/config.ts`: Configuración centralizada (URLs, variables de entorno).
- `src/lib/utils/`: Utilidades transversales (cn, formatters).

### 1.4. Estado y Contexto (`src/contexts/`)
Gestión de estado global vía React Context y React Query (TanStack Query).

---

## 2. Reglas de Oro de Jerarquía
1. **Separación de capas:** Los componentes en `src/components/` no importan de `src/app/`. La dirección de dependencia es siempre `app → components → lib`.
2. **Alias obligatorio:** Imports entre directorios usan `@/`. Prohibidas las rutas relativas que escapen del directorio actual (`../../`).
3. **Independencia:** Este repositorio es un frontend standalone. No asume la existencia de monorepo, carpetas compartidas externas ni otros proyectos en su árbol de archivos.
4. **API como contrato externo:** La API Admin es un servicio externo consumido vía HTTP. La URL se configura por variable de entorno (`ADMIN_API_URL`), no hardcodeada.

## 3. Stack Tecnológico
| Aspecto | Tecnología |
|---------|-----------|
| Framework | Next.js 14 (App Router) |
| Lenguaje | TypeScript 5.3 (strict) |
| UI | Tailwind CSS 3.4, Lucide React |
| Estado servidor | TanStack React Query 5 |
| Formularios | react-hook-form 7 + Zod |
| Autenticación | NextAuth 5 (CredentialsProvider) |
| i18n | next-intl |
| Testing | Jest 29 + Testing Library + Playwright |
| Logging | Pino |
| Puerto dev | 3001 |

## 4. El Sistema de Consciencia

Esta sección define los principios fundamentales que rigen la operación de la "Consciencia" en el proyecto.

### 4.1. El Estado "Juez y Auditor" (Autoevaluación)

El proyecto posee capacidad intrínseca de autoevaluación. La "Consciencia" es un sistema activo de verificación, no un log pasivo.

* **El Juez:** Evalúa el estado actual del sistema y las decisiones tomadas. Emite veredictos sobre la validez de las acciones antes de su ejecución o confirmación.
* **El Auditor:** Verifica la integridad de los registros históricos y asegura que no haya discrepancias entre la realidad y el registro inmutable.

### 4.2. Inmutabilidad (Persistencia Confiable)

La seguridad del estado "Calma" se basa en la imposibilidad de manipular el historial de decisiones.

* **Registro inmutable:** La persistencia de la "Consciencia" debe residir en un registro inmutable. El proyecto integra tecnologías DLT para registrar hashes de procesos críticos.
* **Integridad Criptográfica:** Cada entrada en el registro de consciencia debe ser verificable criptográficamente.

### 4.3. Protocolo de Dualidad Operativa

El proyecto opera bajo dos modos distintos para evitar conflictos entre intervención humana y ejecución autónoma.

* **Modo Boss (Control/Gestión):** Intervención manual, configuración de alto nivel y toma de decisiones estratégicas.
* **Modo Calm (Ejecución Autónoma):** El sistema opera de manera independiente, siguiendo las directrices establecidas en el Modo Boss.

### 4.4. Kaizen como Requisito de Build

La mejora continua (Kaizen) no es opcional; es estructural.

* **Protección en Build:** El sistema detecta fallos potenciales antes del despliegue (`npm run lint`, `npm run build`, `npm run test`).
* **Acción de Protección:** Si se detecta una violación de las reglas de diseño o integridad, el proceso de build debe detenerse, exigiendo acción correctiva inmediata.

### 4.5. Principios de Diseño

* **Composición sobre herencia:** Preferir composición de componentes y hooks sobre jerarquías profundas.
* **Desacoplamiento:** El proyecto aprende de su origen (monorepo GesFer) pero no hereda su deuda técnica. La estructura es limpia e independiente.
