use dioxus::prelude::*;

pub fn todo_item(cx: Scope) -> Element {
    cx.render(rsx! {
        h3 { "todo item" }
    })
}
