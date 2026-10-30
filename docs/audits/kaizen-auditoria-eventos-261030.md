---
audit_id: KAIZEN-AUDITORIA-EVENTOS-261030
date: "2026-10-30"
branch: Kaicen_Auditoria_Eventos_261030
scope: "Higiene bus EDA — reset events.redb y archivo capa filesystem legacy"
status: APPLIED_LOCAL
---

# Kaizen — Auditoría de eventos (261030)

## Acciones aplicadas en instancia

1. **`events.redb` reinicializado** (2026-10-30): borrado del fichero runtime y recreación vacía vía `EventStore::open`. Cola histórica en store eliminada.
2. **Capa `.events/` archivada**: snapshot movido a  
   `.SddIA/runtime/archive/kaizen-auditoria-eventos-261030/events-filesystem-snapshot-20261030/`  
   (incluye testigos FS residuales en `dead-letter/subscribers/`).
3. **Topología FS vacía recreada** bajo `.events/` (`pending`, `processing`, `processed`, `dead-letter`, fractales, `subscribers/`) para rutas lógicas y barridos sin residuos pre-cutover.

## Notas

- En modo **`redb`** (default), el SSOT de cola y entregas es `.SddIA/runtime/events.redb`; `.events/pending/` no debe acumular ECST nuevos.
- Bóvedas (2026-10-30): `.dev/.env` y `.SddIA/.dev/.env` fijan `SDDIA_EVENT_STORE=redb` y `SDDIA_EVENT_STORE_OPEN_BUDGET_MS=2000`; plantillas en `.dev/.env.example` y `.dev/.env.test.example`.
- `pending/` ya estaba vacío antes del archivo final; el backlog histórico (~500+ JSON) había sido depurado en la misma ventana de kaizen.
- Tras el reset, recompilar y levantar `event-watcher` si el launcher reporta ELF fósil (`cargo build -p event-watcher`).

## Referencias

- [Auditoría redb vs FS (2026-10-26)](eda-events-redb-vs-filesystem-20261026.md)
- `SddIA/events/events-contract.md` § instancia ECST / `SDDIA_EVENT_STORE`
