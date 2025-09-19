use crate::pages::game::hot_game::GameVo;
use leptos::{either::EitherOf3, prelude::*};
use serde::{Deserialize, Serialize};

stylance::import_crate_style!(css, "src/pages/game/recommend_game.module.scss");

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct InCommingServer {
    pub id: i32,
    pub time: String,
    pub server_name: String,
    pub game_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RecommendVo {
    pub servers: Vec<InCommingServer>,
    pub games: Vec<GameVo>,
}

#[component]
pub fn RecommendGame(
    #[prop(name = "info")] RecommendVo { servers, games }: RecommendVo,
) -> impl IntoView {
    view! {
        <div class=css::game_recommend>
            <div class=css::left>
                <h1>开服列表 <span>更多</span></h1>
                <ul class=css::list>
                    <For
                        each=move || servers.clone().into_iter()
                        key=|item| format!("{}_{}", item.server_name.clone(), item.game_name.clone())
                        let(game)
                    >
                        <li class=css::item>
                            <div class=css::summary>
                                <img src=format!("/imgs/game/{}/16_16.png", game.id) />
                                <span>{game.game_name.clone()}</span>
                                <span>{game.time.clone()}</span>
                                <span>{game.server_name.clone()}</span>
                            </div>
                            <div class=css::detail>
                                <img src=format!("/imgs/game/{}/80_80.png", game.id) />
                                <div class=css::group>
                                    <h2>{game.game_name.clone()}</h2>
                                    <span>{format!("{} {}", game.time.clone(), game.server_name.clone())}</span>
                                </div>
                                <button>进入</button>
                            </div>
                        </li>
                    </For>
                </ul>
            </div>
            <div class=css::right>
                <h1>推荐游戏 <span>更多</span></h1>
                <ul class=css::list>
                    <For
                        each=move || games.clone().into_iter()
                        key=|item| format!("{}_{}", item.title, item.cover_url)
                        let(game)
                    >
                        <li class=css::item>
                            <div class=css::cover>
                                <img src=game.cover_url loading="lazy" />
                                {match (game.is_new, game.is_hot) {
                                    (true, _) => {
                                        EitherOf3::A(
                                            view! {
                                                <i class=css::new/>
                                            }
                                        )
                                    }
                                    (_, true) => {
                                        EitherOf3::B (
                                            view! {
                                                <i class=css::hot/>
                                            }
                                        )
                                    }
                                    _ => {
                                        EitherOf3::C("")
                                    }
                                }}
                            </div>
                            <div class=css::info>
                                {game.title}
                                <div class=css::right_info>
                                    <a>官网</a>
                                    <a>开始游戏</a>
                                </div>
                            </div>
                        </li>
                    </For>
                </ul>
            </div>
        </div>
    }
}
