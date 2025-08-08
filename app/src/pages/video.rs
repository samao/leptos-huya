use cfg_block::cfg_block;
use leptos::{html::Div, prelude::*};
use leptos_meta::Title;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Barrage {
    from: User,
    msg: Msg,
}

#[derive(Debug, Deserialize, Serialize)]
struct User {
    n: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Msg {
    content: String,
    color: usize,
}

cfg_block! {
    #[cfg(feature = "hydrate")]
    {
        use crate::json;
        use gloo_timers::future::TimeoutFuture;
        use js_sys::Array;
        use leptos::{logging::log, task::spawn_local};
        use std::sync::{Arc, Mutex};
        use wasm_bindgen::{prelude::*, JsValue};

        #[wasm_bindgen]
        extern "C" {
            #[derive(Debug)]
            type Player;
            #[wasm_bindgen(js_namespace=QiePlayer, js_name=createPlayer)]
            fn create_player(config: JsValue);

            #[wasm_bindgen(js_namespace=QiePlayer, js_name=destroyAll)]
            fn destroy_all_player();

            #[wasm_bindgen(method)]
            fn dispatch(this: &Player, payload: JsValue);

            #[wasm_bindgen(method)]
            fn destroy(this: &Player);
            // destroy

            #[wasm_bindgen(js_namespace=["Mock", "Random"])]
            fn csentence(start: usize, len: usize) -> String;

            #[wasm_bindgen(js_namespace=["Mock", "Random"])]
            fn d6() -> usize;

            #[wasm_bindgen(js_namespace=["Mock", "Random"])]
            fn cname() -> String;
        }
    }
}

#[component]
pub fn VideoPage() -> impl IntoView {
    let node_ref = NodeRef::<Div>::new();
    #[cfg(feature = "hydrate")]
    {
        let destoryed = Arc::new(Mutex::new(false));
        let clear_destoryed = Arc::clone(&destoryed);

        Effect::new(move || {
            if node_ref.get().is_none() {
                return;
            }
            let clear_destoryed = Arc::clone(&destoryed);
            spawn_local(async move {
                loop {
                    if crate::has("QiePlayer") {
                        break;
                    }
                    TimeoutFuture::new(1000).await;
                    log!("SDK is loading");
                }

                let rtmp = Closure::<dyn Fn() -> JsValue>::new(|| {
                    log!("Callback executed!");
                    let cdns = Array::new();
                    cdns.set(0, "ws".into());

                    let config = json! {
                        "error" => 0,
                        "data" => json! {
                            "rtmp_url" => "https://www.youtu.tv/stream".to_string(),
                            "rtmp_live" => "live.flv".to_string(),
                            "rtmp_cdn" => "ws".to_string(),
                            "cdns" => cdns,
                        }
                    };
                    config.into()
                });

                let room = Closure::<dyn Fn() -> JsValue>::new(|| {
                    log!("room Callback executed!");
                    let config = json! {
                        "giftBatterConfig" => Array::new(),
                        "giftExBatterConfig" => Array::new(),
                        "show_status" => "1"
                    };
                    config.into()
                });

                let clear_destoryed = Arc::clone(&clear_destoryed);

                let player_handle =
                    Closure::<dyn Fn(String, Player)>::new(move |action, player: Player| {
                        log!("recive: {}", action);
                        let clear_destoryed = Arc::clone(&clear_destoryed);
                        if "PLAYER_CREATED" == action {
                            spawn_local(async move {
                                loop {
                                    if *clear_destoryed.lock().unwrap() {
                                        log!("任务已退出");
                                        player.destroy();
                                        break;
                                    }

                                    log!("FIRE...");
                                    let dm = Barrage {
                                        from: User { n: cname() },
                                        msg: Msg {
                                            content: csentence(5, 20),
                                            color: if js_sys::Math::random() > 0.9 {
                                                d6()
                                            } else {
                                                0
                                            },
                                        },
                                    };
                                    player.dispatch(
                                        json! {
                                            "type" => "socketBarrageNotice".to_string(),
                                            "payload" => serde_json::to_string(&dm).unwrap()
                                        }
                                        .into(),
                                    );
                                    TimeoutFuture::new(1000).await;
                                }
                            })
                        }
                    });

                let config = json! {
                    "isLive" => true,
                    "demuxType" => "flv".to_string(),
                    "rid" => 9527,
                    "loader" => json! {
                        "rtmp" => rtmp.as_ref(),
                        "room" => room.as_ref()
                    },
                    "ui" => json! {
                        "danmu" => true,
                        "effectsBlock" => false,
                        "feedback" => false,
                        "entries" => false,
                        "giftBar" => false,
                        "pip" => true,
                        "filter" => true,
                        "eq" => true,
                    },
                    "websocket" => false,
                    "el" => node_ref.get_untracked().unwrap(),
                    "cb" => player_handle.as_ref()
                };

                create_player(config.into());
            });
        });

        on_cleanup(move || {
            *clear_destoryed.lock().unwrap() = true;
            // destroy_all_player();
            log!("销毁播放器");
        });
    }

    stylance::import_crate_style!(css, "src/pages/video.module.scss");

    view! {
        <Title text="视频播放" />
        <script nonce="mock.min.js" src="/nextStatic/js/mock.min.js" />
        <script nonce="qie-player.js" src="/nextStatic/js/player/3.8.7/qie.player.js" />
        <div id="leptos-player" class=css::player_clsx node_ref=node_ref></div>
    }
}
