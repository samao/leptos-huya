use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
struct FriendLink {
    href: Option<String>,
    label: String,
    icon: Option<Icon>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
enum Icon {
    Icon(String),
    Hover(String, String),
}

impl From<&str> for Icon {
    fn from(value: &str) -> Self {
        Icon::Icon(value.to_owned())
    }
}

impl From<(&str, &str)> for Icon {
    fn from((normal, hover): (&str, &str)) -> Self {
        Icon::Hover(normal.to_owned(), hover.to_owned())
    }
}

cfg_block::cfg_block! {
    #[cfg(feature="ssr")] {
        async fn get_friend_links() -> Vec<FriendLink> {
            vec![
                FriendLink {
                    label: "腾讯游戏频道".into(),
                    ..Default::default()
                },
                FriendLink {
                    label: "快看漫画".into(),
                    ..Default::default()
                },
                FriendLink {
                    label: "爱拍".into(),
                    ..Default::default()
                },
                FriendLink {
                    label: "搜狐56视频".into(),
                    ..Default::default()
                },
                FriendLink {
                    label: "开放平台".into(),
                    ..Default::default()
                },
            ]
        }

        async fn get_anchor_help_links() -> Vec<FriendLink> {
            [
                "新人主播指引",
                "开播工具下载",
                "开播教程引导",
                "虎牙直播学院",
                "虎牙安全门户",
                "虎牙监察反舞弊举报",
            ]
            .into_iter()
            .map(|label| FriendLink {
                label: label.into(),
                ..Default::default()
            })
            .collect()
        }

        async fn get_download_links() -> Vec<FriendLink> {
            [
                (
                    "iPhone版",
                    "/imgs/footer/iphone.png",
                    "/imgs/footer/iphone-hover.png",
                ),
                (
                    "Andriod版",
                    "/imgs/footer/andriod.png",
                    "/imgs/footer/andriod-hover.png",
                ),
                ("TV版", "/imgs/footer/tv.png", "/imgs/footer/tv-hover.png"),
                (
                    "Win10版/WP版",
                    "/imgs/footer/win.png",
                    "/imgs/footer/win-hover.png",
                ),
                (
                    "iPad版",
                    "/imgs/footer/ipad.png",
                    "/imgs/footer/ipad-hover.png",
                ),
                (
                    "Andriod Pad版",
                    "/imgs/footer/apad.png",
                    "/imgs/footer/apad-hover.png",
                ),
                ("PC版", "/imgs/footer/pc.png", "/imgs/footer/pc-hover.png"),
            ]
            .into_iter()
            .map(|(label, normal, hover)| FriendLink {
                label: label.into(),
                icon: Some((normal, hover).into()),
                ..Default::default()
            })
            .collect()
        }
        async fn get_us_concat() -> Vec<FriendLink> {
            vec![
                FriendLink {
                    icon: Some(Icon::Hover(
                        "/imgs/footer/wb.png".into(),
                        "/imgs/footer/wb-hover.png".into(),
                    )),
                    label: "虎牙官方微博".into(),
                    ..Default::default()
                },
                FriendLink {
                    icon: Some(Icon::Hover(
                        "/imgs/footer/tieba.png".into(),
                        "/imgs/footer/tieba-hover.png".into(),
                    )),
                    label: "贴吧交流".into(),
                    ..Default::default()
                },
                FriendLink {
                    label: "不良信息投诉与举报".into(),
                    ..Default::default()
                },
            ]
        }
    }
}
#[server]
#[lazy]
async fn get_all() -> Result<
    (
        Vec<FriendLink>,
        Vec<FriendLink>,
        Vec<FriendLink>,
        Vec<FriendLink>,
    ),
    ServerFnError,
> {
    Ok(tokio::join!(
        get_friend_links(),
        get_anchor_help_links(),
        get_download_links(),
        get_us_concat(),
    ))
}

