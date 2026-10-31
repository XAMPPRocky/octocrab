use crate::engine::codegen::generate_rust_code;
use crate::state::AppState;
use crate::ui::highlight::{highlight_json, highlight_rust};
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
enum ActiveTab {
    Json,
    RustCode,
}

#[component]
pub fn ResultViewer(state: AppState) -> Element {
    let outcome = state.outcome.read();
    let status_msg = state.status_message.read().clone();
    let is_executing = *state.is_executing.read();
    let root_node = state.root_node.read().clone();
    let has_token = !state.token.read().is_empty();

    let mut active_tab = use_signal(|| ActiveTab::Json);
    let mut copied_json = use_signal(|| false);
    let mut copied_code = use_signal(|| false);

    let current_tab = *active_tab.read();
    let generated_code = generate_rust_code(&root_node, has_token);

    rsx! {
        div { class: "flex flex-col h-full bg-base-300 p-4 space-y-3",
            // Header Bar
            div { class: "flex items-center justify-between border-b border-base-300 pb-3",
                // Tabs
                div { class: "flex items-center space-x-3",
                    div { class: "tabs tabs-boxed bg-base-200 p-1",
                        button {
                            r#type: "button",
                            class: if current_tab == ActiveTab::Json { "tab tab-active text-xs font-semibold" } else { "tab text-xs" },
                            onclick: move |_| {
                                active_tab.set(ActiveTab::Json);
                                copied_json.set(false);
                            },
                            "📦 Merged JSON"
                        }

                        button {
                            r#type: "button",
                            class: if current_tab == ActiveTab::RustCode { "tab tab-active text-xs font-semibold" } else { "tab text-xs" },
                            onclick: move |_| {
                                active_tab.set(ActiveTab::RustCode);
                                copied_code.set(false);
                            },
                            "🦀 Rust Code"
                        }
                    }

                    if let Some(msg) = status_msg {
                        span { class: "badge badge-ghost font-mono text-xs",
                            "{msg}"
                        }
                    }
                }

                // Actions bar
                div { class: "flex items-center space-x-2",
                    if current_tab == ActiveTab::Json {
                        if let Some(out) = &*outcome {
                            // Status badge
                            if out.has_errors {
                                span { class: "badge badge-error badge-sm font-semibold",
                                    "Partial Errors"
                                }
                            } else {
                                span { class: "badge badge-success badge-sm font-semibold",
                                    "Success"
                                }
                            }

                            // Execution time
                            span { class: "badge badge-ghost badge-sm font-mono",
                                "{out.total_duration_ms} ms"
                            }

                            // Copy JSON Button
                            {
                                let json_for_copy = serde_json::to_string_pretty(&out.root_result).unwrap_or_default();
                                rsx! {
                                    button {
                                        r#type: "button",
                                        class: "btn btn-outline btn-xs",
                                        onclick: move |_| {
                                            let text = json_for_copy.clone();
                                            copied_json.set(true);
                                            copy_to_clipboard(&text);
                                        },
                                        if *copied_json.read() {
                                            span { class: "text-success font-semibold", "✓ Copied!" }
                                        } else {
                                            span { "📋 Copy JSON" }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        // Rust Code Tab Actions
                        span { class: "badge badge-neutral badge-sm font-mono",
                            "octocrab v0.54"
                        }

                        {
                            let code_for_copy = generated_code.clone();
                            rsx! {
                                button {
                                    r#type: "button",
                                    class: "btn btn-outline btn-warning btn-xs",
                                    onclick: move |_| {
                                        let text = code_for_copy.clone();
                                        copied_code.set(true);
                                        copy_to_clipboard(&text);
                                    },
                                    if *copied_code.read() {
                                        span { class: "text-success font-semibold", "✓ Copied!" }
                                    } else {
                                        span { "📋 Copy Code" }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Body Area
            div { class: "flex-1 overflow-auto rounded-lg bg-base-200 border border-base-300 p-4 font-mono text-xs",
                if current_tab == ActiveTab::Json {
                    if is_executing {
                        div { class: "flex flex-col items-center justify-center h-full space-y-3 opacity-60",
                            span { class: "loading loading-spinner loading-lg text-primary" }
                            p { class: "text-sm", "Executing GitHub queries and assembling hierarchical JSON..." }
                        }
                    } else if let Some(out) = &*outcome {
                        {
                            let json_pretty = serde_json::to_string_pretty(&out.root_result).unwrap_or_default();
                            let json_lines = highlight_json(&json_pretty);
                            rsx! {
                                pre { class: "whitespace-pre leading-relaxed select-text font-mono text-[11px]",
                                    for (line_idx, tokens) in json_lines.iter().enumerate() {
                                        div { key: "{line_idx}", class: "flex hover:bg-base-300/40 px-1 py-0.5 rounded",
                                            span { class: "w-8 text-right pr-3 select-none opacity-30 text-[10px]",
                                                "{line_idx + 1}"
                                            }
                                            span { class: "flex-1",
                                                for (tok_idx, token) in tokens.iter().enumerate() {
                                                    span {
                                                        key: "{tok_idx}",
                                                        class: "{token.token_type.class()}",
                                                        "{token.text}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        div { class: "flex flex-col items-center justify-center h-full space-y-3 opacity-40 text-center p-8",
                            span { class: "text-4xl", "📊" }
                            h3 { class: "text-base font-semibold", "No Query Results Yet" }
                            p { class: "text-xs max-w-sm",
                                "Configure your root entity and hierarchical children on the left, then click '▶ Run Query' in the header to execute."
                            }
                        }
                    }
                } else {
                    // Rust Code Tab with Syntax Highlighting
                    {
                        let rust_lines = highlight_rust(&generated_code);
                        rsx! {
                            pre { class: "whitespace-pre leading-relaxed select-text font-mono text-[11px]",
                                for (line_idx, tokens) in rust_lines.iter().enumerate() {
                                    div { key: "{line_idx}", class: "flex hover:bg-base-300/40 px-1 py-0.5 rounded",
                                        span { class: "w-10 text-right pr-3 select-none opacity-30 text-[10px]",
                                            "{line_idx + 1}"
                                        }
                                        span { class: "flex-1",
                                            for (tok_idx, token) in tokens.iter().enumerate() {
                                                span {
                                                    key: "{tok_idx}",
                                                    class: "{token.token_type.class()}",
                                                    "{token.text}"
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

/// Helper function to copy text to the system clipboard across desktop and web
fn copy_to_clipboard(text: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            let nav = window.navigator();
            let clipboard = nav.clipboard();
            let _ = clipboard.write_text(text);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let text_owned = text.to_string();
        tokio::spawn(async move {
            #[cfg(target_os = "macos")]
            {
                use std::io::Write;
                use std::process::{Command, Stdio};
                if let Some(mut stdin) = Command::new("pbcopy")
                    .stdin(Stdio::piped())
                    .spawn()
                    .ok()
                    .and_then(|mut child| child.stdin.take())
                {
                    let _ = stdin.write_all(text_owned.as_bytes());
                }
            }
        });
    }
}
