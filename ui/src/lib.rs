use yew::{Properties, function_component, use_state, html, Html};

#[derive(Properties, PartialEq)]
pub struct AppProps {
    pub title: String,
}

#[function_component]
pub fn App(app_props: &AppProps) -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <h1>{&app_props.title}</h1>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}
