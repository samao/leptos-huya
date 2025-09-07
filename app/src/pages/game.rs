use leptos::{either::Either, prelude::*};

mod banner;
use banner::Banner;
mod hot_game;
use hot_game::{GameVo, HotGame};
mod recommend_game;
use recommend_game::{RecommendGame, RecommendVo};
mod rest_game;
use rest_game::{RestGame, RestVo};

use cfg_block::cfg_block;

cfg_block! {
    #[cfg(feature="ssr")] {
        use recommend_game::InCommingServer;

        async fn get_banner() -> Vec<String> {
            vec![
                "/imgs/game/202505200401053478.jpg".to_string(),
                "/imgs/game/202503060754563926.jpg".to_string(),
                "/imgs/game/202505230620101782.jpg".to_string(),
                "/imgs/game/202505200400544836.jpg".to_string(),
                "/imgs/game/202507150920442957.jpg".to_string(),
            ]
        }

        async fn get_hot_game() -> Vec<GameVo> {

            vec![
                GameVo {
                    title: "梦回江湖".to_string(),
                    cover_url: "/imgs/game/41/293_196.png".to_string(),
                    backend_url: Some("/imgs/game/41/293_196_h.png".to_string()),
                    game_type: "角色扮演".to_string(),
                    has_gift: true,
                    ..Default::default()
                },
                GameVo {
                    title: "热血战纪".to_string(),
                    cover_url: "/imgs/game/40/293_196.png".to_string(),
                    backend_url: Some("/imgs/game/40/293_196_h.png".to_string()),
                    game_type: "角色扮演".to_string(),
                    ..Default::default()
                },
                GameVo {
                    title: "帝王霸业".to_string(),
                    cover_url: "/imgs/game/55/293_196.png".to_string(),
                    backend_url: Some("/imgs/game/55/293_196_h.png".to_string()),
                    game_type: "角色扮演".to_string(),
                    has_gift: true,
                    ..Default::default()
                },
                GameVo {
                    title: "乾坤天地".to_string(),
                    cover_url: "/imgs/game/62/293_196.png".to_string(),
                    backend_url: Some("/imgs/game/62/293_196_h.png".to_string()),
                    game_type: "角色扮演".to_string(),
                    ..Default::default()
                }
            ]
        }

        async fn get_recommend() -> RecommendVo {
            let servers = vec![
                InCommingServer {
                    id: 64,
                    game_name: "传奇正传".to_string(),
                    server_name : "虎牙13服".to_string(),
                    time: "09-07 10:00".to_string(),
                },
                InCommingServer {
                    id: 62,
                    game_name: "乾坤天地".to_string(),
                    server_name: "虎牙99服".to_string(),
                    time: "09-07 09:00".to_string(),
                },
                InCommingServer {
                    id: 41,
                    game_name: "梦回江湖".to_string(),
                    server_name: "虎牙784服".to_string(),
                    time: "09-07 09:00".to_string(),
                },
                InCommingServer {
                    id: 56,
                    game_name: "开天西游".to_string(),
                    server_name: "虎牙144服".to_string(),
                    time: "09-07 09:00".to_string(),
                },
                InCommingServer {
                    id: 62,
                    game_name: "乾坤天地".to_string(),
                    server_name: "虎牙98服".to_string(),
                    time: "09-06 19:00".to_string(),
                },
                InCommingServer {
                    id: 64,
                    game_name: "传奇正传".to_string(),
                    server_name: "虎牙12服".to_string(),
                    time: "09-06 10:00".to_string(),
                },
                InCommingServer {
                    id: 61,
                    game_name: "龙之女神".to_string(),
                    server_name: "虎牙10服".to_string(),
                    time: "09-06 10:00".to_string(),
                },
                InCommingServer {
                    id: 58,
                    game_name: "龙域世界".to_string(),
                    server_name: "虎牙63服".to_string(),
                    time: "09-06 10:00".to_string(),
                },
                InCommingServer {
                    id: 43,
                    game_name: "传奇岁月".to_string(),
                    server_name: "玛法231服".to_string(),
                    time: "09-06 10:00".to_string(),
                }
            ];
            let games = vec![
                GameVo {
                    cover_url: "/imgs/game/64/272_136.png".to_string(),
                    title: "传奇正传".to_string(),
                    is_new: true,
                    ..Default::default()
                },
                GameVo {
                    cover_url: "/imgs/game/61/272_136.png".to_string(),
                    title: "龙之女神".to_string(),
                    is_new: true,
                    is_hot: true,
                    ..Default::default()
                },
                GameVo {
                    cover_url: "/imgs/game/63/272_136.png".to_string(),
                    title: "王城霸主".to_string(),
                    ..Default::default()
                },
                GameVo {
                    cover_url: "/imgs/game/51/272_136.png".to_string(),
                    is_hot: true,
                    title: "异兽洪荒".to_string(),
                    ..Default::default()
                },
                GameVo {
                    cover_url: "/imgs/game/43/272_136.png".to_string(),
                    title: "传奇岁月".to_string(),
                    ..Default::default()
                },
                GameVo {
                    cover_url: "/imgs/game/17/272_136.png".to_string(),
                    title: "百战沙城".to_string(),
                    ..Default::default()
                },
                GameVo {
                    cover_url: "/imgs/game/58/272_136.png".to_string(),
                    title: "龙域世界".to_string(),
                    ..Default::default()
                },
                GameVo {
                    cover_url: "/imgs/game/56/272_136.png".to_string(),
                    title: "开天西游".to_string(),
                    ..Default::default()
                },
                GameVo {
                    cover_url: "/imgs/game/200030/272_136.png".to_string(),
                    title: "维京传奇".to_string(),
                    ..Default::default()
                }
            ];
            RecommendVo {
                servers,
                games,
            }
        }
        async fn get_rest() -> RestVo {
            RestVo {
                news: vec![
                    "开天西游虎牙144服9月7日9点火爆开启".to_string(),
                    "乾坤天地虎牙99服9月7日9点火爆开启".to_string(),
                    "梦回江湖虎牙784服9月7日9点火爆开启".to_string(),
                    "乾坤天地虎牙98服9月6日19点火爆开启".to_string(),
                    "传奇岁月玛法231服9月6日10点火爆开启".to_string(),
                    "龙域世界虎牙63服9月6日10点火爆开启".to_string(),
                ],
                games: vec![
                    InCommingServer {
                        id: 36,
                        game_name: "上古传说".to_string(),
                        time: "".to_string(),
                        server_name: "".to_string(),
                    },
                    InCommingServer {
                        id: 44,
                        game_name: "刺沙".to_string(),
                        time: "".to_string(),
                        server_name: "".to_string(),
                    },
                    InCommingServer {
                        id: 33,
                        game_name: "王者之心2".to_string(),
                        time: "".to_string(),
                        server_name: "".to_string(),
                    },
                    InCommingServer {
                        id: 50,
                        game_name: "大天神".to_string(),
                        time: "".to_string(),
                        server_name: "".to_string(),
                    },
                    InCommingServer {
                        id: 53,
                        game_name: "破天".to_string(),
                        time: "".to_string(),
                        server_name: "".to_string(),
                    },
                    InCommingServer {
                        id: 59,
                        game_name: "倾国之怒".to_string(),
                        time: "".to_string(),
                        server_name: "".to_string(),
                    },
                    InCommingServer {
                        id: 57,
                        game_name: "千军纵横".to_string(),
                        time: "".to_string(),
                        server_name: "".to_string(),
                    },
                    InCommingServer {
                        id: 46,
                        game_name: "魔魂之刃".to_string(),
                        time: "".to_string(),
                        server_name: "".to_string(),
                    },
                    InCommingServer {
                        id: 64,
                        game_name: "传奇正传".to_string(),
                        time: "".to_string(),
                        server_name: "".to_string(),
                    },
                    InCommingServer {
                        id: 61,
                        game_name: "龙之女神".to_string(),
                        time: "".to_string(),
                        server_name: "".to_string(),
                    },
                ],
            }
        }
    }
}

type PageDataVo = (Vec<String>, Vec<GameVo>, RecommendVo, RestVo);

#[server]
async fn get_game_data() -> Result<PageDataVo, ServerFnError> {
    let all_data = tokio::join!(get_banner(), get_hot_game(), get_recommend(), get_rest());
    Ok(all_data)
}

#[component]
pub fn Game() -> impl IntoView {
    stylance::import_crate_style!(css, "src/pages/game.module.scss");
    let get_page_data = Resource::new(|| (), |_| get_game_data());

    view! {
        <div class=css::game>
            <Suspense>
                {Suspend::new(async move {
                    match get_page_data.await {
                        Ok((banner, hot, recommend, rest)) => {
                            Either::Right(
                                view! {
                                    <Banner imgs=banner />
                                    <HotGame games=hot />
                                    <RecommendGame info=recommend />
                                    <RestGame info=rest/>
                                },
                            )
                        }
                        Err(err) => Either::Left(err.to_string()),
                    }
                })}
            </Suspense>
        </div>
    }
}
