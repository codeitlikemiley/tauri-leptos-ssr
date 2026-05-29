use leptos::logging::log;
use leptos::prelude::*;

#[tokio::main]
async fn main() {
    if std::env::var("LEPTOS_OUTPUT_NAME").is_err() {
        std::env::set_var("LEPTOS_OUTPUT_NAME", "app");
    }
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    let app = app::build_router(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
