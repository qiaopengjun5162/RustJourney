use dioxus::prelude::*;

pub fn todo_input(cx: Scope) -> Element {
    let _draft = use_state(&cx, || "".to_string());
    cx.render(rsx! { h2 { "todo input" }})
}
