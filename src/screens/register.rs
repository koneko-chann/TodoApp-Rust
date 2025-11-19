use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, HtmlInputElement};
use yew::{Callback, Html, TargetCast, function_component, html, use_state};
use yew_router::prelude::use_navigator;
use yew::events::{InputEvent, SubmitEvent};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[function_component(RegisterScreen)]
pub fn register_screen() -> Html {
    let email = use_state(|| String::new());
    let password = use_state(|| String::new());
    let loading = use_state(|| false);
    let error = use_state(|| Option::<String>::None);
    let success = use_state(|| Option::<String>::None);
    let navigator = use_navigator();

    // --- handlers ---

    let handle_email_change = {
        let email = email.clone();
        let error = error.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
            error.set(None);
        })
    };

    let handle_password_change = {
        let password = password.clone();
        let error = error.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
            error.set(None);
        })
    };

    let handle_submit = {
        let email = email.clone();
        let password = password.clone();
        let loading = loading.clone();
        let error = error.clone();
        let success = success.clone();
        let navigator = navigator.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let email_val = (*email).trim().to_string();
            let password_val = (*password).trim().to_string();

            if email_val.is_empty() || password_val.is_empty() {
                error.set(Some("Username and password must not be empty".into()));
                return;
            }

            loading.set(true);
            error.set(None);
            success.set(None);

            let loading = loading.clone();
            let error = error.clone();
            let success = success.clone();
            let navigator = navigator.clone();

            spawn_local(async move {
                let body = RegisterRequest {
                    username: email_val,
                    password: password_val,
                };

                match Request::post("http://127.0.0.1:8080/api/v1/register")
                    .header("Content-Type", "application/json")
                    .json(&body)
                {
                    Ok(req) => match req.send().await {
                        Ok(resp) => {
                            if resp.ok() {
                                console::log_1(&"Register ok".into());
                                success.set(Some("Registration successful!".into()));

                                // After a short delay (or immediately) navigate to Login
                                if let Some(nav) = navigator {
                                    nav.push(&crate::Route::Login);
                                }
                            } else {
                                error.set(Some(
                                    format!("Registration failed, status: {}", resp.status())
                                ));
                            }
                        }
                        Err(e) => {
                            console::log_1(
                                &format!("Send register request error: {:?}", e).into()
                            );
                            error.set(Some("Unable to reach the server".into()));
                        }
                    },
                    Err(e) => {
                        console::log_1(&format!("Build register request error: {:?}", e).into());
                        error.set(Some("Failed to build the request".into()));
                    }
                }

                loading.set(false);
            });
        })
    };

    // Optional: handler for the "Log in" action to navigate back to login
    let handle_go_login = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            if let Some(nav) = &navigator {
                nav.push(&crate::Route::Login);
            }
        })
    };

    // --- UI ---

    html! {
        <div class="min-h-screen flex items-center justify-center bg-gray-900 px-4">
            <div class="w-full max-w-md bg-gray-800 rounded-xl shadow-lg p-8">
                // title
                <div class="mb-6 text-center">
                    <h1 class="text-3xl font-bold text-white mb-2">{ "Register" }</h1>
                    <p class="text-gray-400 text-sm">
                        { "Create an account to use the Todo Application" }
                    </p>
                </div>

                // error
                if let Some(err) = &*error {
                    <div class="mb-4 px-4 py-3 rounded-lg bg-red-900/60 border border-red-500 text-sm text-red-200">
                        { err }
                    </div>
                }

                // success
                if let Some(msg) = &*success {
                    <div class="mb-4 px-4 py-3 rounded-lg bg-green-900/60 border border-green-500 text-sm text-green-200">
                        { msg }
                    </div>
                }

                // form
                <form onsubmit={handle_submit} class="space-y-4">
                    <div>
                        <label class="block mb-1 text-sm font-medium text-gray-300">
                            { "Username" }
                        </label>
                        <input
                            type="text"
                            placeholder="huy2"
                            value={(*email).clone()}
                            oninput={handle_email_change}
                            disabled={*loading}
                            class="w-full px-4 py-2.5 rounded-lg bg-gray-700 border border-gray-600 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500"
                        />
                    </div>

                    <div>
                        <label class="block mb-1 text-sm font-medium text-gray-300">
                            { "Password" }
                        </label>
                        <input
                            type="password"
                            placeholder="********"
                            value={(*password).clone()}
                            oninput={handle_password_change}
                            disabled={*loading}
                            class="w-full px-4 py-2.5 rounded-lg bg-gray-700 border border-gray-600 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500"
                        />
                    </div>

                    <button
                        type="submit"
                        disabled={
                            *loading
                            || (*email).trim().is_empty()
                            || (*password).trim().is_empty()
                        }
                        class="w-full mt-2 px-4 py-2.5 rounded-lg font-semibold text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                        if *loading {
                            { "Registering..." }
                        } else {
                            { "Register" }
                        }
                    </button>
                </form>

                <div class="mt-6 text-center text-sm text-gray-400">
                    { "Already have an account? " }
                    <span
                        class="text-blue-400 hover:underline cursor-pointer"
                        onclick={handle_go_login}
                    >
                        { "Log in" }
                    </span>
                </div>
            </div>
        </div>
    }
}
