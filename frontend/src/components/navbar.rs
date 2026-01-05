use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;
use crate::services::storage::{get_token, remove_token};

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let navigator = use_navigator().unwrap();
    let token = get_token();

    let logout = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            remove_token();
            navigator.push(&Route::Home);
        })
    };

    html! {
        <nav style="background: #34495e; padding: 1rem; color: white; display: flex; justify-content: space-between; align-items: center;">
            <div style="font-weight: bold; font-size: 1.2rem;">
                <Link<Route> to={Route::Home} style="color: white; text-decoration: none;">{ "MicroFund Africa" }</Link<Route>>
            </div>
            <div>
                { if token.is_some() {
                    html! {
                        <>
                            <Link<Route> to={Route::Dashboard} style="color: white; margin-right: 1rem; text-decoration: none;">{ "Dashboard" }</Link<Route>>
                            <button onclick={logout} style="background: none; border: 1px solid white; color: white; padding: 0.3rem 0.8rem; border-radius: 4px; cursor: pointer;">{ "Logout" }</button>
                        </>
                    }
                } else {
                    html! {
                        <>
                            <Link<Route> to={Route::Login} style="color: white; margin-right: 1rem; text-decoration: none;">{ "Login" }</Link<Route>>
                            <Link<Route> to={Route::Register} style="color: white; text-decoration: none; border: 1px solid white; padding: 0.3rem 0.8rem; border-radius: 4px;">{ "Get Started" }</Link<Route>>
                        </>
                    }
                }}
            </div>
        </nav>
    }
}
