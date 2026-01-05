use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[function_component(Register)]
pub fn register() -> Html {
    let username = use_state(|| "".to_string());
    let email = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());

    let onsubmit = {
        let username = username.clone();
        let email = email.clone();
        let password = password.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            log::info!("Registering {} / {} / {}", *username, *email, *password);
        })
    };

    html! {
        <div class="register-page">
            <h2>{ "Register" }</h2>
            <form {onsubmit}>
                <input 
                    type="text" 
                    placeholder="Username" 
                    value={(*username).clone()} 
                    oninput={let u = username.clone(); Callback::from(move |e: InputEvent| u.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} 
                />
                <input 
                    type="email" 
                    placeholder="Email" 
                    value={(*email).clone()} 
                    oninput={let em = email.clone(); Callback::from(move |e: InputEvent| em.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} 
                />
                <input 
                    type="password" 
                    placeholder="Password" 
                    value={(*password).clone()} 
                    oninput={let p = password.clone(); Callback::from(move |e: InputEvent| p.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} 
                />
                <button type="submit">{ "Register" }</button>
            </form>
            <p>{ "Already have an account? " }<Link<Route> to={Route::Login}>{ "Login here" }</Link<Route>></p>
        </div>
    }
}
