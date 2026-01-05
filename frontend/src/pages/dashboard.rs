use yew::prelude::*;
use crate::services::api::{get, post};
use crate::services::storage::{get_cache, set_cache};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::app_context::AppContext;
use crate::utils::i18n::t;
use crate::components::notifications::NotificationType;

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct Loan {
    pub id: Uuid,
    pub amount: f64,
    pub status: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct UserProfile {
    pub username: String,
    pub reputation_score: i32,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct MarketplaceLoan {
    pub id: Uuid,
    pub borrower_username: String,
    pub amount: f64,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct Savings {
    pub id: Uuid,
    pub amount: f64,
    pub goal_name: Option<String>,
}

#[derive(Serialize)]
struct CreateLoanRequest { amount: f64, description: String }

#[derive(Serialize)]
struct CreateSavingsRequest { goal_name: String }

#[derive(Serialize)]
struct DepositRequest { amount: f64, phone_number: Option<String> }

#[derive(Serialize)]
struct RepayRequest { loan_id: Uuid }

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let context = use_context::<AppContext>().unwrap();
    let loans = use_state(|| get_cache::<Vec<Loan>>("cache_loans").unwrap_or_default());
    let marketplace = use_state(|| Vec::<MarketplaceLoan>::new());
    let savings = use_state(|| get_cache::<Vec<Savings>>("cache_savings").unwrap_or_default());
    let profile = use_state(|| get_cache::<UserProfile>("cache_profile").unwrap_or(UserProfile { username: "".to_string(), reputation_score: 100 }));
    
    let loan_amount = use_state(|| 0.0);
    let loan_desc = use_state(|| "".to_string());
    let savings_goal = use_state(|| "".to_string());
    let phone_number = use_state(|| "".to_string());

    let fetch_data = {
        let loans = loans.clone();
        let savings = savings.clone();
        let marketplace = marketplace.clone();
        let profile = profile.clone();
        Callback::from(move |_| {
            let loans = loans.clone();
            let savings = savings.clone();
            let marketplace = marketplace.clone();
            let profile = profile.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(data) = get::<Vec<Loan>>("/loans").await {
                    set_cache("cache_loans", &data);
                    loans.set(data);
                }
                if let Ok(data) = get::<Vec<Savings>>("/savings").await {
                    set_cache("cache_savings", &data);
                    savings.set(data);
                }
                if let Ok(data) = get::<Vec<MarketplaceLoan>>("/loans/marketplace").await {
                    marketplace.set(data);
                }
                if let Ok(data) = get::<UserProfile>("/auth/profile").await {
                    set_cache("cache_profile", &data);
                    profile.set(data);
                }
            });
        })
    };

    {
        let fetch_data = fetch_data.clone();
        use_effect_with((), move |_| {
            fetch_data.emit(());
            || ()
        });
    }

    let on_loan_submit = {
        let amount = loan_amount.clone();
        let desc = loan_desc.clone();
        let fetch_data = fetch_data.clone();
        let context = context.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let amount_val = *amount;
            let desc_val = (*desc).clone();
            let fetch_data = fetch_data.clone();
            let context = context.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match post::<_, Uuid>("/loans", &CreateLoanRequest { amount: amount_val, description: desc_val }).await {
                    Ok(_) => {
                        context.add_notification.emit(("Loan requested successfully!".to_string(), NotificationType::Success));
                        fetch_data.emit(());
                    }
                    Err(e) => context.add_notification.emit((format!("Error: {}", e), NotificationType::Error)),
                }
            });
        })
    };

    let on_savings_submit = {
        let goal = savings_goal.clone();
        let fetch_data = fetch_data.clone();
        let context = context.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let goal_val = (*goal).clone();
            let fetch_data = fetch_data.clone();
            let context = context.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match post::<_, Uuid>("/savings", &CreateSavingsRequest { goal_name: goal_val }).await {
                    Ok(_) => {
                        context.add_notification.emit(("Savings goal created!".to_string(), NotificationType::Success));
                        fetch_data.emit(());
                    }
                    Err(e) => context.add_notification.emit((format!("Error: {}", e), NotificationType::Error)),
                }
            });
        })
    };

    let deposit = |id: Uuid| {
        let fetch_data = fetch_data.clone();
        let phone = phone_number.clone();
        let context = context.clone();
        Callback::from(move |_| {
            let fetch_data = fetch_data.clone();
            let phone_val = (*phone).clone();
            let context = context.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match post::<_, String>(&format!("/savings/{}/deposit", id), &DepositRequest { 
                    amount: 10.0, 
                    phone_number: if phone_val.is_empty() { None } else { Some(phone_val) }
                }).await {
                    Ok(_) => {
                        context.add_notification.emit(("Deposit initiated! Check your phone for STK push.", NotificationType::Info));
                        fetch_data.emit(());
                    }
                    Err(e) => context.add_notification.emit((format!("Error: {}", e), NotificationType::Error)),
                }
            });
        })
    };

    let repay = |id: Uuid| {
        let fetch_data = fetch_data.clone();
        let context = context.clone();
        Callback::from(move |_| {
            let fetch_data = fetch_data.clone();
            let context = context.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match post::<_, String>("/loans/repay", &RepayRequest { loan_id: id }).await {
                    Ok(_) => {
                        context.add_notification.emit(("Loan repaid successfully! Your Trust Score increased.", NotificationType::Success));
                        fetch_data.emit(());
                    }
                    Err(e) => context.add_notification.emit((format!("Error: {}", e), NotificationType::Error)),
                }
            });
        })
    };

    let fund_loan = |id: Uuid| {
        let fetch_data = fetch_data.clone();
        let context = context.clone();
        Callback::from(move |_| {
            let fetch_data = fetch_data.clone();
            let context = context.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match post::<_, String>(&format!("/loans/{}/fund", id), &()).await {
                    Ok(_) => {
                        context.add_notification.emit(("You funded a neighbor's loan! Impact increased.", NotificationType::Success));
                        fetch_data.emit(());
                    }
                    Err(e) => context.add_notification.emit((format!("Error: {}", e), NotificationType::Error)),
                }
            });
        })
    };

    let total_borrowed: f64 = loans.iter().map(|l| l.amount).sum();
    let total_saved: f64 = savings.iter().map(|s| s.amount).sum();

    // Simple SVG Visualization for Savings vs Borrowing
    let chart_width = 200;
    let chart_height = 20;
    let max_val = if total_borrowed > total_saved { total_borrowed } else { total_saved };
    let borrow_width = if max_val > 0.0 { (total_borrowed / max_val) * chart_width as f64 } else { 0.0 };
    let savings_width = if max_val > 0.0 { (total_saved / max_val) * chart_width as f64 } else { 0.0 };

    html! {
        <div class="dashboard-container" style="padding: 0 1rem;">
            <div class="summary-bar">
                <div class="summary-item">
                    <h4>{ "Welcome" }</h4>
                    <p>{ format!("@{}", profile.username) }</p>
                </div>
                <div class="summary-item">
                    <h4>{ t("trust_score", &context.lang) }</h4>
                    <p style="color: #2ecc71;">{ profile.reputation_score }</p>
                </div>
                <div class="summary-item">
                    <h4>{ t("total_impact", &context.lang) }</h4>
                    <p>{ format!("${:.2}", total_borrowed + total_saved) }</p>
                </div>
                <div class="summary-item">
                    <h4>{ "Balance View" }</h4>
                    <svg width={chart_width.to_string()} height={(chart_height * 2 + 5).to_string()} style="margin-top: 5px;">
                        <rect x="0" y="0" width={savings_width.to_string()} height={chart_height.to_string()} fill="#2ecc71" rx="2" />
                        <rect x="0" y={(chart_height + 5).to_string()} width={borrow_width.to_string()} height={chart_height.to_string()} fill="#e74c3c" rx="2" />
                    </svg>
                    <div style="display: flex; gap: 10px; font-size: 0.7rem; margin-top: 2px;">
                        <span><i style="background: #2ecc71; width: 8px; height: 8px; display: inline-block;"></i>{ " Saved" }</span>
                        <span><i style="background: #e74c3c; width: 8px; height: 8px; display: inline-block;"></i>{ " Debt" }</span>
                    </div>
                </div>
            </div>

            <div class="dashboard-grid" style="grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));">
                <section class="section-card">
                    <h3>{ t("marketplace", &context.lang) }</h3>
                    <p style="font-size: 0.9rem; color: #7f8c8d;">{ "Fund a loan to help a neighbor and earn reputation." }</p>
                    <div class="marketplace-list">
                        { for marketplace.iter().map(|m| html! {
                            <div class="stat-card" style="text-align: left; display: flex; justify-content: space-between; align-items: center; border-left-color: #3498db;">
                                <div>
                                    <p style="margin: 0; font-weight: bold;">{ format!("${:.2}", m.amount) }</p>
                                    <p style="margin: 0.2rem 0; font-size: 0.8rem;">{ format!("By: @{}", m.borrower_username) }</p>
                                    <p style="margin: 0; font-size: 0.8rem; color: #7f8c8d;">{ m.description.clone().unwrap_or_default() }</p>
                                </div>
                                <button onclick={fund_loan(m.id)} class="btn" style="width: auto; font-size: 0.8rem; background: #3498db;">{ t("fund", &context.lang) }</button>
                            </div>
                        })}
                    </div>
                </section>

                <section class="section-card">
                    <h3>{ t("microloans", &context.lang) }</h3>
                    <form onsubmit={on_loan_submit} style="margin-bottom: 1.5rem;">
                        <input type="number" placeholder="Amount ($)" oninput={let a = loan_amount.clone(); Callback::from(move |e: InputEvent| a.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().unwrap_or(0.0)))} />
                        <input type="text" placeholder="Purpose (e.g. Seeds, Repair)" oninput={let d = loan_desc.clone(); Callback::from(move |e: InputEvent| d.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
                        <button type="submit">{ t("request_loan", &context.lang) }</button>
                    </form>
                    
                    <div class="loan-list">
                        { for loans.iter().map(|loan| {
                            let status_class = match loan.status.as_str() {
                                "pending" => "status-pending",
                                "approved" => "status-approved",
                                "repaid" => "status-repaid",
                                _ => "",
                            };
                            html! {
                                <div class="stat-card" style="text-align: left; display: flex; justify-content: space-between; align-items: center; border-left-color: #e74c3c;">
                                    <div>
                                        <p style="margin: 0; font-weight: bold;">{ format!("${:.2}", loan.amount) }</p>
                                        <p style="margin: 0.2rem 0; font-size: 0.8rem; color: #7f8c8d;">{ loan.description.clone().unwrap_or_default() }</p>
                                        <span class={classes!("status-badge", status_class)}>{ &loan.status }</span>
                                    </div>
                                    { if loan.status != "repaid" {
                                        html! { <button onclick={repay(loan.id)} class="btn-secondary" style="width: auto; font-size: 0.8rem;">{ t("repay", &context.lang) }</button> }
                                    } else { html! {} }}
                                </div>
                            }
                        })}
                    </div>
                </section>

                <section class="section-card">
                    <h3>{ t("savings_goals", &context.lang) }</h3>
                    <div style="margin-bottom: 1rem;">
                        <input type="text" placeholder="M-Pesa Phone (e.g. 254712...)" 
                            oninput={let p = phone_number.clone(); Callback::from(move |e: InputEvent| p.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
                    </div>
                    <form onsubmit={on_savings_submit} style="margin-bottom: 1.5rem;">
                        <input type="text" placeholder="Goal Name (e.g. School Fees)" oninput={let g = savings_goal.clone(); Callback::from(move |e: InputEvent| g.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
                        <button type="submit" class="btn-secondary">{ t("create_goal", &context.lang) }</button>
                    </form>

                    <div class="savings-list">
                        { for savings.iter().map(|s| html! {
                            <div class="stat-card" style="text-align: left; display: flex; justify-content: space-between; align-items: center; border-left-color: #2ecc71;">
                                <div>
                                    <p style="margin: 0; font-weight: bold;">{ format!("${:.2}", s.amount) }</p>
                                    <p style="margin: 0.2rem 0; font-size: 0.9rem;">{ s.goal_name.clone().unwrap_or_default() }</p>
                                </div>
                                <button onclick={deposit(s.id)} class="btn" style="width: auto; font-size: 0.8rem;">{ "+$10" }</button>
                            </div>
                        })}
                    </div>
                </section>
            </div>
        </div>
    }
}