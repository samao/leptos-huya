use cfg_block::cfg_block;
use leptos::{html::Video, prelude::*};
use leptos_meta::Script;

cfg_block! {
    #[cfg(not(feature="ssr"))] {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsValue;
        use serde::{Deserialize, Serialize};

        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace = console, js_name = log)]
            fn console_log_str(s: &str);

            #[derive(Debug)]
            type Player;

            #[wasm_bindgen(js_namespace = flvjs, js_name = createPlayer)]
            fn create_player(confg: JsValue) -> Player;

            #[wasm_bindgen(method, js_name = attachMediaElement)]
            fn attach_media(this: &Player, el: JsValue);
            #[wasm_bindgen(method, js_name = load)]
            fn load(this: &Player);
            #[wasm_bindgen(method, js_name = play)]
            fn play(this: &Player);
        }

        #[wasm_bindgen]
        #[derive(Debug, Deserialize, Serialize)]
        struct PlayerConfig {
            islive: bool,
            debug: bool,
            url: String,
            r#type: String,
        }
    }
}

#[component]
pub fn Player() -> impl IntoView {
    let el = NodeRef::<Video>::new();

    #[cfg(not(feature = "ssr"))]
    {
        Effect::new(move |_| {
            if let Some(_el) = el.get() {
                use leptos::logging::log;
                // use wasm_bindgen::JsValue;

                let config = PlayerConfig {
                    islive: true,
                    r#type: "flv".into(),
                    debug: true,
                    url: "https://www.youtu.tv/stream/live.flv".to_owned(),
                };
                let js_config = serde_wasm_bindgen::to_value(&config).unwrap();
                // log!("{:?}", js_config);
                let player = create_player(js_config);
                log!("{:?}", player);
                player.attach_media(_el.into());
                player.load();
                player.play();
            }
        });
    }

    view! {
        <Script src="https://www.youtu.tv/js/flv.min.js"></Script>
        <video class="aspect-video bg-black/30" node_ref=el></video>
    }
}
