use super::types::{FieldSchema, FieldType};
use crate::registry::ParentContext;
use dioxus::prelude::*;

#[component]
pub fn SchemaForm(
    schema: Vec<FieldSchema>,
    params: serde_json::Map<String, serde_json::Value>,
    context: ParentContext,
    on_change: EventHandler<(String, serde_json::Value)>,
) -> Element {
    rsx! {
        div { class: "space-y-3",
            for field in schema {
                {
                    let key = field.key.to_string();
                    let key_for_change = key.clone();
                    let current_val = params.get(&key);
                    let inherited_val = context.get(&key);

                    rsx! {
                        div { key: "{field.key}", class: "card bg-base-200 border border-base-300 p-3 shadow-xs space-y-1.5",
                            div { class: "flex items-center justify-between",
                                label { class: "text-xs font-semibold",
                                    "{field.label}"
                                    if field.required {
                                        span { class: "text-error ml-1", "*" }
                                    }
                                }
                                if let Some(inh) = inherited_val {
                                    if field.inheritable {
                                        span { class: "badge badge-info badge-sm font-mono text-[10px]",
                                            "Inherited: {inh}"
                                        }
                                    }
                                }
                            }

                            div { class: "text-xs opacity-60", "{field.description}" }

                            {
                                match &field.field_type {
                                    FieldType::String { placeholder } => {
                                        let display_str = current_val
                                            .and_then(|v| v.as_str())
                                            .or(inherited_val)
                                            .unwrap_or_default();
                                        let ph = placeholder.clone().unwrap_or_default();
                                        let k = key_for_change.clone();

                                        rsx! {
                                            input {
                                                r#type: "text",
                                                class: "input input-bordered input-sm w-full text-xs",
                                                value: "{display_str}",
                                                placeholder: "{ph}",
                                                oninput: move |evt| {
                                                    let val = evt.value();
                                                    on_change.call((k.clone(), serde_json::Value::String(val)));
                                                }
                                            }
                                        }
                                    }
                                    FieldType::Integer { min, max, default } => {
                                        let num_val = current_val
                                            .and_then(|v| v.as_i64())
                                            .or(*default)
                                            .unwrap_or(0);
                                        let min_attr = min.map(|m| m.to_string()).unwrap_or_else(|| "0".to_string());
                                        let max_attr = max.map(|m| m.to_string()).unwrap_or_else(|| "100000".to_string());
                                        let k = key_for_change.clone();

                                        rsx! {
                                            input {
                                                r#type: "number",
                                                min: "{min_attr}",
                                                max: "{max_attr}",
                                                class: "input input-bordered input-sm w-full text-xs",
                                                value: "{num_val}",
                                                oninput: move |evt| {
                                                    if let Ok(parsed) = evt.value().parse::<i64>() {
                                                        on_change.call((k.clone(), serde_json::json!(parsed)));
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    FieldType::Boolean { default } => {
                                        let is_checked = current_val
                                            .and_then(|v| v.as_bool())
                                            .unwrap_or(*default);
                                        let k = key_for_change.clone();

                                        rsx! {
                                            label { class: "label cursor-pointer justify-start gap-2 pt-1",
                                                input {
                                                    r#type: "checkbox",
                                                    checked: "{is_checked}",
                                                    class: "checkbox checkbox-primary checkbox-sm",
                                                    onchange: move |evt| {
                                                        let val = evt.value() == "true";
                                                        on_change.call((k.clone(), serde_json::Value::Bool(val)));
                                                    }
                                                }
                                                span { class: "label-text text-xs", "Enable flag" }
                                            }
                                        }
                                    }
                                    FieldType::Enum { options, default } => {
                                        let selected_val = current_val
                                            .and_then(|v| v.as_str())
                                            .map(|s| s.to_string())
                                            .or_else(|| default.clone())
                                            .or_else(|| options.first().cloned())
                                            .unwrap_or_default();
                                        let k = key_for_change.clone();

                                        rsx! {
                                            select {
                                                class: "select select-bordered select-sm w-full text-xs",
                                                value: "{selected_val}",
                                                onchange: move |evt| {
                                                    on_change.call((k.clone(), serde_json::Value::String(evt.value())));
                                                },
                                                for opt in options {
                                                    option {
                                                        value: "{opt}",
                                                        selected: opt == &selected_val,
                                                        "{opt}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    FieldType::Secret { placeholder } => {
                                        let display_str = current_val.and_then(|v| v.as_str()).unwrap_or_default();
                                        let ph = placeholder.clone().unwrap_or_default();
                                        let k = key_for_change.clone();

                                        rsx! {
                                            input {
                                                r#type: "password",
                                                class: "input input-bordered input-sm w-full text-xs font-mono",
                                                value: "{display_str}",
                                                placeholder: "{ph}",
                                                oninput: move |evt| {
                                                    on_change.call((k.clone(), serde_json::Value::String(evt.value())));
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
