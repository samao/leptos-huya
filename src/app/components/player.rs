use cfg_block::cfg_block;
use leptos::{html::Video, prelude::*};
use leptos_meta::Script;

cfg_block! {
    #[cfg(not(feature="ssr"))] {
        use wasm_bindgen::prelude::*;
        use serde::{Deserialize, Serialize};

        #[wasm_bindgen]
        extern "C" {
            #[derive(Debug)]
            type Hls;

            #[wasm_bindgen(constructor)]
            fn new(config: JsValue) -> Hls;

            #[wasm_bindgen(static_method_of = Hls, js_name=isSupported)]
            fn is_hls_supported() -> bool;

            #[wasm_bindgen(method, js_name=loadSource)]
            fn load_source(this:&Hls, url: String);

            #[wasm_bindgen(method, js_name=attachMedia)]
            fn attach_media(this:&Hls, video: JsValue);

            #[wasm_bindgen(method)]
            fn on(this: &Hls, event_type: JsValue, cb: &Closure<dyn Fn(JsValue)>);
        }

        #[wasm_bindgen]
        #[derive(Debug, Serialize, Deserialize)]
        struct Config {
            #[serde(rename="enableWorker")]
            enable_worker: bool,
            debug: bool,
        }
    }
}

#[component]
pub fn Player() -> impl IntoView {
    let el = NodeRef::<Video>::new();

    #[cfg(not(feature = "ssr"))]
    {
        use leptos::logging::log;
        Effect::new(move |_| {
            if let Some(el) = el.get() {
                if Hls::is_hls_supported() {
                    let config = Config {
                        enable_worker: true,
                        debug: true,
                    };
                    let js_config = serde_wasm_bindgen::to_value(&config).unwrap();
                    let player = Hls::new(js_config);
                    log!("{:?}", player);
                    player.load_source("https://www.youtu.tv/stream/hls/master.m3u8".into());
                    player.attach_media(el.into());
                    log!("VIDEO Manifest WAITING");
                    // player.on("hlsManifestParsed".into(), &Closure::new(|_| {
                    //     log!("VIDEO: Manifest OK");
                    // }));
                } else {
                    log!("不支持hls播放");
                }
            }
        });
    }

    view! {
        <Script src="https://www.youtu.tv/js/hls.min.js"></Script>
        <video controls autoplay class="aspect-video bg-black" node_ref=el></video>
    }
}
