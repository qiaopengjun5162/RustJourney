mod components;

use std::collections::BTreeMap;

use dioxus::prelude::*;
use tracing::info;

use crate::components::{todo_filter, todo_input, todo_item};

#[derive(Debug, Clone, PartialEq)]
pub struct TodoItem {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

pub type Todos = BTreeMap<u32, TodoItem>;

#[derive(Debug, Clone, PartialEq)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Default for Filter {
    fn default() -> Self {
        Filter::All
    }
}

pub fn app(cx: Scope) -> Element {
    let todos = use_state(cx, Todos::default);
    let filter = use_state(cx, Filter::default);

    let filtered_todos = todos
        .iter()
        .filter(|(_, todo)| match **filter {
            Filter::All => true,
            Filter::Active => !todo.completed,
            Filter::Completed => todo.completed,
        })
        .map(|(id, _)| *id)
        .collect::<Vec<_>>();

    info!("filtered_todos: {:?}", filtered_todos);

    cx.render(rsx! {
    section {
        class: "todoapp",
        style { include_str!("style.css")},
        h1 { "Todos" },
        div {
            rsx!(todo_input(cx)),
            ul {
                class: "todo-list",
                filtered_todos.iter().map(|id| {

                    rsx!(todo_item(cx))
                })
            },

            rsx!(todo_filter(cx))

        }
    }
    })
}
