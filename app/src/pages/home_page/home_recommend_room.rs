use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CategroyData<T: ToString + Sized + 'static> {
    id: u32,
    icon: T,
    title: T,
    tags: Vec<T>,
    rooms: Vec<RoomInfo<T>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoomInfo<T: ToString + Sized + 'static> {
    room_name: T,
    cate_name: T,
    img_url: T,
    id: u32,
}

#[server]
#[lazy]
async fn get_live_rooms() -> Result<Vec<CategroyData<String>>, ServerFnError> {
    Ok(vec![
        CategroyData {
            id: 0,
            icon: "/imgs/huya_hot_rec_theme_logo_1574217612.png".into(),
            title: "网游竞技".into(),
            tags: vec![
                "LOL".into(),
                "CS2".into(),
                "无畏契约".into(),
                "穿越火线".into(),
                "云顶之奕".into(),
            ],
            rooms: vec![
                RoomInfo {
                    id: 1,
                    img_url: "/imgs/live/20250616114327.jpg".into(),
                    room_name: "收点CF点东南西北部".into(),
                    cate_name: "穿越火线".into(),
                },
                RoomInfo {
                    id: 1,
                    img_url: "/imgs/live/1750922602.jpg".into(),
                    room_name: "爆裂者三环配件涡轮通风垂稳或耐久".into(),
                    cate_name: "坦克世界".into(),
                },
                RoomInfo {
                    id: 1,
                    img_url: "/imgs/live/1722840231.jpg".into(),
                    room_name: "这群女的唱歌好难听".into(),
                    cate_name: "交友".into(),
                },
            ],
        },
        CategroyData {
            id: 1,
            icon: "/imgs/huya_hot_rec_theme_logo_1573549858.png".into(),
            title: "单机热游".into(),
            tags: vec![
                "天天吃鸡".into(),
                "主机游戏".into(),
                "黑神话：悟空".into(),
                "艾尔登法环".into(),
            ],
            rooms: vec![
                RoomInfo {
                    id: 1,
                    img_url: "/imgs/live/1749607021.jpg".into(),
                    room_name: "高手聚集地~嗨起来".into(),
                    cate_name: "天天吃鸡".into(),
                },
                RoomInfo {
                    id: 1,
                    img_url: "/imgs/live/20250702222153.jpg".into(),
                    room_name: "【咩家军】神魔大陆Ⅱ弑神的男人！".into(),
                    cate_name: "方舟".into(),
                },
                RoomInfo {
                    id: 1,
                    img_url: "/imgs/live/20250616185445.jpg".into(),
                    room_name: "嘎嘎嘎".into(),
                    cate_name: "我的世界".into(),
                },
            ],
        },
        CategroyData {
            id: 2,
            icon: "/imgs/huya_hot_rec_theme_logo_1573549872.png".into(),
            title: "娱乐天地".into(),
            tags: vec![
                "户外".into(),
                "星秀".into(),
                "一起看".into(),
                "原创".into(),
                "体育".into(),
            ],
            rooms: vec![
                RoomInfo {
                    id: 1,
                    img_url: "/imgs/live/1747725363.jpg".into(),
                    room_name: "我又回来了".into(),
                    cate_name: "星秀".into(),
                },
                RoomInfo {
                    id: 1,
                    img_url: "/imgs/live/20250702222211.jpg".into(),
                    room_name: "户外".into(),
                    cate_name: "拳击比赛".into(),
                },
                RoomInfo {
                    id: 1,
                    img_url: "/imgs/live/20250616185440.jpg".into(),
                    room_name: "山鸡哥重现江湖".into(),
                    cate_name: "一起看".into(),
                },
            ],
        },
        CategroyData {
            id: 3,
            icon: "/imgs/huya_hot_rec_theme_logo_1573549866.png".into(),
            title: "手游休闲".into(),
            tags: vec![
                "王者荣耀".into(),
                "和平精英".into(),
                "三角洲行动".into(),
                "金铲铲之战".into(),
                "三国杀".into(),
            ],
            rooms: vec![
                RoomInfo {
                    id: 1,
                    img_url: "/imgs/live/1749810600.jpg".into(),
                    room_name: "李梦源是有点本事在身上的！顶级瞄准！！".into(),
                    cate_name: "三角洲行动".into(),
                },
                RoomInfo {
                    id: 1,
                    img_url: "/imgs/live/20250616185500.jpg".into(),
                    room_name: "全区互/带周常！忍3最牢/老主播茶仙！".into(),
                    cate_name: "忍者必须死3".into(),
                },
                RoomInfo {
                    id: 1,
                    img_url: "/imgs/live/1702631574.jpg".into(),
                    room_name: "接任何~3X3~小拥代播".into(),
                    cate_name: "三角洲行动".into(),
                },
            ],
        },
    ])
}

#[component]
pub fn LiveRooms() -> impl IntoView {
    let async_data = Resource::new(|| (), move |_| get_live_rooms());
    // let async_data = move || async_data.get().unwrap(Ok(vec![])).unwrap();
    stylance::import_crate_style!(css, "src/pages/home_page/home_recommend_room.module.scss");

    view! {
        <Suspense fallback=|| "loading rooms...">
            <div class=css::hot_rooms>
                <For
                    each=move || async_data.get().unwrap_or(Ok(vec![])).unwrap().into_iter()
                    key=|item| item.id
                    let(room_data)
                >
                    <div style=(
                        "--icon-url",
                        move || format!("url({:?})", room_data.icon.to_string()),
                    )>
                        <div class=css::cate>
                            <h1 class=css::room_title_clsx>{room_data.title}</h1>
                            <ul class=css::cate_tags>
                                <For
                                    each=move || room_data.tags.clone().into_iter()
                                    key=|item| item.clone()
                                    let(tag)
                                >
                                    <li class=css::tag>{tag}</li>
                                </For>
                            </ul>
                        </div>
                        <div class=css::cate_rooms>
                            <For
                                each=move || room_data.rooms.clone().into_iter()
                                key=|room| room.id
                                let(room_info)
                            >
                                <div class=css::room_container_clsx>
                                    <img src=room_info.img_url alt="" class=css::cover />
                                    <div class=css::room_img_play_clsx>
                                        <img width="40" height="40" src="/imgs/play.png" />
                                    </div>
                                    <span class=css::cate_name>{room_info.cate_name}</span>
                                    <span class=css::room_name>{room_info.room_name}</span>
                                </div>
                            </For>
                        </div>
                    </div>
                </For>
            </div>
        </Suspense>
    }
}
