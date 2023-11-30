use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use std::env;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/shuttle-leptos.css"/>
        <Title text="Advent Of Code"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Member {
    last_star_ts: u64,
    local_score: u64,
    stars: u64,
    name: String,
    completion_day_level: serde_json::Value,
    id: u64,
    global_score: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Members {
    members: std::collections::HashMap<String, Member>,
}

async fn load_aoc_dashboard_data() -> Vec<Member> {
    let client = reqwest::Client::new();

    let response = match client
        .get(format!(
            "https://adventofcode.com/2023/leaderboard/private/view/{}.json",
            env::var("DASHBOARD_ID").unwrap()
        ))
        .header(
            reqwest::header::COOKIE,
            format!("session={}", env::var("SESSION_COOKIE_TOKEN").unwrap()),
        )
        .send()
        .await
    {
        Ok(r) => r,
        Err(..) => return vec![],
    };

    let json: Members = match response.json().await {
        Ok(j) => j,
        Err(..) => return vec![],
    };

    let data: Vec<Member> = json.members.values().cloned().collect();

    data
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    let async_data = create_resource(
        || (),
        |value| async move { load_aoc_dashboard_data().await },
    );

    let async_result = move || {
        async_data
            .get()
            .map(|value| {
                let inner = value
                    .iter()
                    .map(|v| {
                        view! {
                            <div> {v.name.to_string()} </div>
                        }
                    })
                    .collect_view();

                view! {<div> {inner} </div>}
            })
            .unwrap_or_else(|| {
                view! {
                    <div> "lol not even working"</div>
                }
            })
    };

    let loading = async_data.loading();

    view! {
        <h1>"Advent Of Code Leaderbord"</h1>
        <Suspense fallback=move || view! { <p>"Loading..."</p>}>
            {move || async_result}
        </Suspense>
    }
}
