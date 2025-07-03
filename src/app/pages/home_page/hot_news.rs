use leptos::prelude::*;

use crate::app::components::{Carousel, SlideItem};
use crate::app::pages::home_page::PlayBill;

#[component]
pub fn HotNews() -> impl IntoView {
    view! {
        <div class="flex overflow-hidden justify-between *:rounded-md *:bg-white gxp-x-5 h-[217px] min-[1440px]:h-[288px]">
            <div class="w-[316px] aspect-316/217 min-[1440px]:w-[384px]">
                <Carousel items=vec![
                    SlideItem {
                        img_url: "/imgs/room/pic_1750388549.jpg".to_string(),
                        link: Some("#".to_string()),
                    },
                    SlideItem {
                        img_url: "/imgs/room/pic_1750413784.jpg".to_string(),
                        link: None,
                    },
                    SlideItem {
                        img_url: "/imgs/room/pic_1750388850.png".to_string(),
                        link: None,
                    },
                    SlideItem {
                        img_url: "/imgs/room/pic_1750387905.jpg".to_string(),
                        link: None,
                    },
                    SlideItem {
                        img_url: "/imgs/room/pic_1750327374.jpg".to_string(),
                        link: None,
                    },
                    SlideItem {
                        img_url: "/imgs/room/pic_1750322434.jpg".to_string(),
                        link: None,
                    },
                ] />
            </div>
            <div class="hidden overflow-hidden flex-col justify-between ml-2.5 space-y-2.5 w-[145px] min-[1440px]:flex bg-transparent! *:hover:opacity-80 *:h-22.5">
                <img src="/imgs/room/pic_1750043870.jpg" alt="" />
                <img src="/imgs/room/pic_1750328134.jpg" alt="" />
                <img src="/imgs/room/pic_1750410122.jpg" alt="" />
            </div>
            <div class="overflow-hidden relative flex-auto p-2.5 mx-5 text-left w-[342px] **:[a]:hover:text-[#ff9600]">
                <h1 class="font-bold text-center text-[#ff9600] text-xl/7.5">
                    <a>06.16-06.22一周精彩赛事推荐</a>
                </h1>
                <ul class="flex-auto *:[&li>span]:text-xs *:[li]:relative *:[&li>span]:text-[#9f9f9f] *:[&li>span]:absolute *:[&li>span]:right-0 *:[li]:justify-start *:[li]:items-center *:[li]:hidden *:nth-[-n+3]:flex min-[1440px]:*:[li]:nth-[-n+5]:flex *:[&li>a]:w-[233px] *:[&li>a]:truncate text-[14px]/[32px] *:[li]:gap-x-0 *:[li]:before:inline-flex *:[li]:before:mr-2 *:[li]:before:size-1.5 *:[li]:before:rounded-full *:[li]:before:bg-[#f80]">
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
                <p class="overflow-hidden mt-2.5 text-xs *:bg-[#f4f5f8] text-[#666] h-[72px] *:rounded-3xl leading-6.5 *:inline-block *:float-left *:mr-2.5 *:mb-2.5 *:px-3">
                    <a>广告治理公告</a>
                    <a>车队和坐骑商城下线</a>
                    <a>短剧自动解锁</a>
                    <a>暂停银豆充值公告</a>
                    <a>政策FAQ</a>
                    <a>奖励房间号使用规范</a>
                </p>
                <a class="absolute top-0 right-0">
                    <img src="/imgs/news-more.png" alt="" />
                </a>
            </div>
            <div class="flex flex-col gap-y-2 p-2.5 pr-0 w-[290px]">
                <h1 class="flex gap-2.5 items-center">
                    <img src="/imgs/news-tv.png" alt="" />
                    节目预告
                </h1>
                <PlayBill />
            </div>
        </div>
    }
}
