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
            cover_url: "https://huyaimg.msstatic.com/cdnimage/upcoming/up_17484911694513_pic.jpg",
            time: 1750819375,
        },
        PlayBill {
            is_live: false,
            name: "绝地求生G-COIN超低折扣限时抢购",
            cover_url: "https://huyaimg.msstatic.com/cdnimage/upcoming/up_17496066287645_pic.jpg",
            time: 1750919375,
        },
        PlayBill {
            is_live: true,
            cover_url: "https://huyaimg.msstatic.com/cdnimage/upcoming/up_17491067039260_pic.jpg",
            name: "KPL夏季赛",
            time: 1750922375,
        },
        PlayBill {
            is_live: false,
            name: "英雄联盟峡谷特训赛",
            cover_url: "https://huyaimg.msstatic.com/cdnimage/upcoming/up_17500402787236_pic.jpg",
            time: 1750922375,
        },
        PlayBill {
            is_live: false,
            name: "【CF手游】清凉一夏好货节",
            cover_url: "https://huyaimg.msstatic.com/cdnimage/upcoming/up_17503267926570_pic.jpg",
            time: 1750927375,
        },
        PlayBill {
            is_live: true,
            name: "【2025CFPL夏季赛",
            cover_url: "https://huyaimg.msstatic.com/cdnimage/upcoming/up_17476220616490_pic.jpg",
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
                                                                (0, 0) => {
                                                                    "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAkAAAAVCAIAAAAb51OnAAAAGXRFWHRTb2Z0d2FyZQBBZG9iZSBJbWFnZVJlYWR5ccllPAAAA25pVFh0WE1MOmNvbS5hZG9iZS54bXAAAAAAADw/eHBhY2tldCBiZWdpbj0i77u/IiBpZD0iVzVNME1wQ2VoaUh6cmVTek5UY3prYzlkIj8+IDx4OnhtcG1ldGEgeG1sbnM6eD0iYWRvYmU6bnM6bWV0YS8iIHg6eG1wdGs9IkFkb2JlIFhNUCBDb3JlIDUuNS1jMDIxIDc5LjE1NTc3MiwgMjAxNC8wMS8xMy0xOTo0NDowMCAgICAgICAgIj4gPHJkZjpSREYgeG1sbnM6cmRmPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5LzAyLzIyLXJkZi1zeW50YXgtbnMjIj4gPHJkZjpEZXNjcmlwdGlvbiByZGY6YWJvdXQ9IiIgeG1sbnM6eG1wTU09Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC9tbS8iIHhtbG5zOnN0UmVmPSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvc1R5cGUvUmVzb3VyY2VSZWYjIiB4bWxuczp4bXA9Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC8iIHhtcE1NOk9yaWdpbmFsRG9jdW1lbnRJRD0ieG1wLmRpZDpkNmNhZGQ1OC02YmQ0LWEyNGEtYmE3ZC1lMDJlZWYzZjNhYjkiIHhtcE1NOkRvY3VtZW50SUQ9InhtcC5kaWQ6NDcwNDNGOTQ3MEJDMTFFNkFCRDVEOTkyOEUyNjI2MTYiIHhtcE1NOkluc3RhbmNlSUQ9InhtcC5paWQ6NDcwNDNGOTM3MEJDMTFFNkFCRDVEOTkyOEUyNjI2MTYiIHhtcDpDcmVhdG9yVG9vbD0iQWRvYmUgUGhvdG9zaG9wIENDIDIwMTUgKFdpbmRvd3MpIj4gPHhtcE1NOkRlcml2ZWRGcm9tIHN0UmVmOmluc3RhbmNlSUQ9InhtcC5paWQ6MkU1MEQ4RDg3MEJBMTFFNkExRjE4OEU1OTdFQzczMjAiIHN0UmVmOmRvY3VtZW50SUQ9InhtcC5kaWQ6MkU1MEQ4RDk3MEJBMTFFNkExRjE4OEU1OTdFQzczMjAiLz4gPC9yZGY6RGVzY3JpcHRpb24+IDwvcmRmOlJERj4gPC94OnhtcG1ldGE+IDw/eHBhY2tldCBlbmQ9InIiPz6VZ9+/AAAAoElEQVR42mL88/s3Aw7AxIAbjMrBOFtrsMsx3tjJuKcTSGLIfXvPtDYPxAeS3z9AVf/bWMl4fA7Dl9coVvGI/rdIZvpnFs/w5xe6M/78+meewMQgovwvYxsDEzOSG5hBIiLKIPv+y5mgafsvZwpzy6cXDP/+MjAy/Ve0BJIg9qfnYJN///5778S/RfF/nt8EsoEkkP333kkgmxFPegEIMACeLEfV3LDkxAAAAABJRU5ErkJggg=="
                                                                }
                                                                (0, _) => {
                                                                    "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAkAAAAVCAIAAAAb51OnAAAAGXRFWHRTb2Z0d2FyZQBBZG9iZSBJbWFnZVJlYWR5ccllPAAAA25pVFh0WE1MOmNvbS5hZG9iZS54bXAAAAAAADw/eHBhY2tldCBiZWdpbj0i77u/IiBpZD0iVzVNME1wQ2VoaUh6cmVTek5UY3prYzlkIj8+IDx4OnhtcG1ldGEgeG1sbnM6eD0iYWRvYmU6bnM6bWV0YS8iIHg6eG1wdGs9IkFkb2JlIFhNUCBDb3JlIDUuNS1jMDIxIDc5LjE1NTc3MiwgMjAxNC8wMS8xMy0xOTo0NDowMCAgICAgICAgIj4gPHJkZjpSREYgeG1sbnM6cmRmPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5LzAyLzIyLXJkZi1zeW50YXgtbnMjIj4gPHJkZjpEZXNjcmlwdGlvbiByZGY6YWJvdXQ9IiIgeG1sbnM6eG1wTU09Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC9tbS8iIHhtbG5zOnN0UmVmPSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvc1R5cGUvUmVzb3VyY2VSZWYjIiB4bWxuczp4bXA9Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC8iIHhtcE1NOk9yaWdpbmFsRG9jdW1lbnRJRD0ieG1wLmRpZDpkNmNhZGQ1OC02YmQ0LWEyNGEtYmE3ZC1lMDJlZWYzZjNhYjkiIHhtcE1NOkRvY3VtZW50SUQ9InhtcC5kaWQ6MzNERkMyQjM3MEJDMTFFNkJDRkI4MTIxMUMyQjBEQTIiIHhtcE1NOkluc3RhbmNlSUQ9InhtcC5paWQ6MzNERkMyQjI3MEJDMTFFNkJDRkI4MTIxMUMyQjBEQTIiIHhtcDpDcmVhdG9yVG9vbD0iQWRvYmUgUGhvdG9zaG9wIENDIDIwMTUgKFdpbmRvd3MpIj4gPHhtcE1NOkRlcml2ZWRGcm9tIHN0UmVmOmluc3RhbmNlSUQ9InhtcC5paWQ6MkU1MEQ4RDg3MEJBMTFFNkExRjE4OEU1OTdFQzczMjAiIHN0UmVmOmRvY3VtZW50SUQ9InhtcC5kaWQ6MkU1MEQ4RDk3MEJBMTFFNkExRjE4OEU1OTdFQzczMjAiLz4gPC9yZGY6RGVzY3JpcHRpb24+IDwvcmRmOlJERj4gPC94OnhtcG1ldGE+IDw/eHBhY2tldCBlbmQ9InIiPz7jOB5zAAAAgklEQVR42mL88/s3Aw7AxIAbjMoRIce0tRa7HOONnYx7OoAkQgQYD4wn5jPeOwIS/fySgVf8v4b7fyWb/xaJYH2CMoznVoAkgODzSxBbQBpq5n9113/uNXCjgOz/Gm5I9v3/D7adGdktLDC5v/+CJvw3jmQ8u5zh2zuEW8jxO0CAAQDTcjG49xiWegAAAABJRU5ErkJggg=="
                                                                }
                                                                (id, current) if id == current => {
                                                                    "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAkAAAAJCAIAAABv85FHAAAAGXRFWHRTb2Z0d2FyZQBBZG9iZSBJbWFnZVJlYWR5ccllPAAAAyZpVFh0WE1MOmNvbS5hZG9iZS54bXAAAAAAADw/eHBhY2tldCBiZWdpbj0i77u/IiBpZD0iVzVNME1wQ2VoaUh6cmVTek5UY3prYzlkIj8+IDx4OnhtcG1ldGEgeG1sbnM6eD0iYWRvYmU6bnM6bWV0YS8iIHg6eG1wdGs9IkFkb2JlIFhNUCBDb3JlIDUuNS1jMDIxIDc5LjE1NTc3MiwgMjAxNC8wMS8xMy0xOTo0NDowMCAgICAgICAgIj4gPHJkZjpSREYgeG1sbnM6cmRmPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5LzAyLzIyLXJkZi1zeW50YXgtbnMjIj4gPHJkZjpEZXNjcmlwdGlvbiByZGY6YWJvdXQ9IiIgeG1sbnM6eG1wPSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvIiB4bWxuczp4bXBNTT0iaHR0cDovL25zLmFkb2JlLmNvbS94YXAvMS4wL21tLyIgeG1sbnM6c3RSZWY9Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC9zVHlwZS9SZXNvdXJjZVJlZiMiIHhtcDpDcmVhdG9yVG9vbD0iQWRvYmUgUGhvdG9zaG9wIENDIDIwMTQgKFdpbmRvd3MpIiB4bXBNTTpJbnN0YW5jZUlEPSJ4bXAuaWlkOjM2Q0E5QkZDNjlERjExRTZCMTczOEQ5RjVCREJDMzZEIiB4bXBNTTpEb2N1bWVudElEPSJ4bXAuZGlkOjM2Q0E5QkZENjlERjExRTZCMTczOEQ5RjVCREJDMzZEIj4gPHhtcE1NOkRlcml2ZWRGcm9tIHN0UmVmOmluc3RhbmNlSUQ9InhtcC5paWQ6MzZDQTlCRkE2OURGMTFFNkIxNzM4RDlGNUJEQkMzNkQiIHN0UmVmOmRvY3VtZW50SUQ9InhtcC5kaWQ6MzZDQTlCRkI2OURGMTFFNkIxNzM4RDlGNUJEQkMzNkQiLz4gPC9yZGY6RGVzY3JpcHRpb24+IDwvcmRmOlJERj4gPC94OnhtcG1ldGE+IDw/eHBhY2tldCBlbmQ9InIiPz6DO+49AAAAd0lEQVR42mL8//8/Aw7AAqH+X9v//9AChndPGIRkGO0SGLUcgYKMQH3/rx/4v6YOWQdjcCNQmgmk6eB8NNNAZjAwgORARqEBsAhYTkgGXQ4sApIDWo4mxWgbD3UnyFVAFx2az/DuKYOQNKNdIqO2E9SduPwHEGAA9a8o3zdPdkgAAAAASUVORK5CYII="
                                                                }
                                                                _ => {
                                                                    "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAkAAAAJCAIAAABv85FHAAAAGXRFWHRTb2Z0d2FyZQBBZG9iZSBJbWFnZVJlYWR5ccllPAAAAyZpVFh0WE1MOmNvbS5hZG9iZS54bXAAAAAAADw/eHBhY2tldCBiZWdpbj0i77u/IiBpZD0iVzVNME1wQ2VoaUh6cmVTek5UY3prYzlkIj8+IDx4OnhtcG1ldGEgeG1sbnM6eD0iYWRvYmU6bnM6bWV0YS8iIHg6eG1wdGs9IkFkb2JlIFhNUCBDb3JlIDUuNS1jMDIxIDc5LjE1NTc3MiwgMjAxNC8wMS8xMy0xOTo0NDowMCAgICAgICAgIj4gPHJkZjpSREYgeG1sbnM6cmRmPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5LzAyLzIyLXJkZi1zeW50YXgtbnMjIj4gPHJkZjpEZXNjcmlwdGlvbiByZGY6YWJvdXQ9IiIgeG1sbnM6eG1wPSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvIiB4bWxuczp4bXBNTT0iaHR0cDovL25zLmFkb2JlLmNvbS94YXAvMS4wL21tLyIgeG1sbnM6c3RSZWY9Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC9zVHlwZS9SZXNvdXJjZVJlZiMiIHhtcDpDcmVhdG9yVG9vbD0iQWRvYmUgUGhvdG9zaG9wIENDIDIwMTQgKFdpbmRvd3MpIiB4bXBNTTpJbnN0YW5jZUlEPSJ4bXAuaWlkOkU4OEEwRTJGNjlERDExRTY4OURBRDZCMDdBREI3REQzIiB4bXBNTTpEb2N1bWVudElEPSJ4bXAuZGlkOkU4OEEwRTMwNjlERDExRTY4OURBRDZCMDdBREI3REQzIj4gPHhtcE1NOkRlcml2ZWRGcm9tIHN0UmVmOmluc3RhbmNlSUQ9InhtcC5paWQ6RTg4QTBFMkQ2OUREMTFFNjg5REFENkIwN0FEQjdERDMiIHN0UmVmOmRvY3VtZW50SUQ9InhtcC5kaWQ6RTg4QTBFMkU2OUREMTFFNjg5REFENkIwN0FEQjdERDMiLz4gPC9yZGY6RGVzY3JpcHRpb24+IDwvcmRmOlJERj4gPC94OnhtcG1ldGE+IDw/eHBhY2tldCBlbmQ9InIiPz7QzKPmAAAAXklEQVR42pxPMQ4AIQhTcnEyvEH//yh5g2FiAEk0XG5wuQ6loWkJ2czSBZDuePYQESJi5lpra62U8ubcmHOqqvMY49PpiagKfTyvCi/08fwGIgKAc+99L/PPH5YAAwCplSQNkeconQAAAABJRU5ErkJggg=="
                                                                }
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
                                                                        <i class="inline-block size-6 bg-[url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAYAAADgdz34AAAAGXRFWHRTb2Z0d2FyZQBBZG9iZSBJbWFnZVJlYWR5ccllPAAAAyZpVFh0WE1MOmNvbS5hZG9iZS54bXAAAAAAADw/eHBhY2tldCBiZWdpbj0i77u/IiBpZD0iVzVNME1wQ2VoaUh6cmVTek5UY3prYzlkIj8+IDx4OnhtcG1ldGEgeG1sbnM6eD0iYWRvYmU6bnM6bWV0YS8iIHg6eG1wdGs9IkFkb2JlIFhNUCBDb3JlIDUuNS1jMDIxIDc5LjE1NTc3MiwgMjAxNC8wMS8xMy0xOTo0NDowMCAgICAgICAgIj4gPHJkZjpSREYgeG1sbnM6cmRmPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5LzAyLzIyLXJkZi1zeW50YXgtbnMjIj4gPHJkZjpEZXNjcmlwdGlvbiByZGY6YWJvdXQ9IiIgeG1sbnM6eG1wTU09Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC9tbS8iIHhtbG5zOnN0UmVmPSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvc1R5cGUvUmVzb3VyY2VSZWYjIiB4bWxuczp4bXA9Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC8iIHhtcE1NOkRvY3VtZW50SUQ9InhtcC5kaWQ6OEEyNDlBMzU2QjNEMTFFNjg1NERBNDYwRTU2QzFFMkEiIHhtcE1NOkluc3RhbmNlSUQ9InhtcC5paWQ6OEEyNDlBMzQ2QjNEMTFFNjg1NERBNDYwRTU2QzFFMkEiIHhtcDpDcmVhdG9yVG9vbD0iQWRvYmUgUGhvdG9zaG9wIENDIDIwMTQgKFdpbmRvd3MpIj4gPHhtcE1NOkRlcml2ZWRGcm9tIHN0UmVmOmluc3RhbmNlSUQ9InhtcC5paWQ6NkJDQUREODU2QjNDMTFFNjhGNEY4QzlEMjE3MzU1QzEiIHN0UmVmOmRvY3VtZW50SUQ9InhtcC5kaWQ6NkJDQUREODY2QjNDMTFFNjhGNEY4QzlEMjE3MzU1QzEiLz4gPC9yZGY6RGVzY3JpcHRpb24+IDwvcmRmOlJERj4gPC94OnhtcG1ldGE+IDw/eHBhY2tldCBlbmQ9InIiPz4QEXiqAAAA1klEQVR42mL8//8/Ay0BEwONwfCxQBWI1wLxRyD+TyT+CNWjitcGYCSrA/H7/+SD91AzGLBhELEWqnAZEIvhUogFi0H1/IeagSwXAcRPQfIgzkeoIlIMR7YEBD5D+XxAvBDJd/8YkDgMZGIYMAfiO1D2F2RDqWXBbyi9E4iVaGEBCMxCF2ehYpL3BOIdQzonbwfiWdgyGi0iWZEWkYycTD8jG0rtjLYIPaPRoqiIBOIXQLwUxNGgQmGnga+wg5WoIFd8IsHgT1A96vh8yjhaJw+4BQABBgBoD+h5Gg9WVAAAAABJRU5ErkJggg==)]" />
                                                                        看直播
                                                                    </div>
                                                                },
                                                            )
                                                        } else {
                                                            Either::Left(
                                                                view! {
                                                                    <div class="flex flex-col flex-auto gap-y-2 justify-center items-center bg-sky-400 w-[63px]">
                                                                        <i class="inline-block size-6 bg-[url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAYAAADgdz34AAAAGXRFWHRTb2Z0d2FyZQBBZG9iZSBJbWFnZVJlYWR5ccllPAAAAyZpVFh0WE1MOmNvbS5hZG9iZS54bXAAAAAAADw/eHBhY2tldCBiZWdpbj0i77u/IiBpZD0iVzVNME1wQ2VoaUh6cmVTek5UY3prYzlkIj8+IDx4OnhtcG1ldGEgeG1sbnM6eD0iYWRvYmU6bnM6bWV0YS8iIHg6eG1wdGs9IkFkb2JlIFhNUCBDb3JlIDUuNS1jMDIxIDc5LjE1NTc3MiwgMjAxNC8wMS8xMy0xOTo0NDowMCAgICAgICAgIj4gPHJkZjpSREYgeG1sbnM6cmRmPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5LzAyLzIyLXJkZi1zeW50YXgtbnMjIj4gPHJkZjpEZXNjcmlwdGlvbiByZGY6YWJvdXQ9IiIgeG1sbnM6eG1wPSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvIiB4bWxuczp4bXBNTT0iaHR0cDovL25zLmFkb2JlLmNvbS94YXAvMS4wL21tLyIgeG1sbnM6c3RSZWY9Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC9zVHlwZS9SZXNvdXJjZVJlZiMiIHhtcDpDcmVhdG9yVG9vbD0iQWRvYmUgUGhvdG9zaG9wIENDIDIwMTQgKFdpbmRvd3MpIiB4bXBNTTpJbnN0YW5jZUlEPSJ4bXAuaWlkOjI0NDVFOEVGNjlEMjExRTZCNkVBOUEwMEMwNjNFREVBIiB4bXBNTTpEb2N1bWVudElEPSJ4bXAuZGlkOjI0NDVFOEYwNjlEMjExRTZCNkVBOUEwMEMwNjNFREVBIj4gPHhtcE1NOkRlcml2ZWRGcm9tIHN0UmVmOmluc3RhbmNlSUQ9InhtcC5paWQ6MjQ0NUU4RUQ2OUQyMTFFNkI2RUE5QTAwQzA2M0VERUEiIHN0UmVmOmRvY3VtZW50SUQ9InhtcC5kaWQ6MjQ0NUU4RUU2OUQyMTFFNkI2RUE5QTAwQzA2M0VERUEiLz4gPC9yZGY6RGVzY3JpcHRpb24+IDwvcmRmOlJERj4gPC94OnhtcG1ldGE+IDw/eHBhY2tldCBlbmQ9InIiPz4nkSdLAAABsUlEQVR42qyWzUrDQBSFk4LVhVsFUdIsXIiCK5fd9AVEcGHsKyiIaPtE1dBUxSewQvUNdKsIdaOudNP603gunIE4dJKJzYWvaSc352Qmk3vrxnHspIQH6qAGVsEc+AJ98ASuQRs8GhXEYAweOAXfcXb8MNcbpzVOfBO88+IhaINd4IMymAUrHIuYI/HBsVSDA96RxAVYNswwieSc85oRODQZ7DBBlqVpELsBPcO5Jq8dUeuPgccpSjRS7laFk2KilstLGpzwRJSxHFkGQoc5oTKocN3lYS0VYLBILdGslLBTAyDHDvf3pPEMImoGJb5EEpdOcaG0ajKlPqddsdiSNkukll2i7+JjAKdpMAOGGXem6oqbkVem1mdJG7SNWKNnShSDN35fsBC+NYzrM1Jar2Jwzx/rFgZViulUtbwNHu/E4Io/tgrcRUqr67BK2r5oNsiLNqCmrwZDbqt2AQaRXir0Ync8gXjDVOyEIFGu9/8hvpco14FNwznL0XA6iYZzlLdltsA2N8MU8TnW0lpm3aYnq2cSJmaT1fRDU9N3c/xtWQPzHH/hC9plaX4wCfwKMABgZMkN5ocyPgAAAABJRU5ErkJggg==)]" />
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
