use crate::registry::get_entity;
use crate::schema::FieldType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryNode {
    pub id: String,
    pub entity_id: String,
    pub params: serde_json::Map<String, serde_json::Value>,
    pub children: Vec<QueryNode>,
}

impl QueryNode {
    pub fn new(entity_id: &str) -> Self {
        let id = format!("{}_{}", entity_id, Self::random_id());
        let mut params = serde_json::Map::new();

        if let Some(entity) = get_entity(entity_id) {
            for field in entity.schema() {
                match &field.field_type {
                    FieldType::String { placeholder } => {
                        if let Some(def) = placeholder.as_ref().filter(|_| !field.inheritable) {
                            params.insert(
                                field.key.to_string(),
                                serde_json::Value::String(def.clone()),
                            );
                        }
                    }
                    FieldType::Integer { default, .. } => {
                        if let Some(def) = default {
                            params.insert(field.key.to_string(), serde_json::json!(def));
                        }
                    }
                    FieldType::Boolean { default } => {
                        params.insert(field.key.to_string(), serde_json::Value::Bool(*default));
                    }
                    FieldType::Enum { default, options } => {
                        let val = default
                            .clone()
                            .or_else(|| options.first().cloned())
                            .unwrap_or_default();
                        params.insert(field.key.to_string(), serde_json::Value::String(val));
                    }
                    FieldType::Secret { placeholder } => {
                        if let Some(def) = placeholder.as_ref().filter(|_| !field.inheritable) {
                            params.insert(
                                field.key.to_string(),
                                serde_json::Value::String(def.clone()),
                            );
                        }
                    }
                }
            }
        }

        Self {
            id,
            entity_id: entity_id.to_string(),
            params,
            children: Vec::new(),
        }
    }

    pub fn find_node(&self, target_id: &str) -> Option<&QueryNode> {
        if self.id == target_id {
            return Some(self);
        }
        for child in &self.children {
            if let Some(found) = child.find_node(target_id) {
                return Some(found);
            }
        }
        None
    }

    pub fn find_node_mut(&mut self, target_id: &str) -> Option<&mut QueryNode> {
        if self.id == target_id {
            return Some(self);
        }
        for child in &mut self.children {
            if let Some(found) = child.find_node_mut(target_id) {
                return Some(found);
            }
        }
        None
    }

    pub fn add_child(&mut self, parent_id: &str, entity_id: &str) -> Option<String> {
        if let Some(parent) = self.find_node_mut(parent_id) {
            let child = QueryNode::new(entity_id);
            let child_id = child.id.clone();
            parent.children.push(child);
            Some(child_id)
        } else {
            None
        }
    }

    pub fn remove_node(&mut self, target_id: &str) -> bool {
        let initial_len = self.children.len();
        self.children.retain(|c| c.id != target_id);
        if self.children.len() < initial_len {
            return true;
        }

        for child in &mut self.children {
            if child.remove_node(target_id) {
                return true;
            }
        }
        false
    }

    pub fn update_param(&mut self, node_id: &str, key: &str, val: serde_json::Value) -> bool {
        if let Some(node) = self.find_node_mut(node_id) {
            node.params.insert(key.to_string(), val);
            true
        } else {
            false
        }
    }

    fn random_id() -> String {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1000);
        let count = COUNTER.fetch_add(1, Ordering::Relaxed);
        format!("{:x}", count)
    }
}
