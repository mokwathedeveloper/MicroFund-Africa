use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;
use crate::services::api::post;
use crate::services::storage::set_token;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;

#[derive(Serialize)]
struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct AuthResponse {
    token: String,
    user_id: String,
}

#[function_component(Register)]
pub fn register() -> Html {
    let navigator = use_navigator().unwrap();
    let username = use_state(|| "".to_string());
    let email = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let error = use_state(|| None::<String>);

    let onsubmit = {
        let username = username.clone();
        let email = email.clone();
        let password = password.clone();
        let error = error.clone();
        let navigator = navigator.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username_val = (*username).clone();
            let email_val = (*email).clone();
            let password_val = (*password).clone();
            let error = error.clone();
            let navigator = navigator.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if password_val.len() < 6 {
                    error.set(Some("Password must be at least 6 characters long".to_string()));
                    return;
                }
                if !email_val.contains('@') {
                    error.set(Some("Please enter a valid email address".to_string()));
                    return;
                }

                let res: Result<AuthResponse, String> = post("/auth/register", &RegisterRequest {
                    username: username_val,
                    email: email_val,
                    password: password_val,
                }).await;

                match res {
                    Ok(auth) => {
                        set_token(&auth.token);
                        navigator.push(&Route::Dashboard);
                    }
                    Err(e) => error.set(Some(e)),
                }
            });
        })
    };

    html! {
        <div class="register-page">
            <h2>{ "Create Account" }</h2>
            <p>{ "Join MicroFund Africa and start your journey." }</p>
            { if let Some(err) = &*error {
                html! { <p style="color: red;">{ err }</p> }
            } else {
                html! {}
            }}
            <form {onsubmit}>
                <input 
                    type="text" 
                    placeholder="Username" 
                    value={(*username).clone()} 
                    oninput={let u = username.clone(); Callback::from(move |e: InputEvent| u.set(e.target_unchecked_into::<HtmlInputElement>().value()))} 
                />
                <input 
                    type="email" 
                    placeholder="Email" 
                    value={(*email).clone()} 
                    oninput={let em = email.clone(); Callback::from(move |e: InputEvent| em.set(e.target_unchecked_into::<HtmlInputElement>().value()))} 
                />
                <input 
                    type="password" 
                    placeholder="Password" 
                    value={(*password).clone()} 
                    oninput={let p = password.clone(); Callback::from(move |e: InputEvent| p.set(e.target_unchecked_into::<HtmlInputElement>().value()))} 
                />
                <button type="submit">{ "Register" }</button>
            </form>
            <p>{ "Already have an account? " }<Link<Route> to={Route::Login}>{ "Login here" }</Link<Route>></p>
        </div>
    }
}