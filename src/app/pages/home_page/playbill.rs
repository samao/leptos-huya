use chrono::Utc;
use leptos::{either::Either, prelude::*};
use serde::{Deserialize, Serialize};

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

fn to_time_str(timestamp: i64) -> String {
    let d = chrono::DateTime::from_timestamp(timestamp, 0).unwrap_or(Utc::now());
    format!("{}", d.format("%d日 %H:%M"))
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
        <div class="flex-auto px-3 pr-0 text-left select-none bar-y-hidden">
            {move || {
                match bills() {
                    Ok(bills) => {
                        Either::Right(
                            view! {
                                <div class="border-l border-gray-300 w-[255px]">
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
                                                    class="mx-2.5"
                                                    on:mouseenter=move |_| {
                                                        set_active.set(index);
                                                    }
                                                    open=move || active.get() == index
                                                >
                                                    <summary class="flex relative items-center mb-2 before:-translate-x-1/2 before:-left-2.5 before:absolute before:size-[9px] before:bg-no-repeat before:bg-bottom before:bg-[image:var(--bill-icon)]">
                                                        {if bill.is_live {
                                                            Either::Right(
                                                                view! {
                                                                    <span class="px-1.5 mr-2 rounded-2xl border border-current text-[#f80] text-[12px]/[20px]">
                                                                        正在直播
                                                                    </span>
                                                                },
                                                            )
                                                        } else {
                                                            Either::Left(
                                                                view! {
                                                                    <span class="mr-2 text-sky-400 text-[12px]/[20px]">
                                                                        {to_time_str(bill.time)}
                                                                    </span>
                                                                },
                                                            )
                                                        }} <p class="w-2/3 truncate">{bill.name}</p>
                                                    </summary>
                                                    <div class="flex overflow-hidden mb-2 text-xs text-white rounded-md">
                                                        <img src=bill.cover_url alt="" class="w-[182px]" />
                                                        {if bill.is_live {
                                                            Either::Right(
                                                                view! {
                                                                    <div class="flex flex-col flex-auto gap-y-2 justify-center items-center bg-[#ff9600] w-[63px]">
                                                                        <i class="inline-block size-6 bg-[url(/imgs/bill-live.png)]" />
                                                                        看直播
                                                                    </div>
                                                                },
                                                            )
                                                        } else {
                                                            Either::Left(
                                                                view! {
                                                                    <div class="flex flex-col flex-auto gap-y-2 justify-center items-center bg-sky-400 w-[63px]">
                                                                        <i class="inline-block size-6 bg-[url(/imgs/bill-subscriber.png)]" />
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
