# Índice de process — códice software-engineering

Packing de dominio (`directories.process_domain_roots`). Resolución: domain-first, luego Core.

| Name | UUID | Versión | Context | Aliases | Descripción |
|------|------|---------|---------|---------|-------------|
| feature | 1b4fa69f-4299-47ca-b2ed-380f2263239c | 1.2.0 | ecosystem-evolution, filesystem-ops, source-control | — | Proceso V5 para desarrollo de **features**: cadena **Inicialización → Mayeuta → Dedalo → Tekton → Argos → delivery-close-cycle** (`source_process: feature`). Puente documental a `features-documentation-pattern` bajo `persist_ref`. Sustituye el linaje legacy `spec/clarify/planning/...` purgado. |
| bug-fix | ac8d078c-9785-490b-9f43-ad310fe9df9d | 1.2.0 | ecosystem-evolution, filesystem-ops, source-control | — | Proceso V5 para **corrección de defectos**: cadena **Inicialización → Diseño del fix → Tekton → Argos → delivery-close-cycle** (`source_process: bug-fix`). Subconjunto documental bajo `persist_ref`; Mayeuta opcional si triaje ambiguo. |
| refactorization | ae01e3ff-af68-4b94-90b3-97e5c03d75ee | 1.2.0 | ecosystem-evolution, filesystem-ops, source-control | — | Proceso V5 para **refactorización**: cadena **Inicialización → Estabilización de alcance → Diseño de refactor → Tekton → Argos → delivery-close-cycle** (`source_process: refactorization`). Puente a `features-documentation-pattern` bajo `persist_ref`; sin nueva capacidad funcional. |
| pull-request-review | 6d59f23b-df29-4be5-9bb9-29cede3474b9 | 2.2.0 | quality-assurance, source-control, pr-lifecycle | — | **Aduana de Fricción** reactiva a `PullRequest_Presented`: triaje documental/técnico/RBAC (Argos + Cerbero), sensor DIA → evento `Kaizen_Alert_Required`, Kaizen genérico (Cúmulo), handoff a `accept-pr`. Sin Dedalo; sin merge directo. |
| accept-pr | bd81c878-5e1c-4fe5-b204-85c9136d8bc7 | 1.0.0 | pr-lifecycle, source-control, filesystem-ops | — | Aceptación local soberana: Argos → merge a main → emit-pr-merged-event → push main y limpieza de rama origen. |
| delivery-close-cycle | 5417c92c-da7f-4d46-b245-55cf1b17961a | 1.4.0 | ecosystem-evolution | — | Cierre de entrega: snapshot, Argos/EDA, push, `gh` vía shell-executor, sello **PullRequest_Presented**, higiene local. Merge solo en `accept-pr`. |
