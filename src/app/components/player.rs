use leptos::{html::Video, logging::log, prelude::*};
use leptos_meta::Script;
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
        MediaType::Flv(_) => "https://www.youtu.tv/js/flv.min.js",
        MediaType::Hls(_) => "https://www.youtu.tv/js/hls.min.js",
        MediaType::Full(_, _) => "https://www.youtu.tv/nextStatic/js/player/3.8.3/qie.player.js",
    };
    log!("media: {:?}", lib_url);
    #[cfg(not(feature = "ssr"))]
    {
        use gloo_timers::future::TimeoutFuture;
        use leptos::task::spawn_local;
        use wasm_bindgen::{
            prelude::{wasm_bindgen, Closure},
            JsValue,
        };
        // use web_sys::HtmlVideoElement;

        #[wasm_bindgen]
        extern "C" {
            #[derive(Debug)]
            type Hls;

            #[wasm_bindgen(constructor)]
            fn new(config: JsValue) -> Hls;

            #[wasm_bindgen(static_method_of = Hls, js_name=isSupported, catch)]
            fn is_hls_supported() -> Result<bool, JsValue>;

            #[wasm_bindgen(method, js_name=loadSource)]
            fn load_source(this: &Hls, url: String);

            #[wasm_bindgen(method, js_name=attachMedia)]
            fn attach_media(this: &Hls, video: JsValue);

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

            #[wasm_bindgen(js_namespace=flvjs, js_name=isSupported, catch)]
            fn is_flv_supported() -> Result<bool, JsValue>;

            #[wasm_bindgen(js_name=applyConfig, js_namespace=["flvjs", "LoggingControl"])]
            fn apply_config(config: JsValue);
        }

        #[wasm_bindgen]
        #[derive(Debug, Serialize, Deserialize)]
        struct LoggerConfig {
            #[serde(rename = "enableAll")]
            enable_all: bool,
            #[serde(rename = "enableDebug")]
            enable_debug: bool,
            #[serde(rename = "enableInfo")]
            enable_info: bool,
            #[serde(rename = "enableVerbose")]
            enable_verbose: bool,
            #[serde(rename = "enableWarn")]
            enable_varn: bool,
        }

        #[wasm_bindgen]
        #[derive(Debug, Serialize, Deserialize)]
        struct FlvConfig {
            #[serde(rename = "type")]
            media_type: String,
            url: String,
            #[serde(rename = "isLive")]
            is_live: bool,
            #[serde(rename = "enableWorker", skip_serializing_if = "Option::is_none")]
            enable_worker: Option<bool>,
        }

        #[wasm_bindgen]
        #[derive(Debug, Serialize, Deserialize)]
        struct Config {
            #[serde(rename = "enableWorker", skip_serializing_if = "Option::is_none")]
            enable_worker: Option<bool>,
            debug: bool,
        }

        Effect::new(move |_| {
            if let Some(el) = el.get() {
                match media.clone() {
                    MediaType::Full(is_live, video_url) => {
                        log!("Full video {} {}", is_live, video_url);
                    }
                    MediaType::Flv(video_url) => {
                        // log!("FLV 暂不支持: {}", video_url);
                        spawn_local(async move {
                            loop {
                                match is_flv_supported() {
                                    Ok(true) => {
                                        log!("支持播放");
                                        let config = serde_wasm_bindgen::to_value(&LoggerConfig {
                                            enable_all: false,
                                            enable_debug: false,
                                            enable_info: false,
                                            enable_varn: false,
                                            enable_verbose: false,
                                        })
                                        .unwrap();

                                        apply_config(config);

                                        let config = serde_wasm_bindgen::to_value(&FlvConfig {
                                            is_live: true,
                                            media_type: "flv".to_string(),
                                            url: video_url.clone(),
                                            enable_worker: Some(true),
                                        })
                                        .unwrap();

                                        let player = create_flv_player(config);
                                        player.attach_media(el.into());
                                        player.load();
                                        player.play();
                                        break;
                                    }
                                    Ok(false) => {
                                        log!("不支持");
                                        break;
                                    }
                                    _ => {
                                        log!("sdk loading...");
                                        TimeoutFuture::new(100).await;
                                    }
                                }
                            }
                        });
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
                                            enable_worker: Some(true),
                                            debug: false,
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