#[component]
pub fn Footer() -> impl IntoView {
    let all_data = Resource::new(|| (), move |_| get_all());

    stylance::import_crate_style!(css, "src/components/footer.module.scss");

    view! {
        <footer class=css::footer>
            <div class=css::inner>
                <Suspense fallback=|| {
                    "loading..."
                }>
                    {move || Suspend::new(async move {
                        let (friend_links, help_link, download_link, us_links) = all_data
                            .get()
                            .unwrap_or(Ok((vec![], vec![], vec![], vec![])))
                            .unwrap();

                        view! {
                            <div>
                                <h2>友情链接</h2>
                                <div class=css::links>
                                    <For
                                        each=move || friend_links.clone().into_iter()
                                        key=|link| link.label.clone()
                                        let(link)
                                    >
                                        <a>{link.label}</a>
                                    </For>
                                </div>
                            </div>
                            <div>
                                <h2>主播帮助</h2>
                                <div class=css::helps>
                                    <For
                                        each=move || help_link.clone().into_iter()
                                        key=|link| link.label.clone()
                                        let(link)
                                    >
                                        <a>{link.label}</a>
                                    </For>
                                </div>
                            </div>
                            <div>
                                <h2>虎牙产品下载</h2>
                                <div class=css::download>
                                    <div class=css::app_qrcode />
                                    <div class=css::platforms>
                                        <For
                                            each=move || download_link.clone().into_iter()
                                            key=|link| link.label.clone()
                                            let(link)
                                        >
                                            <a
                                                style=match link.icon {
                                                    None => "".to_string(),
                                                    Some(Icon::Icon(icon_url)) => {
                                                        format!("--icon-url:url({})", icon_url)
                                                    }
                                                    Some(Icon::Hover(icon_url, icon_hover_url)) => {
                                                        format!(
                                                            "--icon-url:url({}); --icon-hover-url:url({})",
                                                            icon_url,
                                                            icon_hover_url,
                                                        )
                                                    }
                                                }
                                                class=css::item
                                            >
                                                {link.label}
                                            </a>
                                        </For>
                                    </div>
                                </div>
                            </div>
                            <div>
                                <h2>关注我们</h2>
                                <div class=css::follows>
                                    <div class=css::qr_code />

                                    <div class=css::group>
                                        <For
                                            each=move || us_links.clone().into_iter()
                                            key=|link| link.label.clone()
                                            let(link)
                                        >
                                            <a
                                                style=match link.icon {
                                                    None => "".to_owned(),
                                                    Some(Icon::Icon(icon_url)) => {
                                                        format!("--icon-url:url({})", icon_url)
                                                    }
                                                    Some(Icon::Hover(icon_url, icon_hover_url)) => {
                                                        format!(
                                                            "--icon-url:url({}); --icon-hover-url:url({})",
                                                            icon_url,
                                                            icon_hover_url,
                                                        )
                                                    }
                                                }
                                                class=match link.icon {
                                                    Some(_) => css::link.to_string(),
                                                    None => format!("{} {}", css::link, css::hiden),
                                                }
                                            >
                                                {link.label}
                                            </a>
                                        </For>
                                    </div>
                                </div>
                            </div>
                        }
                    })}
                </Suspense>
            </div>
            <div class=css::bottom>
                <p>
                    <For
                        each=|| {
                            [
                                "关于虎牙",
                                "Investor Relations",
                                "加入我们",
                                "合作联系",
                                "友情链接",
                                "广告投放",
                                "联系客服",
                                "平台规则",
                                "隐私政策",
                                "版权保护投诉指引",
                                "广告管理",
                            ]
                                .into_iter()
                        }
                        key=|item| item.to_owned()
                        let(label)
                    >
                        <a class=css::link>{label}</a>
                    </For>
                </p>
                <p>
                    <For
                        each=|| {
                            [
                                "营业执照",
                                "广播电视节目制作经营许可证",
                                "营业性演出许可证",
                                "互联网药品信息服务资格证",
                                "粤ICP备16120983号",
                                "互联网宗教信息服务许可证",
                            ]
                                .into_iter()
                        }
                        key=|item| item.to_owned()
                        let(label)
                    >
                        <a class=css::link>{label}</a>
                    </For>
                </p>
                <p class=css::pargraph>
                    <a class=css::no_mouse>
                        增值电信业务经营许可证 粤B2-20170312 | B1-20181380
                    </a>
                    <a>粤网文[2023]0339-030号</a>
                    <a>
                        <img src="/imgs/footer/cnb2.png" class=css::icon alt="" />
                        粤公网安备44011302000433
                    </a>
                    <a>网信算备 440113902256602230019号</a>
                    <a>网信算备 440113902256601240017号</a>
                </p>
                <p class=css::tail>
                    <span>
                        "© 2012-现在 huya.com广州虎牙信息科技有限公司 版权所有All Rights Reserved"
                    </span>
                    <a>HUYA</a>
                    <a>全国文化和旅游市场网上举报投诉处理系统</a>
                    <a>中国互联网不良信息举报中心</a>
                    <span>
                        "违法和不良信息举报：020-22511620 未成年人关怀热线：020-22511510"
                    </span>
                </p>
                <img src="/imgs/footer/huya.png" class=css::end_logo />
            </div>
        </footer>
    }
}
