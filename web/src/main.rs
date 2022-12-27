use ui::{App, AppProps};

fn main() {
    let app_props = AppProps {
        title: String::from("Web App"),
    };
    yew::Renderer::<App>::with_props(app_props).render();
}
