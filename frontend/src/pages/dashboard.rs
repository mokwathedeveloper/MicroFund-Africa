use yew::prelude::*;
use crate::services::api::{get, post};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct Loan {
    pub id: Uuid,
    pub amount: f64,
    pub status: String,
    pub description: Option<String>,
}

#[derive(Serialize)]
struct CreateLoanRequest {
    amount: f64,
    description: String,
}

#[derive(Serialize)]
struct RepayRequest {
    loan_id: Uuid,
}

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let loans = use_state(|| Vec::<Loan>::new());
    let amount = use_state(|| 0.0);
    let description = use_state(|| "".to_string());

    let fetch_loans = {
        let loans = loans.clone();
        Callback::from(move |_| {
            let loans = loans.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(data) = get::<Vec<Loan>>("/loans").await {
                    loans.set(data);
                }
            });
        })
    };

    {
        let fetch_loans = fetch_loans.clone();
        use_effect_with((), move |_| {
            fetch_loans.emit(());
            || ()
        });
    }

    let onsubmit = {
        let amount = amount.clone();
        let description = description.clone();
        let fetch_loans = fetch_loans.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let amount_val = *amount;
            let desc_val = (*description).clone();
            let fetch_loans = fetch_loans.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let _: Result<Uuid, String> = post("/loans", &CreateLoanRequest {
                    amount: amount_val,
                    description: desc_val,
                }).await;
                fetch_loans.emit(());
            });
        })
    };

    let repay = |id: Uuid| {
        let fetch_loans = fetch_loans.clone();
        Callback::from(move |_| {
            let fetch_loans = fetch_loans.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let _: Result<String, String> = post("/loans/repay", &RepayRequest {
                    loan_id: id,
                }).await;
                fetch_loans.emit(());
            });
        })
    };

    html! {
        <div class="dashboard" style="max-width: 600px;">
            <h2>{ "User Dashboard" }</h2>
            
            <div class="loan-form" style="margin-bottom: 2rem; padding: 1rem; border: 1px solid #eee;">
                <h3>{ "Request a Microloan" }</h3>
                <form {onsubmit}>
                    <input type="number" placeholder="Amount" 
                        oninput={let a = amount.clone(); Callback::from(move |e: InputEvent| a.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().unwrap_or(0.0)))} />
                    <input type="text" placeholder="Purpose" 
                        oninput={let d = description.clone(); Callback::from(move |e: InputEvent| d.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))} />
                    <button type="submit">{ "Request Loan" }</button>
                </form>
            </div>

            <h3>{ "Your Loans" }</h3>
            <div class="loan-list">
                { for loans.iter().map(|loan| {
                    let status_class = match loan.status.as_str() {
                        "pending" => "status-pending",
                        "approved" => "status-approved",
                        "repaid" => "status-repaid",
                        _ => "",
                    };
                    html! {
                        <div class="stat-card" style="text-align: left; display: flex; justify-content: space-between; align-items: center;">
                            <div>
                                <p style="margin: 0; font-size: 1.2rem; font-weight: bold;">{ format!("${:.2}", loan.amount) }</p>
                                <p style="margin: 0.5rem 0; color: #7f8c8d;">{ loan.description.clone().unwrap_or_default() }</p>
                                <span class={classes!("status-badge", status_class)}>{ &loan.status }</span>
                            </div>
                            <div>
                                { if loan.status == "pending" || loan.status == "approved" {
                                    html! { <button onclick={repay(loan.id)} class="btn-secondary" style="width: auto; padding: 0.5rem 1rem;">{ "Repay" }</button> }
                                } else {
                                    html! {}
                                }}
                            </div>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}