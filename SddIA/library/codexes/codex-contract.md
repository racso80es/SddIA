---
name: "codex-contract"
version: "1.1.0"
nature: "contract"
target_entity: "domain-codex"
description: "Contrato estructural obligatorio para el orquestador de normas (Códice) en la Librería SddIA."
---

# Contrato de Entidad: Códice de Dominio (Domain Codex)

El Códice es el activo tokenizable de la Librería SddIA. Su función es empaquetar un conjunto de Normas Tácticas e inyectarles una estrategia de dominio.

## 1. Reglas de Cabecera (Frontmatter YAML)
El archivo DEBE contener este frontmatter para permitir el Enrutamiento Semántico:

- `uuid`: (String) Identificador único universal del activo.
- `name`: (String) Nombre estratégico del paquete (ej. `SddIA Codex Next.js Product S+`).
- `version`: (String) Versión del códice.
- `nature`: (String) DEBE ser estrictamente `domain-codex`.
- `author`: (String) Creador del paquete.
- `target_environment`: (Array de Strings) Entornos donde este códice tiene autoridad (ej. `["frontend", "react"]`).
- `certification_grade`: (String) Calificación de madurez (ej. `S+`, `A`, `Pendiente`).
- `composition`: (Array de Objetos) El inventario exacto de normas. Cada objeto DEBE contener:
  - `norm`: (String) El UUID de la norma táctica.
  - `path`: (String) La ruta relativa canónica hacia el archivo `.md` de la norma en la cantera.

## 2. Reglas de Cuerpo (Markdown)
El cuerpo del archivo actúa como la inyección de contexto ("Vibe") para el Agente orquestador. DEBE contener:

1. **Estrategia de Dominio:** Justificación táctica de por qué se han ensamblado estas normas juntas y cuál es el objetivo final de arquitectura.
2. **Instrucciones de Prioridad:** Directrices claras para la IA sobre cómo resolver conflictos si dos normas chocan, o qué norma tiene prioridad absoluta en la validación (Aduana de Fricción).

## 3. Reglas del Índice de Familia (`index.md`)

En `directories.library_codexes` DEBE existir un archivo **`index.md`** que actúe como catálogo maestro de todos los códices de dominio. Responsable de coherencia: **agent:cumulo** (Gobernanza de Índices).

### 3.1 Estructura obligatoria del índice

- Cabecera YAML con, como mínimo: `index_version`, `entity_family` = `library-codexes`, `maintained_by_agent` = `cumulo`, `paths_ref` apuntando a `SddIA/core/cumulo.paths.json`, `directories_key` = `library_codexes`.
- Sección **Catálogo de definiciones** con tabla cuyas columnas son exactamente: **Archivo fuente**, **uuid**, **name**, **version**, **target_environment**, **certification_grade**.

### 3.2 Exclusiones del catálogo

- **`codex-contract.md`** es el contrato de familia; **no** constituye fila del catálogo.
- Ningún otro archivo que no tenga `nature: domain-codex` en su frontmatter puede figurar en la tabla.

### 3.3 Sincronización (invariante)

1. Cada `{slug}.md` con `nature: domain-codex` bajo la cantera debe tener **exactamente una fila** en el índice.
2. Los valores **uuid**, **name**, **version**, **target_environment** y **certification_grade** de la fila deben coincidir literalmente con el frontmatter YAML del archivo fuente.
3. Tras alta, baja o cambio de versión de un códice, el proceso **`codex-creator`** (Fase de Indexación) actualiza el índice en la misma transacción lógica que persiste el `.md` del códice.
4. Entidad física sin fila en índice, o fila con metadatos discordantes = **Ruido de Sistema** (bloqueo de reconocimiento por Cúmulo).