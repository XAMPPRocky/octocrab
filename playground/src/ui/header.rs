use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn Header(mut state: AppState) -> Element {
    let mut show_token = use_signal(|| false);
    let is_executing = *state.is_executing.read();
    let outcome = state.outcome.read();

    let rate_limit_text = if let Some(out) = &*outcome {
        if let (Some(rem), Some(lim)) = (out.rate_limit_remaining, out.rate_limit_limit) {
            format!("Rate: {rem}/{lim}")
        } else {
            "Rate: Available".to_string()
        }
    } else {
        "Rate: --".to_string()
    };

    rsx! {
        header { class: "navbar bg-base-200 border-b border-base-300 px-5 min-h-[3.5rem] flex flex-wrap items-center justify-between gap-3 shadow-sm",
            // Title & Branding
            div { class: "flex items-center space-x-3",
                span { class: "text-2xl", "🦀" }
                div {
                    h1 { class: "text-base font-bold tracking-wide flex items-center gap-2",
                        "Octocrab Playground"
                        span { class: "badge badge-primary badge-sm font-mono",
                            "Cross-Platform"
                        }
                    }
                    p { class: "text-xs opacity-60", "Schema-driven hierarchical GitHub query explorer" }
                }
            }

            // Presets Dropdown
            div { class: "flex items-center space-x-2",
                span { class: "text-xs font-semibold opacity-70 uppercase tracking-wider", "Preset:" }
                select {
                    class: "select select-bordered select-sm text-xs max-w-xs",
                    onchange: move |evt| {
                        state.load_preset(&evt.value());
                    },
                    option { value: "org_deep", "Deep Org: Org ➔ Repos ➔ Pull Requests" }
                    option { value: "repo_deep", "Deep Repo: Repo ➔ Issues ➔ Comments & PRs ➔ Commits" }
                    option { value: "user_ecosystem", "User Ecosystem: User ➔ Repos ➔ Releases & Gists" }
                    option { value: "search_deep", "Search & Inspect: Search Repos ➔ Issues" }
                    option { value: "repo_analytics", "Repo Analytics: Tags + Contributors + Languages + Milestones" }
                    option { value: "gist_deep", "Gist & Comments: Gist ➔ Comments" }
                    option { value: "repo", "Simple Repo + Issues + PRs" }
                    option { value: "org", "Simple Org + Repos + Members" }
                    option { value: "user", "Simple User + Repos + Followers" }
                    option { value: "search", "Simple Search Repositories" }
                }
            }

            // Controls & Actions
            div { class: "flex items-center space-x-2.5",
                // GitHub Token Input
                div { class: "join",
                    input {
                        r#type: if *show_token.read() { "text" } else { "password" },
                        placeholder: "ghp_... (Optional)",
                        class: "input input-bordered input-sm join-item font-mono text-xs w-36 md:w-44",
                        value: "{state.token}",
                        oninput: move |evt| {
                            state.token.set(evt.value());
                        }
                    }
                    button {
                        r#type: "button",
                        class: "btn btn-bordered btn-sm join-item text-xs",
                        onclick: move |_| {
                            let curr = *show_token.read();
                            show_token.set(!curr);
                        },
                        if *show_token.read() { "🙈" } else { "👁️" }
                    }
                }

                // Rate Limit Badge
                div { class: "badge badge-neutral font-mono text-xs py-3 px-3",
                    "{rate_limit_text}"
                }

                // Run Query Button
                button {
                    r#type: "button",
                    disabled: is_executing,
                    class: if is_executing { "btn btn-primary btn-sm btn-disabled" } else { "btn btn-primary btn-sm" },
                    onclick: move |_| {
                        state.execute();
                    },
                    if is_executing {
                        span { class: "loading loading-spinner loading-xs mr-1" }
                        span { "Querying..." }
                    } else {
                        span { "▶ Run Query" }
                    }
                }
            }
        }
    }
}
