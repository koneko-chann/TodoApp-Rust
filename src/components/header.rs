use std::ops::Not;

use crate::components::button::Button;
use jsonwebtoken::dangerous::insecure_decode;
use web_sys::window;
use yew::prelude::*;
use yew_router::{hooks::use_navigator, navigator};
#[derive(Debug, serde::Deserialize)]
pub struct Claims {
    pub(crate) sub: Option<String>,
}
#[derive(Properties, PartialEq, Clone)]
pub struct HeaderProps {
    #[prop_or_default]
    pub class: String, // class CSS cho header
    #[prop_or_default]
    pub disabled: bool, // disable nút Login
    #[prop_or_default]
    pub is_logged_in: bool,
}
pub fn is_logged_in() -> bool {
    //check in local storage or context
    if let Ok(Some(storage)) = window().unwrap().local_storage() {
        let auth_token = storage.get_item("auth_token").unwrap();
        return match auth_token {
            Some(_) => true,
            None => false,
        };
    }
    false
}
pub fn logout() {
    if let Ok(Some(storage)) = window().unwrap().local_storage() {
        let _ = storage.remove_item("auth_token");
    }
}
#[function_component(Header)]
pub fn header( props:  &HeaderProps) -> Html {
let is_logged_in = use_state(|| is_logged_in());
    let mut button_text = "Login".to_string();

    if let Ok(Some(storage)) = window().unwrap().local_storage() {
        if let Ok(Some(token)) = storage.get_item("auth_token") {
            if let Ok(data) = insecure_decode::<Claims>(&token) {
                if let Some(user_id) = data.claims.sub {
                    button_text = user_id;
                }
            } else {
                web_sys::console::log_1(&"Failed to decode token".into());
            }
        }
    }

let on_login_click = {
    let navigator = use_navigator();
    let is_logged_in = is_logged_in.clone();
    Callback::from(move |_| {
        if *is_logged_in {
            logout();
            is_logged_in.set(false);
            
        } else {
            if let Some(navigator) = navigator.clone() {
                navigator.push(&crate::Route::Login);
            }
            is_logged_in.set(true);
        }
    })
};


    html! {
        <header class={props.class.clone()}>
            <div class="flex justify-end">
                <Button
                    disabled={props.disabled}
                    onclick={on_login_click}
                >
                    {if *is_logged_in { html!(button_text.clone()) } else { html!("Login") }}
                </Button>
            </div>
        </header>
    }
}