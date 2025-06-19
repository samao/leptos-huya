use leptos::{html::Div, prelude::*};
use leptos_meta::Title;

#[component]
pub fn VideoPage() -> impl IntoView {
    let node_ref = NodeRef::<Div>::new();
    #[cfg(not(feature = "ssr"))]
    {
        use crate::json;
        use leptos::{logging::log, task::spawn_local};
        use serde::{Deserialize, Serialize};
        use wasm_bindgen::{prelude::*, JsValue};

        Effect::new(move || {
            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(js_namespace=global)]
                fn say(config: JsValue);
            }

            #[derive(Debug, Serialize, Deserialize)]
            struct JSBackConfig {
                name: String,
            }

            spawn_local(async move {
                let callback = Closure::<dyn Fn(JsValue) -> JsValue>::new(|data: JsValue| {
                    log!("Callback executed! {:?}", data);
                    let config = json! {
                        "name" => "刘麻子是你爹".to_string()
                    };
                    config.into()
                });

                let config = json! {
                    "callback" => callback.as_ref(),
                    "name" => "刘麻子".to_string(),
                    "el" => node_ref.get_untracked().unwrap(),
                };

                say(config.into());
            });
        });
    }

    view! {
        <Title text="视频播放" />
        <script>
            "
                var global = {
                    say(config) {
                        console.log(config);
                        // console.log(callback);
                        config.callback({'name': 'JOHO & ' + config.name});
            
                        var rust_result = config.callback({'name': 'BAI_GU & ' + config.name});
                        console.log('收到了:', rust_result);
                    }
                };
            "
        </script>
        <div node_ref=node_ref></div>
    }
}
