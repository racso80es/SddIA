---
uuid: "0f6bf2ff-a067-46fb-9175-ee97e6a5dcd8"
id: sync-client-assets
name: sync-client-assets
version: "1.0.0"
type: process
contract: process-contract v1.4.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
context:
  - ecosystem-evolution
  - knowledge-management
hash_signature: "sha256:9b4b98de9941a7d9469dfe2716086790a6f07185bb38a02b00e837a98b879b91"
inputs:
  - asset_id: UUID del activo solicitado
  - asset_family: library_codexes | library_norms
outputs:
  - synced: true si el activo local fue actualizado
  - local_version: Versión resultante en la instancia
  - hash_verified: Resultado de la validación de integridad
phases:
  - name: Manifiesto-Local
    intent: Leer versión y hash del activo presente en la instancia.
    requires_capability:
      - id: fs:persist
        contract: fs.persist
        version: ">=1.0.0"
  - name: Reclamacion
    intent: Obtener el activo vigente del repositorio maestro (origen opaco).
    delegates_to:
      - action:download-remote-asset
  - name: Aduana-Integridad
    intent: Comparar sha256(content) contra declared_hash; discordancia aborta sin escribir.
  - name: Inyeccion
    intent: Sobrescribir activo en {instancia}/.SddIA/library/ resolviendo ruta por motor.
    requires_capability:
      - id: fs:persist
        contract: fs.persist
        version: ">=1.0.0"
minteo_maximo: null
porcentaje_de_exito: null
---

# Proceso — `sync-client-assets`

Sincronización unidireccional `repositorio maestro → instancia cliente`. Simula el entorno DLT futuro (pivote G7).

## Fases

### Fase 1 · Manifiesto-Local

Lee la versión y hash del activo presente en `{instancia}/.SddIA/library/` según `asset_family`. Requiere `fs:persist` (→ `skill:filesystem-manager`).

### Fase 2 · Reclamacion

Delega a `action:download-remote-asset` con `{ asset_id, asset_family }`. Recibe `{ content, declared_hash, origin_kind }`. El proceso no nombra ni inspecciona al proveedor subyacente.

### Fase 3 · Aduana-Integridad

| Condición | Comportamiento |
|-----------|---------------|
| `sha256(content) == declared_hash` | Continuar a Inyeccion |
| `sha256(content) != declared_hash` | **Abortar**; no escribir fichero; emitir error `hash_mismatch` |

La aduana se ejecuta **antes** de cualquier escritura. No hay rollback post-escritura; la escritura directamente no ocurre.

### Fase 4 · Inyeccion

Ruta de destino resuelta por motor vía `cumulo.paths.json` + `asset_family`. Requiere `fs:persist`. Cero rutas absolutas en genoma (Ceguera Espacial).

## Restricciones de acero

- El proceso **no** nombra al proveedor concreto de `asset:fetch` en ningún punto (G7).
- Ruta de instancia resuelta por motor; nunca cableada en genoma (Ceguera Espacial).
- `correlation_id` = `execution_id` del workspace (trazabilidad completa con `POST /api/sync-assets`).
