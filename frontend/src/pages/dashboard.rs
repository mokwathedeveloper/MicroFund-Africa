use yew::prelude::*;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    html! {
        <div class="dashboard">
            <h2>{ "User Dashboard" }</h2>
            <div class="stats">
                <div class="stat-card">
                    <h3>{ "Active Loans" }</h3>
                    <p>{ "0" }</p>
                </div>
                <div class="stat-card">
                    <h3>{ "Total Repaid" }</h3>
                    <p>{ "$0.00" }</p>
                </div>
            </div>
            <h3>{ "Recent Activity" }</h3>
            <p>{ "No activity found." }</p>
        </div>
    }
}
