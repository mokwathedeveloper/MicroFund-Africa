use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;
use crate::services::api::post;
use crate::services::storage::set_token;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;

#[derive(Serialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct AuthResponse {
    token: String,
    user_id: String,
}

#[function_component(Login)]
pub fn login() -> Html {
    let navigator = use_navigator().unwrap();
    let username = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let error = use_state(|| None::<String>);

    let onsubmit = {
        let username = username.clone();
        let password = password.clone();
        let error = error.clone();
        let navigator = navigator.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username_val = (*username).clone();
            let password_val = (*password).clone();
            let error = error.clone();
            let navigator = navigator.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let res: Result<AuthResponse, String> = post("/auth/login", &LoginRequest {
                    username: username_val,
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

    let login_demo = {
        let username = username.clone();
        let password = password.clone();
        Callback::from(move |_| {
            username.set("demo_user".to_string());
            password.set("password123".to_string());
        })
    };

    html! {
        <div class="login-page">
            <h2>{ "Login" }</h2>
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
                    type="password" 
                    placeholder="Password" 
                    value={(*password).clone()} 
                    oninput={let p = password.clone(); Callback::from(move |e: InputEvent| p.set(e.target_unchecked_into::<HtmlInputElement>().value()))} 
                />
                <button type="submit">{ "Login" }</button>
            </form>
            <button onclick={login_demo} style="background: #7f8c8d; margin-top: 1rem;">{ "Use Demo Account" }</button>
            <p>{ "Don't have an account? " }<Link<Route> to={Route::Register}>{ "Register here" }</Link<Route>></p>
        </div>
    }
}