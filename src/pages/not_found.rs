use yew::{function_component, html};

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <h1>{ "Not Found" }</h1>
    }
}