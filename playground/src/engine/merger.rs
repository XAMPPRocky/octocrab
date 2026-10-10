use serde_json::{Map, Value, json};

pub fn build_node_envelope(
    entity_id: &str,
    status: &str,
    duration_ms: u128,
    error: Option<String>,
    data: Option<Value>,
    children_map: Map<String, Value>,
) -> Value {
    let mut meta = json!({
        "entity": entity_id,
        "status": status,
        "duration_ms": duration_ms,
    });

    if let Some(err) = error {
        meta["error"] = json!(err);
    }

    if let Some(arr) = data.as_ref().and_then(|d| d.as_array()) {
        meta["item_count"] = json!(arr.len());
    }

    let mut result = Map::new();
    result.insert("_meta".to_string(), meta);

    if let Some(d) = data {
        result.insert("data".to_string(), d);
    } else {
        result.insert("data".to_string(), Value::Null);
    }

    if !children_map.is_empty() {
        result.insert("children".to_string(), Value::Object(children_map));
    }

    Value::Object(result)
}
