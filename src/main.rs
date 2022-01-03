use axum::{error_handling::HandleErrorLayer, routing::get, Router};
use clap::{App, Arg};

use std::{net::SocketAddr, time::Duration};
use tower::ServiceBuilder;

use httpmq_rs::service::{handle_error, init, process};

#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "httpmq-rs=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();

    let matches = App::new("httpmq-rs")
        .bin_name("httpmq-rs")
        .arg(
            Arg::new("maxqueue")
                .long("maxqueue")
                .default_value("100000000"),
        )
        .get_matches();

    init(matches);
    // Build our application by composing routes
    let app = Router::new()
        .route("/", get(process))
        // Add middleware to all routes
        .layer(
            ServiceBuilder::new()
                // Handle errors from middleware
                .layer(HandleErrorLayer::new(handle_error))
                .load_shed()
                .concurrency_limit(1024)
                .timeout(Duration::from_secs(10))
                // .layer(TraceLayer::new_for_http())
                // .layer(AddExtensionLayer::new(state))
                .into_inner(),
        );

    // Run our app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 1218));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
