use chrono::{DateTime, Utc};
use leptos::{either::Either, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PlayBill<T: ToString> {
    is_live: bool,
    time: i64,
    name: T,
    cover_url: T,
}

async fn get_playbills() -> Result<Vec<PlayBill<String>>, ServerFnError> {
    Ok([].to_vec())
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
            .unwrap_or(vec![])
    };
    let bills = bills();

    let (active, set_active) = signal(0);

    if !bills.is_empty() {
        Either::Right(view! {
            <div class="border-l border-gray-400">
                {bills
                    .into_iter()
                    .enumerate()
                    .map(|(index, bill)| {
                        view! {
                            <details
                                on:mouseenter=move |_| {
                                    set_active.set(index);
                                }
                                open=move || active.get() == index
                            >
                                <summary class="block">
                                    {if bill.is_live {
                                        Either::Right(view! { <span>正在直播</span> })
                                    } else {
                                        Either::Left(
                                            view! { <span>{to_time_str(bill.time)}</span> },
                                        )
                                    }} {bill.name}
                                </summary>
                                <div class="rounded-md">
                                    <img src=bill.cover_url alt="" />
                                    {if bill.is_live {
                                        Either::Right(
                                            view! {
                                                <div>
                                                    <i class="bg-[url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAYAAADgdz34AAAAGXRFWHRTb2Z0d2FyZQBBZG9iZSBJbWFnZVJlYWR5ccllPAAAAyZpVFh0WE1MOmNvbS5hZG9iZS54bXAAAAAAADw/eHBhY2tldCBiZWdpbj0i77u/IiBpZD0iVzVNME1wQ2VoaUh6cmVTek5UY3prYzlkIj8+IDx4OnhtcG1ldGEgeG1sbnM6eD0iYWRvYmU6bnM6bWV0YS8iIHg6eG1wdGs9IkFkb2JlIFhNUCBDb3JlIDUuNS1jMDIxIDc5LjE1NTc3MiwgMjAxNC8wMS8xMy0xOTo0NDowMCAgICAgICAgIj4gPHJkZjpSREYgeG1sbnM6cmRmPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5LzAyLzIyLXJkZi1zeW50YXgtbnMjIj4gPHJkZjpEZXNjcmlwdGlvbiByZGY6YWJvdXQ9IiIgeG1sbnM6eG1wTU09Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC9tbS8iIHhtbG5zOnN0UmVmPSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvc1R5cGUvUmVzb3VyY2VSZWYjIiB4bWxuczp4bXA9Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC8iIHhtcE1NOkRvY3VtZW50SUQ9InhtcC5kaWQ6OEEyNDlBMzU2QjNEMTFFNjg1NERBNDYwRTU2QzFFMkEiIHhtcE1NOkluc3RhbmNlSUQ9InhtcC5paWQ6OEEyNDlBMzQ2QjNEMTFFNjg1NERBNDYwRTU2QzFFMkEiIHhtcDpDcmVhdG9yVG9vbD0iQWRvYmUgUGhvdG9zaG9wIENDIDIwMTQgKFdpbmRvd3MpIj4gPHhtcE1NOkRlcml2ZWRGcm9tIHN0UmVmOmluc3RhbmNlSUQ9InhtcC5paWQ6NkJDQUREODU2QjNDMTFFNjhGNEY4QzlEMjE3MzU1QzEiIHN0UmVmOmRvY3VtZW50SUQ9InhtcC5kaWQ6NkJDQUREODY2QjNDMTFFNjhGNEY4QzlEMjE3MzU1QzEiLz4gPC9yZGY6RGVzY3JpcHRpb24+IDwvcmRmOlJERj4gPC94OnhtcG1ldGE+IDw/eHBhY2tldCBlbmQ9InIiPz4QEXiqAAAA1klEQVR42mL8//8/Ay0BEwONwfCxQBWI1wLxRyD+TyT+CNWjitcGYCSrA/H7/+SD91AzGLBhELEWqnAZEIvhUogFi0H1/IeagSwXAcRPQfIgzkeoIlIMR7YEBD5D+XxAvBDJd/8YkDgMZGIYMAfiO1D2F2RDqWXBbyi9E4iVaGEBCMxCF2ehYpL3BOIdQzonbwfiWdgyGi0iWZEWkYycTD8jG0rtjLYIPaPRoqiIBOIXQLwUxNGgQmGnga+wg5WoIFd8IsHgT1A96vh8yjhaJw+4BQABBgBoD+h5Gg9WVAAAAABJRU5ErkJggg==)]" />
                                                    看直播
                                                </div>
                                            },
                                        )
                                    } else {
                                        Either::Left(
                                            view! {
                                                <div>
                                                    <i class="bg-[url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAYAAADgdz34AAAAGXRFWHRTb2Z0d2FyZQBBZG9iZSBJbWFnZVJlYWR5ccllPAAAAyZpVFh0WE1MOmNvbS5hZG9iZS54bXAAAAAAADw/eHBhY2tldCBiZWdpbj0i77u/IiBpZD0iVzVNME1wQ2VoaUh6cmVTek5UY3prYzlkIj8+IDx4OnhtcG1ldGEgeG1sbnM6eD0iYWRvYmU6bnM6bWV0YS8iIHg6eG1wdGs9IkFkb2JlIFhNUCBDb3JlIDUuNS1jMDIxIDc5LjE1NTc3MiwgMjAxNC8wMS8xMy0xOTo0NDowMCAgICAgICAgIj4gPHJkZjpSREYgeG1sbnM6cmRmPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5LzAyLzIyLXJkZi1zeW50YXgtbnMjIj4gPHJkZjpEZXNjcmlwdGlvbiByZGY6YWJvdXQ9IiIgeG1sbnM6eG1wPSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvIiB4bWxuczp4bXBNTT0iaHR0cDovL25zLmFkb2JlLmNvbS94YXAvMS4wL21tLyIgeG1sbnM6c3RSZWY9Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC9zVHlwZS9SZXNvdXJjZVJlZiMiIHhtcDpDcmVhdG9yVG9vbD0iQWRvYmUgUGhvdG9zaG9wIENDIDIwMTQgKFdpbmRvd3MpIiB4bXBNTTpJbnN0YW5jZUlEPSJ4bXAuaWlkOjI0NDVFOEVGNjlEMjExRTZCNkVBOUEwMEMwNjNFREVBIiB4bXBNTTpEb2N1bWVudElEPSJ4bXAuZGlkOjI0NDVFOEYwNjlEMjExRTZCNkVBOUEwMEMwNjNFREVBIj4gPHhtcE1NOkRlcml2ZWRGcm9tIHN0UmVmOmluc3RhbmNlSUQ9InhtcC5paWQ6MjQ0NUU4RUQ2OUQyMTFFNkI2RUE5QTAwQzA2M0VERUEiIHN0UmVmOmRvY3VtZW50SUQ9InhtcC5kaWQ6MjQ0NUU4RUU2OUQyMTFFNkI2RUE5QTAwQzA2M0VERUEiLz4gPC9yZGY6RGVzY3JpcHRpb24+IDwvcmRmOlJERj4gPC94OnhtcG1ldGE+IDw/eHBhY2tldCBlbmQ9InIiPz4nkSdLAAABsUlEQVR42qyWzUrDQBSFk4LVhVsFUdIsXIiCK5fd9AVEcGHsKyiIaPtE1dBUxSewQvUNdKsIdaOudNP603gunIE4dJKJzYWvaSc352Qmk3vrxnHspIQH6qAGVsEc+AJ98ASuQRs8GhXEYAweOAXfcXb8MNcbpzVOfBO88+IhaINd4IMymAUrHIuYI/HBsVSDA96RxAVYNswwieSc85oRODQZ7DBBlqVpELsBPcO5Jq8dUeuPgccpSjRS7laFk2KilstLGpzwRJSxHFkGQoc5oTKocN3lYS0VYLBILdGslLBTAyDHDvf3pPEMImoGJb5EEpdOcaG0ajKlPqddsdiSNkukll2i7+JjAKdpMAOGGXem6oqbkVem1mdJG7SNWKNnShSDN35fsBC+NYzrM1Jar2Jwzx/rFgZViulUtbwNHu/E4Io/tgrcRUqr67BK2r5oNsiLNqCmrwZDbqt2AQaRXir0Ync8gXjDVOyEIFGu9/8hvpco14FNwznL0XA6iYZzlLdltsA2N8MU8TnW0lpm3aYnq2cSJmaT1fRDU9N3c/xtWQPzHH/hC9plaX4wCfwKMABgZMkN5ocyPgAAAABJRU5ErkJggg==)]" />
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
        })
    } else {
        Either::Left("")
    }
}
