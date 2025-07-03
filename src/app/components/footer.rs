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

#[component]
pub fn Footer() -> impl IntoView {
    let friend_links = Resource::new(|| (), move |_| get_friend_links());
    let help_link = Resource::new(|| (), move |_| get_anchor_help_links());
    let download_link = Resource::new(|| (), move |_| get_download_links());
    let us_links = Resource::new(|| (), move |_| get_us_concat());

    view! {
        <footer class="mt-10 w-full text-xs text-gray-400 bg-white **:[a]:hover:text-[#f80] **:[a]:cursor-pointer">
            <div class="flex justify-between py-10 mx-auto leading-5 text-left w-[980px] min-[1440px]:w-[1220px] **:[h2]:text-sm **:[h2]:font-bold **:[h2]:mb-2.5">
                <Suspense fallback=|| "loading...">
                    <div>
                        <h2>友情链接</h2>
                        <div class="grid grid-cols-1 gap-2 gap-x-8 min-[1440px]:grid-cols-2">
                            <For
                                each=move || friend_links.get().unwrap().into_iter()
                                key=|link| link.label.clone()
                                let(link)
                            >
                                <a>{link.label}</a>
                            </For>
                        </div>
                    </div>
                    <div>
                        <h2>主播帮助</h2>
                        <div class="flex flex-col space-y-2">
                            <For
                                each=move || help_link.get().unwrap().into_iter()
                                key=|link| link.label.clone()
                                let(link)
                            >
                                <a>{link.label}</a>
                            </For>
                        </div>
                    </div>
                    <div>
                        <h2>虎牙产品下载</h2>
                        <div class="flex gap-x-5">
                            <img
                                src="/imgs/footer/HuyaAppQrCode210909.png"
                                alt=""
                                class="flex flex-col space-x-2.5 size-32.5 after:content-['扫描下载虎牙APP']"
                            />
                            <div class="grid grid-cols-2 gap-2">
                                <For
                                    each=move || download_link.get().unwrap().into_iter()
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
                                        class="flex items-center before:size-6 before:mr-2 before:bg-[image:var(--icon-url)] hover:before:bg-[image:var(--icon-hover-url)]"
                                    >
                                        {link.label}
                                    </a>
                                </For>
                            </div>
                        </div>
                    </div>
                    <div>
                        <h2>关注我们</h2>
                        <div class="flex gap-x-6">
                            <img
                                src="/imgs/footer/huyawxgzh2.png"
                                class="flex flex-col space-x-2.5 size-24 after:content-['扫描关注微信公众号']"
                            />

                            <div class="flex flex-col gap-2">
                                <For
                                    each=move || us_links.get().unwrap().into_iter()
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
                                            Some(_) => {
                                                "flex items-center before:size-6 before:mr-2 before:bg-[image:var(--icon-url)] hover:before:bg-[image:var(--icon-hover-url)]"
                                            }
                                            None => {
                                                "flex items-center before:hidden before:size-6 before:mr-2 before:bg-[image:var(--icon-url)] hover:before:bg-[image:var(--icon-hover-url)]"
                                            }
                                        }
                                    >
                                        {link.label}
                                    </a>
                                </For>
                            </div>
                        </div>
                    </div>
                </Suspense>
            </div>
            <div class="flex flex-col justify-center items-center pt-10 pb-2.5 leading-5 border-0 border-t-1 *:[p]:mb-2.5 border-[#f3f3f3] **:[a]:not-first:hover:before:text-gray-400">
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
                        <a class="inline-flex items-center not-first:before:content-['|'] not-first:before:mx-1.5">
                            {label}
                        </a>
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
                        <a class="inline-flex items-center not-first:before:content-['|'] not-first:before:mx-1.5">
                            {label}
                        </a>
                    </For>
                </p>
                <p class="*:inline-flex *:items-center *:not-first:before:content-['|'] *:not-first:before:mx-1.5">
                    <a class="pointer-events-none">
                        增值电信业务经营许可证 粤B2-20170312 | B1-20181380
                    </a>
                    <a>粤网文[2023]0339-030号</a>
                    <a>
                        <img src="/imgs/footer/cnb2.png" class="inline-block size-5" alt="" />
                        粤公网安备44011302000433
                    </a>
                    <a>网信算备 440113902256602230019号</a>
                    <a>网信算备 440113902256601240017号</a>
                </p>
                <p class="*:[a]:px-1.5">
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
                <img src="/imgs/footer/huya.png" class="h-10.5" />
            </div>
        </footer>
    }
}
