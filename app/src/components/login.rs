use leptos::prelude::*;
use reactive_stores::Store;
use serde::{Deserialize, Serialize};
use web_sys::MouseEvent;

use crate::{GlobalState, GlobalStateStoreFields};

#[derive(Debug, Deserialize, Serialize)]
struct ThirdPart {
    icon: String,
    url: String,
}

#[component]
pub fn Login() -> impl IntoView {
    stylance::import_crate_style!(css, "src/components/login.module.scss");
    let store = use_context::<Store<GlobalState>>();
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
                <div>
                    <p>"打开「虎牙直播APP-我的」扫码登录"</p>
                    <div>
                        <img src="https://udblgn.huya.com/qrLgn/getQrImg?k=DuYMYTiKpGpDdfSjRGG&appId=5002" />
                        // hover tips
                        <div />

                        <div>二维码已过期 <button>点击刷新</button></div>
                    </div>
                    <div>
                        <hr />
                        <span>其他登录方式</span>
                    </div>
                    <ul>
                        <For
                            each=|| {
                                vec![
                                    ThirdPart {
                                        icon: "".to_string(),
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
                <div>
                    <div>
                        <input type="checkbox" />
                        <form>
                            <ul>
                                <li>
                                    密码登录 <div>
                                        <input type="text" placeholder="手机号/虎牙号" />
                                        <input type="text" placeholder="密码" />
                                        <span>忘记密码?</span>
                                    </div>
                                </li>
                                <li>
                                    短信登录 <div>
                                        <div>
                                            <select name="pcode">
                                                <For
                                                    each=|| vec!["+86", "+13"].into_iter()
                                                    key=|item| item.to_string()
                                                    let(code)
                                                >
                                                    <option value=code>{code}</option>
                                                </For>
                                            </select>
                                            <input type="text" placeholder="请输入手机号" />
                                        </div>
                                        <div>
                                            <input type="text" placeholder="请输入验证码" />
                                            <span>获取验证码</span>
                                        </div>
                                        <span>收不到验证码?</span>
                                    </div>
                                </li>
                            </ul>
                            <button type="submit">登录</button>
                            <div>
                                <input type="checkbox" />
                                已阅读并同意
                                <a>"《用户服务协议》"</a>
                                和
                                <a>"《隐私政策》"</a>
                            </div>
                        </form>
                    </div>
                </div>
                <a>快速注册</a>
            </div>

        </div>
    }
}
