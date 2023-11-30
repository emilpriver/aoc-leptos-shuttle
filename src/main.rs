use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web::*;
use aoc::App;
use leptos::*;
use leptos_actix::{generate_route_list, LeptosRoutes};
use std::env;
use tracing::Level;

mod utils;

#[cfg(feature = "ssr")]
#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    // Setting this to None means we'll be using cargo-leptos and its env vars.
    let conf = get_configuration(None).await.unwrap();

    let port: String = env::var("PORT")
        .unwrap_or("3000".to_string())
        .parse()
        .unwrap_or("3000".to_string());

    let mut addr: String = format!("0.0.0.0:{port}");
    if env::var("ENVIRONMENT")
        .unwrap_or("PRODUCTION".to_string())
        .to_lowercase()
        == "development"
    {
        addr = format!("127.0.0.1:{port}");
    }

    let routes = generate_route_list(|| view! { <App/> });

    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_max_level(Level::INFO)
        .with_line_number(true)
        .with_thread_ids(true)
        .finish();

    if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
        panic!("failed to set default tracing subscriber: {}", e)
    }

    utils::load_env_variables();

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .service(Files::new("/assets", site_root))
            .wrap(Logger::default())
            .service(favicon)
            .route("/leptos/{tail:.*}", leptos_actix::handle_server_fns())
            .leptos_routes(
                leptos_options.to_owned(),
                routes.to_owned(),
                || view! { <App/> },
            )
            .app_data(web::Data::new(leptos_options.to_owned()))
            .wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}

#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}
