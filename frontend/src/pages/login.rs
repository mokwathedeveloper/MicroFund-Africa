use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());

    let onsubmit = {
        let username = username.clone();
        let password = password.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            // Handle login logic here
            log::info!("Logging in with {} / {}", *username, *password);
        })
    };

    html! {
        <div class="login-page">
            <h2>{ "Login" }</h2>
            <form {onsubmit}>
                <input 
                    type="text" 
                    placeholder="Username" 
                    value={(*username).clone()} 
                    oninput={let u = username.clone(); Callback::from(move |e: InputEvent| u.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} 
                />
                <input 
                    type="password" 
                    placeholder="Password" 
                    value={(*password).clone()} 
                    oninput={let p = password.clone(); Callback::from(move |e: InputEvent| p.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} 
                />
                <button type="submit">{ "Login" }</button>
            </form>
            <p>{ "Don't have an account? " }<Link<Route> to={Route::Register}>{ "Register here" }</Link<Route>></p>
        </div>
    }
}
