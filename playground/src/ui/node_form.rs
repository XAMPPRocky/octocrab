use crate::registry::get_entity;
use crate::schema::SchemaForm;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn NodeForm(mut state: AppState) -> Element {
    let selected_id = state.selected_node_id.read().clone();
    let root = state.root_node.read().clone();

    let node_opt = root.find_node(&selected_id);
    let context = state.get_context_for_node(&selected_id);

    rsx! {
        div { class: "flex flex-col h-full bg-base-200 border-r border-base-300 p-4 overflow-y-auto",
            if let Some(node) = node_opt {
                {
                    let entity_opt = get_entity(&node.entity_id);
                    let target_id = node.id.clone();
                    let schema = entity_opt.as_ref().map(|e| e.schema()).unwrap_or_default();
                    let title = entity_opt.as_ref().map(|e| e.display_name()).unwrap_or("Entity");
                    let description = entity_opt.as_ref().map(|e| e.description()).unwrap_or("");

                    rsx! {
                        div { class: "space-y-4",
                            // Title & Description
                            div { class: "pb-3 border-b border-base-300",
                                div { class: "flex items-center justify-between",
                                    h2 { class: "text-sm font-bold", "{title}" }
                                    span { class: "badge badge-ghost font-mono text-[10px]", "id: {node.id}" }
                                }
                                p { class: "text-xs opacity-60 mt-1", "{description}" }
                            }

                            // Dynamic Schema-Generated Form
                            SchemaForm {
                                schema: schema,
                                params: node.params.clone(),
                                context: context,
                                on_change: move |ev: (String, serde_json::Value)| {
                                    let (key, value) = ev;
                                    let mut updated_root = state.root_node.read().clone();
                                    if updated_root.update_param(&target_id, &key, value) {
                                        state.root_node.set(updated_root);
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                div { class: "flex items-center justify-center h-full opacity-40 text-xs text-center p-4",
                    "Select an entity node from the hierarchy tree to edit parameters."
                }
            }
        }
    }
}
