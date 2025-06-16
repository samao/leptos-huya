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

async fn get_live_rooms() -> Vec<CategroyData<String>> {
    vec![
        CategroyData {
            id: 0,
            icon: "https://livewebbs2.msstatic.com/huya_hot_rec_theme_logo_1574217612.png".into(),
            title: "网游竞技".into(),
            tags: vec!["LOL".into(), "CS2".into(), "无畏契约".into(), "穿越火线".into(), "云顶之奕".into()],
            rooms: vec![
                RoomInfo {
                    id: 1,
                    img_url: "https://live-cover.msstatic.com/huyalive/1607945497-1607945497-6906073323365466112-3216014450-10057-A-0-1/20250616114327.jpg?x-oss-process=image/resize,limit_0,m_fill,w_224,h_126/sharpen,80/format,jpg/interlace,1/quality,q_90".into(),
                    room_name: "收点CF点东南西北部".into(),
                    cate_name: "穿越火线".into()
                },
                RoomInfo {
                    id: 1,
                    img_url: "https://live-cover.msstatic.com/huyalive/158454344-158454344-680556225389133824-317032144-10057-A-0-1/20250616114315.jpg?x-oss-process=image/resize,limit_0,m_fill,w_224,h_126/sharpen,80/format,jpg/interlace,1/quality,q_90".into(),
                    room_name: "爆裂者三环配件涡轮通风垂稳或耐久".into(),
                    cate_name: "坦克世界".into()
                },
                RoomInfo {
                    id: 1,
                    img_url: "https://anchorpost.msstatic.com/cdnimage/anchorpost/1005/9f/ecb953e0a2d14682aa16bc680b034c_4079_1722840231.jpg?spformat=png,webp&imageview/4/0/w/224/h/126/blur/1".into(),
                    room_name: "这群女的唱歌好难听".into(),
                    cate_name: "交友".into()
                }
            ]
        },
        CategroyData {
            id: 1,
            icon: "https://livewebbs2.msstatic.com/huya_hot_rec_theme_logo_1573549858.png".into(),
            title: "单机热游".into(),
            tags: vec!["天天吃鸡".into(), "主机游戏".into(), "黑神话：悟空".into(), "艾尔登法环".into()],
            rooms: vec![
                RoomInfo {
                    id: 1,
                    img_url: "https://anchorpost.msstatic.com/cdnimage/anchorpost/1058/92/36f269f4bf017fb06ac06784a3ef9c_2793_1749607021.jpg?spformat=png,webp&imageview/4/0/w/224/h/126/blur/1".into(),
                    room_name: "高手聚集地~嗨起来".into(),
                    cate_name: "天天吃鸡".into()
                },
                RoomInfo {
                    id: 1,
                    img_url: "https://live-cover.msstatic.com/huyalive/64757295-64757295-278130464202424320-129638046-10057-A-0-1/20250616185448.jpg?x-oss-process=image/resize,limit_0,m_fill,w_224,h_126/sharpen,80/format,jpg/interlace,1/quality,q_90".into(),
                    room_name: "【咩家军】神魔大陆Ⅱ弑神的男人！".into(),
                    cate_name: "方舟".into()
                },
                RoomInfo {
                    id: 1,
                    img_url: "https://tx-live-cover.msstatic.com/huyalive/1065660189-1065660189-4576975660404178944-2131443834-10057-A-0-1/20250616185445.jpg?x-oss-process=image/resize,limit_0,m_fill,w_224,h_126/sharpen,80/format,jpg/interlace,1/quality,q_90&sign=eWU1WoVLiXL+ru1w1d/2/zc615xhPTEyNTM0OTg3MDEmYj1odXlhLXNjcmVlbnNob3RzLXJldmlldy0xMjUzNDk4NzAxJms9QUtJRFFpcTNSbEJtV0p6ZUxKTVZrMklWdVEybm1pY2RkRWdEJmU9MTc2NTYyMzI5NSZ0PTE3NTAwNzEyOTUmcj0xMjM0NTY3OCZmPS9odXlhbGl2ZS8xMDY1NjYwMTg5LTEwNjU2NjAxODktNDU3Njk3NTY2MDQwNDE3ODk0NC0yMTMxNDQzODM0LTEwMDU3LUEtMC0xLzIwMjUwNjE2MTg1NDQ1LmpwZw==".into(),
                    room_name: "嘎嘎嘎".into(),
                    cate_name: "我的世界".into()
                }
            ]
        },
        CategroyData {
            id: 2,
            icon: "https://livewebbs2.msstatic.com/huya_hot_rec_theme_logo_1573549872.png".into(),
            title: "娱乐天地".into(),
            tags: vec!["户外".into(), "星秀".into(), "一起看".into(), "原创".into(), "体育".into()],
            rooms: vec![
                RoomInfo {
                    id: 1,
                    img_url: "https://anchorpost.msstatic.com/cdnimage/anchorpost/1021/ac/64545b499960c9aca70517c5f74a99_1663_1747725363.jpg?spformat=png,webp&imageview/4/0/w/224/h/126/blur/1".into(),
                    room_name: "我又回来了".into(),
                    cate_name: "星秀".into()
                },
                RoomInfo {
                    id: 1,
                    img_url: "https://tx-live-cover.msstatic.com/huyalive/1493553582-1493553582-6414763789513654272-2987230620-10057-A-0-1/20250616185440.jpg?x-oss-process=image/resize,limit_0,m_fill,w_224,h_126/sharpen,80/format,jpg/interlace,1/quality,q_90&sign=CeS+VjtjLE5retXUPvy9HK5x7aVhPTEyNTM0OTg3MDEmYj1odXlhLXNjcmVlbnNob3RzLXJldmlldy0xMjUzNDk4NzAxJms9QUtJRFFpcTNSbEJtV0p6ZUxKTVZrMklWdVEybm1pY2RkRWdEJmU9MTc2NTYyMzI4MCZ0PTE3NTAwNzEyODAmcj0xMjM0NTY3OCZmPS9odXlhbGl2ZS8xNDkzNTUzNTgyLTE0OTM1NTM1ODItNjQxNDc2Mzc4OTUxMzY1NDI3Mi0yOTg3MjMwNjIwLTEwMDU3LUEtMC0xLzIwMjUwNjE2MTg1NDQwLmpwZw==".into(),
                    room_name: "户外".into(),
                    cate_name: "拳击比赛".into()
                },
                RoomInfo {
                    id: 1,
                    img_url: "https://live-cover.msstatic.com/huyalive/1388472589-1388472589-5963444361147449344-2777068634-10057-A-0-1/20250616185440.jpg?x-oss-process=image/resize,limit_0,m_fill,w_224,h_126/sharpen,80/format,jpg/interlace,1/quality,q_90".into(),
                    room_name: "山鸡哥重现江湖".into(),
                    cate_name: "一起看".into()
                }
            ]
        },
        CategroyData {
            id: 3,
            icon: "https://livewebbs2.msstatic.com/huya_hot_rec_theme_logo_1573549866.png".into(),
            title: "手游休闲".into(),
            tags: vec!["王者荣耀".into(), "和平精英".into(), "三角洲行动".into(), "金铲铲之战".into(), "三国杀".into()],
            rooms: vec![
                RoomInfo {
                    id: 1,
                    img_url: "https://anchorpost.msstatic.com/cdnimage/anchorpost/1044/9e/e6934dea13ca6b5bb2a3c8170cf533_3_0_1749810600.jpg?spformat=png,webp&imageview/4/0/w/224/h/126/blur/1".into(),
                    room_name: "李梦源是有点本事在身上的！顶级瞄准！！".into(),
                    cate_name: "三角洲行动".into()
                },
                RoomInfo {
                    id: 1,
                    img_url: "https://tx-live-cover.msstatic.com/huyalive/1441444985-1441444985-6190959069558210560-2883013426-10057-A-0-1/20250616185500.jpg?x-oss-process=image/resize,limit_0,m_fill,w_224,h_126/sharpen,80/format,jpg/interlace,1/quality,q_90&sign=B/06XpmhEIb0uYp2ZRiudCUZ+gJhPTEyNTM0OTg3MDEmYj1odXlhLXNjcmVlbnNob3RzLXJldmlldy0xMjUzNDk4NzAxJms9QUtJRFFpcTNSbEJtV0p6ZUxKTVZrMklWdVEybm1pY2RkRWdEJmU9MTc2NTYyMzMwMCZ0PTE3NTAwNzEzMDAmcj0xMjM0NTY3OCZmPS9odXlhbGl2ZS8xNDQxNDQ0OTg1LTE0NDE0NDQ5ODUtNjE5MDk1OTA2OTU1ODIxMDU2MC0yODgzMDEzNDI2LTEwMDU3LUEtMC0xLzIwMjUwNjE2MTg1NTAwLmpwZw==".into(),
                    room_name: "全区互/带周常！忍3最牢/老主播茶仙！".into(),
                    cate_name: "忍者必须死3".into()
                },
                RoomInfo {
                    id: 1,
                    img_url: "https://live-cover.msstatic.com/huyalive/1629021815-1629021815-6996595419895562240-3258167086-10057-A-0-1-imgplus/20250616185443.jpg?x-oss-process=image/resize,limit_0,m_fill,w_224,h_126/sharpen,80/format,jpg/interlace,1/quality,q_90".into(),
                    room_name: "接任何~3X3~小拥代播".into(),
                    cate_name: "三角洲行动".into()
                }
            ]
        },
   ]
}

