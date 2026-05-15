---
name: "norms-contract"
version: "1.1.0"
nature: "contract"
target_entity: "tactical-norm"
description: "Contrato estructural obligatorio para toda norma atómica de conocimiento dentro del ecosistema SddIA."
---

# Contrato de Entidad: Norma Táctica (Tactical Norm)

Toda entidad de conocimiento atómico generada en SddIA debe cumplir estrictamente con este contrato de estructura para ser considerada válida (Filtro A) y poder ser indexada por los Códices.

## 1. Reglas de Cabecera (Frontmatter YAML)
El archivo DEBE comenzar con un bloque YAML válido que contenga, como mínimo, las siguientes claves:

- `uuid`: (String) Identificador único universal (v4).
- `name`: (String) Nombre técnico, descriptivo y en formato kebab-case (ej. `nextjs-hydration-client-state`).
- `version`: (String) Versión semántica (ej. `1.0.0`). Evolve con el desarrollo Kaizen.
- `nature`: (String) DEBE ser estrictamente `tactical-norm`.
- `author`: (String) Entidad creadora (ej. `Racso` o el agente automatizado).
- `scope`: (String) Ámbito de aplicación. Valores permitidos: `agnostic`, `frontend`, `backend`, `database`, `infrastructure`, `security`.
- `category`: (String) Clasificación funcional. Valores permitidos: `architecture`, `workflow`, `code-smell`, `convention`, `testing`.
- `dependencies`: (Array de Strings) Lista de UUIDs de otras normas que son prerrequisito para entender esta. Vacío `[]` si es independiente.

## 2. Reglas de Cuerpo (Markdown)
El contenido DEBE dividirse en dos bloques funcionales para la correcta lectura de la IA obrera:

1. **Directriz Core:** La explicación del patrón o norma técnica. Debe ser aséptica, directa y libre de ambigüedad.
2. **Restricciones Duras (Aduana de Fricción):** Un listado estricto de las prohibiciones o condiciones (Filtro de Acero) que el código debe superar en base a esta norma. Si el código viola estas restricciones, la norma no se da como cumplida.

## 3. Reglas del Índice de Familia (`index.md`)

En `directories.library_norms` DEBE existir un archivo **`index.md`** que actúe como catálogo maestro de todas las normas tácticas. Responsable de coherencia: **agent:cumulo** (Gobernanza de Índices).

### 3.1 Estructura obligatoria del índice

- Cabecera YAML con, como mínimo: `index_version`, `entity_family` = `library-norms`, `maintained_by_agent` = `cumulo`, `paths_ref` apuntando a `SddIA/core/cumulo.paths.json`, `directories_key` = `library_norms`.
- Sección **Catálogo de definiciones** con tabla cuyas columnas son exactamente: **Archivo fuente**, **uuid**, **name**, **version**, **scope**, **category**.

### 3.2 Exclusiones del catálogo

- **`norms-contract.md`** es el contrato de familia; **no** constituye fila del catálogo.
- Ningún otro archivo que no tenga `nature: tactical-norm` en su frontmatter puede figurar en la tabla.

### 3.3 Sincronización (invariante)

1. Cada `{name}.md` con `nature: tactical-norm` bajo la cantera debe tener **exactamente una fila** en el índice.
2. Los valores **uuid**, **name**, **version**, **scope** y **category** de la fila deben coincidir literalmente con el frontmatter YAML del archivo fuente.
3. Tras alta, baja o cambio de versión de una norma, el proceso **`norm-creator`** (Fase de Indexación) actualiza el índice en la misma transacción lógica que persiste el `.md` de la norma.
4. Entidad física sin fila en índice, o fila con metadatos discordantes = **Ruido de Sistema** (bloqueo de reconocimiento por Cúmulo).