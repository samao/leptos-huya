use leptos::either::EitherOf3;
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
    avatar_url: T,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
enum Tag<T: ToString> {
    // 蓝光
    Blue(T),
    Play(T),
    Official(T),
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
                    owner: User { id: id + 10000, name: user_name, avatar_url: avator_url },
                    is_live: true,
                    tags: [
                        Tag::Blue("蓝光10M"),
                        Tag::Play("摸个鱼"),
                        Tag::Official("潜力新秀")
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
                    owner: User { id: id + 10000, name: user_name, avatar_url: avator_url },
                    is_live: true,
                    tags: [].to_vec()
                }
            }).collect::<Vec<Room<&'static str>>>(),
        },
        Cate {
            id: 2,
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
                    owner: User { id: id + 10000, name: user_name, avatar_url: avator_url },
                    is_live: true,
                    tags: [
                        Tag::Blue("蓝光6M"),
                        Tag::Play("妲己"),
                        Tag::Official("潜力新秀")
                    ].to_vec()
                }
            }).collect::<Vec<Room<&'static str>>>(),
        },
        Cate {
            id: 3,
            icon_url:
                "https://huyaimg.msstatic.com/cdnimage/gametypelogo/game_15735502223924_logo.png",
            cate_name: "和平精英",
            tags: [
                "鲨鱼哟",
                "雨果",
                "阿顺",
                "一晨",
                "和平慢镜头",
                "董系长",
                "轲南",
                "冰冰妹",
                "伞兵"
            ]
            .into_iter()
            .enumerate()
            .map(|(id, name)| HotRoom { name, id, is_live: id == 0 })
            .collect::<Vec<HotRoom<&'static str>>>(),
            live_total: 212,
            rooms: [
                (
                    ("鲨鱼哟", "https://huyaimg.msstatic.com/avatar/1083/f3/6f3bf8adc63a7432f5e12b8bec4662_180_135.jpg?1595339002"), 
                    "5天不超不求人摆地摊【第二天】", 2313000,"https://live-cover.msstatic.com/huyalive/1067459140-1067459140-4584702096116285440-2135041736-10057-A-0-1-imgplus/20250623183443.jpg?x-oss-process=image/resize,limit_0,m_fill,w_338,h_190/sharpen,80/format,webp/quality,q_90"
                ),
                (
                    ("LK-王小宝【全能王】", "https://huyaimg.msstatic.com/avatar/1099/18/98d9bd27b864d7f438efd18fc257eb_180_135.jpg?1590707456"),
                    "【癫疯赛冲击第一教学】", 233632, "https://anchorpost.msstatic.com/cdnimage/anchorpost/1099/18/98d9bd27b864d7f438efd18fc257eb_3203_1749714584.jpg?spformat=png,webp&imageview/4/0/w/338/h/190/blur/1"
                ),
                (
                    ("TG-老熊【平头哥】", "https://huyaimg.msstatic.com/avatar/1031/70/9c8c33e92deb95ae006acd68a976e0_180_135.jpg?1621699908"),
                    "沙漠狙神1v4 6鸡挑战中", 1457782,"https://anchorpost.msstatic.com/cdnimage/anchorpost/1031/70/9c8c33e92deb95ae006acd68a976e0_3203_1749714734.jpg?spformat=png,webp&imageview/4/0/w/338/h/190/blur/1"
                ),
                (
                    ("和平慢镜头", "https://huyaimg.msstatic.com/avatar/1045/e6/b4b4595ae7a622e9a1c8d6707fd0d4_180_135.jpg?1635271282"),
                    "PEL新赛季训练赛直播", 113517, "//live-cover.msstatic.com/huyalive/110160544-110160544-473135933789569024-220444544-10057-A-0-1-imgplus/20250623183445.jpg?x-oss-process=image/resize,limit_0,m_fill,w_338,h_190/sharpen,80/format,webp/quality,q_90"
                )
            ].into_iter().enumerate().map(|(id,((user_name, avator_url), room_name, hot, img_url) )| {
                Room {
                    id,
                    hot,
                    img_url,
                    name: room_name,
                    owner: User { id: id + 10000, name: user_name, avatar_url: avator_url },
                    is_live: true,
                    tags: [
                        Tag::Blue("蓝光8M"),
                        Tag::Play("妲己"),
                        Tag::Official("潜力新秀")
                    ].to_vec()
                }
            }).collect::<Vec<Room<&'static str>>>(),
        },
        Cate {
            id: 4,
            icon_url:
                "https://huyaimg.msstatic.com/cdnimage/gametypelogo/game_15735502034733_logo.png",
            cate_name: "天天吃鸡",
            tags: [
                "悲喜",
                "韦神",
                "陈子豪",
                "托马斯",
                "BB文",
                "永远",
                "LongSkr",
                "乐神",
                "星魂"
            ]
            .into_iter()
            .enumerate()
            .map(|(id, name)| HotRoom { name, id, is_live: id == 0 })
            .collect::<Vec<HotRoom<&'static str>>>(),
            live_total: 242,
            rooms: [
                (
                    ("天天吃鸡赛事", "https://huyaimg.msstatic.com/avatar/1026/a3/21c624bf332d4ad165d20f66d5b590_180_135.jpg?1742213810"), 
                    "鱼虎巅峰赛 第三轮小组赛DAY1", 2313000,"https://anchorpost.msstatic.com/cdnimage/anchorpost/1026/a3/21c624bf332d4ad165d20f66d5b590_2793_1750041275.jpg?spformat=png,webp&imageview/4/0/w/338/h/190/blur/1"
                ),
                (
                    ("Wsy、3胖", "https://huyaimg.msstatic.com/avatar/1060/7f/def38ec9cd694c33c79410170e88bf_180_135.jpg?1663453237"),
                    "冲第三个生存者，可上车", 233632, "//live-cover.msstatic.com/huyalive/1521785002-1521785002-6536016815133294592-3043693460-10057-A-0-1-imgplus/20250623183426.jpg?x-oss-process=image/resize,limit_0,m_fill,w_338,h_190/sharpen,80/format,webp/quality,q_90"
                ),
                (
                    ("4AMGodv", "http://huyaimg.msstatic.com/avatar/1067/32/062b3abea4cfe237a3dab14dc5753a_180_135.jpg?1514467694"),
                    "ewc预选赛day4. 3分钟延迟", 1457782,"https://anchorpost.msstatic.com/cdnimage/anchorpost/1067/32/062b3abea4cfe237a3dab14dc5753a_3_2793_1749607045.jpg?spformat=png,webp&imageview/4/0/w/338/h/190/blur/1"
                ),
                (
                    ("宿舍战神-BB文", "http://huyaimg.msstatic.com/avatar/1001/78/859091a71db0eaec7b6c24ab4cff5c_180_135.jpg?1460197317"),
                    "睡前小故事", 113517, "https://anchorpost.msstatic.com/cdnimage/anchorpost/1001/78/859091a71db0eaec7b6c24ab4cff5c_3_2793_1750317274.jpg?spformat=png,webp&imageview/4/0/w/338/h/190/blur/1"
                )
            ].into_iter().enumerate().map(|(id,((user_name, avator_url), room_name, hot, img_url) )| {
                Room {
                    id,
                    hot,
                    img_url,
                    name: room_name,
                    owner: User { id: id + 10000, name: user_name, avatar_url: avator_url },
                    is_live: true,
                    tags: [
                        Tag::Blue("蓝光10M"),
                        Tag::Play("妲己"),
                        Tag::Official("潜力新秀")
                    ].to_vec()
                }
            }).collect::<Vec<Room<&'static str>>>(),
        },
        Cate {
            id: 5,
            icon_url:
                "https://huyaimg.msstatic.com/cdnimage/gametypelogo/game_15735502126348_logo.png",
            cate_name: "吃喝玩乐",
            tags: [
                "童锦程",
                "小小小酷哥",
                "张开朗",
                "子圣",
                "集梦会长",
                "小龙女"
            ]
            .into_iter()
            .enumerate()
            .map(|(id, name)| HotRoom { name, id, is_live: id == 0 })
            .collect::<Vec<HotRoom<&'static str>>>(),
            live_total: 492,
            rooms: [
                (
                    ("Time-祁猪冲鸭", "https://huyaimg.msstatic.com/avatar/1029/82/7bd0ded9bc954febba79ae8aa6248b_180_135.jpg?1750285849"), 
                    "【炒粉四倍】今天就祝福我80大寿！！！！", 2313000,"https://anchorpost.msstatic.com/cdnimage/anchorpost/1029/82/7bd0ded9bc954febba79ae8aa6248b_2633_1750231559.jpg?spformat=png,webp&imageview/4/0/w/338/h/190/blur/1"
                ),
                (
                    ("倪海杉", "https://huyaimg.msstatic.com/avatar/1040/83/20b606aaaf63ace67eccfbcfeb623b_180_135.jpg?1737897689"),
                    "重庆", 233632, "//live-cover.msstatic.com/huyalive/1199629984215-1199629984215-5729952939456331776-2399260091886-10057-A-0-1/20250623183725.jpg?x-oss-process=image/resize,limit_0,m_fill,w_338,h_190/sharpen,80/format,webp/quality,q_90"
                ),
                (
                    ("阿布【只为1380】", "https://huyaimg.msstatic.com/avatar/1059/06/17c17227ad6afbf51c6e9a70a8e670_180_135.jpg?1607314586"),
                    "荒野求生 第一天", 1457782,"//live-cover.msstatic.com/huyalive/1677942375-1677942375-7206707625197568000-3356008206-10057-A-0-1-imgplus/20250623183723.jpg?x-oss-process=image/resize,limit_0,m_fill,w_338,h_190/sharpen,80/format,webp/quality,q_90"
                ),
                (
                    ("Hot-芥末", "https://anchorpost.msstatic.com/cdnimage/anchorpost/1005/44/79c31272ac119ad9588951f9a79e76_2633_1749434026.jpg?spformat=png,webp&imageview/4/0/w/338/h/190/blur/1"),
                    "【唱】来听歌叭", 113517, "//live-cover.msstatic.com/huyalive/110160544-110160544-473135933789569024-220444544-10057-A-0-1-imgplus/20250623183445.jpg?x-oss-process=image/resize,limit_0,m_fill,w_338,h_190/sharpen,80/format,webp/quality,q_90"
                )
            ].into_iter().enumerate().map(|(id,((user_name, avator_url), room_name, hot, img_url) )| {
                Room {
                    id,
                    hot,
                    img_url,
                    name: room_name,
                    owner: User { id: id + 10000, name: user_name, avatar_url: avator_url },
                    is_live: true,
                    tags: [
                        Tag::Blue("蓝光8M"),
                        Tag::Play("妲己"),
                        Tag::Official("潜力新秀")
                    ].to_vec()
                }
            }).collect::<Vec<Room<&'static str>>>(),
        },
        Cate {
            id: 6,
            icon_url:
                "https://huyaimg.msstatic.com/cdnimage/gametypelogo/game_15735501875101_logo.png",
            cate_name: "穿越火线",
            tags: [
                "阿飞",
                "绝迹",
                "梧桐",
                "飞段",
                "飞机文",
                "老街",
                "宠儿",
                "周虎"
            ]
            .into_iter()
            .enumerate()
            .map(|(id, name)| HotRoom { name, id, is_live: id == 0 })
            .collect::<Vec<HotRoom<&'static str>>>(),
            live_total: 212,
            rooms: [
                (
                    ("WH-妖狼", "https://huyaimg.msstatic.com/avatar/1041/17/ee4794a5ea86b87dab424edcab886c_180_135.jpg?1603237015"), 
                    "30人生化乱斗村子 没单不排队", 2313000,"//live-cover.msstatic.com/huyalive/194819851-194819851-836744888656592896-389763158-10057-A-0-1/20250623183731.jpg?x-oss-process=image/resize,limit_0,m_fill,w_338,h_190/sharpen,80/format,webp/quality,q_90"
                ),
                (
                    ("AzZ丶阿飞512", "https://huyaimg.msstatic.com/avatar/1003/23/4c8c7a479660cf8694cbcd549f4361_180_135.jpg?1538161823"),
                    "【癫疯赛冲击第一教学】", 233632, "https://tx-live-cover.msstatic.com/huyalive/154841937-154841937-665041055464292352-309807330-10057-A-0-1/20250623183727.jpg?x-oss-process=image/resize,limit_0,m_fill,w_338,h_190/sharpen,80/format,webp/quality,q_90&sign=Tc4y6IygeKDuhrcwj1N7MYgUhe5hPTEyNTM0OTg3MDEmYj1odXlhLXNjcmVlbnNob3RzLXJldmlldy0xMjUzNDk4NzAxJms9QUtJRFFpcTNSbEJtV0p6ZUxKTVZrMklWdVEybm1pY2RkRWdEJmU9MTc2NjIyNzA0NyZ0PTE3NTA2NzUwNDcmcj0xMjM0NTY3OCZmPS9odXlhbGl2ZS8xNTQ4NDE5MzctMTU0ODQxOTM3LTY2NTA0MTA1NTQ2NDI5MjM1Mi0zMDk4MDczMzAtMTAwNTctQS0wLTEvMjAyNTA2MjMxODM3MjcuanBn"
                ),
                (
                    ("穿越火线赛事", "https://huyaimg.msstatic.com/avatar/1054/90/bb80adc2e5f5e3e95cdd25cf8fc9be_180_135.jpg?1747568096"),
                    "【预告】19点 WE vs EP 2025CFPL夏季赛", 1457782,"https://anchorpost.msstatic.com/cdnimage/anchorpost/1054/90/bb80adc2e5f5e3e95cdd25cf8fc9be_4_1750614328.jpg?spformat=png,webp&imageview/4/0/w/338/h/190/blur/1"
                ),
                (
                    ("AG绝迹", "https://huyaimg.msstatic.com/avatar/1027/10/8bc5fbfd959766028e82c176aa7f4d_180_135.jpg?1619779464"),
                    "妹子代播到晚上七点，排位啦", 113517, "https://anchorpost.msstatic.com/cdnimage/anchorpost/1027/10/8bc5fbfd959766028e82c176aa7f4d_4_1747729822.jpg?spformat=png,webp&imageview/4/0/w/338/h/190/blur/1"
                )
            ].into_iter().enumerate().map(|(id,((user_name, avator_url), room_name, hot, img_url) )| {
                Room {
                    id,
                    hot,
                    img_url,
                    name: room_name,
                    owner: User { id: id + 10000, name: user_name, avatar_url: avator_url },
                    is_live: true,
                    tags: [
                        Tag::Blue("蓝光6M"),
                        Tag::Play("妲己"),
                        Tag::Official("潜力新秀")
                    ].to_vec()
                }
            }).collect::<Vec<Room<&'static str>>>(),
        },
        Cate {
            id: 7,
            icon_url:
                "https://huyaimg.msstatic.com/cdnimage/gametypelogo/game_15735502396542_logo.png",
            cate_name: "主机游戏",
            tags: [
                "楚河",
                "小宇",
                "贱圣",
                "导演",
                "星辉",
                "老鬼",
                "马桶",
                "柚子"
            ]
            .into_iter()
            .enumerate()
            .map(|(id, name)| HotRoom { name, id, is_live: id == 0 })
            .collect::<Vec<HotRoom<&'static str>>>(),
            live_total: 775,
            rooms: [
                (
                    ("狂鸟丶楚河-90327", "https://huyaimg.msstatic.com/avatar/1086/bf/fd6f69d69c0015eaface1f6024869e_180_135.jpg?1619540458"), 
                    "直播上万款游戏新主播，很紧张！", 2313000,"//live-cover.msstatic.com/huyalive/294636272-294636272-1265453152455360512-589396000-10057-A-0-1-imgplus/20250623184315.jpg?x-oss-process=image/resize,limit_0,m_fill,w_338,h_190/sharpen,80/format,webp/quality,q_90"
                ),
                (
                    ("狂鸟丶爱吃鱼", "http://huyaimg.msstatic.com/avatar/1055/22/b2053d8985301ea3d7a327220fc645_180_135.jpg?1515638572"),
                    "漫漫长夜 大熊岛活地图永不迷路~", 233632, "//live-cover.msstatic.com/huyalive/1361173-1361173-5846193519198208-2845802-10057-A-0-1/20250623184306.jpg?x-oss-process=image/resize,limit_0,m_fill,w_338,h_190/sharpen,80/format,webp/quality,q_90"
                ),
                (
                    ("叫我Happy呀", "https://huyaimg.msstatic.com/avatar/1078/dd/ea71a96d700ecf09a8337edf40377d_180_135.jpg?1696857432"),
                    "独狼挑战5小时毕业！", 1457782,"//live-cover.msstatic.com/huyalive/1199636641223-1199636641223-5758544571105542144-2399273405902-10057-A-0-1/20250623184326.jpg?x-oss-process=image/resize,limit_0,m_fill,w_338,h_190/sharpen,80/format,webp/quality,q_90"
                ),
                (
                    ("萨摩直播实况", "https://huyaimg.msstatic.com/avatar/1004/1d/45240573150f09aa27dd407e0b26b2_180_135.jpg?1697528751"),
                    "8月28日船长猎人", 113517, "https://v-huya-img2.msstatic.com/screenshot/2335/911151576/4.jpg?x-oss-process=image/resize,limit_0,m_fill,w_338,h_190/sharpen,80/format,webp/quality,q_90"
                )
            ].into_iter().enumerate().map(|(id,((user_name, avator_url), room_name, hot, img_url) )| {
                Room {
                    id,
                    hot,
                    img_url,
                    name: room_name,
                    owner: User { id: id + 10000, name: user_name, avatar_url: avator_url },
                    is_live: true,
                    tags: [
                        Tag::Blue("蓝光4M"),
                        Tag::Play("妲己"),
                        Tag::Official("潜力新秀")
                    ].to_vec()
                }
            }).collect::<Vec<Room<&'static str>>>(),
        },
        Cate {
            id: 8,
            icon_url:
                "https://huyaimg.msstatic.com/cdnimage/gametypelogo/game_17228323681551_logo.png",
            cate_name: "二次元",
            tags: [
                "芊若呀丶",
                "可爱的埋埋",
                "温舒蕾",
                "Ayo夏哟哟",
                "知栀",
                "铁甲小宝",
            ]
            .into_iter()
            .enumerate()
            .map(|(id, name)| HotRoom { name, id, is_live: id == 0 })
            .collect::<Vec<HotRoom<&'static str>>>(),
            live_total: 99,
            rooms: [
                (
                    ("Time-祁猪冲鸭", "https://huyaimg.msstatic.com/avatar/1029/82/7bd0ded9bc954febba79ae8aa6248b_180_135.jpg?1750285849"), 
                    "【炒粉四倍】今天就祝福我80大寿！！！！", 2313000,"https://anchorpost.msstatic.com/cdnimage/anchorpost/1029/82/7bd0ded9bc954febba79ae8aa6248b_2633_1750231559.jpg?spformat=png,webp&imageview/4/0/w/338/h/190/blur/1"
                ),
                (
                    ("正恒RM丶原子不可爱", "https://huyaimg.msstatic.com/avatar/1093/18/81f911f3f306fc812b95bf09cfbfdc_180_135.jpg?1703614596"),
                    "【3D音】 晚上好", 233632, "//live-cover.msstatic.com/huyalive/2214583392-2214583392-9511563242904748032-4429290240-10057-A-0-1/20250623184310.jpg?x-oss-process=image/resize,limit_0,m_fill,w_338,h_190/sharpen,80/format,webp/quality,q_90"
                ),
                (
                    ("Hot-芥末", "https://huyaimg.msstatic.com/avatar/1005/44/79c31272ac119ad9588951f9a79e76_180_135.jpg?1747723810"),
                    "【唱】来听歌叭", 1457782,"https://anchorpost.msstatic.com/cdnimage/anchorpost/1005/44/79c31272ac119ad9588951f9a79e76_2633_1749434026.jpg?spformat=png,webp&imageview/4/0/w/338/h/190/blur/1"
                ),
                (
                    ("京梦-小拂", "https://huyaimg.msstatic.com/avatar/1055/53/ec6ff6c9efc147cbda12dff86faf23_180_135.jpg?1744980611"),
                    "【保灯】也想被偏爱", 113517, "https://anchorpost.msstatic.com/cdnimage/anchorpost/1055/53/ec6ff6c9efc147cbda12dff86faf23_2633_1744165320.jpg?spformat=png,webp&imageview/4/0/w/338/h/190/blur/1"
                )
            ].into_iter().enumerate().map(|(id,((user_name, avator_url), room_name, hot, img_url) )| {
                Room {
                    id,
                    hot,
                    img_url,
                    name: room_name,
                    owner: User { id: id + 10000, name: user_name, avatar_url: avator_url },
                    is_live: true,
                    tags: [
                        Tag::Blue("蓝光10M"),
                        Tag::Play("妲己"),
                        Tag::Official("潜力新秀")
                    ].to_vec()
                }
            }).collect::<Vec<Room<&'static str>>>(),
        },
        Cate {
            id: 9,
            icon_url:
                "https://huyaimg.msstatic.com/cdnimage/gametypelogo/game_15735502635279_logo.png",
            cate_name: "交友",
            tags: []
            .into_iter()
            .enumerate()
            .map(|(id, name)| HotRoom { name, id, is_live: id == 0 })
            .collect::<Vec<HotRoom<&'static str>>>(),
            live_total: 212,
            rooms: [
                (
                    ("Dae-爸比我甜-视频厅", "https://huyaimg.msstatic.com/avatar/1081/c6/b7d898c2ac3c1990d8dd956a69450d_180_135.jpg?1689334452"), 
                    "来源：附近的人 距离你3km", 2313000,"https://anchorpost.msstatic.com/cdnimage/anchorpost/1081/c6/b7d898c2ac3c1990d8dd956a69450d_4079_1687939665.jpg?spformat=png,webp&imageview/4/0/w/338/h/190/blur/1"
                ),
                (
                    ("yM-有幸遇见", "https://huyaimg.msstatic.com/avatar/1014/da/44e864d4d9b64af3ec1aa4e726f9db_180_135.jpg?1738770424"),
                    " 白毛浮绿水 这里全是腿【久久冠】", 233632, "https://anchorpost.msstatic.com/cdnimage/anchorpost/1014/da/44e864d4d9b64af3ec1aa4e726f9db_4079_1689141780.jpg?spformat=png,webp&imageview/4/0/w/338/h/190/blur/1"
                ),
                (
                    ("逸鹿-摇曳的心", "https://huyaimg.msstatic.com/avatar/1011/b3/495db98a89a9f983a13e27b1dd5ecc_180_135.jpg?1576828825"),
                    "越努力 越幸福！女神推荐：开心", 1457782,"https://anchorpost.msstatic.com/cdnimage/anchorpost/1011/b3/495db98a89a9f983a13e27b1dd5ecc_4079_1719546892.jpg?spformat=png,webp&imageview/4/0/w/338/h/190/blur/1"
                ),
                (
                    ("金水-hello姐姐", "https://huyaimg.msstatic.com/avatar/1021/49/1ff9df4ce065d2ca6c75ed13efc650_180_135.jpg?1707220258"),
                    "幸福吗？包的", 113517, "https://anchorpost.msstatic.com/cdnimage/anchorpost/1021/49/1ff9df4ce065d2ca6c75ed13efc650_4079_1705483693.jpg?spformat=png,webp&imageview/4/0/w/338/h/190/blur/1"
                )
            ].into_iter().enumerate().map(|(id,((user_name, avator_url), room_name, hot, img_url) )| {
                Room {
                    id,
                    hot,
                    img_url,
                    name: room_name,
                    owner: User { id: id + 10000, name: user_name, avatar_url: avator_url },
                    is_live: true,
                    tags: [
                        Tag::Blue("蓝光10M"),
                        Tag::Play("妲己"),
                        Tag::Official("潜力新秀")
                    ].to_vec()
                }
            }).collect::<Vec<Room<&'static str>>>(),
        },
        Cate {
            id: 10,
            icon_url:
                "https://huyaimg.msstatic.com/cdnimage/gametypelogo/game_15735502715195_logo.png",
            cate_name: "暴雪专区",
            tags: [
                "安德罗妮",
                "瓦莉拉",
                "雪妍",
                "春哥",
                "上帝",
                "吊打权限狗",
                "暴雪游戏频道",
            ]
            .into_iter()
            .enumerate()
            .map(|(id, name)| HotRoom { name, id, is_live: id == 0 })
            .collect::<Vec<HotRoom<&'static str>>>(),
            live_total: 360,
            rooms: [
                (
                    ("瓦莉拉", "https://huyaimg.msstatic.com/avatar/1019/ee/ffabf13c4cc3effa9398c2859ac9c8_180_135.jpg?1640700311"), 
                    "战棋瓦：万八冲两万了！做大技播！", 2313000,"//live-cover.msstatic.com/huyalive/175484368-175484368-753699621519228928-351092192-10057-A-0-1-imgplus/20250623212227.jpg?x-oss-process=image/resize,limit_0,m_fill,w_338,h_190/sharpen,80/format,webp/quality,q_90"
                ),
                (
                    ("炉石春哥", "https://huyaimg.msstatic.com/avatar/1007/ca/85bab76cc83a7feac8fa4f8a7fef31_180_135.jpg?1674501042"),
                    "毒号正在教育主播", 233632, "https://tx-live-cover.msstatic.com/huyalive/98720253-98720253-424000258087845888-197563962-10057-A-0-1/20250623211613.jpg?x-oss-process=image/resize,limit_0,m_fill,w_338,h_190/sharpen,80/format,webp/quality,q_90&sign=3bYNjyMjkwULPT16GXyay2AJNxxhPTEyNTM0OTg3MDEmYj1odXlhLXNjcmVlbnNob3RzLXJldmlldy0xMjUzNDk4NzAxJms9QUtJRFFpcTNSbEJtV0p6ZUxKTVZrMklWdVEybm1pY2RkRWdEJmU9MTc2NjIzNjU3MyZ0PTE3NTA2ODQ1NzMmcj0xMjM0NTY3OCZmPS9odXlhbGl2ZS85ODcyMDI1My05ODcyMDI1My00MjQwMDAyNTgwODc4NDU4ODgtMTk3NTYzOTYyLTEwMDU3LUEtMC0xLzIwMjUwNjIzMjExNjEzLmpwZw=="
                ),
                (
                    ("老中医", "http://huyaimg.msstatic.com/avatar/1066/c6/c7842e28339e0cd8c3d0473579b956_180_135.jpg?0"),
                    "医：今日雪耻 目标8000！", 1457782,"https://anchorpost.msstatic.com/cdnimage/anchorpost/1066/c6/c7842e28339e0cd8c3d0473579b956_393_1747053190.jpg?spformat=png,webp&imageview/4/0/w/338/h/190/blur/1"
                ),
                (
                    ("IG随缘风", "https://huyaimg.msstatic.com/avatar/1063/8b/87bdf9f5e7d1a662766b87bd128721_180_135.jpg?1713538392"),
                    "品鉴一下平台赛选手卡组", 113517, "https://tx-live-cover.msstatic.com/huyalive/1834384567-1834384567-7878621723552120832-3668892590-10057-A-0-1/20250623212226.jpg?x-oss-process=image/resize,limit_0,m_fill,w_338,h_190/sharpen,80/format,webp/quality,q_90&sign=jVU/P/xnCCAatvn9mjD4ssUxUj9hPTEyNTM0OTg3MDEmYj1odXlhLXNjcmVlbnNob3RzLXJldmlldy0xMjUzNDk4NzAxJms9QUtJRFFpcTNSbEJtV0p6ZUxKTVZrMklWdVEybm1pY2RkRWdEJmU9MTc2NjIzNjk0NiZ0PTE3NTA2ODQ5NDYmcj0xMjM0NTY3OCZmPS9odXlhbGl2ZS8xODM0Mzg0NTY3LTE4MzQzODQ1NjctNzg3ODYyMTcyMzU1MjEyMDgzMi0zNjY4ODkyNTkwLTEwMDU3LUEtMC0xLzIwMjUwNjIzMjEyMjI2LmpwZw=="
                )
            ].into_iter().enumerate().map(|(id,((user_name, avator_url), room_name, hot, img_url) )| {
                Room {
                    id,
                    hot,
                    img_url,
                    name: room_name,
                    owner: User { id: id + 10000, name: user_name, avatar_url: avator_url },
                    is_live: true,
                    tags: [
                        Tag::Blue("蓝光10M"),
                        Tag::Play("妲己"),
                        Tag::Official("潜力新秀")
                    ].to_vec()
                }
            }).collect::<Vec<Room<&'static str>>>(),
        },
        Cate {
            id: 11,
            icon_url:
                "https://huyaimg.msstatic.com/cdnimage/gametypelogo/game_15735502786566_logo.png",
            cate_name: "棋牌桌游",
            tags: [
                "鲍勃哥",
                "渝乐",
                "单走一张六",
                "刘小怂",
                "怂俊",
            ]
            .into_iter()
            .enumerate()
            .map(|(id, name)| HotRoom { name, id, is_live: id == 0 })
            .collect::<Vec<HotRoom<&'static str>>>(),
            live_total: 360,
            rooms: [
                (
                    ("绽放曙光", "https://huyaimg.msstatic.com/avatar/1015/a9/0b7172fb741f595706a33dcc25e2a9_180_135.jpg?1621490139"), 
                    "呵 弱爆", 2313000,"https://anchorpost.msstatic.com/cdnimage/anchorpost/1015/a9/0b7172fb741f595706a33dcc25e2a9_2_2688_1735902634.jpg?spformat=png,webp&imageview/4/0/w/338/h/190/blur/1"
                ),
                (
                    ("渝乐", "http://huyaimg.msstatic.com/avatar/1056/43/588a8bfdd0c7ddf8415b7f5cbe582c_180_135.jpg?0"),
                    "毒号正在教育主播", 233632, "https://tx-live-cover.msstatic.com/huyalive/98720253-98720253-424000258087845888-197563962-10057-A-0-1/20250623211613.jpg?x-oss-process=image/resize,limit_0,m_fill,w_338,h_190/sharpen,80/format,webp/quality,q_90&sign=3bYNjyMjkwULPT16GXyay2AJNxxhPTEyNTM0OTg3MDEmYj1odXlhLXNjcmVlbnNob3RzLXJldmlldy0xMjUzNDk4NzAxJms9QUtJRFFpcTNSbEJtV0p6ZUxKTVZrMklWdVEybm1pY2RkRWdEJmU9MTc2NjIzNjU3MyZ0PTE3NTA2ODQ1NzMmcj0xMjM0NTY3OCZmPS9odXlhbGl2ZS85ODcyMDI1My05ODcyMDI1My00MjQwMDAyNTgwODc4NDU4ODgtMTk3NTYzOTYyLTEwMDU3LUEtMC0xLzIwMjUwNjIzMjExNjEzLmpwZw=="
                ),
                (
                    ("背谱王子", "https://huyaimg.msstatic.com/avatar/1021/dc/24159304d9a4a213827190c975b806_180_135.jpg?1586585146"),
                    "象棋必胜套路走法！", 1457782,"https://anchorpost.msstatic.com/cdnimage/anchorpost/1021/dc/24159304d9a4a213827190c975b806_1671_1607443335.jpg?spformat=png,webp&imageview/4/0/w/338/h/190/blur/1"
                ),
                (
                    ("盛世-苏晓", "https://huyaimg.msstatic.com/avatar/1090/fb/f7962abb11dc559a27ec06688db06f_180_135.jpg?1736750983"),
                    "【苏晓】来来来,最好看的杂交版国战来了！", 113517, "https://anchorpost.msstatic.com/cdnimage/anchorpost/1090/fb/f7962abb11dc559a27ec06688db06f_3_1669_1679396816.jpg?spformat=png,webp&imageview/4/0/w/338/h/190/blur/1"
                )
            ].into_iter().enumerate().map(|(id,((user_name, avator_url), room_name, hot, img_url) )| {
                Room {
                    id,
                    hot,
                    img_url,
                    name: room_name,
                    owner: User { id: id + 10000, name: user_name, avatar_url: avator_url },
                    is_live: true,
                    tags: [
                        Tag::Blue("蓝光6M"),
                        Tag::Play("妲己"),
                        Tag::Official("潜力新秀")
                    ].to_vec()
                }
            }).collect::<Vec<Room<&'static str>>>(),
        },
        Cate {
            id: 12,
            icon_url:
                "https://a.msstatic.com/huya/main3/static/img/icon_match.png",
            cate_name: "电竞赛事",
            tags: [
                "传奇杯S2",
                "KPL",
                "PEL",
                "PCL",
                "CFPL",
                "CFML",
            ]
            .into_iter()
            .enumerate()
            .map(|(id, name)| HotRoom { name, id, is_live: id == 0 })
            .collect::<Vec<HotRoom<&'static str>>>(),
            live_total: 360,
            rooms: [
                (
                    ("王者荣耀赛事王者荣耀", "https://huyaimg.msstatic.com/avatar/1010/66/6aba6b4323ab3c52960e7bf169d08e_180_135.jpg?1737770255"), 
                    "济南RW 0:2 上海EDG KPL夏季赛", 2313000,"https://anchorpost.msstatic.com/cdnimage/anchorpost/1010/66/6aba6b4323ab3c52960e7bf169d08e_2336_1750665770.jpg?spformat=png,webp&imageview/4/0/w/338/h/190/blur/1/format/webp"
                ),
                (
                    ("天天吃鸡赛事", "https://huyaimg.msstatic.com/avatar/1026/a3/21c624bf332d4ad165d20f66d5b590_180_135.jpg?1742213810"),
                    "鱼虎巅峰赛 第三轮小组赛DAY1", 233632, "https://anchorpost.msstatic.com/cdnimage/anchorpost/1026/a3/21c624bf332d4ad165d20f66d5b590_2793_1750041275.jpg?spformat=png,webp&imageview/4/0/w/338/h/190/blur/1/format/webp"
                ),
                (
                    ("峡谷特训赛", "https://huyaimg.msstatic.com/avatar/1081/57/07061fcf3f531ec75e4d0366f81684_180_135.jpg?1749879375"),
                    "火力旋转王 2:0 自转队 淘汰赛", 1457782,"https://anchorpost.msstatic.com/cdnimage/anchorpost/1081/57/07061fcf3f531ec75e4d0366f81684_0_1749879744.jpg?spformat=png,webp&imageview/4/0/w/338/h/190/blur/1/format/webp"
                ),
                (
                    ("云顶之弈赛事", "https://huyaimg.msstatic.com/avatar/1009/64/d2d15d9cdf2e0f1963b69f47cf7893_180_135.jpg?1750044387"),
                    "TOC11小组赛开打！幻灭官少登场", 113517, "https://anchorpost.msstatic.com/cdnimage/anchorpost/1009/64/d2d15d9cdf2e0f1963b69f47cf7893_5485_1750658957.jpg?spformat=png,webp&imageview/4/0/w/338/h/190/blur/1/format/webp"
                )
            ].into_iter().enumerate().map(|(id,((user_name, avator_url), room_name, hot, img_url) )| {
                Room {
                    id,
                    hot,
                    img_url,
                    name: room_name,
                    owner: User { id: id + 10000, name: user_name, avatar_url: avator_url },
                    is_live: true,
                    tags: [
                        Tag::Blue("蓝光10M"),
                        Tag::Play("妲己"),
                        Tag::Official("潜力新秀")
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
                            <h1 class="flex gap-x-2.5 items-center mr-4 duration-300 text-[26px] leading-[33px] hover:text-[#f80]">
                                <img src=cate.icon_url width=32 height=32 />
                                {cate.cate_name}
                            </h1>
                            <ul class="flex gap-x-1.5 leadding-[26px] *:duration-300 *:select-none *:px-3 *:rounded-3xl *:border *:border-gray-400 *:text-gray-400 *:hover:text-[#f80]! *:hover:border-[#f80]">
                                {cate
                                    .tags
                                    .into_iter()
                                    .map(|tag| {
                                        view! {
                                            <li class=if tag.is_live {
                                                Some("text-[#3a3a3a]! ")
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
                                <a class="ml-1 duration-300 hover:text-[#f80]">"更多 >"</a>
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
            <div>
                <div class="flex relative justify-start items-end mb-4 text-[14px]">
                    <h1 class="flex gap-x-2.5 items-center mr-4 duration-300 text-[26px] leading-[33px] hover:text-[#f80]">
                        <img
                            src="data:image/png;base64,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"
                            width=32
                            height=32
                        />
                        官方活动

                    </h1>
                    <span class="absolute right-0 hover:text-[#f80]">更多 ></span>
                </div>
                <div class="flex gap-x-5 justify-between *:rounded-md *:flex-auto *:w-30">
                    <img
                        src="https://huyaimg.msstatic.com/cdnimage/actprop/php24FYzp1749607412.jpg"
                        loading="lazy"
                        alt=""
                    />
                    <img
                        src="https://huyaimg.msstatic.com/cdnimage/actprop/phpqWP2m31750146373.jpg"
                        loading="lazy"
                        alt=""
                    />
                    <img
                        src="https://huyaimg.msstatic.com/cdnimage/actprop/php90jNV31750045683.jpg"
                        loading="lazy"
                        alt=""
                    />
                    <img
                        src="https://huyaimg.msstatic.com/cdnimage/actprop/phpNRsz521749190835.jpg"
                        loading="lazy"
                        alt=""
                    />
                </div>
            </div>
        </figure>
    }
}

#[component]
fn RoomCard(data: Room<&'static str>) -> impl IntoView {
    let tag = !data.tags.is_empty();
    let tags = data.tags;
    view! {
        <div class="flex overflow-hidden relative flex-col bg-white rounded-md duration-200 flex-1/4 group/room-card hover:drop-shadow-md hover:drop-shadow-black/20">
            <div class="overflow-hidden relative after:duration-300 after:bg-no-repeat after:bg-transparent after:scale-150 after:opacity-0 after:absolute after:bg-center after:left-1/2 after:top-1/2 after:-translate-1/2 after:size-full after:bg-[url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAyCAYAAAAeP4ixAAAAGXRFWHRTb2Z0d2FyZQBBZG9iZSBJbWFnZVJlYWR5ccllPAAAAyFpVFh0WE1MOmNvbS5hZG9iZS54bXAAAAAAADw/eHBhY2tldCBiZWdpbj0i77u/IiBpZD0iVzVNME1wQ2VoaUh6cmVTek5UY3prYzlkIj8+IDx4OnhtcG1ldGEgeG1sbnM6eD0iYWRvYmU6bnM6bWV0YS8iIHg6eG1wdGs9IkFkb2JlIFhNUCBDb3JlIDUuNS1jMDE0IDc5LjE1MTQ4MSwgMjAxMy8wMy8xMy0xMjowOToxNSAgICAgICAgIj4gPHJkZjpSREYgeG1sbnM6cmRmPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5LzAyLzIyLXJkZi1zeW50YXgtbnMjIj4gPHJkZjpEZXNjcmlwdGlvbiByZGY6YWJvdXQ9IiIgeG1sbnM6eG1wPSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvIiB4bWxuczp4bXBNTT0iaHR0cDovL25zLmFkb2JlLmNvbS94YXAvMS4wL21tLyIgeG1sbnM6c3RSZWY9Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC9zVHlwZS9SZXNvdXJjZVJlZiMiIHhtcDpDcmVhdG9yVG9vbD0iQWRvYmUgUGhvdG9zaG9wIENDIChXaW5kb3dzKSIgeG1wTU06SW5zdGFuY2VJRD0ieG1wLmlpZDpGNjc3QzBENEZBNTkxMUU2QjkyNzlGRkY5QzBBMkE1QSIgeG1wTU06RG9jdW1lbnRJRD0ieG1wLmRpZDpGNjc3QzBENUZBNTkxMUU2QjkyNzlGRkY5QzBBMkE1QSI+IDx4bXBNTTpEZXJpdmVkRnJvbSBzdFJlZjppbnN0YW5jZUlEPSJ4bXAuaWlkOkY2NzdDMEQyRkE1OTExRTZCOTI3OUZGRjlDMEEyQTVBIiBzdFJlZjpkb2N1bWVudElEPSJ4bXAuZGlkOkY2NzdDMEQzRkE1OTExRTZCOTI3OUZGRjlDMEEyQTVBIi8+IDwvcmRmOkRlc2NyaXB0aW9uPiA8L3JkZjpSREY+IDwveDp4bXBtZXRhPiA8P3hwYWNrZXQgZW5kPSJyIj8+7vnOlQAAB19JREFUeNrcWmtMVEcU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)] hover:after:scale-100 hover:after:opacity-100 hover:after:bg-black/40">
                <img src=data.img_url alt="" loading="lazy" class="aspect-290/163" />
                <Show when=move || tag>
                    <div class="flex absolute top-0 left-0 flex-row-reverse gap-x-2 justify-start items-center p-1 w-full text-xs leading-5 text-white *:rounded-md *:px-2">
                        {tags
                            .iter()
                            .map(|tag| {
                                match tag {
                                    Tag::Blue(title) => {
                                        EitherOf3::A(
                                            view! { <span class="bg-sky-500">{*title}</span> },
                                        )
                                    }
                                    Tag::Play(title) => {
                                        EitherOf3::B(
                                            view! { <span class="text-right">{*title}</span> },
                                        )
                                    }
                                    Tag::Official(title) => {
                                        EitherOf3::C(
                                            view! {
                                                <span class="absolute left-1 bg-black/50">{*title}</span>
                                            },
                                        )
                                    }
                                }
                            })
                            .collect_view()}
                    </div>
                </Show>
            </div>
            <div class="flex flex-col gap-y-1.5 p-2.5 text-left">
                <p class="w-4/5 truncate group-hover/room-card:text-[#f80]">{data.name}</p>
                <div class="flex relative gap-x-1 justify-start items-center text-xs text-gray-400">
                    <img
                        src=data.owner.avatar_url
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
                                format!("{:.2}万", (data.hot as f64) / 10000.0)
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
