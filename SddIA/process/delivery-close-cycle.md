---
uuid: 5417c92c-da7f-4d46-b245-55cf1b17961a
name: delivery-close-cycle
version: 1.0.0
contract: process-contract v1.3.0
context:
- ecosystem-evolution
hash_signature: sha256:804b4cc1b4168e2c902cac9fba953dbadc70a2ca17735f313c1f59f5ed926e08
inputs:
- source_process: 'Origen del flujo: feature | bug-fix | refactorization'
- persist_ref: Carpeta de tarea / referencia de persistencia acordada en el ciclo
- branch_name: Nombre de rama de trabajo bajo control de versión
outputs:
- pr_url: URL del pull request abierto o actualizado
- closed_branch: Rama cerrada o higienizada según política local
- evolution_entry: Referencia opcional a entrada en evolution/
phases:
- name: Snapshot final
  intent: Consolidar commit final del trabajo antes del cierre remoto.
  delegates_to:
  - skill:git-manager
- name: Impacto SddIA condicional
  intent: Si source_process == feature y existen mutaciones bajo SddIA/, Argos registra
    impacto y evolución del Core.
  delegates_to:
  - agent:argos
- name: Sync remoto y PR
  intent: Publicar cambios y abrir/actualizar PR según normas de git y PR.
  delegates_to:
  - skill:git-manager
- name: Higiene local
  intent: 'Cerrar ciclo local (close-cycle): limpieza de ramas temporales y estado
    de repo consistente.'
  delegates_to:
  - skill:git-manager
minteo_maximo: null
porcentaje_de_exito: null
---

# delivery-close-cycle

Proceso paramétrico de **cierre de entrega** reutilizable desde `feature`, `bug-fix` y `refactorization`. Encadena snapshot git, evaluación condicional de impacto en el Core SddIA, sincronización remota con PR e higiene local.

## Notas operativas

* La fase **Impacto SddIA condicional** debe evaluarse como no-op documentado cuando no aplique (`source_process != feature` o sin cambios bajo `SddIA/`), sin bloquear el resto del ciclo.
* Todas las rutas y políticas se resuelven exclusivamente vía `cumulo.paths.json` y normas enlazadas (`git-operations`, `pull-request-orchestration`).
