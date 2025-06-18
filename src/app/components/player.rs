use cfg_block::cfg_block;
use leptos::{html::Video, prelude::*};
use leptos_meta::Script;
use serde::{Deserialize, Serialize};

cfg_block! {
    #[cfg(not(feature="ssr"))] {
        use wasm_bindgen::{prelude::*, JsValue};
        use leptos::{logging::log, task::spawn_local};

        #[wasm_bindgen]
        extern "C" {
            #[derive(Debug)]
            type Hls;

            #[wasm_bindgen(constructor)]
            fn new(config: JsValue) -> Hls;

            #[wasm_bindgen(static_method_of = Hls, js_name=isSupported, catch)]
            fn is_hls_supported() -> Result<bool, JsValue>;

            #[wasm_bindgen(method, js_name=loadSource)]
            fn load_source(this:&Hls, url: String);

            #[wasm_bindgen(method, js_name=attachMedia)]
            fn attach_media(this:&Hls, video: JsValue);

            #[wasm_bindgen(method)]
            fn once(this: &Hls, event_type: JsValue, cb: &Closure<dyn Fn(JsValue)>);
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

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MediaType {
    Flv(String),
    Hls(String),
}

#[component]
pub fn Player(
    #[prop(default=MediaType::Hls("https://www.youtu.tv/stream/hls/master.m3u8".to_owned()))] media: MediaType,
) -> impl IntoView {
    let el = NodeRef::<Video>::new();

    let lib_url = match media {
        MediaType::Flv(_) => "https://www.youtu.tv/js/flv.min.js",
        MediaType::Hls(_) => "https://www.youtu.tv/js/hls.min.js",
    };

    #[cfg(not(feature = "ssr"))]
    {
        log!("media: {:?}", lib_url);

        Effect::new(move |_| {
            if let Some(el) = el.get() {
                match media.clone() {
                    MediaType::Flv(video_url) => {
                        log!("FLV 暂不支持: {}", video_url)
                    }
                    MediaType::Hls(video_url) => {
                        spawn_local(async move {
                            let handle = Closure::new(|data: JsValue| {
                                log!("Rust {:?}", data);
                            });
                            loop {
                                match Hls::is_hls_supported() {
                                    Ok(true) => {
                                        let config = Config {
                                            enable_worker: true,
                                            debug: true,
                                        };
                                        let js_config =
                                            serde_wasm_bindgen::to_value(&config).unwrap();
                                        let player = Hls::new(js_config);
                                        log!("instance: {:?}", player);
                                        player.load_source(video_url.clone());
                                        player.attach_media(el.into());
                                        log!("VIDEO Manifest WAITING");

                                        player
                                            .once(JsValue::from_str("hlsManifestParsed"), &handle);

                                        break;
                                    }
                                    Ok(false) => {
                                        log!("浏览器不支持播放");
                                        break;
                                    }
                                    Err(_) => {
                                        use gloo_timers::future::TimeoutFuture;
                                        TimeoutFuture::new(100).await;
                                    }
                                }
                            }
                            handle.forget();
                        });
                    }
                }
            }
        });
    }

    view! {
        <Script src=lib_url></Script>
        <video
            controls
            autoplay
            loop
            muted
            disablepictureinpicture
            disableremoteplayback
            controlslist="play volume nofullscreen nodownload noremoteplayback noplaybackrate"
            class="bg-black shadow-md aspect-video shadow-black/60"
            node_ref=el
        ></video>
    }
}
