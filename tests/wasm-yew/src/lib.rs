use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let _media_type = octocrab::format_media_type("json");
    let _builder = octocrab::OctocrabBuilder::default();

    html! {
        <main>
            { "octocrab + yew wasm compile check" }
        </main>
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn formats_github_media_type() {
        assert_eq!(octocrab::format_media_type("json"), "application/vnd.github.v3.json");
    }
}
