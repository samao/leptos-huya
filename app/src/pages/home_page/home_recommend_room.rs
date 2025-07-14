use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::clsx;

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

async fn get_live_rooms() -> Vec<CategroyData<String>> {
    vec![
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
    ]
}

#[component]
pub fn LiveRooms() -> impl IntoView {
    let async_data = Resource::new(|| (), move |_| get_live_rooms());
    let room_title_clsx = clsx! {
        "flex items-center font-bold cursor-default text-[18px]",
        "before:bg-center before:mr-2 before:bg-contain before:bg-[image:var(--icon-url)] before:size-6.5 hover:text-[#ff8a00]"
    };
    let room_container_clsx = clsx! {
        "overflow-hidden relative rounded-md w-[224px] aspect-156/88 group",
        "after:absolute after:bottom-0 after:left-0 after:w-full after:h-7 after:bg-linear-to-t",
        "after:from-0% after:to-100% after:from-black/30 after:to-transparent"
    };
    let room_img_play_clsx = clsx! {
        "flex absolute top-0 left-0 justify-center items-center w-full h-full",
        "bg-black opacity-0 duration-300 group-hover:opacity-60"
    };
    view! {
        <Suspense fallback=|| "loading rooms...">
            <div class="grid grid-cols-2 gap-x-3 gap-y-4 mt-10 mb-8 text-xs">
                <For
                    each=move || async_data.get().unwrap().into_iter()
                    key=|item| item.id
                    let(room_data)
                >
                    <div style=(
                        "--icon-url",
                        move || format!("url({:?})", room_data.icon.to_string()),
                    )>
                        <div class="flex gap-x-2.5 items-center mb-2 text-[#555]">
                            <h1 class=room_title_clsx>
                                {room_data.title}
                            </h1>
                            <ul class="flex whitespace-nowrap cursor-default *:hover:text-[#ff8a00]">
                                <For
                                    each=move || room_data.tags.clone().into_iter()
                                    key=|item| item.clone()
                                    let(tag)
                                >
                                    <li class="flex items-center before:text-[#555] before:content-['·'] before:px-2 first:before:hidden">
                                        {tag}
                                    </li>
                                </For>
                            </ul>
                        </div>
                        <div class="flex gap-x-2 text-white">
                            <For
                                each=move || room_data.rooms.clone().into_iter()
                                key=|room| room.id
                                let(room_info)
                            >
                                <div class=room_container_clsx>
                                    <img src=room_info.img_url alt="" class="w-full h-full" />
                                    <div class=room_img_play_clsx>
                                        <img
                                            width="40"
                                            height="40"
                                            class="opacity-0 duration-300 group-hover:opacity-100 group-hover:scale-100 scale-130"
                                            src="/imgs/play.png"
                                        />
                                    </div>
                                    <span class="hidden absolute top-1 right-2 px-2 leading-5 rounded-md group-hover:block bg-black/50">
                                        {room_info.cate_name}
                                    </span>
                                    <span class="inline-block absolute bottom-1 left-2 z-10 w-4/5 text-left truncate">
                                        {room_info.room_name}
                                    </span>
                                </div>
                            </For>
                        </div>
                    </div>
                </For>
            </div>
        </Suspense>
    }
}
