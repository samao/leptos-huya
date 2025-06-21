use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        use leptos_axum::ResponseOptions;
        let resp = expect_context::<ResponseOptions>();
        resp.set_status(axum::http::StatusCode::NOT_FOUND);
    }

    #[cfg(feature = "hydrate")]
    {
        use gloo_timers::future::TimeoutFuture;
        use leptos::{logging::log, task::spawn_local};
        use std::sync::{Arc, Mutex};

        let cancelled = Arc::new(Mutex::new(false));
        let clear_cancelled = Arc::clone(&cancelled);

        // 在路由切换时触发清理
        on_cleanup(move || {
            // 设置取消标志
            *clear_cancelled.lock().unwrap() = true;
            log!("任务已取消");
        });

        Effect::new(move |_| {
            // 创建任务的取消标志副本
            let task_cancelled = Arc::clone(&cancelled);

            spawn_local(async move {
                loop {
                    // 检查是否取消
                    if *task_cancelled.lock().unwrap() {
                        log!("任务已退出");
                        break;
                    }

                    // 执行循环任务
                    log!("执行任务...");

                    // 添加异步等待（避免阻塞）
                    // 使用 leptos 的延迟函数
                    TimeoutFuture::new(1000).await;
                }
            });

            on_cleanup(|| {});
        });
    }

    view! {
        <Title text="404" />
        <div class="flex flex-col justify-center items-center mx-auto h-[710px]">
            <img
                class="size-50"
                src="https://ssr-static.msstatic.com/h5/HuyaMainSSR/@public-online/imgs/error.2c65e354350f47124a3c7d2e4f20a9cf.png"
                alt="error"
            />
            <p title="UNKNOWN">页面走丢了</p>
        </div>
    }
}
