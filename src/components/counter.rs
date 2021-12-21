use yew::{function_component, html, use_state, Callback, Properties};

fn use_counter(initial_value: i32, increment_by: i32) -> (i32, Callback<()>) {
    let counter = use_state(|| initial_value);
    let increment = {
        let counter = counter.clone();
        Callback::from(move |_: ()| counter.set(*counter + increment_by))
    };

    ((*counter), increment)
}

#[derive(Properties, PartialEq)]
pub struct CounterProps {
    #[prop_or(0)]
    initial_value: i32,
    #[prop_or(1)]
    increment_by: i32,
}

#[function_component(Counter)]
pub fn counter(props: &CounterProps) -> Html {
    let (counter, increment) = use_counter(props.initial_value, props.increment_by);

    html! {
        <button class="" onclick={move |_| increment.emit(())}>
            { counter }
        </button>
    }
}
