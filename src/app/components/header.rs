use leptos::prelude::*;
use leptos_router::components::{Form, A};
use std::{collections::HashMap, sync::LazyLock};

static CATEGROY: LazyLock<HashMap<&str, Vec<&str>>> = LazyLock::new(|| {
    HashMap::from([
        (
            "热门分类",
            vec![
                "英雄联盟",
                "王者荣耀",
                "CS2",
                "户外",
                "体育",
                "一起看",
                "穿越火线",
                "天天吃鸡",
                "无畏契约",
                "星秀",
                "原始",
                "二次元",
            ],
        ),
        (
            "玩家推荐",
            vec![
                "三国杀",
                "欢乐斗地主",
                "JJ斗地主",
                "坦克世界",
                "我的世界",
                "魔兽世界",
                "守望先锋",
                "欢乐麻将",
                "问道",
                "梦三国",
                "天天狼人",
                "命运方舟",
            ],
        ),
    ])
});

#[component]
fn Ad() -> impl IntoView {
    let show_ad = RwSignal::new(true);

    view! {
        <Show when=move || show_ad.get() fallback=|| "">
            <div class="relative">
                <img
                    src="https://livewebbs2.msstatic.com/huya_1716264051_content.gif"
                    width="274"
                    height="65"
                />
                <span
                    class="size-5 bg-gray-300/50 text-black/20 absolute top-0 right-0"
                    on:click=move |_| show_ad.set(false)
                >
                    x
                </span>
            </div>
        </Show>
    }
}
#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="flex whitespace-nowrap w-[980px] min-[1440px]:w-[1220px] mx-auto h-[60px] text-[16px]/[32px] items-center justify-start text-white">
            <a
                href="/"
                class="inline-block flex-none mr-5 bg-[url(https://a.msstatic.com/huya/hd/h5/static-source/main/logo.png)] w-30 h-9 bg-size-[100%] bg-center object-contain"
            />
            <nav class="flex flex-auto gap-x-2 *:hover:bg-[#ff9600] *:rounded-2xl *:px-4 *:hover:[form]:bg-transparent *:[form]:relative *:[form]:mr-2 *:[form]:flex *:[form]:items-center *:[form]:px-0
            *:aria-[current]:bg-[#ff9600] **:[i]:inline-block *:flex *:items-center **:[i]:duration-200 *:gap-x-2 
            *:has-[i]:hover:*:[i]:rotate-180 *:relative *:has-[i]:hover:*:data-[active]:flex **:[i]:w-[9px] **:[i]:h-[5px] **:[i]:bg-[url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAkAAAAFCAYAAACXU8ZrAAAABGdBTUEAALGPC/xhBQAAACBjSFJNAAB6JgAAgIQAAPoAAACA6AAAdTAAAOpgAAA6mAAAF3CculE8AAAABmJLR0QAAAAAAAD5Q7t/AAAACXBIWXMAAAsSAAALEgHS3X78AAAALUlEQVQI12P8////fwYCgJGBgYEBn0JGRkZGRhgHm0KYPCOyILJCZAMwADYTAdkUE/0YEgvwAAAAAElFTkSuQmCC)]">
                <A href="">首页</A>
                <A href="l">直播</A>
                <A href="g">
                    分类 <i />
                    <div
                        on:click:target=move |ev| {
                            ev.prevent_default();
                        }
                        data-active
                        class="hidden before:h-2 before:w-full before:absolute before:-top-2 flex-col gap-y-2 text-[12px]/[20px] text-[#333] w-[304px] p-4 bg-white rounded-md absolute -translate-x-1/2 translate-y-2 z-10 left-1/2 top-full"
                    >
                        <For
                            each=|| CATEGROY.clone().into_iter()
                            key=|item| item.0
                            let((title, tags))
                        >
                            <h1 class="text-[14px] text-left font-bold">{title}</h1>
                            <div class="grid grid-cols-3 gap-2 *:rounded-xl *:border">
                                <For each=move || tags.clone().into_iter() key=|tag| *tag let(tag)>
                                    <span class="inline-block text-center border-[#e3e7e8] hover:text-white hover:bg-[#ff9600]">
                                        {tag}
                                    </span>
                                </For>
                            </div>
                        </For>
                        <div class="rounded-md bg-gray-300/80 text-center text-[12px]/[28px]  hover:text-white hover:bg-[#ff9600]">
                            更多 >
                        </div>

                        <Ad />
                    </div>
                </A>
                <a
                    href="m"
                    class="inline-block bg-[url(https://diy-assets.msstatic.com/header-match-icon/icon.png)] w-[74px] bg-center bg-contain bg-no-repeat hover:bg-transparent!"
                ></a>
                <A href="video">视频<i /></A>
                <A href="game">游戏<i /></A>

                <Form method="GET" action="/search">
                    <input
                        type="text"
                        name="hsk"
                        placeholder="寻寻觅觅"
                        class="min-[1440px]:w-[140px] w-[100px] text-xs/[32px] rounded-2xl bg-white/40 peer placeholder:text-[#555]/70 focus:bg-white/80 hover:bg-white/80 focus-visible:outline-none pl-4 pr-10 focus:text-[#555] focus:border-none "
                    />
                    <svg
                        class="absolute right-2 size-6 fill-white peer-focus:fill-red-500 peer-hover:fill-red-500"
                        viewBox="0 0 1024 1024"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <path d="M909.6 854.5L649.9 594.8C690.2 542.7 712 479 712 412c0-80.2-31.3-155.4-87.9-212.1-56.6-56.7-132-87.9-212.1-87.9s-155.5 31.3-212.1 87.9C143.2 256.5 112 331.8 112 412c0 80.1 31.3 155.5 87.9 212.1C256.5 680.8 331.8 712 412 712c67 0 130.6-21.8 182.7-62l259.7 259.6c3.2 3.2 8.4 3.2 11.6 0l43.6-43.5c3.2-3.2 3.2-8.4 0-11.6zM570.4 570.4C528 612.7 471.8 636 412 636s-116-23.3-158.4-65.6C211.3 528 188 471.8 188 412s23.3-116.1 65.6-158.4C296 211.3 352.2 188 412 188s116.1 23.2 158.4 65.6S636 352.2 636 412s-23.3 116.1-65.6 158.4z"></path>
                    </svg>
                    <input
                        type="submit"
                        value=""
                        class="absolute right-2 size-6 bg-red-400 opacity-0"
                    />
                </Form>
            </nav>
            <ul class="flex gap-x-6 text-xs/1.5 items-center *:flex *:flex-col *:justify-center *:before:size-[26px] *:gap-1.5 *:before:bg-cover *:before:bg-no-repeat *:bg-center">
                <li class="before:bg-[url(https://a.msstatic.com/huya/hd/h5/header/components/HeaderDynamic/NavItem/img/ls-2.9113b9d316856ca1d795c0e54079d940.png)]">
                    开播
                </li>
                <li class="before:bg-[url(https://a.msstatic.com/huya/hd/h5/header/components/HeaderDynamic/NavItem/img/download-2.c9310be282ee8f2196da396cf89c916b.png)]">
                    下载
                </li>
                <li class="before:bg-[url(https://a.msstatic.com/huya/hd/h5/header/components/HeaderDynamic/NavItem/img/history-2.6157ee9c44045989cf42ff47033a592f.png)]">
                    历史
                </li>
                <li class="before:bg-[url(https://a.msstatic.com/huya/hd/h5/header/components/HeaderDynamic/NavItem/img/cal-3.e73b55e606fac48daf82cf9982d6ef25.png)]">
                    任务
                </li>
                <li class="border-white/35 ring-2 rounded-full text-[14px]/[34px] size-[34px] before:hidden">
                    登录
                </li>
            </ul>
        </header>
    }
}
