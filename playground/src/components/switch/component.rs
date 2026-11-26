use dioxus::prelude::*;
use dioxus_primitives::switch::{self, SwitchProps};

#[css_module("/src/components/switch/style.css")]
struct Styles;

#[component]
pub fn Switch(props: SwitchProps) -> Element {
    rsx! {
        switch::Switch {
            class: Styles::dx_switch,
            checked: props.checked,
            default_checked: props.default_checked,
            disabled: props.disabled,
            required: props.required,
            name: props.name,
            value: props.value,
            on_checked_change: props.on_checked_change,
            attributes: props.attributes,
            switch::SwitchThumb { class: Styles::dx_switch_thumb }
        }
    }
}
