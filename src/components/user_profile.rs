use yew::{function_component, html, use_context};

#[derive(Clone, PartialEq)]
pub struct User {
    pub name: String,
}

#[function_component(UserProfile)]
pub fn user_profile() -> Html {
    let user = use_context::<User>().expect("No provider found");

    html! {
        <h1>{user.name}</h1>
    }
}
