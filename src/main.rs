use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    // build our application with a single route
    let app = Router::new()
        .route(
            "/",
            get(|| async {
                info!("GET /");
                "Hello, World!"
            }),
        )
        .route("/user", get(get_user))
        .layer(TraceLayer::new_for_http());
    let port: u16 = 3000;
    let address = format!("0.0.0.0:{port}");

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    info!("Listening on {}", address);
    info!("Port: {}", port);
    axum::serve(listener, app).await.unwrap();
}

async fn get_user() -> String {
    info!("GET /user");
    "Hello, User!".to_string()
}
