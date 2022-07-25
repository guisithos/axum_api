#[tokio::main]
async fn main() {

    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(root))
        .route("/hello/:name", get(json_hello));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
