---
document_id: reproducir-skills-en-otros-entornos-sddia
document_type: guia_portabilidad
norm_ref: SddIA/norms/paths-via-cumulo.md
process_ref: paths.processPath/create-skill/
skills_contract_ref: SddIA/skills/skills-contract.md
capsule_io_ref: SddIA/norms/capsule-json-io.md
evolution_ref: SddIA/norms/sddia-evolution-sync.md
cumulo_ref: SddIA/agents/cumulo.json
paths_contract_ref: SddIA/agents/cumulo.paths.json
idioma: es-ES
version: "1.0.0"
descripcion: >-
  Guía para portar una skill concreta (definición, cápsula, índice, Cúmulo, Rust opcional)
  a otro repositorio SddIA.
---

# Reproducir skills concretas en otros entornos SddIA

Esta guía complementa **`reproducir-create-skill-en-otros-entornos-sddia.md`**, que cubre el **proceso de tarea** `create-skill`. Aquí se detalla el portado de **una skill ya definida**: artefactos bajo **paths.skillsDefinitionPath**, **paths.skillCapsules**, **paths.skillsIndexPath** y entradas en el mapa **skillCapsules** del contrato de paths.

## 1. Prerrequisitos

| Requisito | Uso |
|-----------|-----|
| **Cúmulo** | Claves `paths.skillsDefinitionPath`, `paths.skillCapsules`, `paths.skillsIndexPath`, `paths.skillsRustPath`, `paths.skillsPath`. |
| **Contrato** | `paths.skillsDefinitionPath/skills-contract.md` y `skills-contract.json`. |
| **Nueva skill en destino** | Preferible seguir `paths.processPath/create-skill/` para dejar trazabilidad; esta guía asume que ya existe un **skill-id** en kebab-case. |

## 2. Checklist: definición (`paths.skillsDefinitionPath/<skill-id>/`)

1. Copiar o crear carpeta **`<skill-id>/`** con **spec.md** y **spec.json** (frontmatter + paridad MD ↔ JSON según entidades de dominio).
2. En **spec.json**, si hay ejecutable: **implementation_path_ref** apuntando a `paths.skillCapsules.<skill-id>` (no rutas literales en el cuerpo normativo).

## 3. Checklist: implementación (cápsula)

1. Carpeta **`paths.skillCapsules[<skill-id>]`** en el destino: manifest, launchers (`.bat`/`.ps1`), documentación `.md`, y **`&lt;nombre&gt;.exe` en la raíz de la cápsula** cuando aplique Rust (ver `skills-contract.md`; `bin/` solo como legado).
2. Si el binario usa JSON por stdin/stdout: alinear a `SddIA/norms/capsule-json-io.md` y documentar el `request`/`result` en la definición.

## 4. Checklist: índice y Cúmulo

1. **paths.skillsIndexPath:** añadir o fusionar entrada del skill (convención del índice del destino).
2. **cumulo.paths.json** → **paths.skillCapsules:** clave **`<skill-id>`** con ruta relativa a la cápsula.
3. Código fuente Rust (si aplica): copiar o enlazar módulo bajo **paths.skillsRustPath** y ajustar build del destino.

*Nota (este repositorio):* el envelope **JSON v2.0** para skills nuevas está en el crate **gesfer-skills** (`src/capsule_json.rs`, `src/git_cmd.rs`), no en un crate externo `gesfer-capsule`. Otros clones pueden portar esos módulos o sustituirlos por una librería compartida equivalente.

## 5. Visibilidad opcional

- **SddIA/skills/README.md** o índice local de definiciones.
- Disparador **#Skill** / `interaction-triggers.md` si el destino lista skills descubribles al usuario.

## 6. Evolution SddIA

Si el portado altera `./SddIA/`, registrar según `SddIA/norms/sddia-evolution-sync.md` (detalle UUID + índice).

## 7. Verificación mínima

1. Existe `paths.skillsDefinitionPath/<skill-id>/spec.md` (y `spec.json` si el destino lo exige).  
2. Resuelve `paths.skillCapsules.<skill-id>` y el launcher invoca el `.exe` o el fallback `.ps1` esperado.  
3. **paths.skillsIndexPath** y Cúmulo coherentes (sin skill-id huérfano).  
4. Búsqueda global: referencias `paths.skillCapsules.<skill-id>` sin carpeta destino.

## 8. Referencias cruzadas

- Proceso **create-skill:** `paths.processPath/create-skill/spec.md`  
- Portar solo el proceso (no skills): `SddIA/skills/reproducir-create-skill-en-otros-entornos-sddia.md`  
- Rutas: `SddIA/norms/paths-via-cumulo.md`

---

*Guía de portabilidad de skills. Mantener alineada con skills-contract y Cúmulo al evolucionar.*
