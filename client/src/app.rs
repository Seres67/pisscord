use std::io::Write;

use gloo_timers::future::TimeoutFuture;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
struct PrintTextArgs<'a> {
    text: &'a str,
}

#[component]
pub fn App<G: Html>(cx: Scope) -> View<G> {
    let text = create_signal(cx, String::new());
    let latest_msg = create_signal(cx, String::new());
    let msgs = create_signal(cx, Vec::<String>::new());

    let print_text = move |_| {
        spawn_local_scoped(cx, async move {
            invoke(
                "print_text",
                to_value(&PrintTextArgs { text: &text.get() }).unwrap(),
            )
            .await;
            text.set("".to_string());
        });
    };

    spawn_local_scoped(cx, async move {
        loop {
            TimeoutFuture::new(10).await;
            let msg = invoke("get_message", JsValue::null())
                .await
                .as_string()
                .unwrap();
            if !msg.is_empty() {
                latest_msg.set(msg);
            }
        }
    });

    view! { cx,
        div(class="flex flex-col justify-center text-center bg-background-950 text-text-50 h-full w-full fixed") {
            div(class="grow") {
                form() {
                    input(type="text",id="text",bind:value=text,class="text-text-50 bg-background-900")
                    button(type="button",on:click=print_text) {
                        "Print"
                    }
                }
                p(class="text-text-50") {
                    (*latest_msg.get())
                }
            }
        }
    }
}
