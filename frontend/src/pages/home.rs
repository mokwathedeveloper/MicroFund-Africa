use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

use crate::app_context::AppContext;
use crate::utils::i18n::t;

#[function_component(Home)]
pub fn home() -> Html {
    let context = use_context::<AppContext>().unwrap();
    html! {
        <div class="home-container" style="max-width: 800px; margin: 0 auto; text-align: center;">
            <section class="hero" style="padding: 4rem 1rem;">
                <h1 style="font-size: 3rem; margin-bottom: 1rem;">{ t("welcome", &context.lang) }</h1>
                <p style="font-size: 1.2rem; color: #7f8c8d; margin-bottom: 2rem;">
                    { t("hero_sub", &context.lang) }
                </p>
                <div class="actions">
                    <Link<Route> to={Route::Register} classes="btn" style="padding: 1rem 2rem; font-size: 1.1rem;">{ t("get_started", &context.lang) }</Link<Route>>
                </div>
            </section>

            <section class="features" style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 2rem; padding: 2rem 1rem;">
                <div class="feature-card" style="background: white; padding: 2rem; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.05);">
                    <h3 style="color: #2ecc71;">{ "Fast & Easy" }</h3>
                    <p>{ "Apply for a microloan in minutes. No credit score required, just trust and community." }</p>
                </div>
                <div class="feature-card" style="background: white; padding: 2rem; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.05);">
                    <h3 style="color: #2ecc71;">{ "Transparent" }</h3>
                    <p>{ "All transactions are logged on the Solana blockchain, ensuring absolute transparency and security." }</p>
                </div>
                <div class="feature-card" style="background: white; padding: 2rem; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.05);">
                    <h3 style="color: #2ecc71;">{ "Community Driven" }</h3>
                    <p>{ "Support your neighbors and build a stronger local economy through peer-to-peer lending." }</p>
                </div>
            </section>

            <footer style="margin-top: 4rem; padding: 2rem; color: #bdc3c7;">
                <p>{ "© 2026 MicroFund Africa. Built with Rust for the Africa Hackathon." }</p>
            </footer>
        </div>
    }
}