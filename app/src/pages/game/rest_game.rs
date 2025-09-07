use leptos::prelude::*;
stylance::import_crate_style!(css, "src/pages/game/rest_game.module.scss");

use serde::{Deserialize, Serialize};

use crate::pages::game::recommend_game::InCommingServer;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RestVo {
    pub news: Vec<String>,
    pub games: Vec<InCommingServer>,
}
#[component]
pub fn RestGame(#[prop(name = "info")] RestVo { news, games }: RestVo) -> impl IntoView {
    view! {
        <div class=css::game_recommend>
            <div class=css::left>
                <h1>新闻公告 <span>更多</span></h1>
                <ul class=css::list>
                    <For
                        each=move || news.clone().into_iter()
                        key=|title| title.clone()
                        let(msg)
                    >
                        <li>{msg}</li>
                    </For>
                <li class=css::help>
                    <div>在线客服</div>
                    <div>家长监控</div>
                </li>
                </ul>
            </div>
            <div class=css::right>
                <h1>其它游戏 <span>更多</span></h1>
                <ul class=css::list>
                    <For
                        each=move || games.clone().into_iter()
                        key=|game| game.id
                        let(game)
                    >
                        <li class=css::item>
                            <img src=format!("/imgs/game/{}/80_80.png", game.id) />
                            <span class=css::game_name>{game.game_name}</span>
                            <span>">>进入游戏<<"</span>
                        </li>
                    </For>
                </ul>
            </div>
        </div>
    }
}
