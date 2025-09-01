use leptos::prelude::*;

use crate::components::{Carousel, SlideItem};
use crate::pages::home_page::PlayBill;

#[component]
pub fn HotNews() -> impl IntoView {
    stylance::import_crate_style!(css, "src/pages/home_page/hot_news.module.scss");

    view! {
        <div class=css::hot_news>
            <div class=css::carouse_box>
                <Carousel items=vec![
                    SlideItem {
                        img_url: "/imgs/room/pic_1750388549.jpg".to_string(),
                        link: Some("#".to_string()),
                        title: None,
                    },
                    SlideItem {
                        img_url: "/imgs/room/pic_1750413784.jpg".to_string(),
                        link: None,
                        title: None,
                    },
                    SlideItem {
                        img_url: "/imgs/room/pic_1750388850.png".to_string(),
                        link: None,
                        title: None,
                    },
                    SlideItem {
                        img_url: "/imgs/room/pic_1750387905.jpg".to_string(),
                        link: None,
                        title: None,
                    },
                    SlideItem {
                        img_url: "/imgs/room/pic_1750327374.jpg".to_string(),
                        link: None,
                        title: None,
                    },
                    SlideItem {
                        img_url: "/imgs/room/pic_1750322434.jpg".to_string(),
                        link: None,
                        title: None,
                    },
                ] />
            </div>
            <div class=css::carousel_slide_clsx>
                <img src="/imgs/room/pic_1750043870.jpg" alt="" />
                <img src="/imgs/room/pic_1750328134.jpg" alt="" />
                <img src="/imgs/room/pic_1750410122.jpg" alt="" />
            </div>
            <div class=css::news_box>
                <h1 class=css::title>
                    <a>06.16-06.22一周精彩赛事推荐</a>
                </h1>
                <ul class=css::news_clsx>
                    <li>
                        <a href="">DNFM雷龙来袭</a>
                        <span>06/20</span>
                    </li>
                    <li>
                        <a href="">"四巨头G币限时抢夺战开启！"</a>
                        <span>06/20</span>
                    </li>
                    <li>
                        <a href="">真男人挑战-AJ药酱cfm铠甲局</a>
                        <span>06/20</span>
                    </li>
                    <li>
                        <a href="">
                            {r#""拳霸天下"拳头系品类营收活动-第五期奖励公示"#}
                        </a>
                        <span>06/20</span>
                    </li>
                    <li>
                        <a href="">传奇游戏榜单政策</a>
                        <span>06/20</span>
                    </li>
                    <li>
                        <a href="">全村荣耀村运会x新华社成都站决赛</a>
                        <span>06/20</span>
                    </li>
                    <li>
                        <a href="">虎牙HYPER电竞嘉年华燃动蓉城</a>
                        <span>06/20</span>
                    </li>
                </ul>
                <p class=css::news_tag_clsx>
                    <a>广告治理公告</a>
                    <a>车队和坐骑商城下线</a>
                    <a>短剧自动解锁</a>
                    <a>暂停银豆充值公告</a>
                    <a>政策FAQ</a>
                    <a>奖励房间号使用规范</a>
                </p>
                <a class=css::more>
                    <img src="/imgs/news-more.png" alt="" />
                </a>
            </div>
            <div class=css::bill_board>
                <h1>
                    <img src="/imgs/news-tv.png" alt="" />
                    节目预告
                </h1>
                <PlayBill />
            </div>
        </div>
    }
}
