use crate::components::todo_list::TodoList;
use yew::{function_component, html};

#[function_component(Todo)]
pub fn todo() -> Html {
    html! {
        <TodoList />
    }
}
