use yew::prelude::*;

fn use_counter(initial_value: i32, increment_by: i32) -> (i32, Callback<()>) {
    let counter = use_state(|| initial_value);
    let increment = {
        let counter = counter.clone();
        Callback::from(move |_: ()| counter.set(*counter + increment_by))
    };

    ((*counter), increment)
}

#[derive(Properties, PartialEq)]
struct CounterProps {
    #[prop_or(0)]
    initial_value: i32,
    #[prop_or(1)]
    increment_by: i32,
}

#[function_component(Counter)]
fn counter(props: &CounterProps) -> Html {
    let (counter, increment) = use_counter(props.initial_value, props.increment_by);

    html! {
        <button onclick={move |_| increment.emit(())}>
            { counter }
        </button>
    }
}

#[function_component(UserProfile)]
fn user_profile() -> Html {
    let user = use_context::<User>().expect("No provider found");

    html! {
        <h1>{user.name}</h1>
    }
}

#[derive(Clone, PartialEq)]
struct User {
    name: String,
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <ContextProvider<User> context={User {
            name: String::from("Mokshit"),
        }}>
            <UserProfile />
            <Counter initial_value={1} increment_by={2} />
        </ContextProvider<User>>
    }
}

fn main() {
    yew::start_app::<App>();
}
