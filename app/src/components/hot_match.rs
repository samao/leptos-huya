use leptos::{either::Either, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct MatchData<T: ToString> {
    title: T,
    is_live: bool,
    home_team: Team<T>,
    guest_team: Team<T>,
    time: i64,
}

impl From<MatchData<&str>> for MatchData<String> {
    fn from(value: MatchData<&str>) -> MatchData<String> {
        MatchData {
            title: value.title.to_owned(),
            home_team: value.home_team.into(),
            guest_team: value.guest_team.into(),
            is_live: value.is_live,
            time: value.time,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Team<T: ToString> {
    name: T,
    score: u8,
}

impl From<Team<&str>> for Team<String> {
    fn from(value: Team<&str>) -> Team<String> {
        Team {
            name: value.name.to_owned(),
            score: value.score,
        }
    }
}

#[server]
async fn get_top_matchs() -> Result<Vec<MatchData<String>>, ServerFnError> {
    Ok(vec![
        MatchData {
            title: "无畏契约VCT联赛",
            is_live: true,
            time: 1750927375,
            home_team: Team {
                name: "JDG",
                score: 1,
            },
            guest_team: Team {
                name: "Te",
                score: 1,
            },
        },
        MatchData {
            title: "2025CFPL夏季联赛",
            is_live: true,
            time: 1750927375,
            home_team: Team {
                name: "eStar",
                score: 2,
            },
            guest_team: Team {
                name: "LGD",
                score: 0,
            },
        },
        MatchData {
            title: "KPL夏季赛",
            is_live: false,
            time: 1750927375,
            home_team: Team {
                name: "西安WE",
                score: 0,
            },
            guest_team: Team {
                name: "长沙TES.A",
                score: 0,
            },
        },
    ]
    .into_iter()
    .map(|md| md.into())
    .collect())
}

#[component]
pub fn HotMatch() -> impl IntoView {
    let get_matchs = LocalResource::new(|| get_top_matchs());
    stylance::import_crate_style!(css, "src/components/hot_match.module.scss");
    view! {
        <div class=css::hot_match>
            <Suspense fallback=|| {
                "..."
            }>
                {move || match get_matchs.get() {
                    Some(Ok(matchs)) => {
                        Either::Right(
                            view! {
                                <For
                                    each=move || matchs.clone().into_iter()
                                    key=|item| item.title.clone()
                                    let(data)
                                >
                                    <div>
                                        <a class=css::link>
                                            <i class=format!(
                                                "{} {}",
                                                css::icon,
                                                if data.is_live { css::live } else { "" },
                                            ) />
                                            {data.title}
                                        </a>
                                        <p class=css::des>
                                            {if data.is_live {
                                                Either::Right(
                                                    view! {
                                                        {data.home_team.name}
                                                        <b>{data.home_team.score}</b>
                                                        :
                                                        <b>{data.guest_team.score}</b>
                                                        {data.guest_team.name}
                                                    },
                                                )
                                            } else {
                                                Either::Left(
                                                    view! {
                                                        {crate::to_time_str_format(data.time, "%d %H:%M ")}
                                                        {data.home_team.name}
                                                        <b>VS</b>
                                                        {data.guest_team.name}
                                                    },
                                                )
                                            }}
                                        </p>
                                    </div>
                                </For>
                            },
                        )
                    }
                    _ => Either::Left("###"),
                }}
            </Suspense>
            <a class=css::more>"更多精彩赛事>"</a>
        </div>
    }
}
