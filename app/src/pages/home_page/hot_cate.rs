use leptos::{either::Either, prelude::*};
use serde::{Deserialize, Serialize};

use crate::clsx;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct CateLink<T>
where
    T: ToString,
{
    id: u32,
    img_url: T,
    name: T,
    room_conut: u32,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
struct Streamer<T>
where
    T: ToString,
{
    avator: T,
    name: T,
    description: T,
    is_live: bool,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
struct GameSet<T>
where
    T: ToString,
{
    name: T,
    gtype: T,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
struct HotCateData<T>
where
    T: ToString,
{
    cates: Vec<CateLink<T>>,
    streamers: Vec<Streamer<T>>,
    game_set: Vec<GameSet<T>>,
    live_count: u32,
}

#[server]
async fn get_hot_cate() -> Result<HotCateData<String>, ServerFnError> {
    Ok(HotCateData {
        live_count: 478,
        cates: [
            CateLink {
                id: 0,
                img_url: "/imgs/game/1-MS.png".to_string(),
                name: "英雄联盟".to_string(),
                room_conut: 465,
            },
            CateLink {
                id: 1,
                img_url: "/imgs/game/2336-MS.png".to_string(),
                name: "王者荣耀".to_string(),
                room_conut: 495,
            },
            CateLink {
                id: 2,
                img_url: "/imgs/game/2165-MS.png".to_string(),
                name: "户外".to_string(),
                room_conut: 322,
            },
            CateLink {
                id: 3,
                img_url: "/imgs/game/2356-MS.png".to_string(),
                name: "体育".to_string(),
                room_conut: 495,
            },
            CateLink {
                id: 4,
                img_url: "/imgs/game/862-MS.png".to_string(),
                name: "CS2".to_string(),
                room_conut: 495,
            },
            CateLink {
                id: 5,
                img_url: "/imgs/game/4-MS.png".to_string(),
                name: "穿越火线".to_string(),
                room_conut: 495,
            },
            CateLink {
                id: 6,
                img_url: "/imgs/game/2135-MS.png".to_string(),
                name: "一起看".to_string(),
                room_conut: 495,
            },
            CateLink {
                id: 7,
                img_url: "/imgs/game/2793-MS.png".to_string(),
                name: "天天吃鸡".to_string(),
                room_conut: 495,
            },
            CateLink {
                id: 8,
                img_url: "/imgs/game/1663-MS.png".to_string(),
                name: "星秀".to_string(),
                room_conut: 495,
            },
            CateLink {
                id: 9,
                img_url: "/imgs/game/5937-MS.png".to_string(),
                name: "无畏契约".to_string(),
                room_conut: 495,
            },
            CateLink {
                id: 10,
                img_url: "/imgs/game/5485-MS.png".to_string(),
                name: "lol云顶之奕".to_string(),
                room_conut: 495,
            },
            CateLink {
                id: 11,
                img_url: "/imgs/game/100032-MS.png".to_string(),
                name: "主机游戏".to_string(),
                room_conut: 495,
            },
        ]
        .to_vec(),
        streamers: [
            Streamer {
                avator: "/imgs/user/huya_famous_b_1391089114_s1465291548.jpg".to_string(),
                is_live: true,
                name: "安德罗妮".to_string(),
                description: "炉石传说一哥，神抽狗协会会长，麾下十万军师".to_string(),
            },
            Streamer {
                avator: "/imgs/user/huya_famous_b_1131992426_s1634815305.jpg".to_string(),
                is_live: true,
                name: "鲨鱼哟".to_string(),
                description: "颜值代表，背心战神，拉枪线第一人".to_string(),
            },
            Streamer {
                avator: "/imgs/user/huya_famous_b_976431143_s1517479364.jpg".to_string(),
                is_live: false,
                name: "拉风龙".to_string(),
                description: "人称“吃鸡总教练”，电竞顺风车励志龙。".to_string(),
            },
            Streamer {
                avator: "/imgs/user/huya_famous_b_635563237_s1464780406.jpg".to_string(),
                is_live: false,
                name: "Yumiko".to_string(),
                description: "前魔兽3职业选手，WC3L世界总决赛亚军".to_string(),
            },
            Streamer {
                avator: "/imgs/user/huya_famous_b_917707_s1464780836.jpg".to_string(),
                is_live: false,
                name: "董小飒".to_string(),
                description: "连续两届YSL冠军得主，年度最受欢迎男主播".to_string(),
            },
            Streamer {
                avator: "/imgs/user/huya_famous_b_2120076550_s1747120275.png".to_string(),
                is_live: false,
                name: "杨齐家丶".to_string(),
                description: "三角洲扶贫王，拯救大兵计划发起人".to_string(),
            },
            Streamer {
                avator: "/imgs/user/huya_famous_b_442654609_s1512648787.jpg".to_string(),
                is_live: false,
                name: "韦神".to_string(),
                description: "韦神，LPL冠军；绝地求生4AM战队核心".to_string(),
            },
            Streamer {
                avator: "/imgs/user/huya_famous_b_20540844_s1723811606.jpg".to_string(),
                is_live: false,
                name: "CSBOY".to_string(),
                description: "CSGO人气组合".to_string(),
            },
        ]
        .to_vec(),
        game_set: [
            GameSet {
                gtype: "ol".to_string(),
                name: "网游竞技".to_string(),
            },
            GameSet {
                gtype: "pc".to_string(),
                name: "单机热游".to_string(),
            },
            GameSet {
                gtype: "yl".to_string(),
                name: "娱乐天地".to_string(),
            },
            GameSet {
                gtype: "sy".to_string(),
                name: "手游休闲".to_string(),
            },
        ]
        .to_vec(),
    })
}

#[component]
pub fn HotCate() -> impl IntoView {
    let get_data = LocalResource::new(async || get_hot_cate().await);
    let get_data = move || {
        get_data
            .get()
            .unwrap_or(Err(ServerFnError::ServerError("loading".to_string())))
    };

    let title_style = clsx! {
        "text-[26px]/[33px] text-white font-bold flex items-end gap-x-3 justify-start",
        "hover:text-[#f80] *:[img]:size-8 *:[img]:bg-center *:[img]:bg-no-repeat"
    };
    let game_set_clsx = clsx! {
        "flex gap-x-1.5 leading-7 *:px-4 *:hover:text-[#f80] *:rounded-2xl",
        "*:hover:border-current *:bg-[#e2e2e2] *:border-[#e2e2e2]"
    };
    let cate_clsx = clsx! {
        "flex flex-col justify-center items-center bg-white rounded-md group/cate-card w-[125px] h-[147px]",
        "min-[1440px]:w-[137px] min-[1440px]:h-[167px]"
    };
    let live_icon_clsx = clsx! {
        "inline-block overflow-hidden absolute right-0 bottom-0 rounded-full size-[18px]",
        "bg-[#f80] animate-living bg-[url(/imgs/live-icon.png)]"
    };
    move || match get_data() {
        Ok(HotCateData {
            cates,
            streamers,
            game_set,
            live_count,
        }) => Either::Right(view! {
            <div class="flex gap-x-5 text-[14px] size-full **:[a]:hover:text-[#f80] **:[a]:duration-300">
                <div class="flex flex-col justify-between text-[#333]">
                    <div class="flex relative gap-x-2 justify-start items-center font-[14px]">
                        <h1 class=title_style>
                            <img src="/imgs/hot-cate.png" alt="" />
                            热门分类
                        </h1>
                        <ul class=game_set_clsx>
                            {game_set
                                .into_iter()
                                .map(|gset| {
                                    view! { <li>{gset.name}</li> }
                                })
                                .collect_view()}
                        </ul>
                        <p class="absolute right-0 text-white max-[1440px]:*:not-[a]:hidden">
                            <span>当前共有</span>
                            <em class="px-1.5 text-[#f80]">{live_count}</em>
                            <span>款游戏直播</span>
                            <a class="pl-2.5">"更多 >"</a>
                        </p>
                    </div>
                    <div class="grid grid-cols-5 gap-4 min-[1440px]:grid-cols-6 max-[1440px]:*:nth-[n+11]:hidden">
                        {cates
                            .into_iter()
                            .map(|cate| {
                                view! {
                                    <div class=cate_clsx>
                                        <img
                                            class="mb-2 size-19 min-[1440px]:size-22"
                                            src=cate.img_url
                                            alt=""
                                        />
                                        <span class="group-hover/cate-card:text-[#f80]">
                                            {cate.name}
                                        </span>
                                        <span class="text-gray-400">
                                            {cate.room_conut}场直播
                                        </span>
                                    </div>
                                }
                            })
                            .collect_view()}
                    </div>
                </div>
                <div class="flex flex-col justify-between w-[290px]">
                    <div class="flex gap-x-2 justify-between items-center text-white text-[14px]">
                        <h1 class=title_style>
                            <img src="/imgs/hot-star.png" alt="" />
                            明星大神
                        </h1>
                        <a>"成为主播 >"</a>
                    </div>
                    <div class="px-3 text-left bg-white rounded-md bar-y-hidden h-[310px] min-[1440px]:h-[350px]">
                        <ul class="*:not-first:mt-2 *:rounded-md *:hover:bg-gray-200">
                            {streamers
                                .into_iter()
                                .map(|streamer| {
                                    view! {
                                        <li class="flex relative gap-x-2 p-1">
                                            <div class="relative flex-none size-16">
                                                <img
                                                    class="rounded-full size-full"
                                                    src=streamer.avator
                                                    alt=""
                                                />
                                                <Show when=move || streamer.is_live>
                                                    <i class=live_icon_clsx />
                                                </Show>
                                            </div>
                                            <div class="flex flex-col flex-auto justify-start py-1">
                                                <h1 class="font-bold">{streamer.name}</h1>
                                                <p class="text-xs text-gray-400 line-clamp-2">
                                                    {streamer.description}
                                                </p>
                                            </div>
                                        </li>
                                    }
                                })
                                .collect_view()}
                        </ul>
                    </div>
                </div>
            </div>
        }),
        Err(_err) => Either::Left(format!("loading")),
    }
}
