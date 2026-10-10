use dioxus::prelude::*;
use octocrab_playground::state::AppState;
use octocrab_playground::ui::{Header, NodeForm, ResultViewer, TreeEditor};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let state = use_hook(AppState::new);

    rsx! {
        // DaisyUI & Tailwind CDN
        document::Link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/daisyui@latest/dist/full.min.css"
        }
        document::Link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css"
        }
        document::Title { "Octocrab Playground" }

        div {
            class: "flex flex-col h-screen w-screen overflow-hidden bg-base-300 text-base-content font-sans select-none",
            "data-theme": "dark",
            // Header bar
            Header { state: state }

            // 3-Pane Responsive Layout
            div { class: "flex flex-1 overflow-hidden",
                // Left Pane: Query Tree Hierarchy
                div { class: "w-72 flex-shrink-0 h-full overflow-hidden",
                    TreeEditor { state: state }
                }

                // Middle Pane: Selected Node Schema Parameters Form
                div { class: "w-80 flex-shrink-0 h-full overflow-hidden",
                    NodeForm { state: state }
                }

                // Right Pane: Merged JSON Result Viewer
                div { class: "flex-1 h-full overflow-hidden",
                    ResultViewer { state: state }
                }
            }
        }
    }
}
