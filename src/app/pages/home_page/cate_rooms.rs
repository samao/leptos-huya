use leptos::{prelude::*, task::spawn_local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Cate<T: ToString> {
    id: u32,
    icon_url: T,
    cate_name: T,
    #[serde(default)]
    tags: Vec<HotRoom<T>>,
    live_total: u32,
    #[serde(default)]
    rooms: Vec<Room<T>>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct HotRoom<T: ToString> {
    id: usize,
    name: T,
    #[serde(default)]
    is_live: bool,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Room<T: ToString> {
    id: usize,
    img_url: T,
    name: T,
    hot: u64,
    owner: User<T>,
    #[serde(default)]
    tags: Vec<Tag<T>>,
    is_live: bool,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct User<T: ToString> {
    id: usize,
    name: T,
    avator_url: T,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
enum Tag<T: ToString> {
    // 蓝光
    Blue,
    Play(T),
    Offical(T),
}

async fn get_recommend_cate_rooms() -> Result<Vec<Cate<&'static str>>, ServerFnError> {
    let cates = vec![
        Cate {
            id: 0,
            icon_url:
                "https://huyaimg.msstatic.com/cdnimage/gametypelogo/game_15735501549253_logo.png",
            cate_name: "英雄联盟",
            tags: [
                "卡尔",
                "骚男",
                "Letme",
                "mlxg",
                "姿态",
                "青蛙",
                "星痕",
                "霸哥",
                "文森特",
            ]
            .into_iter()
            .enumerate()
            .map(|(id, name)| HotRoom { name, id, is_live: id == 0 })
            .collect::<Vec<HotRoom<&'static str>>>(),
            live_total: 987,
            rooms: [
                (
                    ("小超梦ovo", "https://huyaimg.msstatic.com/avatar/1009/e4/837d557cdf07c6adf85a62540ce53d_180_135.jpg?1692469066"), 
                    "已王者 大师顶级教学看了包上分！", 909000,"https://live-cover.msstatic.com/huyalive/2368274334-2368274334-10171660812486180864-4736672124-10057-A-0-1-imgplus/20250623103935.jpg?x-oss-process=image/resize,limit_0,m_fill,w_338,h_190/sharpen,80/format,webp/quality,q_90"
                ),
                (
                    ("霸哥", "https://huyaimg.msstatic.com/avatar/1080/8b/b1a406dd163efa3a547bf752c3a756_180_135.jpg?1585911909"),
                    "峡谷MVP 3+4目前5/7", 783632, "https://live-cover.msstatic.com/huyalive/1724691-1724691-7407491440705536-3572838-10057-A-0-1-imgplus/20250623103952.jpg?x-oss-process=image/resize,limit_0,m_fill,w_338,h_190/sharpen,80/format,webp/quality,q_90"
                ),
                (
                    ("胖炸", "http://huyaimg.msstatic.com/avatar/1095/83/2aa2f6905fe4382221d08b66d7cdcb_180_135.jpg?1410599038"),
                    "200N进180【第二届美女如云巅峰赛】", 1457782,"https://live-cover.msstatic.com/huyalive/17363578-17363578-74575999651545088-34850612-10057-A-0-1-imgplus/20250623103942.jpg?x-oss-process=image/resize,limit_0,m_fill,w_338,h_190/sharpen,80/format,webp/quality,q_90"
                ),
                (
                    ("微竞丶莎莉", "https://huyaimg.msstatic.com/avatar/1094/63/f20eec58c49c79f9925e88c60463e0_180_135.jpg?1537334316"),
                    "复刻全英雄打野上大师教学！", 763517, "https://anchorpost.msstatic.com/cdnimage/anchorpost/1094/63/f20eec58c49c79f9925e88c60463e0_0_1708941537.jpg?spformat=png,webp&imageview/4/0/w/338/h/190/blur/1"
                )
            ].into_iter().enumerate().map(|(id,((user_name, avator_url), room_name, hot, img_url) )| {
                Room {
                    id,
                    hot,
                    img_url,
                    name: room_name,
                    owner: User { id: id + 10000, name: user_name, avator_url },
                    is_live: true,
                    tags: [
                        Tag::Blue,
                        Tag::Play("摸个鱼"),
                        Tag::Offical("潜力新秀")
                    ].to_vec()
                }
            }).collect::<Vec<Room<&'static str>>>(),
        },
        Cate {
            id: 1,
            icon_url:
                "https://huyaimg.msstatic.com/cdnimage/gametypelogo/game_15735501961519_logo.png",
            cate_name: "王者荣耀",
            tags: [
                "吕德华",
                "孤影",
                "赖神",
                "猪猪小悠",
                "韩涵",
                "微凉",
                "小锦儿",
                "西西",
                "宇晨",
            ]
            .into_iter()
            .enumerate()
            .map(|(id, name)| HotRoom { name, id, is_live: id == 0 })
            .collect::<Vec<HotRoom<&'static str>>>(),
            live_total: 468,
            rooms: [
                (
                    ("为输出铠证明的南瓜", "https://huyaimg.msstatic.com/avatar/1035/c9/9dae31ad175ccbd76b61e533d5d5c3_180_135.jpg?1730865400"), 
                    "身法铠 巅峰教学", 313000,"https://tx-live-cover.msstatic.com/huyalive/1850258061-1850258061-7518742024043733105-3700639578-10057-A-1750651152-1/20250623121411.jpg?x-oss-process=image/resize,limit_0,m_fill,w_338,h_190/sharpen,80/format,webp/quality,q_90&sign=nrJzbYeRML2vhdXvhUEWhsc9J/NhPTEyNTM0OTg3MDEmYj1odXlhLXNjcmVlbnNob3RzLXJldmlldy0xMjUzNDk4NzAxJms9QUtJRFFpcTNSbEJtV0p6ZUxKTVZrMklWdVEybm1pY2RkRWdEJmU9MTc2NjIwNDA1MSZ0PTE3NTA2NTIwNTEmcj0xMjM0NTY3OCZmPS9odXlhbGl2ZS8xODUwMjU4MDYxLTE4NTAyNTgwNjEtNzUxODc0MjAyNDA0MzczMzEwNS0zNzAwNjM5NTc4LTEwMDU3LUEtMTc1MDY1MTE1Mi0xLzIwMjUwNjIzMTIxNDExLmpwZw=="
                ),
                (
                    ("小炎【妲己的神】", "https://huyaimg.msstatic.com/avatar/1082/d1/684be77eb8ab1b8196588daeeedb3f_180_135.jpg?1707136218"),
                    "6万场牢妲己国标号冲百星！", 233632, "https://live-cover.msstatic.com/huyalive/1830004985-1830004985-7859811562091970560-3660133426-10057-A-0-1-imgplus/20250623121415.jpg?x-oss-process=image/resize,limit_0,m_fill,w_338,h_190/sharpen,80/format,webp/quality,q_90"
                ),
                (
                    ("正恒-紫宸【相声木兰】", "https://huyaimg.msstatic.com/avatar/1045/89/a67056877955cf214026e4ceca103f_180_135.jpg?1591152284"),
                    "让你三分钟爱上花木兰，来", 1457782,"//live-cover.msstatic.com/huyalive/1069843731-1069843731-4594943836475621376-2139810918-10057-A-0-1-imgplus/20250623121414.jpg?x-oss-process=image/resize,limit_0,m_fill,w_338,h_190/sharpen,80/format,webp/quality,q_90"
                ),
                (
                    ("M丶赵一", "https://huyaimg.msstatic.com/avatar/1020/4a/33e7481e899d6fd6625a6082ebb016_180_135.jpg?1594620825"),
                    "第一阿轲 130星排位教学", 113517, "https://tx-live-cover.msstatic.com/huyalive/1719416634-1719416634-7384838211228401664-3438956724-10057-A-0-1/20250623121413.jpg?x-oss-process=image/resize,limit_0,m_fill,w_338,h_190/sharpen,80/format,webp/quality,q_90&sign=t3JhdIEpP4F+CNOZAW8B9O0KrDphPTEyNTM0OTg3MDEmYj1odXlhLXNjcmVlbnNob3RzLXJldmlldy0xMjUzNDk4NzAxJms9QUtJRFFpcTNSbEJtV0p6ZUxKTVZrMklWdVEybm1pY2RkRWdEJmU9MTc2NjIwNDA1MyZ0PTE3NTA2NTIwNTMmcj0xMjM0NTY3OCZmPS9odXlhbGl2ZS8xNzE5NDE2NjM0LTE3MTk0MTY2MzQtNzM4NDgzODIxMTIyODQwMTY2NC0zNDM4OTU2NzI0LTEwMDU3LUEtMC0xLzIwMjUwNjIzMTIxNDEzLmpwZw=="
                )
            ].into_iter().enumerate().map(|(id,((user_name, avator_url), room_name, hot, img_url) )| {
                Room {
                    id,
                    hot,
                    img_url,
                    name: room_name,
                    owner: User { id: id + 10000, name: user_name, avator_url },
                    is_live: true,
                    tags: [
                        Tag::Blue,
                        Tag::Play("妲己"),
                        Tag::Offical("潜力新秀")
                    ].to_vec()
                }
            }).collect::<Vec<Room<&'static str>>>(),
        },
        Cate {
            id: 3,
            icon_url:
                "https://huyaimg.msstatic.com/cdnimage/gametypelogo/game_15735501794392_logo.png",
            cate_name: "星秀",
            tags: [
                "骚俊",
                "阿布",
                "车老板呢",
                "啵啵超Q",
                "VIKI",
                "小乖"
            ]
            .into_iter()
            .enumerate()
            .map(|(id, name)| HotRoom { name, id, is_live: id == 0 })
            .collect::<Vec<HotRoom<&'static str>>>(),
            live_total: 468,
            rooms: [
                (
                    ("Ck-白允儿", "https://huyaimg.msstatic.com/avatar/1008/23/c14c4ff3ed481f363f2fb378c586db_180_135.jpg?1730121954"), 
                    "求接升~舞蹈~~【腼腆女孩】", 313000,"https://anchorpost.msstatic.com/cdnimage/anchorpost/1030/7f/417f6f692cc018d7cf8f7ff7ac429f_2_1663_1733730509.jpg?spformat=png,webp&imageview/4/0/w/338/h/190/blur/1"
                ),
                (
                    ("映画-赫拉拉不拉【桃】", "https://huyaimg.msstatic.com/avatar/1063/dd/c3093ea82cf572c87577e92d58ae8f_180_135.jpg?1721893204"),
                    "【温柔小女生】", 233632, "https://anchorpost.msstatic.com/cdnimage/anchorpost/1063/dd/c3093ea82cf572c87577e92d58ae8f_1663_1736305022.jpg?spformat=png,webp&imageview/4/0/w/338/h/190/blur/1"
                ),
                (
                    ("HX温柔", "https://huyaimg.msstatic.com/avatar/1082/e5/423aef7b077fbe3429f7ac34bfaae3_180_135.jpg?1697474968"),
                    "大爷 来玩啊~", 1457782,"https://anchorpost.msstatic.com/cdnimage/anchorpost/1082/e5/423aef7b077fbe3429f7ac34bfaae3_1663_1747710542.jpg?spformat=png,webp&imageview/4/0/w/338/h/190/blur/1"
                ),
                (
                    ("正恒YJ-VIKI【浈】", "https://huyaimg.msstatic.com/avatar/1035/a4/2ed1bd7ace2c24597a3b2a09238bec_180_135.jpg?1697010059"),
                    "我来了~长沙舞蹈主播    ", 113517, "https://anchorpost.msstatic.com/cdnimage/anchorpost/1035/a4/2ed1bd7ace2c24597a3b2a09238bec_1663_1723466103.jpg?spformat=png,webp&imageview/4/0/w/338/h/190/blur/1"
                )
            ].into_iter().enumerate().map(|(id,((user_name, avator_url), room_name, hot, img_url) )| {
                Room {
                    id,
                    hot,
                    img_url,
                    name: room_name,
                    owner: User { id: id + 10000, name: user_name, avator_url },
                    is_live: true,
                    tags: [
                        Tag::Blue,
                        Tag::Play("妲己"),
                        Tag::Offical("潜力新秀")
                    ].to_vec()
                }
            }).collect::<Vec<Room<&'static str>>>(),
        }
    
    ];
    Ok(cates)
}

#[component]
pub fn CateRooms() -> impl IntoView {
    let (rooms, set_rooms) = signal(None);
    Effect::new(move || {
        spawn_local(async move {
            if let Ok(rooms) = get_recommend_cate_rooms().await {
                set_rooms.set(Some(rooms));
            }
        });
    });
    view! {
        <figure class="flex flex-col gap-y-10 mt-10">
            <Show when=move || rooms.get().is_some()>
                <For
                    each=move || rooms.get().unwrap_or(vec![]).into_iter()
                    key=|cate| cate.id
                    let(cate)
                >
                    <div>
                        <div class="flex relative justify-start items-end mb-4 text-[14px]">
                            <h1 class="flex gap-x-2.5 items-center mr-4 text-[26px] leading-[33px]">
                                <img src=cate.icon_url width=32 height=32 />
                                {cate.cate_name}
                            </h1>
                            <ul class="flex gap-x-1.5 leadding-[26px] *:px-3 *:rounded-3xl *:border *:text-gray-400 *:hover:text-[#f80] *:border-current">
                                {cate
                                    .tags
                                    .into_iter()
                                    .map(|tag| {
                                        view! {
                                            <li class=if tag.is_live {
                                                Some("text-[#f80]!")
                                            } else {
                                                None
                                            }>{tag.name}</li>
                                        }
                                    })
                                    .collect_view()}
                            </ul>
                            <p class="absolute right-0">
                                <span class="mr-1 text-[#f80]">548</span>
                                个主播正在直播
                                <a>"更多 >"</a>
                            </p>
                        </div>
                        <div class="flex gap-x-5 justify-between">
                            {cate
                                .rooms
                                .into_iter()
                                .map(|room| {
                                    view! { <RoomCard data=room /> }
                                })
                                .collect_view()}
                        </div>
                    </div>
                </For>
            </Show>
        </figure>
    }
}

#[component]
fn RoomCard(data: Room<&'static str>) -> impl IntoView {
    view! {
        <div class="flex overflow-hidden relative flex-col bg-white rounded-md">
            <div>
                <img src=data.img_url alt="" loading="lazy" class="aspect-290/163" />
            </div>
            <div class="flex flex-col gap-y-1.5 p-2.5 text-left">
                <p class="w-4/5 truncate">{data.name}</p>
                <div class="flex relative gap-x-1 justify-start items-center text-xs text-gray-400">
                    <img
                        src=data.owner.avator_url
                        class="rounded-full"
                        alt=""
                        width=24
                        height=24
                        loading="lazy"
                    />
                    <span class="inline-block w-3/5 truncate">{data.owner.name}</span>
                    <div class="flex absolute right-0">
                        <span class="flex gap-x-1.5 bg-no-repeat before:inline-block before:w-3 before:h-4 before:bg-cover before:bg-center before: before:bg-[url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABgAAAAgCAYAAAAIXrg4AAAACXBIWXMAABYlAAAWJQFJUiTwAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAPkSURBVHgB1VbNbttGEJ5dUo6LoCiBXnKLfPNRt+ZW+RKoVdKUdWtX7cH0E8R+AllPEOfYk+VDoVqBQge20B4CyEFPPdV9ArO3AAEC5Ud/JHcnM0vSkBzbkpFcMgdySA7nm7/9dgE+dRGXfWz4f+ZtVD4iFBBgu/86qq2vu124gsjLPpLzHXaeRrJx/Qv734bv5+FjAWSCAjwEEZCSt3TuZO/RwRbMKNMAHL4IkMHqcnkBpdgyz1JUZwWZBpDnS+/V6D++r7rlmtaweRWQ95q84/vO5zC3hhrz1NgNBKTo7y6M2zRabc8C3GFdA2z+vHxnG2bN4Lqe29Eat9l5EoE4OmtTWS7XFYj11MGD35vtgn9B8ycAeCyppd+T2jVNNQiyft6PDELpm8jnbOxE1Pzm44OTs1M2AWBpXU3Vfa2gxuVZ+eHbZ3CB2DKqCUHBYDIMZsrQ7oyDnAI0qa5k4bFTFUa1ygqVIcZNuERclxadECYLAqH/xT6DSJXzJwBMaQRU0yi2KhU3YPWX1e/2YYrYED40g/Djnd2cDNe5tJRV4Y/W4cYpgCkN0tSg2GdDuIJwFjqMl051rbPmm4AFR29pdcIPVJqFLPoPkebjww4gFJXGJZkDfS99X/8YzlmESEbbkmLNRjRjCVqLqfVm6XQ683Fs30KJeVojN41DKZ7rKPrn9u2lY36mNfSEbltMlDYphi0Vwv8zOHeUsteowI4YIwFa9TeEZd97+vTvry0r3u12u0EEOcoEiO7TGf51pXx8nlPukdTqPu8F7JyY1bkoAP7GNo7j/PbiZY9fOTZMkXTxec6XnznkIJhmzyARWl9lzzRNCSW02u2bF/zimavWBZhRopEqpf8GvA6SxgyweNYw4aZEYoWLsdbzMIMoHS0m/kUgpYQj1ol2vbOGFuixeuN8FI4WYQaJQmXsaJrq0oJwlwmLhqLYeHRQnIgE5MQGH47C4rQsVKwdpRMAGt1ncpywLCGq48YV95vAgJtoRKBp4gb9fvEygLe9t16qmoVruCghLGo2ZbHXOnww8QdCnW9CYMD3OFa3BsPBuQ3v9QdFjehkjMzvDABnISS4HC0fT/ZabT/j9FjjkxSIzkbJpj8cRqUwim9MRt4rhWFYJLUrlHQz2pnYk5t+u4AafAGYz9IUGnfRotIRedFUbKBFq1ijAbp2LfeXZdnPh4NhiSJnwC4RnFv56e5R5nNiR1txy8c6lES9op6+8uioYpgxSQK7fLLIji+jUVTq9wceO+eygBJL487fy2BcGg2iiDl1X7Bz2kDI+1HvTeRmR8dGs+3RCFYpW6q5oGNl+PCqx8pPQ94BiQr61haSciEAAAAASUVORK5CYII=)]">
                            {if data.hot > 10000 {
                                format!("{:.2}", (data.hot as f64) / 10000.0)
                            } else {
                                data.hot.to_string()
                            }}
                        </span>
                    </div>
                </div>
            </div>
        </div>
    }
}
