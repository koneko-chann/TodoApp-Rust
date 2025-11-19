use jsonwebtoken::dangerous::insecure_decode;
use web_sys::console::log_1;
use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;
mod context;
mod components;
mod containers;
mod screens;
mod types;
mod utils;  
use crate::context::auth::AuthProvider;
use crate::screens::counter::Counter;
use crate::screens::home::Home;
use crate::screens::not_found::NotFound;
use crate::screens::register::RegisterScreen;
use crate::screens::todo::TodoScreen;
use crate::components::header::{ Header};
use crate::screens::login::LoginScreen;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Counter,
    #[at("/*path")]
    NotFound,
    #[at("/home")]
    Home,
    #[at("/todo")]
    Todo,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
}
fn is_logged_in() -> bool {
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
#[function_component]
fn App() -> Html {
    let on_login = Callback::from(|e:MouseEvent| {
    //       let mut button_text = "Login".to_string();

    // if let Ok(Some(storage)) = window().unwrap().local_storage() {
    //     if let Ok(Some(token)) = storage.get_item("auth_token") {
    //         if let Ok(data) = insecure_decode::<Claims>(&token) {
    //             if let Some(user_id) = data.claims.sub {
    //                 button_text = user_id;
    //             }
    //         } else {
    //             web_sys::console::log_1(&"Failed to decode token".into());
    //         }
    //     }
    // }

    });

    fn switch(routes: Route) -> Html {
        match routes {
            Route::Counter => html! { <Redirect<Route> to={Route::Todo} /> },
            Route::NotFound => html! { <NotFound /> },
            Route::Home => html! { <Redirect<Route> to={Route::Todo} /> },
            Route::Todo => html! { <TodoScreen />    },
            Route::Login => html! { <LoginScreen /> },
            Route::Register => html! { <RegisterScreen /> },
        }
        
    }

    html! {
            <BrowserRouter>
                <Header class="bg-gray-900 p-4" disabled={false} />
                <Switch<Route> render={switch} />
            </BrowserRouter>
    }
}


fn main() {
    
    yew::Renderer::<App>::new().render();
}
