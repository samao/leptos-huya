use leptos::prelude::*;

use crate::app::components::{Carousel, SlideItem};
#[component]
pub fn HotNews() -> impl IntoView {
    view! {
        <div class="flex overflow-hidden justify-between *:rounded-md *:bg-white gxp-x-5 h-[217px] min-[1440px]:h-[288px]">
            <div class="w-[316px] aspect-316/217 min-[1440px]:w-[384px]">
                <Carousel items=vec![
                    SlideItem {
                        img_url: "https://huyaimg.msstatic.com/cdnimage/roompublicity/pic_1750388549.jpg"
                            .to_string(),
                        link: Some("#".to_string()),
                    },
                    SlideItem {
                        img_url: "https://huyaimg.msstatic.com/cdnimage/roompublicity/pic_1750413784.jpg"
                            .to_string(),
                        link: None,
                    },
                    SlideItem {
                        img_url: "https://huyaimg.msstatic.com/cdnimage/roompublicity/pic_1750388850.png"
                            .to_string(),
                        link: None,
                    },
                    SlideItem {
                        img_url: "https://huyaimg.msstatic.com/cdnimage/roompublicity/pic_1750387905.jpg"
                            .to_string(),
                        link: None,
                    },
                    SlideItem {
                        img_url: "https://huyaimg.msstatic.com/cdnimage/roompublicity/pic_1750327374.jpg"
                            .to_string(),
                        link: None,
                    },
                    SlideItem {
                        img_url: "https://huyaimg.msstatic.com/cdnimage/roompublicity/pic_1750322434.jpg"
                            .to_string(),
                        link: None,
                    },
                ] />
            </div>
            <div class="hidden overflow-hidden flex-col justify-between ml-2.5 space-y-2.5 w-[145px] min-[1440px]:flex bg-transparent! *:hover:opacity-80 *:h-22.5">
                <img
                    src="https://huyaimg.msstatic.com/cdnimage/roompublicity/pic_1750043870.jpg"
                    alt=""
                />
                <img
                    src="https://huyaimg.msstatic.com/cdnimage/roompublicity/pic_1750328134.jpg"
                    alt=""
                />
                <img
                    src="https://huyaimg.msstatic.com/cdnimage/roompublicity/pic_1750410122.jpg"
                    alt=""
                />
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
                            r#"“拳霸天下"拳头系品类营收活动-第五期奖励公示"#r
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
                    <img
                        src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAB8AAAAfCAYAAAAfrhY5AAAAGXRFWHRTb2Z0d2FyZQBBZG9iZSBJbWFnZVJlYWR5ccllPAAAAyZpVFh0WE1MOmNvbS5hZG9iZS54bXAAAAAAADw/eHBhY2tldCBiZWdpbj0i77u/IiBpZD0iVzVNME1wQ2VoaUh6cmVTek5UY3prYzlkIj8+IDx4OnhtcG1ldGEgeG1sbnM6eD0iYWRvYmU6bnM6bWV0YS8iIHg6eG1wdGs9IkFkb2JlIFhNUCBDb3JlIDUuNi1jMTM4IDc5LjE1OTgyNCwgMjAxNi8wOS8xNC0wMTowOTowMSAgICAgICAgIj4gPHJkZjpSREYgeG1sbnM6cmRmPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5LzAyLzIyLXJkZi1zeW50YXgtbnMjIj4gPHJkZjpEZXNjcmlwdGlvbiByZGY6YWJvdXQ9IiIgeG1sbnM6eG1wPSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvIiB4bWxuczp4bXBNTT0iaHR0cDovL25zLmFkb2JlLmNvbS94YXAvMS4wL21tLyIgeG1sbnM6c3RSZWY9Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC9zVHlwZS9SZXNvdXJjZVJlZiMiIHhtcDpDcmVhdG9yVG9vbD0iQWRvYmUgUGhvdG9zaG9wIENDIDIwMTcgKFdpbmRvd3MpIiB4bXBNTTpJbnN0YW5jZUlEPSJ4bXAuaWlkOjE2MzMzNEE3OUM4ODExRThBQjBCRjZDN0Q0NDlFNTg1IiB4bXBNTTpEb2N1bWVudElEPSJ4bXAuZGlkOjE2MzMzNEE4OUM4ODExRThBQjBCRjZDN0Q0NDlFNTg1Ij4gPHhtcE1NOkRlcml2ZWRGcm9tIHN0UmVmOmluc3RhbmNlSUQ9InhtcC5paWQ6MTYzMzM0QTU5Qzg4MTFFOEFCMEJGNkM3RDQ0OUU1ODUiIHN0UmVmOmRvY3VtZW50SUQ9InhtcC5kaWQ6MTYzMzM0QTY5Qzg4MTFFOEFCMEJGNkM3RDQ0OUU1ODUiLz4gPC9yZGY6RGVzY3JpcHRpb24+IDwvcmRmOlJERj4gPC94OnhtcG1ldGE+IDw/eHBhY2tldCBlbmQ9InIiPz4rAFxwAAAA90lEQVR42sSVzQ3CMAyFHStSxwH2YAK4s0bLFh0AxATswYEV2IATl+JAK0FJmp/axtJTJTvR++xEqekO0ABADbJxJ11JJ1JLerikIXMXGgBDXEhr0g37hDPfK5kvSWdShR9JTYAFaYejpCbABj3JeQDb7q14rDBQ0JhAhRNFcQCM1EUBbMKapv/WwTNOyR9NdueiE7AZa/0TGHc0dOzptLRzkQlgwR42ACzcxwJgZ+xNuwMCnbNMABmOrhgAmS5uEQAyvhnZAJzm2QDc5lkAEubJAFLmSQCS5lEAafNJAA3zIICWuRdA0/wHQNv8C8DCf+L1O34KMADoRTOOQ2zL2AAAAABJRU5ErkJggg=="
                        alt=""
                    />
                </a>
            </div>
            <div class="w-[290px]">2</div>
        </div>
    }
}
