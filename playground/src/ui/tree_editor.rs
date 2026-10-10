use crate::engine::QueryNode;
use crate::registry::{all_root_entities, allowed_child_entities, get_entity};
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn TreeEditor(mut state: AppState) -> Element {
    let root = state.root_node.read().clone();
    let selected_id = state.selected_node_id.read().clone();

    rsx! {
        div { class: "flex flex-col h-full bg-base-200 border-r border-base-300 p-4 overflow-y-auto",
            // Header / Change Root
            div { class: "flex items-center justify-between pb-3 border-b border-base-300 mb-3",
                span { class: "text-xs font-bold opacity-70 uppercase tracking-wider", "Query Hierarchy" }
                // Change Root Dropdown
                select {
                    class: "select select-bordered select-xs max-w-[10rem]",
                    value: "{root.entity_id}",
                    onchange: move |evt| {
                        let new_entity_id = evt.value();
                        let new_node = QueryNode::new(&new_entity_id);
                        let new_id = new_node.id.clone();
                        state.root_node.set(new_node);
                        state.selected_node_id.set(new_id);
                        state.outcome.set(None);
                    },
                    for ent in all_root_entities() {
                        option {
                            value: "{ent.id()}",
                            selected: ent.id() == root.entity_id,
                            "{ent.display_name()}"
                        }
                    }
                }
            }

            // Tree Nodes
            div { class: "space-y-2 flex-1",
                TreeNodeItem {
                    node: root.clone(),
                    depth: 0,
                    selected_id: selected_id.clone(),
                    is_root: true,
                    state: state,
                }
            }

            // Quick Info Callout
            div { class: "mt-4 p-2.5 rounded bg-base-300/60 border border-base-300 text-xs opacity-70",
                p {
                    span { class: "text-primary font-semibold", "Tip: " }
                    "Click a node to configure parameters. Add child entities to automatically query and merge related GitHub data hierarchically."
                }
            }
        }
    }
}

#[component]
fn TreeNodeItem(
    node: QueryNode,
    depth: usize,
    selected_id: String,
    is_root: bool,
    mut state: AppState,
) -> Element {
    let node_id_for_select = node.id.clone();
    let node_id_for_delete = node.id.clone();
    let node_id_for_add = node.id.clone();

    let is_selected = selected_id == node.id;
    let entity = get_entity(&node.entity_id);
    let entity_name = entity
        .as_ref()
        .map(|e| e.display_name())
        .unwrap_or(node.entity_id.as_str());

    let allowed_children = allowed_child_entities(&node.entity_id);
    let indent_px = depth * 16;

    rsx! {
        div { class: "flex flex-col space-y-1.5",
            // Node Header Row
            div {
                class: if is_selected {
                    "flex items-center justify-between px-2.5 py-1.5 rounded bg-primary/20 border border-primary/60 shadow-xs cursor-pointer transition"
                } else {
                    "flex items-center justify-between px-2.5 py-1.5 rounded bg-base-100 hover:bg-base-300 border border-base-300 cursor-pointer transition"
                },
                style: "margin-left: {indent_px}px",
                onclick: move |_| {
                    state.selected_node_id.set(node_id_for_select.clone());
                },
                // Icon + Name
                div { class: "flex items-center space-x-2 truncate",
                    span { class: "text-xs font-mono opacity-50",
                        if depth == 0 { "●" } else { "└─" }
                    }
                    span { class: if is_selected { "text-xs font-semibold text-primary" } else { "text-xs font-medium" },
                        "{entity_name}"
                    }
                    if is_root {
                        span { class: "badge badge-secondary badge-xs font-mono",
                            "root"
                        }
                    }
                }

                // Node Actions (Add Child, Delete)
                div { class: "flex items-center space-x-1.5",
                    // Add Child dropdown if allowed
                    if !allowed_children.is_empty() {
                        select {
                            class: "select select-bordered select-xs text-[11px] text-primary h-6 min-h-0",
                            onchange: move |evt| {
                                let child_entity_id = evt.value();
                                if !child_entity_id.is_empty() {
                                    let mut root = state.root_node.read().clone();
                                    if let Some(new_child_id) = root.add_child(&node_id_for_add, &child_entity_id) {
                                        state.root_node.set(root);
                                        state.selected_node_id.set(new_child_id);
                                    }
                                }
                            },
                            option { value: "", selected: true, disabled: true, "+ Child" }
                            for child in allowed_children {
                                option { value: "{child.id()}", "{child.display_name()}" }
                            }
                        }
                    }

                    // Delete button (non-root)
                    if !is_root {
                        button {
                            r#type: "button",
                            class: "btn btn-ghost btn-xs text-error p-0.5 h-6 min-h-0",
                            title: "Remove child entity",
                            onclick: move |evt| {
                                evt.stop_propagation();
                                let mut root = state.root_node.read().clone();
                                if root.remove_node(&node_id_for_delete) {
                                    let root_id = root.id.clone();
                                    state.root_node.set(root);
                                    state.selected_node_id.set(root_id);
                                }
                            },
                            "✕"
                        }
                    }
                }
            }

            // Children Render
            for child in &node.children {
                TreeNodeItem {
                    key: "{child.id}",
                    node: child.clone(),
                    depth: depth + 1,
                    selected_id: selected_id.clone(),
                    is_root: false,
                    state: state,
                }
            }
        }
    }
}
