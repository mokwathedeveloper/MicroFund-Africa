use yew::prelude::*;
use crate::services::api::{get, post};
use crate::services::storage::{get_cache, set_cache};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct Loan {
    pub id: Uuid,
    pub amount: f64,
    pub status: String,
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
struct DepositRequest { amount: f64 }

#[derive(Serialize)]
struct RepayRequest { loan_id: Uuid }

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct MarketplaceLoan {
    pub id: Uuid,
    pub borrower_username: String,
    pub amount: f64,
    pub description: Option<String>,
}

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let loans = use_state(|| get_cache::<Vec<Loan>>("cache_loans").unwrap_or_default());
    let marketplace = use_state(|| Vec::<MarketplaceLoan>::new());
    let savings = use_state(|| get_cache::<Vec<Savings>>("cache_savings").unwrap_or_default());
    
    let loan_amount = use_state(|| 0.0);
    let loan_desc = use_state(|| "".to_string());
    let savings_goal = use_state(|| "".to_string());

    let fetch_data = {
        let loans = loans.clone();
        let savings = savings.clone();
        let marketplace = marketplace.clone();
        Callback::from(move |_| {
            let loans = loans.clone();
            let savings = savings.clone();
            let marketplace = marketplace.clone();
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
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let amount_val = *amount;
            let desc_val = (*desc).clone();
            let fetch_data = fetch_data.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let _: Result<Uuid, String> = post("/loans", &CreateLoanRequest { amount: amount_val, description: desc_val }).await;
                fetch_data.emit(());
            });
        })
    };

    let on_savings_submit = {
        let goal = savings_goal.clone();
        let fetch_data = fetch_data.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let goal_val = (*goal).clone();
            let fetch_data = fetch_data.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let _: Result<Uuid, String> = post("/savings", &CreateSavingsRequest { goal_name: goal_val }).await;
                fetch_data.emit(());
            });
        })
    };

    let deposit = |id: Uuid| {
        let fetch_data = fetch_data.clone();
        Callback::from(move |_| {
            let fetch_data = fetch_data.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let _: Result<String, String> = post(&format!("/savings/{}/deposit", id), &DepositRequest { amount: 10.0 }).await;
                fetch_data.emit(());
            });
        })
    };

    let repay = |id: Uuid| {
        let fetch_data = fetch_data.clone();
        Callback::from(move |_| {
            let fetch_data = fetch_data.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let _: Result<String, String> = post("/loans/repay", &RepayRequest { loan_id: id }).await;
                fetch_data.emit(());
            });
        })
    };

    let fund_loan = |id: Uuid| {
        let fetch_data = fetch_data.clone();
        Callback::from(move |_| {
            let fetch_data = fetch_data.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let _: Result<String, String> = post(&format!("/loans/{}/fund", id), &()).await;
                fetch_data.emit(());
            });
        })
    };

    let total_borrowed: f64 = loans.iter().map(|l| l.amount).sum();
    let total_saved: f64 = savings.iter().map(|s| s.amount).sum();

    html! {
        <div class="dashboard-container" style="padding: 0 1rem;">
            <div class="summary-bar">
                <div class="summary-item">
                    <h4>{ "Total Impact" }</h4>
                    <p>{ format!("${:.2}", total_borrowed + total_saved) }</p>
                </div>
                <div class="summary-item">
                    <h4>{ "Total Saved" }</h4>
                    <p>{ format!("${:.2}", total_saved) }</p>
                </div>
                <div class="summary-item">
                    <h4>{ "Marketplace" }</h4>
                    <p>{ marketplace.len() }</p>
                </div>
            </div>

            <div class="dashboard-grid" style="grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));">
                <section class="section-card">
                    <h3>{ "P2P Marketplace" }</h3>
                    <p style="font-size: 0.9rem; color: #7f8c8d;">{ "Fund a loan to help a neighbor and earn reputation." }</p>
                    <div class="marketplace-list">
                        { for marketplace.iter().map(|m| html! {
                            <div class="stat-card" style="text-align: left; display: flex; justify-content: space-between; align-items: center; border-left-color: #3498db;">
                                <div>
                                    <p style="margin: 0; font-weight: bold;">{ format!("${:.2}", m.amount) }</p>
                                    <p style="margin: 0.2rem 0; font-size: 0.8rem;">{ format!("By: @{}", m.borrower_username) }</p>
                                    <p style="margin: 0; font-size: 0.8rem; color: #7f8c8d;">{ m.description.clone().unwrap_or_default() }</p>
                                </div>
                                <button onclick={fund_loan(m.id)} class="btn" style="width: auto; font-size: 0.8rem; background: #3498db;">{ "Fund" }</button>
                            </div>
                        })}
                    </div>
                </section>

                <section class="section-card">
                    <h3>{ "Microloans" }</h3>
                    <form onsubmit={on_loan_submit} style="margin-bottom: 1.5rem;">
                        <input type="number" placeholder="Amount ($)" oninput={let a = loan_amount.clone(); Callback::from(move |e: InputEvent| a.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().unwrap_or(0.0)))} />
                        <input type="text" placeholder="Purpose (e.g. Seeds, Repair)" oninput={let d = loan_desc.clone(); Callback::from(move |e: InputEvent| d.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
                        <button type="submit">{ "Request Loan" }</button>
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
                                        html! { <button onclick={repay(loan.id)} class="btn-secondary" style="width: auto; font-size: 0.8rem;">{ "Repay" }</button> }
                                    } else { html! {} }}
                                </div>
                            }
                        })}
                    </div>
                </section>

                <section class="section-card">
                    <h3>{ "Savings Goals" }</h3>
                    <form onsubmit={on_savings_submit} style="margin-bottom: 1.5rem;">
                        <input type="text" placeholder="Goal Name (e.g. School Fees)" oninput={let g = savings_goal.clone(); Callback::from(move |e: InputEvent| g.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
                        <button type="submit" class="btn-secondary">{ "Create Goal" }</button>
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
