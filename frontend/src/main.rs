use yew::prelude::*;
use yew_router::prelude::*;

mod app_context;
mod components;
mod pages;
mod services;
mod utils;

use app_context::AppContextProvider;
use components::navbar::Navbar;
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

#[function_component(App)]
fn app() -> Html {
    html! {
        <AppContextProvider>
            <BrowserRouter>
                <Navbar />
                <main style="padding-top: 2rem;">
                    <Switch<Route> render={switch} />
                </main>
            </BrowserRouter>
        </AppContextProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
