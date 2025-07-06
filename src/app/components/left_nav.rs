use crate::app::components::HotMatch;
use leptos::{either::Either, prelude::*};
use serde::{Deserialize, Serialize};

#[server]
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
            <li data-adv class="hidden relative">
                <a>
                    <img
                        src="/imgs/huya_1729682040_content.gif"
                        class="size-full rounded-xs"
                        alt=""
                    />
                </a>
                <button
                    class="absolute top-0 right-0 hover:text-white rounded-tr-xs size-5 bg-black/30 text-white/70"
                    on:click=move |_| {
                        set_show.set(false);
                    }
                >
                    X
                </button>
            </li>
        </Show>
    }
}

#[component]
pub fn LeftNav() -> impl IntoView {
    let get_data = LocalResource::new(|| get_left_nav());

    view! {
        <div class="inline-flex relative flex-col h-full text-left whitespace-nowrap text-[#aaaeb9] bg-[#2f3035]">
            <label class="group peer">
                <input class="hidden opacity-0 peer" type="checkbox" />
                <i class="inline-block absolute top-1/2 left-full w-3 rounded-full size-5 -translate-1/2 h-[138px] bg-[url(/imgs/left-close.png)] peer-checked:bg-[url(/imgs/left-open.png)] peer-checked:hover:bg-[url(/imgs/left-open-hover.png)] hover:bg-[url(/imgs/left-close-hover.png)]"></i>
            </label>

            <ul class="flex flex-col flex-auto gap-y-5 justify-start p-3 py-4 h-full text-xs duration-300 bar-y-hidden w-12.5 [&>li>h1]:hover:text-[#f80] [&>li>h1>i]:mr-0 [&>li>h1]:gap-y-2 [&>li>aside]:mt-3 [&>li>h1]:text-xs [&>li>aside]:hidden [&>li>h1>span]:hidden peer-has-checked:[&>li]:last:hidden peer-has-checked:[&>li]:data-adv:block peer-has-checked:[&>li>h1>i]:mr-2 peer-has-checked:[&>li>h1]:text-[16px] peer-has-checked:[&>li>h1]:flex-row peer-has-checked:w-60 peer-has-checked:[&>li>aside]:grid peer-has-checked:[&>li>h1>span]:inline-block">
                <li>
                    <h1 class="flex flex-col items-center group">
                        <i class="inline-block bg-cover hover:bg-left-bottom size-[21px] bg-[url(/imgs/heart.png)] group-hover:bg-[url(/imgs/heart-hover.png)]" />
                        <span>我的</span>
                        关注
                        <span>(请登录)</span>
                    </h1>
                </li>
                <li>
                    <h1 class="flex flex-col items-center group">
                        <i class="inline-block bg-cover hover:bg-left-bottom size-[21px] bg-[url(/imgs/all-live.png)] group-hover:bg-[url(/imgs/all-live-hover.png)]" />
                        <span>全部</span>
                        直播
                    </h1>
                </li>
                <li>
                    <h1 class="flex flex-col items-center group">
                        <i class="inline-block bg-cover hover:bg-left-bottom size-[21px] bg-[url(/imgs/match.png)] group-hover:bg-[url(/imgs/match-hover.png)]" />
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
                                            <h1 class="flex flex-col items-center hover:[&>i]:bg-left-bottom">
                                                <i
                                                    style=format!("--game-icon-url: url({});", icon)
                                                    class="inline-block bg-cover size-[21px] bg-[image:var(--game-icon-url)]"
                                                />
                                                {title.chars().take(2).collect::<String>()}
                                                <span>{title.chars().skip(2).collect::<String>()}</span>
                                            </h1>
                                            <aside class="grid grid-cols-3 gap-1 leading-7 text-center *:hover:text-[#f80] *:truncate *:bg-[#38393e] *:rounded-xs *:h-7 *:px-1">
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
                <li class="absolute bottom-10">
                    <h1 class="flex flex-col items-center group">
                        <i class="inline-block bg-cover hover:bg-left-bottom size-[21px] bg-[url(/imgs/left-cam.png)] group-hover:bg-[url(/imgs/left-cam-hover.png)]" />
                        开播
                    </h1>
                    <aside>list</aside>
                </li>
            </ul>

            <div class="hidden flex-col flex-none gap-y-3 justify-center items-center pt-5 pb-2 text-xs w-12.5 peer-has-checked:flex peer-has-checked:w-60">
                <div class="flex gap-3 text-white leading-[30px] [&>button>img]:w-4 [&>button>img]:h-[15px] *:bg-[#f80] *:rounded-3xl *:flex *:gap-x-1 *:items-center *:px-3">
                    <button>
                        <img src="/imgs/left-download.png" alt="" />
                        手机虎牙
                    </button>
                    <button>
                        <img src="/imgs/left-anchor.png" alt="" />
                        成为主播
                    </button>
                </div>
                <p class="flex gap-x-2">
                    <a href="">问题反馈</a>
                    |
                    <a href="">12318举报</a>
                </p>
            </div>
        </div>
    }
}
