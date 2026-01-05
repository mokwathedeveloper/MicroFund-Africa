use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="container">
            <h1>{ "MicroFund Africa" }</h1>
            <p>{ "Empowering the unbanked with microloans and secure savings." }</p>
            <div class="actions">
                <Link<Route> to={Route::Login} classes="btn">{ "Login" }</Link<Route>>
                <Link<Route> to={Route::Register} classes="btn btn-secondary">{ "Register" }</Link<Route>>
            </div>
        </div>
    }
}
