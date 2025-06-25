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
                img_url: "https://huyaimg.msstatic.com/cdnimage/game/1-MS.png".to_string(),
                name: "英雄联盟".to_string(),
                room_conut: 465,
            },
            CateLink {
                id: 1,
                img_url: "https://huyaimg.msstatic.com/cdnimage/game/2336-MS.png".to_string(),
                name: "王者荣耀".to_string(),
                room_conut: 495,
            },
            CateLink {
                id: 2,
                img_url: "https://huyaimg.msstatic.com/cdnimage/game/2165-MS.png".to_string(),
                name: "户外".to_string(),
                room_conut: 322,
            },
            CateLink {
                id: 3,
                img_url: "https://huyaimg.msstatic.com/cdnimage/game/2356-MS.png".to_string(),
                name: "体育".to_string(),
                room_conut: 495,
            },
            CateLink {
                id: 4,
                img_url: "https://huyaimg.msstatic.com/cdnimage/game/862-MS.png".to_string(),
                name: "CS2".to_string(),
                room_conut: 495,
            },
            CateLink {
                id: 5,
                img_url: "https://huyaimg.msstatic.com/cdnimage/game/4-MS.png".to_string(),
                name: "穿越火线".to_string(),
                room_conut: 495,
            },
            CateLink {
                id: 6,
                img_url: "https://huyaimg.msstatic.com/cdnimage/game/2135-MS.png".to_string(),
                name: "一起看".to_string(),
                room_conut: 495,
            },
            CateLink {
                id: 7,
                img_url: "https://huyaimg.msstatic.com/cdnimage/game/2793-MS.png".to_string(),
                name: "天天吃鸡".to_string(),
                room_conut: 495,
            },
            CateLink {
                id: 8,
                img_url: "https://huyaimg.msstatic.com/cdnimage/game/1663-MS.png".to_string(),
                name: "星秀".to_string(),
                room_conut: 495,
            },
            CateLink {
                id: 9,
                img_url: "https://huyaimg.msstatic.com/cdnimage/game/5937-MS.png".to_string(),
                name: "无畏契约".to_string(),
                room_conut: 495,
            },
            CateLink {
                id: 10,
                img_url: "https://huyaimg.msstatic.com/cdnimage/game/5485-MS.png".to_string(),
                name: "lol云顶之奕".to_string(),
                room_conut: 495,
            },
            CateLink {
                id: 11,
                img_url: "https://huyaimg.msstatic.com/cdnimage/game/100032-MS.png".to_string(),
                name: "主机游戏".to_string(),
                room_conut: 495,
            },
        ]
        .to_vec(),
        streamers: [
            Streamer {
                avator: "https://livewebbs2.msstatic.com/huya_famous_b_1391089114_s1465291548.jpg"
                    .to_string(),
                is_live: true,
                name: "安德罗妮".to_string(),
                description: "炉石传说一哥，神抽狗协会会长，麾下十万军师".to_string(),
            },
            Streamer {
                avator: "https://livewebbs2.msstatic.com/huya_famous_b_1131992426_s1634815305.jpg"
                    .to_string(),
                is_live: true,
                name: "鲨鱼哟".to_string(),
                description: "颜值代表，背心战神，拉枪线第一人".to_string(),
            },
            Streamer {
                avator: "https://livewebbs2.msstatic.com/huya_famous_b_976431143_s1517479364.jpg"
                    .to_string(),
                is_live: false,
                name: "拉风龙".to_string(),
                description: "人称“吃鸡总教练”，电竞顺风车励志龙。".to_string(),
            },
            Streamer {
                avator: "https://livewebbs2.msstatic.com/huya_famous_b_635563237_s1464780406.jpg"
                    .to_string(),
                is_live: false,
                name: "Yumiko".to_string(),
                description: "前魔兽3职业选手，WC3L世界总决赛亚军".to_string(),
            },
            Streamer {
                avator: "https://livewebbs2.msstatic.com/huya_famous_b_917707_s1464780836.jpg"
                    .to_string(),
                is_live: false,
                name: "董小飒".to_string(),
                description: "连续两届YSL冠军得主，年度最受欢迎男主播".to_string(),
            },
            Streamer {
                avator: "https://livewebbs2.msstatic.com/huya_famous_b_2120076550_s1747120275.png"
                    .to_string(),
                is_live: false,
                name: "杨齐家丶".to_string(),
                description: "三角洲扶贫王，拯救大兵计划发起人".to_string(),
            },
            Streamer {
                avator: "https://livewebbs2.msstatic.com/huya_famous_b_442654609_s1512648787.jpg"
                    .to_string(),
                is_live: false,
                name: "韦神".to_string(),
                description: "韦神，LPL冠军；绝地求生4AM战队核心".to_string(),
            },
            Streamer {
                avator: "https://livewebbs2.msstatic.com/huya_famous_b_20540844_s1723811606.jpg"
                    .to_string(),
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

    let title_style = "text-[26px]/[33px] text-white font-bold flex items-end gap-x-3 justify-start hover:text-[#f80] *:[img]:size-8 *:[img]:bg-center *:[img]:bg-no-repeat";
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
                            <img
                                src="data:image/png;base64,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"
                                alt=""
                            />
                            热门分类
                        </h1>
                        <ul class="flex gap-x-1.5 leading-7 *:px-4 *:hover:text-[#f80] *:rounded-2xl *:hover:border-current *:bg-[#e2e2e2] *:border-[#e2e2e2]">
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
                                    <div class="flex flex-col justify-center items-center bg-white rounded-md group/cate-card w-[125px] h-[147px] min-[1440px]:w-[137px] min-[1440px]:h-[167px]">
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
                            <img
                                src="data:image/png;base64,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"
                                alt=""
                            />
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
                                                    <i class="inline-block overflow-hidden absolute right-0 bottom-0 rounded-full size-[18px] bg-[#f80] animate-living bg-[url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAH4AAAASCAYAAACdFWqpAAAAGXRFWHRTb2Z0d2FyZQBBZG9iZSBJbWFnZVJlYWR5ccllPAAAA3ZpVFh0WE1MOmNvbS5hZG9iZS54bXAAAAAAADw/eHBhY2tldCBiZWdpbj0i77u/IiBpZD0iVzVNME1wQ2VoaUh6cmVTek5UY3prYzlkIj8+IDx4OnhtcG1ldGEgeG1sbnM6eD0iYWRvYmU6bnM6bWV0YS8iIHg6eG1wdGs9IkFkb2JlIFhNUCBDb3JlIDUuNi1jMDY3IDc5LjE1Nzc0NywgMjAxNS8wMy8zMC0yMzo0MDo0MiAgICAgICAgIj4gPHJkZjpSREYgeG1sbnM6cmRmPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5LzAyLzIyLXJkZi1zeW50YXgtbnMjIj4gPHJkZjpEZXNjcmlwdGlvbiByZGY6YWJvdXQ9IiIgeG1sbnM6eG1wTU09Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC9tbS8iIHhtbG5zOnN0UmVmPSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvc1R5cGUvUmVzb3VyY2VSZWYjIiB4bWxuczp4bXA9Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC8iIHhtcE1NOk9yaWdpbmFsRG9jdW1lbnRJRD0ieG1wLmRpZDowNzQ4ZDMyZi0zZGM0LTc1NDAtYTAyNi01ZGQ3MGRmYWU5MTgiIHhtcE1NOkRvY3VtZW50SUQ9InhtcC5kaWQ6QjA1MjU0RjBERTUxMTFFN0FCNThERTJEMUUzODBFMzEiIHhtcE1NOkluc3RhbmNlSUQ9InhtcC5paWQ6QjA1MjU0RUZERTUxMTFFN0FCNThERTJEMUUzODBFMzEiIHhtcDpDcmVhdG9yVG9vbD0iQWRvYmUgUGhvdG9zaG9wIENDIDIwMTUgKFdpbmRvd3MpIj4gPHhtcE1NOkRlcml2ZWRGcm9tIHN0UmVmOmluc3RhbmNlSUQ9InhtcC5paWQ6YjUyMzcxNDctMGZiMi03ODQzLTk4NzgtNTQzZjVjMjcyNTg5IiBzdFJlZjpkb2N1bWVudElEPSJ4bXAuZGlkOjA3NDhkMzJmLTNkYzQtNzU0MC1hMDI2LTVkZDcwZGZhZTkxOCIvPiA8L3JkZjpEZXNjcmlwdGlvbj4gPC9yZGY6UkRGPiA8L3g6eG1wbWV0YT4gPD94cGFja2V0IGVuZD0iciI/Pkuwn9gAAABySURBVHja7NlBDoAwCABBy///jBcfYIwiymzSa6FsoGm6MnPDPEIJiAfxIB5jxeexutI9v5b1iA8ImiK29JyhQDMnR7woqLrQVfHOxrkrn0v7hNE6cwJF485xFz+Yr+ec5xwmsXzS6HgQD+JBPP7DLsAAoN0v8x/NfHMAAAAASUVORK5CYII=)]" />
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
