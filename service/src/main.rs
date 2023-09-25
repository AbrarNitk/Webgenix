pub struct HttpService;

impl hyper::service::Service<hyper::Request<hyper::Body>> for HttpService {
    type Response = hyper::Response<hyper::Body>;
    type Error = hyper::Error;
    type Future = std::pin::Pin<
        Box<dyn futures::Future<Output=Result<Self::Response, Self::Error>> + Send>,
    >;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: hyper::Request<hyper::Body>) -> Self::Future {
        Box::pin(async {
            match service::router::handler(req).await {
                Ok(r) => Ok(r),
                Err(e) => {
                    tracing::error!(target = "ServerHandlerError", "Error: {}", e);
                    Ok(service::router::response(
                        serde_json::to_string(&serde_json::json!({
                            "success": false,
                            "message": "INTERNAL_SERVER_ERROR"
                        }))
                            .expect(""),
                        hyper::StatusCode::INTERNAL_SERVER_ERROR,
                    ))
                }
            }
        })
    }
}


async fn http_main() -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    // Setting the environment variables
    let env_path = format!("{}.env", service::utils::read_env());
    dotenv::from_path(env_path.as_str()).ok();
    tracing::info!("Environment set: {}", env_path);

    // Initializing the database pool
    // db::pg::init_db_pool();
    // db::redis::init_redis_pool();

    // Creating the tcp listener
    let port = service::utils::read_port_env();
    let socket_address: std::net::SocketAddr = ([0, 0, 0, 0], port).into();
    let listener = tokio::net::TcpListener::bind(socket_address).await?;
    tracing::info!(
        "#### Started at: {}:{} ####",
        socket_address.ip(),
        socket_address.port()
    );

    // if needed so get the pool and send it to the `HttpService`
    // Database pool
    // let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env var not found");
    // let db_pool = db::pg::get_connection_pool(db_url.as_str());

    // Redis pool
    // let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL env var not found");
    // let redis_pool = db::redis::get_pool(redis_url.as_str());

    loop {
        let (tcp_stream, _) = listener.accept().await?;
        tokio::task::spawn(async move {
            if let Err(http_err) = hyper::server::conn::Http::new()
                .serve_connection(tcp_stream, HttpService {})
                .await
            {
                tracing::error!("Error while serving HTTP connection: {}", http_err);
            }
        });
    }
}

async fn http_main_wrapper() {
    http_main().await.expect("service error")
}

async fn traced_main() {
    use tracing_subscriber::layer::SubscriberExt;
    let level = std::env::var("TRACING")
        .unwrap_or_else(|_| "info".to_owned())
        .parse::<tracing_forest::util::LevelFilter>()
        .unwrap_or(tracing_forest::util::LevelFilter::INFO);

    if service::utils::is_traced() {
        tracing_forest::worker_task()
            .set_global(true)
            .build_with(|_layer: tracing_forest::ForestLayer<_, _>| {
                tracing_subscriber::Registry::default()
                    .with(tracing_forest::ForestLayer::default())
                    .with(level)
            })
            .on(http_main_wrapper())
            .await
    } else {
        tracing_forest::worker_task()
            .set_global(true)
            .build_with(|_layer: tracing_forest::ForestLayer<_, _>| {
                tracing_subscriber::FmtSubscriber::default().with(level)
                // tracing_subscriber::Registry::default()
                //     .with()
                //     .with(level)
            })
            .on(http_main_wrapper())
            .await
    }
}

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(traced_main())
}
