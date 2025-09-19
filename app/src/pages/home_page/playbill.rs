use leptos::{either::Either, prelude::*};
use serde::{Deserialize, Serialize};

stylance::import_crate_style!(css, "src/pages/home_page/playbill.module.scss");

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PlayBill<T: ToString> {
    is_live: bool,
    time: i64,
    name: T,
    cover_url: T,
}

impl From<PlayBill<&str>> for PlayBill<String> {
    fn from(value: PlayBill<&str>) -> Self {
        Self {
            name: value.name.to_string(),
            cover_url: value.cover_url.to_string(),
            is_live: value.is_live,
            time: value.time,
        }
    }
}

#[server]
#[lazy]
async fn get_playbills() -> Result<Vec<PlayBill<String>>, ServerFnError> {
    Ok([
        PlayBill {
            is_live: true,
            name: "英雄联盟Uzi名人堂皮肤",
            cover_url: "/imgs/upcoming/up_17484911694513_pic.jpg",
            time: 1750819375,
        },
        PlayBill {
            is_live: false,
            name: "绝地求生G-COIN超低折扣限时抢购",
            cover_url: "/imgs/upcoming/up_17496066287645_pic.jpg",
            time: 1750919375,
        },
        PlayBill {
            is_live: true,
            cover_url: "/imgs/upcoming/up_17491067039260_pic.jpg",
            name: "KPL夏季赛",
            time: 1750922375,
        },
        PlayBill {
            is_live: false,
            name: "英雄联盟峡谷特训赛",
            cover_url: "/imgs/upcoming/up_17500402787236_pic.jpg",
            time: 1750922375,
        },
        PlayBill {
            is_live: false,
            name: "【CF手游】清凉一夏好货节",
            cover_url: "/imgs/upcoming/up_17503267926570_pic.jpg",
            time: 1750927375,
        },
        PlayBill {
            is_live: true,
            name: "【2025CFPL夏季赛",
            cover_url: "/imgs/upcoming/up_17476220616490_pic.jpg",
            time: 1750927375,
        },
    ]
    .map(|pb| pb.into())
    .to_vec())
}

#[component]
pub fn PlayBill() -> impl IntoView {
    let play_bills = LocalResource::new(async || get_playbills().await);
    let bills = move || {
        play_bills
            .get()
            .unwrap_or(Err(ServerFnError::new("Some Error".to_string())))
    };

    let (active, set_active) = signal(0);

    view! {
        <div class=css::playbill>
            {move || {
                match bills() {
                    Ok(bills) => {
                        Either::Right(
                            view! {
                                <div class=css::bill_item>
                                    {bills
                                        .into_iter()
                                        .enumerate()
                                        .map(|(index, bill)| {
                                            view! {
                                                <details
                                                    style=move || {
                                                        format!(
                                                            "--bill-icon: url({})",
                                                            match (index, active.get()) {
                                                                (0, 0) => "/imgs/bill-n1-hover.png",
                                                                (0, _) => "/imgs/bill-n1.png",
                                                                (id, current) if id == current => {
                                                                    "/imgs/bill-icon-hover.png"
                                                                }
                                                                _ => "/imgs/bill-icon.png",
                                                            },
                                                        )
                                                    }
                                                    class=css::detail
                                                    on:mouseenter=move |_| {
                                                        set_active.set(index);
                                                    }
                                                    open=move || active.get() == index
                                                >
                                                    <summary class=css::item_clsx>
                                                        {if bill.is_live {
                                                            Either::Right(
                                                                view! { <span class=css::live>正在直播</span> },
                                                            )
                                                        } else {
                                                            Either::Left(
                                                                view! {
                                                                    <span class=css::descri>
                                                                        {crate::to_time_str(bill.time)}
                                                                    </span>
                                                                },
                                                            )
                                                        }} <p class=css::info>{bill.name}</p>
                                                    </summary>
                                                    <div class=css::details>
                                                        <img src=bill.cover_url alt="" />
                                                        {if bill.is_live {
                                                            Either::Right(
                                                                view! {
                                                                    <div class=format!(
                                                                        "{} {}",
                                                                        css::is_live_base_clsx,
                                                                        css::live,
                                                                    )>
                                                                        <i />
                                                                        看直播
                                                                    </div>
                                                                },
                                                            )
                                                        } else {
                                                            Either::Left(
                                                                view! {
                                                                    <div class=format!(
                                                                        "{} {}",
                                                                        css::is_live_base_clsx,
                                                                        css::descri,
                                                                    )>
                                                                        <i />
                                                                        订阅
                                                                    </div>
                                                                },
                                                            )
                                                        }}
                                                    </div>
                                                </details>
                                            }
                                        })
                                        .collect_view()}
                                </div>
                            },
                        )
                    }
                    _ => Either::Left("..."),
                }
            }}
        </div>
    }
}
