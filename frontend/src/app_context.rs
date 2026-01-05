use yew::prelude::*;
use crate::utils::i18n::Language;
use crate::components::notifications::Notification;

#[derive(Clone, PartialEq)]
pub struct AppContext {
    pub lang: Language,
    pub set_lang: Callback<Language>,
    pub add_notification: Callback<(String, crate::components::notifications::NotificationType)>,
}

#[derive(Properties, PartialEq)]
pub struct AppContextProviderProps {
    pub children: Children,
}

#[function_component(AppContextProvider)]
pub fn app_context_provider(props: &AppContextProviderProps) -> Html {
    let lang = use_state(|| Language::English);
    let notifications = use_state(|| Vec::<Notification>::new());
    let next_id = use_state(|| 0);

    let set_lang = {
        let lang = lang.clone();
        Callback::from(move |l: Language| lang.set(l))
    };

    let add_notification = {
        let notifications = notifications.clone();
        let next_id = next_id.clone();
        Callback::from(move |(message, n_type): (String, crate::components::notifications::NotificationType)| {
            let mut new_notifs = (*notifications).clone();
            let id = *next_id;
            new_notifs.push(Notification { id, message, n_type });
            notifications.set(new_notifs);
            next_id.set(id + 1);
        })
    };

    let on_close_notification = {
        let notifications = notifications.clone();
        Callback::from(move |id: usize| {
            let new_notifs: Vec<_> = (*notifications).iter().filter(|n| n.id != id).cloned().collect();
            notifications.set(new_notifs);
        })
    };

    let context = AppContext {
        lang: (*lang),
        set_lang,
        add_notification,
    };

    html! {
        <ContextProvider<AppContext> context={context}>
            <crate::components::notifications::NotificationArea 
                notifications={(*notifications).clone()} 
                on_close={on_close_notification} 
            />
            { props.children.clone() }
        </ContextProvider<AppContext>>
    }
}
