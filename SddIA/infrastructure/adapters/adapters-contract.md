---
contract_version: "1.0.0"
entity_type: "infrastructure-adapter"
jurisdiction: "Core SddIA (definición) / Host nativo (delivery)"
capabilities:
  - "infrastructure-adapter-schema-governance"
  - "spatial-blindness-enforcement"
---

# Contrato de Adaptadores de Infraestructura (S+ Grade)

Este documento rige los **adaptadores de infraestructura**: crates host nativos bajo `directories.infrastructure_adapters` que materializan puertos del hexágono (memoria, persistencia, etc.) sin contaminar `SddIA/core/`.

**Principio rector:** el Core dicta identidad documental y señal de `status`; el delivery (Rust crate, layout, driver) es local al adaptador. La resolución física pasa por Cúmulo (`cumulo.paths.json`).

## 1. Identidad atómica (innegociable)

Cada adaptador debe tener un archivo `{name}.md` en kebab-case bajo `directories.infrastructure_adapters` con cabecera YAML obligatoria:

| Campo | Obligatorio | Descripción |
|-------|:-----------:|-------------|
| `id` | Sí | Identificador kebab-case (nombre lógico; coincide con `name`) |
| `uuid` | Sí | UUID v4 inmutable |
| `type` | Sí | Valor fijo: `infrastructure-adapter` (etiqueta de familia local; no taxonomía Constitución) |
| `version` | Sí | SemVer de la ficha |
| `status` | Sí | `placeholder` \| `active` \| `deprecated` — señal para observabilidad (Espejo, Cúmulo) |
| `crate_name` | Sí | Nombre del paquete Cargo |
| `impl_dir` | Sí | Directorio del crate, relativo a `directories.infrastructure_adapters` |
| `contract` | Sí | `adapters-contract v{contract_version}` |

Prohibido `spec.json` u otro formato paralelo. Entidades sin ficha conforme se marcan **Entropía/Código Fósil**.

## 2. Censo (`index.md`)

`SddIA/infrastructure/adapters/index.md` es el inventario gobernado por Cúmulo. Cada fila del catálogo debe:

1. Referenciar un `{name}.md` existente.
2. Coincidir en `uuid`, `name`, `version`, `status`, `crate_name`, `impl_dir` con el frontmatter de la ficha.
3. Tener un `impl_dir` con directorio físico verificable (`stat`).

## 3. Anti-Alucinación espacial

- Prohibido listar adaptadores en consumidores (Espejo, bridge) sin leer primero `directories.infrastructure_adapters` y `index.md`.
- Prohibido inferir `status` desde telemetría de ejecución cuando la ficha declara `placeholder` (un stub puede devolver `Ok` vacío).
- Cúmulo es el único agente autorizado a proponer cambios en `cumulo.paths.json` para rutas de infraestructura.

## 4. Relación con otras entidades

| Entidad | Distinción |
|---------|------------|
| `tool` | Capacidad invocable vía cápsula WASI/CLI; catalogada en `directories.tools` |
| `skill` | Cápsula de dominio; `directories.skills` |
| `infrastructure-adapter` | Driver host del hexágono; **no** es cápsula ni entidad `entity-manager` en v1 |

IOTA (`tool:iota-immutable-publisher`) permanece en el catálogo de tools; no se duplica como adaptador de infra.

## 5. Referencias

- SSOT rutas: `SddIA/core/cumulo.paths.json`
- Agente bibliotecario: `SddIA/agents/cumulo.md`
- Deuda origen: `PBI-ARCH-INFRA-ADAPTERS-SSOT-001` (Espejo DD-7)
