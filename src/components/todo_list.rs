use web_sys::{FormData, HtmlFormElement};
use yew::{
    function_component, html, use_node_ref, use_state, Callback, FocusEvent, Html, Properties,
};

#[derive(Debug, Clone, PartialEq)]
struct Todo {
    text: String,
    complete: bool,
}

impl Todo {
    fn new(text: String) -> Self {
        Self {
            text,
            complete: false,
        }
    }
}

#[function_component(TodoList)]
pub fn todo_list() -> Html {
    let form_ref = use_node_ref();
    let todos = use_state(|| Vec::<Todo>::new());
    let add_todo = {
        let todos = todos.clone();
        move |text: String| {
            let mut new_vec = (&*todos).into_iter().map(|t| t.clone()).collect::<Vec<_>>();
            new_vec.push(Todo::new(text));
            todos.set(new_vec);
        }
    };
    let on_toggle_todo = {
        let todos = todos.clone();
        Callback::from(move |index: usize| {
            // log::info!("Toggling {}", index);
            web_sys::console::log_1(&format!("Toggling {}", index).into());
            let new_vec = (&*todos)
                .iter()
                .enumerate()
                .map(|(i, t)| {
                    let mut t = t.clone();
                    if i == index {
                        t.complete = !t.complete;
                        // log::info!("Toggled {:?}", t);
                    }

                    t
                })
                .collect::<Vec<_>>();
            todos.set(new_vec);
        })
    };
    let on_form_submit = {
        let form_ref = form_ref.clone();
        move |e: FocusEvent| {
            e.prevent_default();
            if let Some(form) = form_ref.cast::<HtmlFormElement>() {
                let form_data = FormData::new_with_form(&form).unwrap();
                let value = form_data.get("text").as_string().unwrap();
                add_todo(value);
                form.reset();
            }
        }
    };

    html! {
        <div>
            <form ref={form_ref} onsubmit={on_form_submit}>
                <input name="text" />
                <button type="submit">{ "Create" }</button>
            </form>
            <div>
                { (&*todos)
                    .iter()
                    .enumerate()
                    .map(|(index, todo)| {
                        html! {
                            <TodoItem
                                {index}
                                on_toggle_todo={on_toggle_todo.clone()}
                                todo={todo.clone()}
                            />
                        }
                    })
                    .collect::<Html>() }
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct TodoItemProps {
    todo: Todo,
    index: usize,
    on_toggle_todo: Callback<usize>,
}

#[function_component(TodoItem)]
fn todo_item(props: &TodoItemProps) -> Html {
    let todo = props.todo.clone();
    let oninput = {
        let on_toggle_todo = props.on_toggle_todo.clone();
        let index = props.index;
        Callback::from(move |_| on_toggle_todo.emit(index))
    };

    html! {
        <div>
          <span>{todo.text}</span>
          <input type="checkbox" checked={todo.complete} {oninput} />
        </div>
    }
}
