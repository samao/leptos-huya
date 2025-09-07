use leptos::prelude::*;

use serde::{Deserialize, Serialize};
stylance::import_crate_style!(css, "src/pages/game/hot_game.module.scss");

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GameVo {
    pub title: String,
    pub cover_url: String,
    pub is_new: bool,
    pub is_hot: bool,
    pub has_gift: bool,
    pub icon: Option<String>,
    pub backend_url: Option<String>,
    pub game_type: String,
}
#[component]
pub fn HotGame(games: Vec<GameVo>) -> impl IntoView {
    view! {
        <ul class=css::hot_game>
            <For each=move || games.clone().into_iter() key=|item| item.title.clone() let(game)>
                <li class=css::game>
                    <div class=css::cover>
                        <img src=game.cover_url loading="lazy" />
                        <div class=css::hover_cover>
                            <img src=game.backend_url loading="lazy" />
                            <span>{game.game_type}</span>
                            <span>[官网]</span>
                            <Show when=move || game.has_gift>
                                <span>[礼包]</span>
                            </Show>
                        </div>
                    </div>
                    <div class=css::info>{game.title} <button>进入新服</button></div>
                </li>
            </For>
        </ul>
    }
}
