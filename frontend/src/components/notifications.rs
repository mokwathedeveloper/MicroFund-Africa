use yew::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum NotificationType {
    Success,
    Error,
    Info,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Notification {
    pub id: usize,
    pub message: String,
    pub n_type: NotificationType,
}

#[derive(Properties, PartialEq)]
pub struct NotificationProps {
    pub notifications: Vec<Notification>,
    pub on_close: Callback<usize>,
}

#[function_component(NotificationArea)]
pub fn notification_area(props: &NotificationProps) -> Html {
    html! {
        <div style="position: fixed; top: 20px; right: 20px; z-index: 9999; display: flex; flex-direction: column; gap: 10px;">
            { for props.notifications.iter().map(|n| {
                let color = match n.n_type {
                    NotificationType::Success => "#2ecc71",
                    NotificationType::Error => "#e74c3c",
                    NotificationType::Info => "#3498db",
                };
                let id = n.id;
                let on_close = props.on_close.clone();
                html! {
                    <div style={format!("background: {}; color: white; padding: 1rem; border-radius: 4px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); min-width: 250px; display: flex; justify-content: space-between; align-items: center;", color)}>
                        <span>{ &n.message }</span>
                        <button onclick={move |_| on_close.emit(id)} style="background: none; border: none; color: white; cursor: pointer; font-size: 1.2rem; font-weight: bold;">{ "×" }</button>
                    </div>
                }
            })}
        </div>
    }
}
