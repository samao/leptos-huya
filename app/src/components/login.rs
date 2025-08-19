use leptos::prelude::*;
use models::User as ModelUser;
use reactive_stores::Store;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use web_sys::MouseEvent;

use crate::app::{GlobalState, GlobalStateStoreFields};
use crate::components::Zcode;

stylance::import_crate_style!(css, "src/components/login.module.scss");

#[derive(Debug, Deserialize, Serialize)]
struct ThirdPart {
    icon: String,
    url: String,
}

#[component]
pub fn Password(placeholder: String, set_signal: RwSignal<String>) -> impl IntoView {
    let (show_pwd, set_show_pwd) = signal(false);
    view! {
        <div class=css::password_box>
            <input
                placeholder=placeholder
                bind:value=set_signal
                type=move || if show_pwd.get() { "text" } else { "password" }
            />
            <span on:click=move |_| {
                set_show_pwd.update(|data| *data = !*data);
            } />
        </div>
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
enum LoginOptions {
    //手机号登录
    MOBILE { phone: String, sms: String },
    //密码登录
    USER { id: String, password: String },
}

#[server]
async fn login_query(data: LoginOptions) -> Result<ModelUser, ServerFnError> {
    use axum::http::header::{HeaderValue, SET_COOKIE};
    use axum_extra::extract::cookie::{Cookie, Expiration, SameSite};
    use database::{
        establish_connection,
        users::{login, phone_login},
    };
    use leptos::logging::log;
    use leptos_axum::ResponseOptions;
    use time::{Duration, OffsetDateTime};

    log!("用户登录：{:?}", data);
    let conn = &mut establish_connection();
    let user = match data {
        LoginOptions::MOBILE { phone, sms } => phone_login(conn, phone, sms),
        LoginOptions::USER { id, password } => login(conn, id, password),
    };
    let response = expect_context::<ResponseOptions>();
    match user {
        Ok(user) => {
            log!("用户登录成功: {:?}", user.token);
            let cookie = Cookie::build(("token", user.token.clone()))
                .path("/")
                .http_only(true)
                .same_site(SameSite::Lax)
                .expires(Expiration::DateTime(
                    OffsetDateTime::now_utc() + Duration::minutes(1),
                ));

            response.insert_header(
                SET_COOKIE,
                HeaderValue::from_str(&cookie.to_string())
                    .map_err(|er| ServerFnError::new(er.to_string()))?,
            );
            Ok(user)
        }
        Err(er) => {
            log!("登录失败: {:?}", er);
            Err(ServerFnError::new("登录失败".to_string()))
        }
    }
}

#[server]
#[allow(dead_code)]
async fn register_user(
    phone: String,
    password: String,
    zcode: String,
    sms: String,
) -> Result<ModelUser, ServerFnError> {
    let conn = &mut database::establish_connection();
    match database::users::register(conn, phone, Some(password)) {
        Ok(user) => {
            use leptos::logging::log;
            log!(
                "user code: <{}>, sms:<{}> token: {:?}",
                zcode,
                sms,
                user.token,
            );
            Ok(user)
        }
        Err(er) => Err(ServerFnError::new(format!("注册失败: {}", er))),
    }
}

#[component]
pub fn Login() -> impl IntoView {
    let store = use_context::<Store<GlobalState>>();
    let mode = RwSignal::new(false);
    let (expired, set_expired) = signal(false);
    let change_mode = move |val: bool| *mode.write() = val;
    let (register, set_register) = signal(false);
    let (zcode, set_zcode) = signal(("+86".to_string(), "中国".to_string()));
    let phone_or_huya = RwSignal::new("".to_string());
    let password = RwSignal::new("".to_string());

    let phone = RwSignal::new("".to_string());
    let sms_code = RwSignal::new("".to_string());
    let agreed = RwSignal::new(false);

    let register_pwd = RwSignal::new("".to_string());

    let register_action = ServerAction::<RegisterUser>::new();
    let login_action = ServerAction::<LoginQuery>::new();

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
                        class=move || {
                            format!(
                                "{} {}",
                                css::login_form,
                                if register.get() { css::register } else { "" },
                            )
                        }
                        on:submit=move |e| {
                            e.prevent_default();
                            if agreed.get() {
                                if register.get() {
                                    register_action
                                        .dispatch(RegisterUser {
                                            phone: phone.get(),
                                            sms: sms_code.get(),
                                            password: register_pwd.get(),
                                            zcode: zcode.get().0,
                                        });
                                    return;
                                }
                                login_action
                                    .dispatch(
                                        if mode.get() {
                                            LoginQuery {
                                                data: LoginOptions::MOBILE {
                                                    phone: phone.get(),
                                                    sms: sms_code.get(),
                                                },
                                            }
                                        } else {
                                            LoginQuery {
                                                data: LoginOptions::USER {
                                                    id: phone_or_huya.get(),
                                                    password: password.get(),
                                                },
                                            }
                                        },
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
                            <li class=css::register_title>快速注册</li>
                        </ul>
                        <div class=css::views>
                            <div class=css::pwd_view>
                                <input
                                    type="text"
                                    placeholder="手机号/虎牙号"
                                    bind:value=phone_or_huya
                                />
                                // <input type="password" placeholder="密码" bind:value=password />
                                <Password placeholder="密码".to_string() set_signal=password />
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
                                <div class=css::register_pwd>
                                    // <input bind:value=register_pwd type="password" placeholder="请设置6-20位至少两种字符组合密码"/>
                                    <Password
                                        placeholder="请设置6-20位至少两种字符组合密码"
                                            .to_string()
                                        set_signal=register_pwd
                                    />
                                </div>
                                <span>收不到验证码?</span>
                            </div>
                        </div>
                        <button type="submit" class=css::login_btn>
                            {move || if register.get() { "立即注册" } else { "登录" }}
                        </button>
                        <div class=css::privcy>
                            <label>
                                <input type="checkbox" bind:checked=agreed />
                            </label>
                            已阅读并同意
                            <a>"《用户服务协议》"</a>
                            和
                            <a>"《隐私政策》"</a>
                        </div>
                    </form>
                </div>
                <a on:click=move |_| {
                    set_register
                        .update(|data| {
                            *data = !*data;
                            *mode.write() = *data;
                        });
                }>{move || if register.get() { "账号/短信登录" } else { "快速注册" }}</a>
            </div>
        </div>
    }
}