#[component]
pub fn LiveRooms() -> impl IntoView {
    let async_data = Resource::new(|| (), move |_| get_live_rooms());

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
                            <h1 class="flex items-center font-bold cursor-default text-[18px] before:bg-center before:mr-2 before:bg-contain before:bg-[image:var(--icon-url)] before:size-6.5 hover:text-[#ff8a00]">
                                {room_data.title}
                            </h1>
                            <ul class="flex whitespace-nowrap cursor-default *:hover:text-[#ff8a00]">
                                <For
                                    each=move || room_data.tags.clone().into_iter()
                                    key=|item| item.clone()
                                    let(tag)
                                >
                                    <li class="flex items-center before:content-['·'] before:px-2 first:before:hidden">
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
                                <div class="overflow-hidden relative rounded-md w-[224px] aspect-156/88 group after:absolute after:bottom-0 after:left-0 after:w-full after:h-7 after:bg-linear-to-t after:from-0% after:to-100% after:from-black/30 after:to-transparent">
                                    <img src=room_info.img_url alt="" class="w-full h-full" />
                                    <div class="flex absolute top-0 left-0 justify-center items-center w-full h-full bg-black opacity-0 duration-300 group-hover:opacity-60">
                                        <img
                                            width="40"
                                            height="40"
                                            class="opacity-0 duration-300 group-hover:opacity-100 group-hover:scale-100 scale-130"
                                            src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAyCAYAAAAeP4ixAAAAGXRFWHRTb2Z0d2FyZQBBZG9iZSBJbWFnZVJlYWR5ccllPAAAAyFpVFh0WE1MOmNvbS5hZG9iZS54bXAAAAAAADw/eHBhY2tldCBiZWdpbj0i77u/IiBpZD0iVzVNME1wQ2VoaUh6cmVTek5UY3prYzlkIj8+IDx4OnhtcG1ldGEgeG1sbnM6eD0iYWRvYmU6bnM6bWV0YS8iIHg6eG1wdGs9IkFkb2JlIFhNUCBDb3JlIDUuNS1jMDE0IDc5LjE1MTQ4MSwgMjAxMy8wMy8xMy0xMjowOToxNSAgICAgICAgIj4gPHJkZjpSREYgeG1sbnM6cmRmPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5LzAyLzIyLXJkZi1zeW50YXgtbnMjIj4gPHJkZjpEZXNjcmlwdGlvbiByZGY6YWJvdXQ9IiIgeG1sbnM6eG1wPSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvIiB4bWxuczp4bXBNTT0iaHR0cDovL25zLmFkb2JlLmNvbS94YXAvMS4wL21tLyIgeG1sbnM6c3RSZWY9Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC9zVHlwZS9SZXNvdXJjZVJlZiMiIHhtcDpDcmVhdG9yVG9vbD0iQWRvYmUgUGhvdG9zaG9wIENDIChXaW5kb3dzKSIgeG1wTU06SW5zdGFuY2VJRD0ieG1wLmlpZDpGNjc3QzBENEZBNTkxMUU2QjkyNzlGRkY5QzBBMkE1QSIgeG1wTU06RG9jdW1lbnRJRD0ieG1wLmRpZDpGNjc3QzBENUZBNTkxMUU2QjkyNzlGRkY5QzBBMkE1QSI+IDx4bXBNTTpEZXJpdmVkRnJvbSBzdFJlZjppbnN0YW5jZUlEPSJ4bXAuaWlkOkY2NzdDMEQyRkE1OTExRTZCOTI3OUZGRjlDMEEyQTVBIiBzdFJlZjpkb2N1bWVudElEPSJ4bXAuZGlkOkY2NzdDMEQzRkE1OTExRTZCOTI3OUZGRjlDMEEyQTVBIi8+IDwvcmRmOkRlc2NyaXB0aW9uPiA8L3JkZjpSREY+IDwveDp4bXBtZXRhPiA8P3hwYWNrZXQgZW5kPSJyIj8+7vnOlQAAB19JREFUeNrcWmtMVEcU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"
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
