use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;
mod services;
mod utils;

use pages::home::Home;
use pages::login::Login;
use pages::register::Register;
use pages::dashboard::Dashboard;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/dashboard")]
    Dashboard,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Login => html! { <Login /> },
        Route::Register => html! { <Register /> },
        Route::Dashboard => html! { <Dashboard /> },
        Route::NotFound => html! { <h1>{ "404 - Not Found" }</h1> },
    }
}

use components::navbar::Navbar;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Navbar />
            <main style="padding-top: 2rem;">
                <Switch<Route> render={switch} />
            </main>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
