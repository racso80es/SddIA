//! Filtros, cursor y paginación compartidos (MAN-43 O1).

use super::contract::{
    EventFamily, EventQueueState, EventUri, IndexedEvent, ListDeliveriesRequest,
    ListDeliveriesResult, ListEventsRequest, ListEventsResult, ListTerminalEventsRequest,
    ListedDelivery, ListedEvent,
};
use super::error::EventManagerError;

pub fn parse_family_filter(
    family: &Option<String>,
) -> Result<Option<EventFamily>, EventManagerError> {
    match family.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        None => Ok(None),
        Some(raw) => EventFamily::parse(raw)
            .map(Some)
            .map_err(EventManagerError::InvalidRequest),
    }
}

fn apply_event_filters(
    rows: Vec<IndexedEvent>,
    family: Option<EventFamily>,
    state: Option<EventQueueState>,
    event_type: Option<&str>,
) -> Vec<ListedEvent> {
    rows.into_iter()
        .filter(|r| {
            if let Some(st) = state {
                if r.state != st {
                    return false;
                }
            }
            if let Some(f) = family {
                let Some(uri) = &r.event_uri else {
                    return false;
                };
                if uri.family != f {
                    return false;
                }
            }
            if let Some(et) = event_type {
                if r.ecst.get("event_type").and_then(|v| v.as_str()) != Some(et) {
                    return false;
                }
            }
            true
        })
        .map(|r| ListedEvent {
            event_id: r.event_id,
            ecst: r.ecst,
            trace_id: r.trace_id,
        })
        .collect()
}

fn page_events(
    mut events: Vec<ListedEvent>,
    cursor: &Option<String>,
    limit: usize,
) -> Result<(Vec<ListedEvent>, Option<String>), EventManagerError> {
    events.sort_by(|a, b| a.event_id.cmp(&b.event_id));
    if let Some(c) = cursor.as_deref().filter(|s| !s.is_empty()) {
        let c = c.to_string();
        if events.iter().all(|e| e.event_id <= c) && !events.is_empty() {
            return Err(EventManagerError::InvalidRequest(format!("unknown cursor: {c}")));
        }
        events.retain(|e| e.event_id > c);
    }
    let has_more = events.len() > limit;
    events.truncate(limit);
    let next_cursor = if has_more {
        events.last().map(|e| e.event_id.clone())
    } else {
        None
    };
    Ok((events, next_cursor))
}

pub fn filter_and_page_events(
    rows: Vec<IndexedEvent>,
    req: &ListEventsRequest,
) -> Result<ListEventsResult, EventManagerError> {
    let family = parse_family_filter(&req.family)?;
    let limit = req.limit.unwrap_or(100) as usize;
    let filtered = apply_event_filters(rows, family, req.state, req.event_type.as_deref());
    let (events, next_cursor) = page_events(filtered, &req.cursor, limit)?;
    Ok(ListEventsResult {
        events,
        next_cursor,
    })
}

pub fn filter_terminal_events(
    rows: Vec<IndexedEvent>,
    req: &ListTerminalEventsRequest,
) -> Result<ListEventsResult, EventManagerError> {
    let terminal: Vec<IndexedEvent> = rows
        .into_iter()
        .filter(|r| {
            matches!(
                r.state,
                EventQueueState::Processed | EventQueueState::DeadLetter
            )
        })
        .collect();
    let list_req = ListEventsRequest {
        family: req.family.clone(),
        event_type: None,
        state: None,
        limit: req.limit,
        cursor: req.cursor.clone(),
    };
    filter_and_page_events(terminal, &list_req)
}

pub fn page_deliveries(
    mut deliveries: Vec<ListedDelivery>,
    req: &ListDeliveriesRequest,
) -> Result<ListDeliveriesResult, EventManagerError> {
    deliveries.sort_by(|a, b| a.delivery_id.cmp(&b.delivery_id));
    let limit = req.limit.unwrap_or(u32::MAX) as usize;
    if let Some(c) = req.cursor.as_deref().filter(|s| !s.is_empty()) {
        let c = c.to_string();
        if deliveries.iter().all(|d| d.delivery_id <= c) && !deliveries.is_empty() {
            return Err(EventManagerError::InvalidRequest(format!("unknown cursor: {c}")));
        }
        deliveries.retain(|d| d.delivery_id > c);
    }
    let has_more = deliveries.len() > limit;
    deliveries.truncate(limit);
    let next_cursor = if has_more {
        deliveries.last().map(|d| d.delivery_id.clone())
    } else {
        None
    };
    Ok(ListDeliveriesResult {
        deliveries,
        next_cursor,
    })
}

pub fn indexed_from_parts(
    event_uri: Option<EventUri>,
    event_id: String,
    ecst: serde_json::Value,
    trace_id: Option<String>,
    state: EventQueueState,
) -> IndexedEvent {
    IndexedEvent {
        event_uri,
        event_id,
        ecst,
        trace_id,
        state,
    }
}
