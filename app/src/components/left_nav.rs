use crate::components::HotMatch;
use leptos::{either::Either, prelude::*};
use serde::{Deserialize, Serialize};

stylance::import_crate_style!(css, "src/components/left_nav.module.scss");

#[server]
#[lazy]
async fn get_left_nav() -> Result<Vec<BigCate<String>>, ServerFnError> {
    let all_cate = vec![
        BigCate {
            icon: "/imgs/php9KLDhu1570589492.png",
            title: "网游竞技",
            tags: vec![
                "英雄联盟",
                "CS2",
                "无畏契约",
                "云顶之弈",
                "穿越火线",
                "DOTA2",
                "炉石传说",
                "逆战",
                "暗区突围：无限",
                "DOTA1",
                "和平精英",
                "无畏契约：源能行动",
            ],
        },
        BigCate {
            icon: "/imgs/phpEmn7BH1570589502.png",
            title: "单机热游",
            tags: vec![
                "天天吃鸡",
                "主机游戏",
                "黑神话悟空",
                "我的世界",
                "逃离塔科夫",
                "互动点播",
                "解限机",
                "Rematch！开球",
                "彩虹六号",
            ],
        },
        BigCate {
            icon: "/imgs/php5SJeiF1570589511.png",
            title: "娱乐天地",
            tags: vec![
                "户外",
                "体育",
                "一起看",
                "星秀",
                "原创",
                "二次元",
                "交友",
                "颜值",
                "上分",
            ],
        },
        BigCate {
            icon: "/imgs/phpXBYqMz1570589520.png",
            title: "手游休闲",
            tags: vec![
                "王者荣耀",
                "和平精英",
                "三角洲行动",
                "金铲铲之战",
                "棋牌休闲",
                "新游广场",
                "QQ飞车手游",
                "火影忍者手游",
                "逆水寒手游",
                "CF手游",
                "第五人格",
                "好游鉴赏",
                "群星纪元",
                "LOL手游",
                "DNF手游",
            ],
        },
    ]
    .into_iter()
    .map(|cate| cate.into())
    .collect();
    Ok(all_cate)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct BigCate<T>
where
    T: ToString,
{
    icon: T,
    title: T,
    tags: Vec<T>,
}

impl From<BigCate<&str>> for BigCate<String> {
    fn from(BigCate { icon, title, tags }: BigCate<&str>) -> Self {
        BigCate {
            icon: icon.to_string(),
            title: title.to_string(),
            tags: tags.iter().map(|tag| tag.to_string()).collect(),
        }
    }
}

#[component]
fn LeftAdv() -> impl IntoView {
    let (show, set_show) = signal(true);

    view! {
        <Show when=move || show.get()>
            <li data-adv class=css::adv>
                <a>
                    <img src="/imgs/huya_1729682040_content.gif" alt="" />
                </a>
                <button on:click=move |_| {
                    set_show.set(false);
                }>X</button>
            </li>
        </Show>
    }
}

#[component]
pub fn LeftNav() -> impl IntoView {
    let get_data = LocalResource::new(|| get_left_nav());
    view! {
        <div class=css::left_nav>
            <label class=format!("{} {}", css::group, css::peer)>
                <input class=format!("{} {}", css::handle, css::peer) type="checkbox" />
                <i class=css::collapse_btn_clsx></i>
            </label>

            <ul class=css::navs_container_clsx>
                <li>
                    <h1 class=format!("{} {}", css::group, css::title)>
                        <i class=css::icon />
                        <span>我的</span>
                        关注
                        <span>(请登录)</span>
                    </h1>
                </li>
                <li>
                    <h1 class=format!("{} {}", css::group, css::title)>
                        <i class=format!("{} {}", css::icon, css::live) />
                        <span>全部</span>
                        直播
                    </h1>
                </li>
                <li>
                    <h1 class=format!("{} {}", css::group, css::title)>
                        <i class=format!("{} {}", css::icon, css::game) />
                        赛事
                        <span>直播</span>
                    </h1>
                    <aside>
                        <HotMatch />
                    </aside>
                </li>
                <Suspense fallback=|| {
                    "..."
                }>
                    {move || match get_data.get() {
                        Some(Ok(cates)) => {
                            Either::Left(
                                view! {
                                    <For
                                        each=move || cates.clone().into_iter()
                                        key=|item| item.icon.clone()
                                        let(BigCate { icon, title, tags })
                                    >
                                        <li>
                                            <h1 class=css::cate_item>
                                                <i
                                                    style=format!("--game-icon-url: url({});", icon)
                                                    class=css::icon
                                                />
                                                {title.chars().take(2).collect::<String>()}
                                                <span>{title.chars().skip(2).collect::<String>()}</span>
                                            </h1>
                                            <aside class=css::item_body>
                                                <For
                                                    each=move || tags.clone().into_iter()
                                                    key=|tag| tag.clone()
                                                    let(tag)
                                                >
                                                    <span>{tag}</span>
                                                </For>
                                            </aside>
                                        </li>
                                    </For>
                                },
                            )
                        }
                        _ => Either::Right(view! { "###" }),
                    }}
                </Suspense>
                <LeftAdv />
                <li class=css::start>
                    <h1 class=format!("{} {}", css::group, css::title)>
                        <i class=format!("{} {}", css::icon, css::cam) />
                        开播
                    </h1>
                    <aside>list</aside>
                </li>
            </ul>

            <div class=css::tools>
                <div class=css::tool_inner>
                    <button>
                        <img src="/imgs/left-download.png" alt="" />
                        手机虎牙
                    </button>
                    <button>
                        <img src="/imgs/left-anchor.png" alt="" />
                        成为主播
                    </button>
                </div>
                <p class=css::btns>
                    <a href="">问题反馈</a>
                    |
                    <a href="">12318举报</a>
                </p>
            </div>
        </div>
    }
}
