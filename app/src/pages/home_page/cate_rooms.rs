use leptos::either::EitherOf3;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

stylance::import_crate_style!(css, "src/pages/home_page/cate_room.module.scss");

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

impl From<Cate<&str>> for Cate<String> {
    fn from(value: Cate<&str>) -> Self {
        Self {
            id: value.id,
            icon_url: value.icon_url.to_owned(),
            cate_name: value.cate_name.to_owned(),
            tags: value.tags.into_iter().map(|tag| tag.into()).collect(),
            live_total: value.live_total,
            rooms: value.rooms.into_iter().map(|room| room.into()).collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct HotRoom<T: ToString> {
    id: usize,
    name: T,
    #[serde(default)]
    is_live: bool,
}
impl From<HotRoom<&str>> for HotRoom<String> {
    fn from(value: HotRoom<&str>) -> Self {
        Self {
            id: value.id,
            name: value.name.to_owned(),
            is_live: value.is_live,
        }
    }
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

impl From<Room<&str>> for Room<String> {
    fn from(value: Room<&str>) -> Self {
        Self {
            id: value.id,
            img_url: value.img_url.to_owned(),
            name: value.name.to_owned(),
            hot: value.hot,
            owner: value.owner.into(),
            tags: value.tags.into_iter().map(|tag| tag.into()).collect(),
            is_live: value.is_live,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User<T: ToString> {
    id: usize,
    name: T,
    avatar_url: T,
}

impl From<User<&str>> for User<String> {
    fn from(value: User<&str>) -> Self {
        Self {
            id: value.id,
            name: value.name.to_owned(),
            avatar_url: value.avatar_url.to_owned(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
enum Tag<T: ToString> {
    // 蓝光
    Blue(T),
    Play(T),
    Official(T),
}

impl From<Tag<&str>> for Tag<String> {
    fn from(value: Tag<&str>) -> Self {
        match value {
            Tag::Blue(val) => Self::Blue(val.to_owned()),
            Tag::Play(val) => Self::Play(val.to_owned()),
            Tag::Official(val) => Self::Official(val.to_owned()),
        }
    }
}

#[server]
#[lazy]
async fn get_recommend_cate_rooms() -> Result<Vec<Cate<String>>, ServerFnError> {
    let cates = vec![
        Cate {
            id: 0,
            icon_url: "/imgs/game/game_15735501549253_logo.png",
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
            .map(|(id, name)| HotRoom {
                name,
                id,
                is_live: id % 3 == 0,
            })
            .collect::<Vec<HotRoom<&'static str>>>(),
            live_total: 987,
            rooms: [
                (
                    (
                        "小超梦ovo",
                        "/imgs/user/837d557cdf07c6adf85a62540ce53d_180_135.jpg",
                    ),
                    "已王者 大师顶级教学看了包上分！",
                    909000,
                    "/imgs/live/20250703214233.jpg",
                ),
                (
                    (
                        "霸哥",
                        "/imgs/user/b1a406dd163efa3a547bf752c3a756_180_135.jpg",
                    ),
                    "峡谷MVP 3+4目前5/7",
                    783632,
                    "/imgs/live/3626ef611e990cbbcefab8e1bbc561_1_1751514294.jpg",
                ),
                (
                    (
                        "胖炸",
                        "/imgs/user/2aa2f6905fe4382221d08b66d7cdcb_180_135.jpg",
                    ),
                    "200N进180【第二届美女如云巅峰赛】",
                    1457782,
                    "/imgs/live/20250623103942.jpg",
                ),
                (
                    (
                        "微竞丶莎莉",
                        "/imgs/user/f20eec58c49c79f9925e88c60463e0_180_135.jpg",
                    ),
                    "复刻全英雄打野上大师教学！",
                    763517,
                    "/imgs/live/0c1deb1298b5a2904fa4173addd4fd_0_1751071263.jpg",
                ),
            ]
            .into_iter()
            .enumerate()
            .map(
                |(id, ((user_name, avator_url), room_name, hot, img_url))| Room {
                    id,
                    hot,
                    img_url,
                    name: room_name,
                    owner: User {
                        id: id + 10000,
                        name: user_name,
                        avatar_url: avator_url,
                    },
                    is_live: true,
                    tags: [
                        Tag::Blue("蓝光10M"),
                        Tag::Play("摸个鱼"),
                        Tag::Official("潜力新秀"),
                    ]
                    .to_vec(),
                },
            )
            .collect::<Vec<Room<&'static str>>>(),
        },
        Cate {
            id: 1,
            icon_url: "/imgs/game/game_15735501961519_logo.png",
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
            .map(|(id, name)| HotRoom {
                name,
                id,
                is_live: id == 0,
            })
            .collect::<Vec<HotRoom<&'static str>>>(),
            live_total: 468,
            rooms: [
                (
                    (
                        "为输出铠证明的南瓜",
                        "/imgs/user/9dae31ad175ccbd76b61e533d5d5c3_180_135.jpg",
                    ),
                    "身法铠 巅峰教学",
                    313000,
                    "/imgs/live/20250623121411.png",
                ),
                (
                    (
                        "小炎【妲己的神】",
                        "/imgs/user/684be77eb8ab1b8196588daeeedb3f_180_135.jpg",
                    ),
                    "6万场牢妲己国标号冲百星！",
                    233632,
                    "/imgs/live/dc363a04fbca13d9ef00dd8113a46b_2336_1751385174.jpg",
                ),
                (
                    (
                        "正恒-紫宸【相声木兰】",
                        "/imgs/user/a67056877955cf214026e4ceca103f_180_135.jpg",
                    ),
                    "让你三分钟爱上花木兰，来",
                    1457782,
                    "/imgs/live/90067a97bc6982d27d9076c1347958_0_1751524145.jpg",
                ),
                (
                    (
                        "M丶赵一",
                        "/imgs/user/33e7481e899d6fd6625a6082ebb016_180_135.jpg",
                    ),
                    "第一阿轲 130星排位教学",
                    113517,
                    "/imgs/live/a4286776aa02881faa959bbb2a94d5_2336_1751385049.jpg",
                ),
            ]
            .into_iter()
            .enumerate()
            .map(
                |(id, ((user_name, avator_url), room_name, hot, img_url))| Room {
                    id,
                    hot,
                    img_url,
                    name: room_name,
                    owner: User {
                        id: id + 10000,
                        name: user_name,
                        avatar_url: avator_url,
                    },
                    is_live: true,
                    tags: [].to_vec(),
                },
            )
            .collect::<Vec<Room<&'static str>>>(),
        },
        Cate {
            id: 2,
            icon_url: "/imgs/game/game_15735501794392_logo.png",
            cate_name: "星秀",
            tags: ["骚俊", "阿布", "车老板呢", "啵啵超Q", "VIKI", "小乖"]
                .into_iter()
                .enumerate()
                .map(|(id, name)| HotRoom {
                    name,
                    id,
                    is_live: id == 0,
                })
                .collect::<Vec<HotRoom<&'static str>>>(),
            live_total: 468,
            rooms: [
                (
                    (
                        "Ck-白允儿",
                        "/imgs/user/c14c4ff3ed481f363f2fb378c586db_180_135.jpg",
                    ),
                    "求接升~舞蹈~~【腼腆女孩】",
                    313000,
                    "/imgs/live/417f6f692cc018d7cf8f7ff7ac429f_2_1663_1733730509.jpg",
                ),
                (
                    (
                        "映画-赫拉拉不拉【桃】",
                        "/imgs/user/c3093ea82cf572c87577e92d58ae8f_180_135.jpg",
                    ),
                    "【温柔小女生】",
                    233632,
                    "/imgs/live/c3093ea82cf572c87577e92d58ae8f_1663_1736305022.jpg",
                ),
                (
                    (
                        "HX温柔",
                        "/imgs/user/423aef7b077fbe3429f7ac34bfaae3_180_135.jpg",
                    ),
                    "大爷 来玩啊~",
                    1457782,
                    "/imgs/live/423aef7b077fbe3429f7ac34bfaae3_1663_1747710542.jpg",
                ),
                (
                    (
                        "正恒YJ-VIKI【浈】",
                        "/imgs/user/2ed1bd7ace2c24597a3b2a09238bec_180_135.jpg",
                    ),
                    "我来了~长沙舞蹈主播    ",
                    113517,
                    "/imgs/live/2ed1bd7ace2c24597a3b2a09238bec_1663_1723466103.jpg",
                ),
            ]
            .into_iter()
            .enumerate()
            .map(
                |(id, ((user_name, avator_url), room_name, hot, img_url))| Room {
                    id,
                    hot,
                    img_url,
                    name: room_name,
                    owner: User {
                        id: id + 10000,
                        name: user_name,
                        avatar_url: avator_url,
                    },
                    is_live: true,
                    tags: [
                        Tag::Blue("蓝光6M"),
                        Tag::Play("妲己"),
                        Tag::Official("潜力新秀"),
                    ]
                    .to_vec(),
                },
            )
            .collect::<Vec<Room<&'static str>>>(),
        },
        Cate {
            id: 3,
            icon_url: "/imgs/game/game_15735502223924_logo.png",
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
                "伞兵",
            ]
            .into_iter()
            .enumerate()
            .map(|(id, name)| HotRoom {
                name,
                id,
                is_live: id == 0,
            })
            .collect::<Vec<HotRoom<&'static str>>>(),
            live_total: 212,
            rooms: [
                (
                    (
                        "鲨鱼哟",
                        "/imgs/user/6f3bf8adc63a7432f5e12b8bec4662_180_135.jpg",
                    ),
                    "5天不超不求人摆地摊【第二天】",
                    2313000,
                    "/imgs/live/20250623183443.png",
                ),
                (
                    (
                        "LK-王小宝【全能王】",
                        "/imgs/user/98d9bd27b864d7f438efd18fc257eb_180_135.jpg",
                    ),
                    "【癫疯赛冲击第一教学】",
                    233632,
                    "/imgs/live/98d9bd27b864d7f438efd18fc257eb_3203_1749714584.jpg",
                ),
                (
                    (
                        "TG-老熊【平头哥】",
                        "/imgs/user/9c8c33e92deb95ae006acd68a976e0_180_135.jpg",
                    ),
                    "沙漠狙神1v4 6鸡挑战中",
                    1457782,
                    "/imgs/live/9c8c33e92deb95ae006acd68a976e0_3203_1749714734.jpg",
                ),
                (
                    (
                        "和平慢镜头",
                        "/imgs/user/b4b4595ae7a622e9a1c8d6707fd0d4_180_135.jpg",
                    ),
                    "PEL新赛季训练赛直播",
                    113517,
                    "/imgs/live/20250623183445.png",
                ),
            ]
            .into_iter()
            .enumerate()
            .map(
                |(id, ((user_name, avator_url), room_name, hot, img_url))| Room {
                    id,
                    hot,
                    img_url,
                    name: room_name,
                    owner: User {
                        id: id + 10000,
                        name: user_name,
                        avatar_url: avator_url,
                    },
                    is_live: true,
                    tags: [
                        Tag::Blue("蓝光8M"),
                        Tag::Play("妲己"),
                        Tag::Official("潜力新秀"),
                    ]
                    .to_vec(),
                },
            )
            .collect::<Vec<Room<&'static str>>>(),
        },
        Cate {
            id: 4,
            icon_url: "/imgs/game/game_15735502034733_logo.png",
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
                "星魂",
            ]
            .into_iter()
            .enumerate()
            .map(|(id, name)| HotRoom {
                name,
                id,
                is_live: id == 0,
            })
            .collect::<Vec<HotRoom<&'static str>>>(),
            live_total: 242,
            rooms: [
                (
                    (
                        "天天吃鸡赛事",
                        "/imgs/user/21c624bf332d4ad165d20f66d5b590_180_135.jpg",
                    ),
                    "鱼虎巅峰赛 第三轮小组赛DAY1",
                    2313000,
                    "/imgs/live/21c624bf332d4ad165d20f66d5b590_2793_1750041275.jpg",
                ),
                (
                    (
                        "Wsy、3胖",
                        "/imgs/user/def38ec9cd694c33c79410170e88bf_180_135.jpg",
                    ),
                    "冲第三个生存者，可上车",
                    233632,
                    "/imgs/live/20250623183426.png",
                ),
                (
                    (
                        "4AMGodv",
                        "/imgs/user/062b3abea4cfe237a3dab14dc5753a_180_135.jpg",
                    ),
                    "ewc预选赛day4. 3分钟延迟",
                    1457782,
                    "/imgs/live/062b3abea4cfe237a3dab14dc5753a_3_2793_1749607045.jpg",
                ),
                (
                    (
                        "宿舍战神-BB文",
                        "/imgs/user/859091a71db0eaec7b6c24ab4cff5c_180_135.jpg",
                    ),
                    "睡前小故事",
                    113517,
                    "/imgs/live/859091a71db0eaec7b6c24ab4cff5c_3_2793_1750317274.jpg",
                ),
            ]
            .into_iter()
            .enumerate()
            .map(
                |(id, ((user_name, avator_url), room_name, hot, img_url))| Room {
                    id,
                    hot,
                    img_url,
                    name: room_name,
                    owner: User {
                        id: id + 10000,
                        name: user_name,
                        avatar_url: avator_url,
                    },
                    is_live: true,
                    tags: [
                        Tag::Blue("蓝光10M"),
                        Tag::Play("妲己"),
                        Tag::Official("潜力新秀"),
                    ]
                    .to_vec(),
                },
            )
            .collect::<Vec<Room<&'static str>>>(),
        },
        Cate {
            id: 5,
            icon_url: "/imgs/game/game_15735502126348_logo.png",
            cate_name: "吃喝玩乐",
            tags: [
                "童锦程",
                "小小小酷哥",
                "张开朗",
                "子圣",
                "集梦会长",
                "小龙女",
            ]
            .into_iter()
            .enumerate()
            .map(|(id, name)| HotRoom {
                name,
                id,
                is_live: id == 0,
            })
            .collect::<Vec<HotRoom<&'static str>>>(),
            live_total: 492,
            rooms: [
                (
                    (
                        "Time-祁猪冲鸭",
                        "/imgs/user/7bd0ded9bc954febba79ae8aa6248b_180_135.jpg",
                    ),
                    "【炒粉四倍】今天就祝福我80大寿！！！！",
                    2313000,
                    "/imgs/live/7bd0ded9bc954febba79ae8aa6248b_2633_1750231559.jpg",
                ),
                (
                    (
                        "倪海杉",
                        "/imgs/user/20b606aaaf63ace67eccfbcfeb623b_180_135.jpg",
                    ),
                    "重庆",
                    233632,
                    "/imgs/live/20250703220603.png",
                ),
                (
                    (
                        "阿布【只为1380】",
                        "/imgs/user/17c17227ad6afbf51c6e9a70a8e670_180_135.jpg",
                    ),
                    "荒野求生 第一天",
                    1457782,
                    "/imgs/live/20250703220604.png",
                ),
                (
                    (
                        "Hot-芥末",
                        "/imgs/user/bf520b62b2011884630b277d92aa04_180_135.jpg",
                    ),
                    "【唱】来听歌叭",
                    113517,
                    "/imgs/live/20250623183445.webp",
                ),
            ]
            .into_iter()
            .enumerate()
            .map(
                |(id, ((user_name, avator_url), room_name, hot, img_url))| Room {
                    id,
                    hot,
                    img_url,
                    name: room_name,
                    owner: User {
                        id: id + 10000,
                        name: user_name,
                        avatar_url: avator_url,
                    },
                    is_live: true,
                    tags: [
                        Tag::Blue("蓝光8M"),
                        Tag::Play("妲己"),
                        Tag::Official("潜力新秀"),
                    ]
                    .to_vec(),
                },
            )
            .collect::<Vec<Room<&'static str>>>(),
        },
        Cate {
            id: 6,
            icon_url: "/imgs/game/game_15735501875101_logo.png",
            cate_name: "穿越火线",
            tags: [
                "阿飞",
                "绝迹",
                "梧桐",
                "飞段",
                "飞机文",
                "老街",
                "宠儿",
                "周虎",
            ]
            .into_iter()
            .enumerate()
            .map(|(id, name)| HotRoom {
                name,
                id,
                is_live: id == 0,
            })
            .collect::<Vec<HotRoom<&'static str>>>(),
            live_total: 212,
            rooms: [
                (
                    (
                        "WH-妖狼",
                        "/imgs/user/ee4794a5ea86b87dab424edcab886c_180_135.jpg",
                    ),
                    "30人生化乱斗村子 没单不排队",
                    2313000,
                    "/imgs/live/8bc5fbfd959766028e82c176aa7f4d_4_1747729822.jpg",
                ),
                (
                    (
                        "AzZ丶阿飞512",
                        "/imgs/user/4c8c7a479660cf8694cbcd549f4361_180_135.jpg",
                    ),
                    "【癫疯赛冲击第一教学】",
                    233632,
                    "/imgs/live/20250703220601.webp",
                ),
                (
                    (
                        "穿越火线赛事",
                        "/imgs/user/bb80adc2e5f5e3e95cdd25cf8fc9be_180_135.jpg",
                    ),
                    "【预告】19点 WE vs EP 2025CFPL夏季赛",
                    1457782,
                    "/imgs/live/bb80adc2e5f5e3e95cdd25cf8fc9be_4_1750614328.jpg",
                ),
                (
                    (
                        "AG绝迹",
                        "/imgs/user/8bc5fbfd959766028e82c176aa7f4d_180_135.jpg",
                    ),
                    "妹子代播到晚上七点，排位啦",
                    113517,
                    "/imgs/live/559caa7f721b4acc14563ea5d5cdc5_4_1748598139.jpg",
                ),
            ]
            .into_iter()
            .enumerate()
            .map(
                |(id, ((user_name, avator_url), room_name, hot, img_url))| Room {
                    id,
                    hot,
                    img_url,
                    name: room_name,
                    owner: User {
                        id: id + 10000,
                        name: user_name,
                        avatar_url: avator_url,
                    },
                    is_live: true,
                    tags: [
                        Tag::Blue("蓝光6M"),
                        Tag::Play("妲己"),
                        Tag::Official("潜力新秀"),
                    ]
                    .to_vec(),
                },
            )
            .collect::<Vec<Room<&'static str>>>(),
        },
        Cate {
            id: 7,
            icon_url: "/imgs/game/game_15735502396542_logo.png",
            cate_name: "主机游戏",
            tags: [
                "楚河", "小宇", "贱圣", "导演", "星辉", "老鬼", "马桶", "柚子",
            ]
            .into_iter()
            .enumerate()
            .map(|(id, name)| HotRoom {
                name,
                id,
                is_live: id == 0,
            })
            .collect::<Vec<HotRoom<&'static str>>>(),
            live_total: 775,
            rooms: [
                (
                    (
                        "狂鸟丶楚河-90327",
                        "/imgs/user/fd6f69d69c0015eaface1f6024869e_180_135.jpg",
                    ),
                    "直播上万款游戏新主播，很紧张！",
                    2313000,
                    "/imgs/live/20250623184315.webp",
                ),
                (
                    (
                        "狂鸟丶爱吃鱼",
                        "/imgs/user/b2053d8985301ea3d7a327220fc645_180_135.jpg",
                    ),
                    "漫漫长夜 大熊岛活地图永不迷路~",
                    233632,
                    "/imgs/live/20250623184306.webp",
                ),
                (
                    (
                        "叫我Happy呀",
                        "/imgs/user/ea71a96d700ecf09a8337edf40377d_180_135.jpg",
                    ),
                    "独狼挑战5小时毕业！",
                    1457782,
                    "/imgs/live/20250623184326.webp",
                ),
                (
                    (
                        "萨摩直播实况",
                        "/imgs/user/45240573150f09aa27dd407e0b26b2_180_135.jpg",
                    ),
                    "8月28日船长猎人",
                    113517,
                    "/imgs/live/4.webp",
                ),
            ]
            .into_iter()
            .enumerate()
            .map(
                |(id, ((user_name, avator_url), room_name, hot, img_url))| Room {
                    id,
                    hot,
                    img_url,
                    name: room_name,
                    owner: User {
                        id: id + 10000,
                        name: user_name,
                        avatar_url: avator_url,
                    },
                    is_live: true,
                    tags: [
                        Tag::Blue("蓝光4M"),
                        Tag::Play("妲己"),
                        Tag::Official("潜力新秀"),
                    ]
                    .to_vec(),
                },
            )
            .collect::<Vec<Room<&'static str>>>(),
        },
        Cate {
            id: 8,
            icon_url: "/imgs/game/game_17228323681551_logo.png",
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
            .map(|(id, name)| HotRoom {
                name,
                id,
                is_live: id == 0,
            })
            .collect::<Vec<HotRoom<&'static str>>>(),
            live_total: 99,
            rooms: [
                (
                    (
                        "Time-祁猪冲鸭",
                        "/imgs/user/7bd0ded9bc954febba79ae8aa6248b_180_135.jpg",
                    ),
                    "【炒粉四倍】今天就祝福我80大寿！！！！",
                    2313000,
                    "/imgs/live/20250703220602.webp",
                ),
                (
                    (
                        "正恒RM丶原子不可爱",
                        "/imgs/user/81f911f3f306fc812b95bf09cfbfdc_180_135.jpg",
                    ),
                    "【3D音】 晚上好",
                    233632,
                    "/imgs/live/20250623184310.webp",
                ),
                (
                    (
                        "Hot-芥末",
                        "/imgs/user/79c31272ac119ad9588951f9a79e76_180_135.jpg",
                    ),
                    "【唱】来听歌叭",
                    1457782,
                    "/imgs/live/79c31272ac119ad9588951f9a79e76_2633_1749434026.jpg",
                ),
                (
                    (
                        "京梦-小拂",
                        "/imgs/user/ec6ff6c9efc147cbda12dff86faf23_180_135.jpg",
                    ),
                    "【保灯】也想被偏爱",
                    113517,
                    "/imgs/live/ec6ff6c9efc147cbda12dff86faf23_2633_1744165320.jpg",
                ),
            ]
            .into_iter()
            .enumerate()
            .map(
                |(id, ((user_name, avator_url), room_name, hot, img_url))| Room {
                    id,
                    hot,
                    img_url,
                    name: room_name,
                    owner: User {
                        id: id + 10000,
                        name: user_name,
                        avatar_url: avator_url,
                    },
                    is_live: true,
                    tags: [
                        Tag::Blue("蓝光10M"),
                        Tag::Play("妲己"),
                        Tag::Official("潜力新秀"),
                    ]
                    .to_vec(),
                },
            )
            .collect::<Vec<Room<&'static str>>>(),
        },
        Cate {
            id: 9,
            icon_url: "/imgs/game/game_15735502635279_logo.png",
            cate_name: "交友",
            tags: []
                .into_iter()
                .enumerate()
                .map(|(id, name)| HotRoom {
                    name,
                    id,
                    is_live: id == 0,
                })
                .collect::<Vec<HotRoom<&'static str>>>(),
            live_total: 212,
            rooms: [
                (
                    (
                        "Dae-爸比我甜-视频厅",
                        "/imgs/user/b7d898c2ac3c1990d8dd956a69450d_180_135.jpg",
                    ),
                    "来源：附近的人 距离你3km",
                    2313000,
                    "/imgs/live/f42c225332e3d05ebcc65ecefa1cc4_4079_1708411494.jpg",
                ),
                (
                    (
                        "yM-有幸遇见",
                        "/imgs/user/44e864d4d9b64af3ec1aa4e726f9db_180_135.jpg",
                    ),
                    " 白毛浮绿水 这里全是腿【久久冠】",
                    233632,
                    "/imgs/live/44e864d4d9b64af3ec1aa4e726f9db_4079_1689141780.jpg",
                ),
                (
                    (
                        "逸鹿-摇曳的心",
                        "/imgs/user/495db98a89a9f983a13e27b1dd5ecc_180_135.jpg",
                    ),
                    "越努力 越幸福！女神推荐：开心",
                    1457782,
                    "/imgs/live/495db98a89a9f983a13e27b1dd5ecc_4079_1719546892.jpg",
                ),
                (
                    (
                        "金水-hello姐姐",
                        "/imgs/user/1ff9df4ce065d2ca6c75ed13efc650_180_135.jpg",
                    ),
                    "幸福吗？包的",
                    113517,
                    "/imgs/live/1ff9df4ce065d2ca6c75ed13efc650_4079_1705483693.jpg",
                ),
            ]
            .into_iter()
            .enumerate()
            .map(
                |(id, ((user_name, avator_url), room_name, hot, img_url))| Room {
                    id,
                    hot,
                    img_url,
                    name: room_name,
                    owner: User {
                        id: id + 10000,
                        name: user_name,
                        avatar_url: avator_url,
                    },
                    is_live: true,
                    tags: [
                        Tag::Blue("蓝光10M"),
                        Tag::Play("妲己"),
                        Tag::Official("潜力新秀"),
                    ]
                    .to_vec(),
                },
            )
            .collect::<Vec<Room<&'static str>>>(),
        },
        Cate {
            id: 10,
            icon_url: "/imgs/game/game_15735502715195_logo.png",
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
            .map(|(id, name)| HotRoom {
                name,
                id,
                is_live: id == 0,
            })
            .collect::<Vec<HotRoom<&'static str>>>(),
            live_total: 360,
            rooms: [
                (
                    (
                        "瓦莉拉",
                        "/imgs/user/ffabf13c4cc3effa9398c2859ac9c8_180_135.jpg",
                    ),
                    "战棋瓦：万八冲两万了！做大技播！",
                    2313000,
                    "/imgs/live/20250703220607.webp",
                ),
                (
                    (
                        "炉石春哥",
                        "/imgs/user/85bab76cc83a7feac8fa4f8a7fef31_180_135.jpg",
                    ),
                    "毒号正在教育主播",
                    233632,
                    "/imgs/live/20250703220607o.webp",
                ),
                (
                    (
                        "老中医",
                        "/imgs/user/c7842e28339e0cd8c3d0473579b956_180_135.jpg",
                    ),
                    "医：今日雪耻 目标8000！",
                    1457782,
                    "/imgs/live/c7842e28339e0cd8c3d0473579b956_393_1747053190.jpg",
                ),
                (
                    (
                        "IG随缘风",
                        "/imgs/user/87bdf9f5e7d1a662766b87bd128721_180_135.jpg",
                    ),
                    "品鉴一下平台赛选手卡组",
                    113517,
                    "/imgs/live/20250623212226.webp",
                ),
            ]
            .into_iter()
            .enumerate()
            .map(
                |(id, ((user_name, avator_url), room_name, hot, img_url))| Room {
                    id,
                    hot,
                    img_url,
                    name: room_name,
                    owner: User {
                        id: id + 10000,
                        name: user_name,
                        avatar_url: avator_url,
                    },
                    is_live: true,
                    tags: [
                        Tag::Blue("蓝光10M"),
                        Tag::Play("妲己"),
                        Tag::Official("潜力新秀"),
                    ]
                    .to_vec(),
                },
            )
            .collect::<Vec<Room<&'static str>>>(),
        },
        Cate {
            id: 11,
            icon_url: "/imgs/game/game_15735502786566_logo.png",
            cate_name: "棋牌桌游",
            tags: ["鲍勃哥", "渝乐", "单走一张六", "刘小怂", "怂俊"]
                .into_iter()
                .enumerate()
                .map(|(id, name)| HotRoom {
                    name,
                    id,
                    is_live: id == 0,
                })
                .collect::<Vec<HotRoom<&'static str>>>(),
            live_total: 360,
            rooms: [
                (
                    (
                        "绽放曙光",
                        "/imgs/user/0b7172fb741f595706a33dcc25e2a9_180_135.jpg",
                    ),
                    "呵 弱爆",
                    2313000,
                    "/imgs/live/0b7172fb741f595706a33dcc25e2a9_2_2688_1735902634.jpg",
                ),
                (
                    (
                        "渝乐",
                        "/imgs/user/588a8bfdd0c7ddf8415b7f5cbe582c_180_135.jpg",
                    ),
                    "毒号正在教育主播",
                    233632,
                    "/imgs/live/20250703224414.webp",
                ),
                (
                    (
                        "背谱王子",
                        "/imgs/user/24159304d9a4a213827190c975b806_180_135.jpg",
                    ),
                    "象棋必胜套路走法！",
                    1457782,
                    "/imgs/live/24159304d9a4a213827190c975b806_1671_1607443335.jpg",
                ),
                (
                    (
                        "盛世-苏晓",
                        "/imgs/user/f7962abb11dc559a27ec06688db06f_180_135.jpg",
                    ),
                    "【苏晓】来来来,最好看的杂交版国战来了！",
                    113517,
                    "/imgs/live/f7962abb11dc559a27ec06688db06f_3_1669_1679396816.jpg",
                ),
            ]
            .into_iter()
            .enumerate()
            .map(
                |(id, ((user_name, avator_url), room_name, hot, img_url))| Room {
                    id,
                    hot,
                    img_url,
                    name: room_name,
                    owner: User {
                        id: id + 10000,
                        name: user_name,
                        avatar_url: avator_url,
                    },
                    is_live: true,
                    tags: [
                        Tag::Blue("蓝光6M"),
                        Tag::Play("妲己"),
                        Tag::Official("潜力新秀"),
                    ]
                    .to_vec(),
                },
            )
            .collect::<Vec<Room<&'static str>>>(),
        },
        Cate {
            id: 12,
            icon_url: "/imgs/game/icon_match.png",
            cate_name: "电竞赛事",
            tags: ["传奇杯S2", "KPL", "PEL", "PCL", "CFPL", "CFML"]
                .into_iter()
                .enumerate()
                .map(|(id, name)| HotRoom {
                    name,
                    id,
                    is_live: id == 0,
                })
                .collect::<Vec<HotRoom<&'static str>>>(),
            live_total: 360,
            rooms: [
                (
                    (
                        "王者荣耀赛事王者荣耀",
                        "/imgs/user/6aba6b4323ab3c52960e7bf169d08e_180_135.jpg",
                    ),
                    "济南RW 0:2 上海EDG KPL夏季赛",
                    2313000,
                    "/imgs/live/6aba6b4323ab3c52960e7bf169d08e_2336_1750665770.jpg",
                ),
                (
                    (
                        "天天吃鸡赛事",
                        "/imgs/user/21c624bf332d4ad165d20f66d5b590_180_135.jpg",
                    ),
                    "鱼虎巅峰赛 第三轮小组赛DAY1",
                    233632,
                    "/imgs/live/d5f2a572c6b2b525f32feb613dc4c1_3203_1751438340.jpg",
                ),
                (
                    (
                        "峡谷特训赛",
                        "/imgs/user/07061fcf3f531ec75e4d0366f81684_180_135.jpg",
                    ),
                    "火力旋转王 2:0 自转队 淘汰赛",
                    1457782,
                    "/imgs/live/07061fcf3f531ec75e4d0366f81684_0_1749879744.jpg",
                ),
                (
                    (
                        "云顶之弈赛事",
                        "/imgs/user/d2d15d9cdf2e0f1963b69f47cf7893_180_135.jpg",
                    ),
                    "TOC11小组赛开打！幻灭官少登场",
                    113517,
                    "/imgs/live/d2d15d9cdf2e0f1963b69f47cf7893_5485_1750658957.jpg",
                ),
            ]
            .into_iter()
            .enumerate()
            .map(
                |(id, ((user_name, avator_url), room_name, hot, img_url))| Room {
                    id,
                    hot,
                    img_url,
                    name: room_name,
                    owner: User {
                        id: id + 10000,
                        name: user_name,
                        avatar_url: avator_url,
                    },
                    is_live: true,
                    tags: [
                        Tag::Blue("蓝光10M"),
                        Tag::Play("妲己"),
                        Tag::Official("潜力新秀"),
                    ]
                    .to_vec(),
                },
            )
            .collect::<Vec<Room<&'static str>>>(),
        },
    ];
    Ok(cates.into_iter().map(|cate| cate.into()).collect())
}

#[component]
pub fn CateRooms() -> impl IntoView {
    let get_data = Resource::new(|| (), |_| get_recommend_cate_rooms());

    view! {
        <figure class=css::cate_rooms_list>
            <Suspense fallback=|| {
                "loading..."
            }>
                {move || Suspend::new(async move {
                    let rooms = get_data.get().unwrap_or(Ok(vec![])).unwrap();

                    view! {
                        <For each=move || rooms.clone().into_iter() key=|cate| cate.id let(cate)>
                            <div>
                                <div class=css::head>
                                    <h1 class=css::title>
                                        <img src=cate.icon_url width=32 height=32 />
                                        {cate.cate_name}
                                    </h1>
                                    <ul class=css::rooms_clx>
                                        {cate
                                            .tags
                                            .into_iter()
                                            .map(|tag| {
                                                view! {
                                                    <li class=if tag.is_live {
                                                        Some(css::live)
                                                    } else {
                                                        None
                                                    }>{tag.name}</li>
                                                }
                                            })
                                            .collect_view()}
                                    </ul>
                                    <p class=css::status>
                                        <span>548</span>
                                        个主播正在直播
                                        <a>"更多 >"</a>
                                    </p>
                                </div>
                                <div class=css::rooms>
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
                    }
                })}
            </Suspense>

            <div>
                <div class=css::activity>
                    <h1 class=css::head>
                        <img src="/imgs/activity.png" width=32 height=32 />
                        官方活动

                    </h1>
                    <span class=css::more>更多 ></span>
                </div>
                <div class=css::imgs>
                    <img src="/imgs/php24FYzp1749607412.jpg" loading="lazy" alt="" />
                    <img src="/imgs/phpqWP2m31750146373.jpg" loading="lazy" alt="" />
                    <img src="/imgs/php90jNV31750045683.jpg" loading="lazy" alt="" />
                    <img src="/imgs/phpNRsz521749190835.jpg" loading="lazy" alt="" />
                </div>
            </div>
        </figure>
    }
}

#[component]
fn RoomCard(data: Room<String>) -> impl IntoView {
    let tag = !data.tags.is_empty();
    let tags = data.tags;

    view! {
        <div class=css::container_clx>
            <div class=css::img_box_clx>
                <img src=data.img_url alt="" loading="lazy" class=css::room_img />
                <Show when=move || tag>
                    <div class=css::tag_box_clx>
                        {tags
                            .iter()
                            .map(|tag| {
                                match tag {
                                    Tag::Blue(title) => {
                                        EitherOf3::A(
                                            view! { <span class=css::blue>{title.clone()}</span> },
                                        )
                                    }
                                    Tag::Play(title) => {
                                        EitherOf3::B(
                                            view! { <span class=css::play>{title.clone()}</span> },
                                        )
                                    }
                                    Tag::Official(title) => {
                                        EitherOf3::C(
                                            view! { <span class=css::official>{title.clone()}</span> },
                                        )
                                    }
                                }
                            })
                            .collect_view()}
                    </div>
                </Show>
            </div>
            <div class=css::anchor_info>
                <p class=css::room_name>{data.name}</p>
                <div class=css::info_container>
                    <img src=data.owner.avatar_url alt="" width=24 height=24 loading="lazy" />
                    <span class=css::user_name>{data.owner.name}</span>
                    <div class=css::hot>
                        <span class=css::room_hot_clx>
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
