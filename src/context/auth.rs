// auth.rs
use yew::prelude::*;

pub type AuthCtx = UseStateHandle<Option<String>>; // lưu token (hoặc sub)

#[derive(Properties, PartialEq)]
pub struct AuthProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(AuthProvider)]
pub fn auth_provider(props: &AuthProviderProps) -> Html {
    let token = use_state(|| None::<String>);

    // lúc app khởi động: load từ localStorage vào context
    {
        let token = token.clone();
        use_effect(move || {
            if let Ok(Some(storage)) = web_sys::window().unwrap().local_storage() {
                if let Ok(Some(t)) = storage.get_item("auth_token") {
                    token.set(Some(t));
                }
            }
            || ()
        });
    }

    html! {
        <ContextProvider<AuthCtx> context={token}>
            { for props.children.iter() }
        </ContextProvider<AuthCtx>>
    }
}
