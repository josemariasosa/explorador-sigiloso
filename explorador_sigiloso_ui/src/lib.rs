// explorador_sigiloso_ui/src/lib.rs

use leptos::{prelude::*, task::spawn_local};

#[component]
pub fn App() -> impl IntoView {
    let (address, set_address) = signal(String::new());
    let (balance, set_balance) = signal(None::<f64>);
    let (loading, set_loading) = signal(false);

    let check_balance = move |_| {
        let addr = address.get();
        if addr.is_empty() {
            return;
        }
        set_loading.set(true);

        spawn_local(async move {
            let url = format!("http://127.0.0.1:3000/btc/balance/{}", addr);
            match reqwest::get(&url).await {
                Ok(response) => {
                    if let Ok(json) = response.json::<serde_json::Value>().await {
                        let value = json["balance"].as_f64();
                        set_balance.set(value);
                    }
                }
                Err(_) => {
                    set_balance.set(None);
                }
            }
            set_loading.set(false);
        });
    };

    view! {
        <div>
            <h1>"Explorador Sigiloso 🕵️‍♂️"</h1>
            <input
                type="text"
                placeholder="Enter BTC testnet address"
                on:input=move |e| set_address.set(event_target_value(&e))
                prop:value=address
            />
            <button on:click=check_balance disabled=move || loading.get()>
                {move || if loading.get() {"Checking..."} else {"Check Balance"}}
            </button>
            <div>
                {move || match balance.get() {
                    Some(val) => view! { <p>{format!("Balance: {} BTC", val)}</p> },
                    None => view! { <p>"No balance found or error.".to_owned()</p> },
                }}
            </div>
        </div>
    }
}
