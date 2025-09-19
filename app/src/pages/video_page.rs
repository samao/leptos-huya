use crate::components::{Carousel, SlideItem};
use leptos::{either::Either, prelude::*};
use leptos_meta::Title;
use models::{Banner, Site};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PageInfo {
    pub cates: Vec<models::VodSet>,
    pub recommends: Vec<models::FullVod>,
    pub sites: Vec<Site>,
    pub banner: Vec<Banner>,
}

#[server]
#[lazy]
async fn get_all_vod() -> Result<PageInfo, ServerFnError> {
    use database::{
        establish_connection,
        vod_page::{get_cate_vods, get_hot_vods},
    };
    let conn = &mut establish_connection();
    let cates = get_cate_vods(conn).map_err(|er| ServerFnError::new(er.to_string()))?;
    let recommends = get_hot_vods(conn).map_err(|er| ServerFnError::new(er.to_string()))?;

    Ok(PageInfo {
        cates,
        recommends,
        sites: vec![
            Site {
                title: "绝地求生".to_string(),
                list: vec![
                    "RASH悲喜",
                    "沫子",
                    "托马斯Czh",
                    "韦神",
                    "陈子豪",
                    "BB文",
                    "星魂",
                ]
                .iter()
                .map(|tag| tag.to_string())
                .collect(),
            },
            Site {
                title: "英雄联盟".to_string(),
                list: vec!["云顶之弈", "青蛙", "UZI", "毒纪", "官总", "卡尔"]
                    .iter()
                    .map(|tag| tag.to_string())
                    .collect(),
            },
            Site {
                title: "和平精英".to_string(),
                list: vec!["董系长", "鲨鱼", "烤羊腿", "LK-雨果", "轲南"]
                    .iter()
                    .map(|tag| tag.to_string())
                    .collect(),
            },
            Site {
                title: "王者荣耀".to_string(),
                list: vec!["耀扬", "孤影", "北慕", "吕德华", "树叶", "赖神", "口袋"]
                    .iter()
                    .map(|tag| tag.to_string())
                    .collect(),
            },
        ],
        banner: vec![
            Banner {
                title: "LOL心之钢单曲《PARANOIA》".to_string(),
                cover: "/imgs/vhuya/3510de14-6333-4f0b-bf16-31fba46452dc.jpg".to_string(),
            },
            Banner {
                title: "《时差5小时2》EP03".to_string(),
                cover: "/imgs/v_pic_l/975da0320bcd428ba3d903fa3ff907ce.jpg".to_string(),
            },
        ],
    })
}

fn align_hot(hot: i32) -> String {
    if hot > 10000 {
        format!("{:.2}万", (hot as f64) / 10000.0)
    } else {
        hot.to_string()
    }
}

#[component]
pub fn VideoPage() -> impl IntoView {
    stylance::import_crate_style!(css, "src/pages/video_page.module.scss");
    let get_all_data = Resource::new(|| (), |_| async move { get_all_vod().await });
    view! {
        <Title text="虎牙视频_好看的视频在线直播平台_电竞娱乐游戏视频免费观看" />
        <div class=css::video_page>
            <Suspense fallback=move || {
                "..."
            }>
                {Suspend::new(async move {
                    match get_all_data.await {
                        Ok(PageInfo { cates: list, recommends, sites, banner }) => {
                            Either::Right(
                                view! {
                                    <div class=css::recommends>
                                        <div class=css::carsoul>
                                            <Carousel items=banner
                                                .clone()
                                                .into_iter()
                                                .map(|item| SlideItem {
                                                    img_url: item.cover.clone(),
                                                    link: None,
                                                    title: Some(item.title.clone()),
                                                })
                                                .collect() />
                                        </div>
                                        <ul class=css::vods>
                                            <For
                                                each=move || recommends.clone().into_iter()
                                                key=|item| item.id
                                                let(item)
                                            >
                                                <li>
                                                    <img src=item.img_url />
                                                    <div>
                                                        <span>{item.title}</span>
                                                        <span>{item.user.name}</span>
                                                        <span>{align_hot(item.hots)}</span>
                                                    </div>
                                                </li>
                                            </For>
                                        </ul>
                                    </div>
                                    <div class=css::sites>
                                        <For
                                            each=move || sites.clone().into_iter()
                                            key=|item| item.title.clone()
                                            let(item)
                                        >
                                            <div class=css::site>
                                                <h2>{item.title}</h2>
                                                {item
                                                    .list
                                                    .iter()
                                                    .map(|tag| {
                                                        view! { <span>{tag.clone()}</span> }
                                                    })
                                                    .collect_view()}
                                            </div>
                                        </For>
                                    </div>
                                    <For
                                        each=move || list.clone().into_iter()
                                        key=|item| item.title.clone()
                                        let(cate)
                                    >
                                        <section class=css::cate>
                                            <div class=css::cate_th>
                                                <div class=css::left>
                                                    <h1>
                                                        <img src=cate.cover />
                                                        {cate.title.clone()}
                                                    </h1>
                                                    <ul>
                                                        <For
                                                            each=move || cate.tags.clone().into_iter()
                                                            key=|item| item.clone()
                                                            let(item)
                                                        >
                                                            <li>{item}</li>
                                                        </For>
                                                    </ul>
                                                    <div class=css::more>
                                                        <button>换一换</button>
                                                        <p>"更多 >"</p>
                                                    </div>
                                                </div>
                                                <div class=css::right>
                                                    {format!("{}视频排行榜", cate.title)}<p>"更多 >"</p>
                                                </div>
                                            </div>
                                            <div class=css::cate_tb>
                                                <ul class=css::left>
                                                    <For
                                                        each=move || cate.list.clone().into_iter()
                                                        key=|item| item.title.clone()
                                                        let(item)
                                                    >
                                                        <li>
                                                            <div class=css::cover>
                                                                <img src=item.img_url loading="lazy" />
                                                                <span>{item.duration}</span>
                                                            </div>
                                                            <div class=css::owner>
                                                                <span class=css::title>{item.title}</span>
                                                                <img src=item.owner.avatar />
                                                                <span class=css::name>{item.owner.name}</span>
                                                                <span class=css::hots>{align_hot(item.hots)}</span>
                                                            </div>
                                                        </li>
                                                    </For>
                                                </ul>
                                                <ul class=css::rank>
                                                    <For
                                                        each=move || cate.rank.clone().into_iter()
                                                        key=|item| item.title.clone()
                                                        let(item)
                                                    >
                                                        <li class=css::item>
                                                            <img src=item.img_url loading="lazy" />
                                                            <div class=css::rank_info>
                                                                <span>{item.title}</span>
                                                                <span>{item.owner.name}</span>
                                                            </div>
                                                        </li>
                                                    </For>
                                                </ul>
                                            </div>
                                        </section>
                                    </For>
                                },
                            )
                        }
                        Err(er) => Either::Left(er.to_string()),
                    }
                })}
            </Suspense>
        </div>
    }
}
