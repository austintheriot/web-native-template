use ui::{App, AppProps};

fn main() {
    let app_props = AppProps {
        title: String::from("Native App"),
    };
    yew::Renderer::<App>::with_props(app_props).render();
}
