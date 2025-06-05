use leptos::prelude::*;
use leptos_router::components::{Form, A};

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="flex whitespace-nowrap w-[980px] min-[1440px]:w-[1220px] mx-auto h-[60px] text-[16px]/[32px] items-center justify-start text-white">
            <a
                href="/"
                class="inline-block flex-none mr-5 bg-[url(https://a.msstatic.com/huya/hd/h5/static-source/main/logo.png)] w-30 h-9 bg-size-[100%] bg-center object-contain"
            />
            <nav class="flex flex-auto gap-x-2 *:hover:bg-[#ff9600] *:rounded-2xl *:px-4 *:hover:[form]:bg-transparent *:[form]:relative *:[form]:flex *:[form]:items-center *:[form]:px-0
            *:aria-[current]:bg-[#ff9600]">
                <A href="">首页</A>
                <A href="l">直播</A>
                <A href="g">分类</A>
                <a
                    href="m"
                    class="inline-block bg-[url(https://diy-assets.msstatic.com/header-match-icon/icon.png)] w-[74px] bg-center bg-contain bg-no-repeat hover:bg-transparent!"
                ></a>
                <A href="video">视频</A>
                <A href="game">游戏</A>

                <Form method="GET" action="/search">
                    <input
                        type="text"
                        name="hsk"
                        placeholder="寻寻觅觅"
                        class="w-[140px] text-xs/[32px] rounded-2xl bg-white/40 peer placeholder:text-[#555]/70 focus:bg-white/80 hover:bg-white/80 focus-visible:outline-none pl-4 pr-10 focus:text-[#555] focus:border-none "
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
