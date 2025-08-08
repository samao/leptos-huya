use leptos::{either::Either, prelude::*};
use serde::{Deserialize, Serialize};

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

    stylance::import_crate_style!(css, "src/pages/home_page/hot_cate.module.scss");

    move || match get_data() {
        Ok(HotCateData {
            cates,
            streamers,
            game_set,
            live_count,
        }) => Either::Right(view! {
            <div class=css::hot_cate>
                <div class=css::cates>
                    <div class=css::head>
                        <h1 class=css::title_style>
                            <img src="/imgs/hot-cate.png" alt="" />
                            热门分类
                        </h1>
                        <ul class=css::game_set_clsx>
                            {game_set
                                .into_iter()
                                .map(|gset| {
                                    view! { <li>{gset.name}</li> }
                                })
                                .collect_view()}
                        </ul>
                        <p class=css::cate_status>
                            <span>当前共有</span>
                            <em>{live_count}</em>
                            <span>款游戏直播</span>
                            <a>"更多 >"</a>
                        </p>
                    </div>
                    <div class=css::cate_list>
                        {cates
                            .into_iter()
                            .map(|cate| {
                                view! {
                                    <div class=css::cate_clsx>
                                        <img class=css::cate_img src=cate.img_url alt="" />
                                        <span class=css::cate_name>{cate.name}</span>
                                        <span class=css::total>{cate.room_conut}场直播</span>
                                    </div>
                                }
                            })
                            .collect_view()}
                    </div>
                </div>
                <div class=css::anchors>
                    <div class=css::head>
                        <h1 class=css::title_style>
                            <img src="/imgs/hot-star.png" alt="" />
                            明星大神
                        </h1>
                        <a>"成为主播 >"</a>
                    </div>
                    <div class=css::anchors>
                        <ul class=css::inner_list>
                            {streamers
                                .into_iter()
                                .map(|streamer| {
                                    view! {
                                        <li class=css::item>
                                            <div class=css::avator>
                                                <img src=streamer.avator alt="" />
                                                <Show when=move || streamer.is_live>
                                                    <i class=css::live_icon_clsx />
                                                </Show>
                                            </div>
                                            <div class=css::info>
                                                <h1>{streamer.name}</h1>
                                                <p>{streamer.description}</p>
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
