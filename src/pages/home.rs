use crate::components::counter::Counter;
use yew::{function_component, html};

#[function_component(Home)]
pub fn home() -> Html {
    return html! { <Counter /> };
}
