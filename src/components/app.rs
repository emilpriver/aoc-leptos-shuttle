use leptos::{leptos_dom::helpers::IntervalHandle, leptos_dom::logging::console_log, *};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;
use tracing;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Member {
    last_star_ts: u64,
    local_score: u64,
    stars: u64,
    name: Option<String>,
    completion_day_level: serde_json::Value,
    id: u64,
    global_score: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Members {
    members: std::collections::HashMap<String, Member>,
}

#[server(GetDashboardStats, "/leptos")]
async fn load_aoc_dashboard_data() -> Result<Vec<Member>, ServerFnError> {
    let client = reqwest::Client::new();

    let response = match client
        .get(format!(
            "https://adventofcode.com/{}/leaderboard/private/view/{}.json",
            env::var("LEADERBOARD_YEAR").unwrap_or("2023".to_string()),
            env::var("LEADERBOARD_ID").unwrap()
        ))
        .header(
            reqwest::header::COOKIE,
            format!("session={}", env::var("SESSION_COOKIE_TOKEN").unwrap()),
        )
        .send()
        .await
    {
        Ok(r) => r,
        Err(err) => {
            tracing::error!("{:?}", err);
            return Ok(vec![]);
        }
    };

    let json: Members = match response.json().await {
        Ok(j) => j,
        Err(err) => {
            tracing::error!("{:?}", err);
            return Ok(vec![]);
        }
    };

    let mut data: Vec<Member> = json.members.values().cloned().collect();
    data.sort_by(|a, b| b.local_score.partial_cmp(&a.local_score).unwrap());

    Ok(data)
}

#[component]
pub fn HomePage() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    let change_count = move || {
        set_count.update(|id| if *id != 2 { *id = 2 } else { *id = 1 });
    };

    use_interval(900000, move || change_count());

    let async_data = create_resource(
        count,
        |_count| async move { load_aoc_dashboard_data().await },
    );

    let async_result = move || {
        async_data
            .get()
            .map(|value| {
                match value {
                    Ok(r) => {
                        view! {
                            <>
                                <For
                                    each=move || r.clone()
                                    key=|m| m.id.clone()
                                    let:child
                                >
                                    <tr  class="item">
                                        <th>{child.name.clone().unwrap_or("anonymous user".to_string()).to_string()} </th>
                                        <th>{child.stars} </th>
                                        <th>{child.local_score} </th>
                                    </tr>
                                </For>
                            </>
                        }
                    },
                    Err(err) => {
                        console_log(err.to_string().as_str());
                        view! {
                            <>
                                <div> "lol not even working"</div>
                            </>
                        }
                    }
                }
            })
            .unwrap_or_else(|| {
                view! {
                    <>
                        <div> "lol not even working"</div>
                    </>
                }
            })
    };

    view! {
        <h1>"Advent Of Code Leaderbord"</h1>
        <div class="container">
            <button class="refresh-button" on:click=move |_| change_count()>
                "Refresh leaderboard"
            </button>
            <Suspense fallback=move || view! { <p>"Loading..."</p>}>
            <table class="table">
                <tr class="item">
                    <th>"Name" </th>
                    <th>"Stars"</th>
                    <th>"Local Score"</th>
                </tr>
                {move || async_result}
            </table>
            </Suspense>
        </div>
    }
}

/// Hook to wrap the underlying `setInterval` call and make it reactive w.r.t.
/// possible changes of the timer interval.
pub fn use_interval<T, F>(interval_millis: T, f: F)
where
    F: Fn() + Clone + 'static,
    T: Into<MaybeSignal<u64>> + 'static,
{
    let interval_millis = interval_millis.into();
    create_effect(move |prev_handle: Option<IntervalHandle>| {
        // effects get their previous return value as an argument
        // each time the effect runs, it will return the interval handle
        // so if we have a previous one, we cancel it
        if let Some(prev_handle) = prev_handle {
            prev_handle.clear();
        };

        // here, we return the handle
        set_interval_with_handle(
            f.clone(),
            // this is the only reactive access, so this effect will only
            // re-run when the interval changes
            Duration::from_millis(interval_millis.get()),
        )
        .expect("could not create interval")
    });
}
