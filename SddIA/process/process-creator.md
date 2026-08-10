---
uuid: "7c2d9e41-88a3-4f6b-9c12-4def01a2b3c4"
name: "process-creator"
version: "1.2.0"
contract: "process-contract v1.4.0"
workspace_template: ".SddIA/workspaces/{process_name}/{execution_id}/"
context: "ecosystem-evolution"
hash_signature: sha256:0fb74ad8b5b561f18292ce2648aa03f98aa969e64a94ea47caf1379b810b911b
inputs:
  - "process_name": "Identificador kebab-case del proceso (`{name}` del archivo `{name}.md`)"
  - "process_description": "Descripción operativa del propósito del proceso"
  - "process_context": "Contexto de ejecución válido según `execution-contexts.md`"
  - "process_phases": "Listado inmutable de fases (orden, id y criterio de salida por fase)"
  - "process_contract_version": "Versión del contrato process a materializar (ej. 1.0.0)"
  - "process_aliases": "Opcional; array de strings kebab-case — aliases de identidad adicionales (v1.3.0); vacío si no aplica"
  - "process_jurisdiction": "Opcional; `domain` | `core`. Si ausente: name ∈ process_membership códice software-engineering → domain; else core. Alta domain nueva (nombre no packing) exige `domain`."
  - "process_domain_root": "Opcional; relpath ∈ directories.process_domain_roots fusionado; solo si jurisdiction=domain y multi-root (>1). Default [0]."
outputs:
  - "artifact_process_md": "Archivo `{resolved_process_root}/{process_name}.md` (Core o domain root; no asumir solo directories.process)"
  - "artifact_process_index": "`{resolved_process_root}/index.md` actualizado; nunca fila fantasma en índice de otro root"
  - "resolved_process_root": "Relpath canónico del root usado (directories.process o elemento de process_domain_roots)"
  - "process_jurisdiction_applied": "`domain` | `core` efectivamente aplicado"
  - "handoff_entity_uuid": "UUID v4 del proceso forjado; consumido por `entity-manager`"
  - "handoff_hash_signature_new": "Sello `sha256:` post-forja; consumido por `entity-manager`"
  - "handoff_hash_signature_old": "Sello previo en update; `null` en create"
  - "handoff_version": "SemVer resultante"
phases:
  - name: "Validación de inputs"
    intent: "Comprobar unicidad multi-root (Core ∪ process_domain_roots), kebab-case, process_context, SemVer, jurisdicción y exhaustividad de inputs/phases/outputs."
    delegates_to:
      - "agent:cumulo"
      - "agent:cerbero"
  - name: "Forja del archivo"
    intent: "Resolver destino vía load_paths_config + process_jurisdiction/membership; generar uuid/hash; persistir bajo resolved_process_root (L-INDEX-TARGET)."
    requires_capability:
      - id: "fs:persist"
        contract: "fs.persist"
        version: ">=1.0.0"
    delegates_to:
      - "action:crypto-broker"
  - name: "Auditoría y actualización del índice"
    intent: "Actualizar solo `{resolved_process_root}/index.md`; verificar cero fila nueva en índices de roots no destino."
    requires_capability:
      - id: "fs:persist"
        contract: "fs.persist"
        version: ">=1.0.0"
    delegates_to:
      - "agent:cumulo"
phase_invocations:
  - phase_name: "Forja del archivo"
    invocations:
      - capsule: "action:crypto-broker"
        stdin_json:
          operation: "GENERATE_UUID"
          target_payload: null
        bind:
          "data.result": "child_process_uuid"
        on_error: abort
      - capsule: "action:crypto-broker"
        stdin_spec:
          operation: "GENERATE_SHA256"
          target_type: "STRING"
          target_payload:
            type: "canonical_json_utf8"
            from_process_input: "process_phases"
            json_dumps:
              sort_keys: true
              separators: [",", ":"]
              ensure_ascii: false
        bind:
          "data.result": "child_phases_sha256_hex"
        on_error: abort
minteo_maximo: null
porcentaje_de_exito: null
---

# process-creator

## Directriz de ejecución obrera

