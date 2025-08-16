use std::time::Duration;

use leptos::prelude::*;
use reactive_stores::Store;
use serde::{Deserialize, Serialize};
use web_sys::MouseEvent;

use crate::app::{GlobalState, GlobalStateStoreFields};
use crate::components::Zcode;

#[derive(Debug, Deserialize, Serialize)]
struct ThirdPart {
    icon: String,
    url: String,
}

#[component]
pub fn Login() -> impl IntoView {
    stylance::import_crate_style!(css, "src/components/login.module.scss");
    let store = use_context::<Store<GlobalState>>();
    let mode = RwSignal::new(false);
    let (expired, set_expired) = signal(false);
    let change_mode = move |val: bool| *mode.write() = val;
    let (zcode, set_zcode) = signal(("+86".to_string(), "中国".to_string()));
    let phone_or_huya = RwSignal::new("".to_string());
    let password = RwSignal::new("".to_string());

    let phone = RwSignal::new("".to_string());
    let sms_code = RwSignal::new("".to_string());
    let agree = RwSignal::new(false);

    Effect::new(move || {
        leptos::logging::log!("Zcode is: {:?}", zcode.get());
    });

    Effect::new(move || {
        if expired.get() {
            return;
        }
        if let Ok(handle) = set_timeout_with_handle(
            move || {
                set_expired.set(true);
            },
            Duration::from_secs(10),
        ) {
            on_cleanup(move || {
                handle.clear();
            });
        }
    });

    view! {
        <div
            class=css::global_login_pop
            on:click=move |_| {
                let logined = store.unwrap().logined();
                logined.set(false);
            }
        >
            <div
                class=css::inner
                on:click=|e: MouseEvent| {
                    e.stop_propagation();
                }
            >
                <div class=css::left>
                    <p>"打开「虎牙直播APP-我的」扫码登录"</p>
                    <div class=move || {
                        format!(
                            "{} {}",
                            css::qrcode_box,
                            if expired.get() { css::is_expired } else { "" },
                        )
                    }>
                        <img src="/imgs/login/getQrImg.png" />
                        // hover tips
                        <div class=css::hover_tips />

                        <div class=css::expired>
                            二维码已过期
                            <button
                                on:click=move |_| {
                                    set_expired.set(false);
                                }
                                class=css::refresh
                            >
                                点击刷新
                            </button>
                        </div>
                    </div>
                    <div class=css::divide>
                        <hr />
                        <span>其他登录方式</span>
                    </div>
                    <ul class=css::partner>
                        <For
                            each=|| {
                                vec![
                                    ThirdPart {
                                        icon: "/imgs/login/wechat-icon_de04bfb.png".to_string(),
                                        url: "".to_string(),
                                    },
                                    ThirdPart {
                                        icon: "/imgs/login/qq-icon_ccdb939.png".to_string(),
                                        url: "".to_string(),
                                    },
                                    ThirdPart {
                                        icon: "/imgs/login/weibo-icon_623b022.png".to_string(),
                                        url: "".to_string(),
                                    },
                                ]
                                    .into_iter()
                            }
                            key=|item| item.icon.clone()
                            let(data)
                        >
                            <li>
                                <a href=data.url>
                                    <img src=data.icon />
                                </a>
                            </li>
                        </For>
                    </ul>
                </div>
                <div class=css::right>
                    <input type="checkbox" bind:checked=mode class=css::mode_type />
                    <form
                        class=css::login_form
                        on:submit=move |e| {
                            e.prevent_default();
                            if agree.get() {
                                leptos::logging::log!(
                                    "{}", if mode.get() { format!("phone: {}{}, sms: {}", zcode.get().0, phone.get(), sms_code.get()) } else { format!("phone:{}, password: {}", phone_or_huya.get(), password.get())}
                                );
                            } else {
                                leptos::logging::log!("The checkbox must be checked!");
                                store
                                    .unwrap()
                                    .toast()
                                    .update(|msg| *msg = "请阅读并同意协议".to_string());
                            }
                        }
                    >
                        <ul>
                            <li on:click=move |_| change_mode(false)>密码登录</li>
                            <li on:click=move |_| change_mode(true)>短信登录</li>
                        </ul>
                        <div class=css::views>
                            <div class=css::pwd_view>
                                <input
                                    type="text"
                                    placeholder="手机号/虎牙号"
                                    bind:value=phone_or_huya
                                />
                                <input type="password" placeholder="密码" bind:value=password />
                                <span>忘记密码?</span>
                            </div>
                            <div class=css::sms_view>
                                <div>
                                    <Zcode set_active=set_zcode />
                                    <input
                                        type="text"
                                        placeholder="请输入手机号"
                                        bind:value=phone
                                    />
                                </div>
                                <div>
                                    <input
                                        type="text"
                                        placeholder="请输入验证码"
                                        bind:value=sms_code
                                    />
                                    <span>获取验证码</span>
                                </div>
                                <span>收不到验证码?</span>
                            </div>
                        </div>
                        <button type="submit" class=css::login_btn>
                            登录
                        </button>
                        <div class=css::privcy>
                            <label>
                                <input type="checkbox" bind:checked=agree />
                            </label>
                            已阅读并同意
                            <a>"《用户服务协议》"</a>
                            和
                            <a>"《隐私政策》"</a>
                        </div>
                    </form>
                </div>
                <a>快速注册</a>
            </div>
        </div>
    }
}
