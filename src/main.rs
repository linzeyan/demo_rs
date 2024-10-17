use axum::{extract::State, http::StatusCode, routing::any, Router};
use clap::Parser;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Specify return string
    #[arg(short, long, default_value = "Hello, World!")]
    a: String,
}

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let args = Args::parse();

    let app = Router::new()
        .route("/", any(handler))
        .with_state(args.clone())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    tracing::info!("server listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

/// handler
async fn handler(State(args): State<Args>) -> Result<String, StatusCode> {
    Ok(args.a.clone())
}
