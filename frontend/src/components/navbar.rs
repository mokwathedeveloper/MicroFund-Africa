use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;
use crate::services::storage::{get_token, remove_token};

use crate::app_context::{AppContext, Theme};
use crate::utils::i18n::{Language, t};

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let navigator = use_navigator().unwrap();
    let token = get_token();
    let context = use_context::<AppContext>().unwrap();

    let logout = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            remove_token();
            navigator.push(&Route::Home);
        })
    };

    let toggle_lang = {
        let context = context.clone();
        Callback::from(move |_| {
            let new_lang = match context.lang {
                Language::English => Language::Swahili,
                Language::Swahili => Language::English,
            };
            context.set_lang.emit(new_lang);
        })
    };

    let toggle_theme = {
        let context = context.clone();
        Callback::from(move |_| {
            let new_theme = match context.theme {
                Theme::Light => Theme::Dark,
                Theme::Dark => Theme::Light,
            };
            context.set_theme.emit(new_theme);
        })
    };

    html! {
        <nav style="padding: 1rem; color: white; display: flex; justify-content: space-between; align-items: center;">
            <div style="font-weight: bold; font-size: 1.2rem; display: flex; align-items: center; gap: 10px;">
                <div style="width: 24px; height: 24px; background: #2ecc71; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: 0.8rem;">{ "M" }</div>
                <Link<Route> to={Route::Home} style="color: white; text-decoration: none;">{ "MicroFund Africa" }</Link<Route>>
            </div>
            <div style="display: flex; align-items: center; gap: 1rem;">
                <button onclick={toggle_theme} style="background: none; border: 1px solid #7f8c8d; color: #bdc3c7; padding: 0.2rem 0.5rem; border-radius: 4px; cursor: pointer; font-size: 0.8rem;">
                    { if let Theme::Light = context.theme { "Dark" } else { "Light" } }
                </button>
                <button onclick={toggle_lang} style="background: none; border: 1px solid #7f8c8d; color: #bdc3c7; padding: 0.2rem 0.5rem; border-radius: 4px; cursor: pointer; font-size: 0.8rem;">
                    { if let Language::English = context.lang { "Swahili" } else { "English" } }
                </button>
                { if token.is_some() {
                    html! {
                        <>
                            <Link<Route> to={Route::Dashboard} style="color: white; text-decoration: none;">{ t("dashboard", &context.lang) }</Link<Route>>
                            <button onclick={logout} style="background: none; border: 1px solid white; color: white; padding: 0.3rem 0.8rem; border-radius: 4px; cursor: pointer;">{ t("logout", &context.lang) }</button>
                        </>
                    }
                } else {
                    html! {
                        <>
                            <Link<Route> to={Route::Login} style="color: white; text-decoration: none;">{ t("login", &context.lang) }</Link<Route>>
                            <Link<Route> to={Route::Register} style="color: white; text-decoration: none; border: 1px solid white; padding: 0.3rem 0.8rem; border-radius: 4px;">{ t("get_started", &context.lang) }</Link<Route>>
                        </>
                    }
                }}
            </div>
        </nav>
    }
}