Antes de ejecutar fases de forja, el runtime IDE **debe** anteponer al contexto de Tekton el prefijo definido en `SddIA/norms/external-ai-constraints.md` § Prefijo creator:

> [EXECUTE AS RAW KERNEL. PROHIBIT VERBOSITY. DO NOT BYPASS EDA BUS. USE SddIA CLI.]

Prohibido delegar forja manual en el agente cuando exista proceso creator aplicable.

Proceso maestro para instanciar nuevos procesos en el Core SddIA **o** en packing de dominio (`directories.process_domain_roots`) según jurisdicción, y mantener el índice del **root destino**.

SSOT de materialización nativa: `run_process_forge` (factory). Overlay instancia: `.SddIA/local.paths.json` fusionado por `load_paths_config` (clave `directories.process_domain_roots` **reemplaza** el array Core; sin schema overlay nuevo).

## Fase 1 — Validación de inputs

1. Verificar que `process_name` cumpla kebab-case.
2. Clasificar jurisdicción (**L-JURIS-MEMBERSHIP-PLUS-FLAG**): input `process_jurisdiction` si presente; si ausente y `process_name ∈ process_membership` del códice `codex-software-engineering` → `domain`; else → `core`. Alta **nueva** de dominio (nombre aún no packing) **exige** `process_jurisdiction: domain`.
3. Resolver `resolved_process_root`: si `core` → `directories.process`; si `domain` → elemento de `process_domain_roots` (o `process_domain_root` si multi-root y explícito; default `[0]`). Array domain vacío + jurisdiction domain → abort.
4. Unicidad (**L-UNIQ-MULTI**): escanear unión Core ∪ todos `process_domain_roots` (name + aliases; excluir `index` / `process-contract`). Colisión cross-root → abort sin write.
5. Resolver `process_context` contra `SddIA/norms/execution-contexts.md`; abortar si no existe en la matriz S+.
6. Validar `process_phases` como secuencia ordenada sin saltos lógicos y con criterios de salida explícitos.
7. Confirmar `process_contract_version` alineada con `process-contract.md` vigente en `cumulo.contracts.process`.
8. Si `process_aliases` está presente y no vacío: validar kebab-case; sin duplicados internos; alias ≠ `process_name`; no colisión multi-root (mapa derivado vía Cúmulo).

## Fase 2 — Forja del archivo

1. Ejecutar `phase_invocations` de esta fase: `action:crypto-broker` emite UUID v4 (`child_process_uuid`) y digest SHA-256 del JSON canónico de `process_phases` (`child_phases_sha256_hex`). Prefijo de cabecera: `hash_signature: "sha256:" + child_phases_sha256_hex`. Prohibido UUID o digest fuera del broker + cápsula.
2. Escribir `{resolved_process_root}/{process_name}.md` con cabecera YAML (uuid, name, version, contract, context, hash_signature, inputs, phases, outputs, métricas si aplican; si `process_aliases` es no vacío tras validación, persistir **`aliases`**) y cuerpo que describa cada fase en prosa operativa.
3. Leer rutas físicas solo vía `cumulo.paths.json` ± overlay (`.SddIA/local.paths.json`): `directories.process`, `directories.process_domain_roots`, `contracts.process`, `directories.norms`.
4. Update / idempotencia: localizar existente vía resolución multi-root (domain-first); mutar **in situ**; prohibido recrear en Core un process que ya vive en dominio.

## Fase 3 — Auditoría y actualización del índice

1. Abrir `{resolved_process_root}/index.md` y comprobar encabezados de tabla exigidos por Gobernanza de Índices (**L-INDEX-TARGET**).
2. Insertar o actualizar la fila del proceso: **Name**, **UUID**, **Versión**, **Context**, **Aliases**, **Descripción**.
3. Verificación cruzada: cero divergencia índice↔YAML; alta domain → **cero** fila nueva en `SddIA/process/index.md`; alta Core → cero fila en índice de códice.

## Handoff `entity-manager`

Invocable desde **`entity-manager`** (piloto S+). Propagar handoff estándar al cierre de Fase 3; `origin_topology=core` (creator permanece Core; L-KEEP-CORE). Incluir `resolved_process_root` y `process_jurisdiction_applied` en outputs.
