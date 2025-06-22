use leptos::{html::Video, prelude::*};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MediaType {
    Flv(String),
    Hls(String),
    Full(bool, String),
}

#[component]
pub fn Player(
    #[prop(default=MediaType::Hls("https://www.youtu.tv/stream/hls/master.m3u8".to_owned()))] media: MediaType,
) -> impl IntoView {
    let el = NodeRef::<Video>::new();

    let lib_url = match media {
        MediaType::Flv(_) => "/nextStatic/js/flv.min.js",
        MediaType::Hls(_) => "/nextStatic/js/hls.min.js",
        MediaType::Full(_, _) => "/nextStatic/js/player/3.8.3/qie.player.js",
    };

    #[cfg(not(feature = "ssr"))]
    {
        use crate::{has, json};
        use gloo_timers::future::TimeoutFuture;
        use leptos::{logging::log, task::spawn_local};
        use std::sync::{Arc, Mutex};
        use wasm_bindgen::{
            prelude::{wasm_bindgen, Closure},
            JsValue,
        };

        enum Player {
            FLV(FlvPlayer),
            HLS(Hls),
            None,
        }

        #[wasm_bindgen]
        extern "C" {
            #[derive(Debug)]
            type Hls;

            #[wasm_bindgen(constructor)]
            fn new(config: JsValue) -> Hls;

            #[wasm_bindgen(static_method_of = Hls, js_name=isSupported)]
            fn is_hls_supported() -> bool;

            #[wasm_bindgen(method, js_name=loadSource)]
            fn load_source(this: &Hls, url: String);

            #[wasm_bindgen(method, js_name=attachMedia)]
            fn attach_media(this: &Hls, video: JsValue);

            #[wasm_bindgen(method)]
            fn destroy(this: &Hls);

            #[wasm_bindgen(method)]
            fn once(this: &Hls, event_type: JsValue, cb: &Closure<dyn Fn(JsValue)>);

            // FLV
            #[derive(Debug)]
            type FlvPlayer;

            #[wasm_bindgen(js_name=createPlayer, js_namespace=flvjs)]
            fn create_flv_player(config: JsValue) -> FlvPlayer;

            #[wasm_bindgen(method, js_name=attachMediaElement)]
            fn attach_media(this: &FlvPlayer, video: JsValue);

            #[wasm_bindgen(method)]
            fn load(this: &FlvPlayer);

            #[wasm_bindgen(method)]
            fn play(this: &FlvPlayer);

            #[wasm_bindgen(method)]
            fn destroy(this: &FlvPlayer);

            #[wasm_bindgen(js_namespace=flvjs, js_name=isSupported)]
            fn is_flv_supported() -> bool;

            #[wasm_bindgen(js_name=applyConfig, js_namespace=["flvjs", "LoggingControl"])]
            fn apply_config(config: JsValue);
        }

        log!("media: {:?}", lib_url);

        let destoryed = Arc::new(Mutex::new(false));
        let clear_destoryed = Arc::clone(&destoryed);

        Effect::new(move |_| {
            let clear_destoryed = Arc::clone(&destoryed);
            if let Some(el) = el.get() {
                let media = media.clone();
                spawn_local(async move {
                    match media {
                        MediaType::Full(is_live, video_url) => {
                            log!("Full video {} {}", is_live, video_url);
                        }
                        MediaType::Flv(video_url) => {
                            loop {
                                if has("flvjs") {
                                    break;
                                }
                                TimeoutFuture::new(200).await;
                            }
                            if is_flv_supported() {
                                log!("支持播放FLV");
                                let config = json! {
                                    "enableAll" => true,
                                    "enableDebug" => true,
                                    "enableInfo" => true,
                                    "enableWarn" => true,
                                    "enableVerbose" => false,
                                };
                                apply_config(config.into());
                                let config = json! {
                                    "is_live" => true,
                                    "type" => "flv".to_string(),
                                    "url" => video_url.clone(),
                                    "enabledWorker" => true,
                                };
                                let player = create_flv_player(config.into());
                                player.attach_media(el.into());
                                player.load();
                                player.play();

                                loop {
                                    if *clear_destoryed.lock().unwrap() {
                                        log!("任务已退出");
                                        break;
                                    }
                                    TimeoutFuture::new(1000).await;
                                }
                                player.destroy();
                            } else {
                                log!("不支持FLV播放");
                            }
                        }
                        MediaType::Hls(video_url) => {
                            loop {
                                if has("Hls") {
                                    break;
                                }
                                TimeoutFuture::new(200).await;
                            }

                            if Hls::is_hls_supported() {
                                let handle = Closure::new(|data: JsValue| {
                                    log!("Rust {:?}", data);
                                });
                                let config = json! {
                                    "enableWorker" => true,
                                    "debug" => false,
                                };
                                let player = Hls::new(config.into());
                                log!("instance: {:?}", player);
                                player.load_source(video_url.clone());
                                player.attach_media(el.into());
                                log!("VIDEO Manifest WAITING");
                                player.once("hlsManifestParsed".into(), &handle);
                                handle.forget();
                                loop {
                                    if *clear_destoryed.lock().unwrap() {
                                        log!("任务已退出");
                                        break;
                                    }
                                    TimeoutFuture::new(1000).await;
                                }
                                player.destroy();
                            }
                        }
                    }
                });
            }
        });

        on_cleanup(move || {
            *clear_destoryed.lock().unwrap() = true;
            log!("销毁播放器PLAYER");
        });
    }

    view! {
        <script nonce=lib_url src=lib_url></script>
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
