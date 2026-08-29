//! Extracción y agregación de `telemetry_receipt` (DD-1, L3–L5).

use serde_json::{json, Value};

pub const COGNITIVE_DEGRADED_KEY: &str = "cognitive-degraded";

pub fn map_thermodynamic_cost(cost: &Value) -> Value {
    let prompt = cost
        .get("tokens_in")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let completion = cost
        .get("tokens_out")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let latency = cost
        .get("duration_ms")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    json!({
        "prompt_tokens": prompt,
        "completion_tokens": completion,
        "provider_latency_ms": latency,
        COGNITIVE_DEGRADED_KEY: prompt + completion == 0,
    })
}

fn normalize_receipt(mut receipt: Value) -> Value {
    if receipt.get(COGNITIVE_DEGRADED_KEY).is_none() {
        receipt[COGNITIVE_DEGRADED_KEY] = json!(false);
    }
    receipt
}

fn receipt_from_object(obj: &Value, capsule_id: &str) -> Option<Value> {
    if !obj.is_object() || obj.as_object().is_some_and(|o| o.is_empty()) {
        return None;
    }
    let mut receipt = normalize_receipt(obj.clone());
    receipt["capsule_id"] = json!(capsule_id);
    Some(receipt)
}

pub fn extract_from_capsule_body(body: &Value, capsule_id: &str) -> Option<Value> {
    if let Some(r) = body.get("telemetry_receipt") {
        if let Some(receipt) = receipt_from_object(r, capsule_id) {
            return Some(receipt);
        }
    }
    if let Some(r) = body
        .get("result")
        .and_then(|v| v.get("telemetry_receipt"))
    {
        if let Some(receipt) = receipt_from_object(r, capsule_id) {
            return Some(receipt);
        }
    }
    if let Some(data) = body.get("data") {
        if let Some(r) = data.get("telemetry_receipt") {
            if let Some(receipt) = receipt_from_object(r, capsule_id) {
                return Some(receipt);
            }
        }
    }
    let cost = body
        .get("thermodynamic_cost")
        .or_else(|| body.get("result").and_then(|v| v.get("thermodynamic_cost")));
    if let Some(cost) = cost.filter(|c| c.is_object()) {
        let mut receipt = map_thermodynamic_cost(cost);
        receipt["capsule_id"] = json!(capsule_id);
        return Some(receipt);
    }
    None
}

pub fn accumulate_in_state(state: &mut Value, receipt: Value, capsule_id: &str) {
    let mut entry = receipt;
    if entry.get("capsule_id").is_none() {
        entry["capsule_id"] = json!(capsule_id);
    }
    let Some(obj) = state.as_object_mut() else {
        return;
    };
    if !obj.contains_key("telemetry_receipts") {
        obj.insert("telemetry_receipts".into(), json!([]));
    }
    if let Some(arr) = obj.get_mut("telemetry_receipts").and_then(|v| v.as_array_mut()) {
        arr.push(entry);
    }
}

pub fn merge_for_peaje(receipts: &[Value]) -> Option<(Value, String)> {
    if receipts.is_empty() {
        return None;
    }
    let mut prompt_sum: u64 = 0;
    let mut completion_sum: u64 = 0;
    let mut max_latency: u64 = 0;
    let mut degraded = false;
    let mut last_model: Option<Value> = None;
    let mut last_tier: Option<Value> = None;
    let mut last_capsule_id = String::new();

    for r in receipts {
        prompt_sum += r
            .get("prompt_tokens")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        completion_sum += r
            .get("completion_tokens")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        max_latency = max_latency.max(
            r.get("provider_latency_ms")
                .and_then(|v| v.as_u64())
                .unwrap_or(0),
        );
        degraded |= r
            .get(COGNITIVE_DEGRADED_KEY)
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        if let Some(cid) = r.get("capsule_id").and_then(|v| v.as_str()) {
            if !cid.is_empty() {
                last_capsule_id = cid.to_string();
            }
        }
        let is_degraded = r
            .get(COGNITIVE_DEGRADED_KEY)
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        if !is_degraded {
            if r.get("llm_model").is_some() {
                last_model = r.get("llm_model").cloned();
            }
            if r.get("tier").is_some() {
                last_tier = r.get("tier").cloned();
            }
        } else if last_model.is_none() {
            last_model = r.get("llm_model").cloned();
            last_tier = r.get("tier").cloned();
        }
    }

    let mut merged = json!({
        "prompt_tokens": prompt_sum,
        "completion_tokens": completion_sum,
        "provider_latency_ms": max_latency,
        COGNITIVE_DEGRADED_KEY: degraded,
    });
    if let Some(m) = last_model {
        merged["llm_model"] = m;
    }
    if let Some(t) = last_tier {
        merged["tier"] = t;
    }
    let capsule_id = if last_capsule_id.is_empty() {
        receipts
            .last()?
            .get("capsule_id")?
            .as_str()?
            .to_string()
    } else {
        last_capsule_id
    };
    Some((merged, capsule_id))
}

pub fn attach_to_ref_payload(payload: &mut Value, state: &Value) {
    let receipts = state
        .get("telemetry_receipts")
        .and_then(|v| v.as_array())
        .map(|a| a.as_slice())
        .unwrap_or(&[]);
    if let Some((receipt, capsule_id)) = merge_for_peaje(receipts) {
        payload["telemetry_receipt"] = receipt;
        payload["capsule_id"] = json!(capsule_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_thermodynamic_cost_to_receipt() {
        let cost = json!({"tokens_in": 10, "tokens_out": 5, "duration_ms": 120});
        let r = map_thermodynamic_cost(&cost);
        assert_eq!(r["prompt_tokens"], 10);
        assert_eq!(r["completion_tokens"], 5);
        assert_eq!(r["provider_latency_ms"], 120);
        assert_eq!(r[COGNITIVE_DEGRADED_KEY], false);
    }

    #[test]
    fn merge_sums_tokens_and_or_degraded() {
        let a = json!({
            "prompt_tokens": 10,
            "completion_tokens": 5,
            "provider_latency_ms": 100,
            COGNITIVE_DEGRADED_KEY: false,
            "llm_model": "m1",
            "capsule_id": "skill:a"
        });
        let b = json!({
            "prompt_tokens": 3,
            "completion_tokens": 2,
            "provider_latency_ms": 200,
            COGNITIVE_DEGRADED_KEY: true,
            "capsule_id": "skill:b"
        });
        let (merged, cid) = merge_for_peaje(&[a, b]).unwrap();
        assert_eq!(merged["prompt_tokens"], 13);
        assert_eq!(merged["completion_tokens"], 7);
        assert_eq!(merged["provider_latency_ms"], 200);
        assert_eq!(merged[COGNITIVE_DEGRADED_KEY], true);
        assert_eq!(merged["llm_model"], "m1");
        assert_eq!(cid, "skill:b");
    }

    #[test]
    fn extract_from_thermodynamic_cost_in_body() {
        let body = json!({"thermodynamic_cost": {"tokens_in": 1, "tokens_out": 2, "duration_ms": 9}});
        let r = extract_from_capsule_body(&body, "skill:x").unwrap();
        assert_eq!(r["prompt_tokens"], 1);
        assert_eq!(r["capsule_id"], "skill:x");
    }
}
