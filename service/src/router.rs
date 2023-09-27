use futures::{SinkExt, StreamExt};


#[derive(thiserror::Error, Debug)]
pub enum BodyError {
    #[error("HyperBodyReadError: {}", _0)]
    HyperBodyRead(#[from] hyper::Error),
    #[error("SerdeDeserialize: {}", _0)]
    SerdeDeserialize(#[from] serde_json::Error),
}

async fn from_body<T: serde::de::DeserializeOwned>(b: hyper::Body) -> Result<T, BodyError> {
    let b = hyper::body::to_bytes(b).await?;
    Ok(serde_json::from_slice(b.as_ref())?)
}

async fn send_file(p: &str) -> Result<hyper::Response<hyper::Body>, std::io::Error> {
    use tokio::io::AsyncReadExt;
    let mut f = tokio::fs::File::open(p).await?;
    let mut data = Vec::new();
    f.read_to_end(&mut data).await?;
    Ok(hyper::Response::new(data.into()))
}

pub async fn handler(
    req: hyper::Request<hyper::Body>,
) -> Result<hyper::Response<hyper::Body>, http_service::errors::RouteError> {
    tracing::info!(target = "request", method = req.method().as_str(), path = req.uri().path());
    match (req.method(), req.uri().path()) {
        (&hyper::Method::GET, "/api/health/") => {
            let mut response = hyper::Response::new(hyper::Body::empty());
            let resp = http_service::controller::get_user_profile()?;
            *response.body_mut() = hyper::Body::from(serde_json::to_string(&resp)?);
            *response.status_mut() = hyper::StatusCode::OK;
            response.headers_mut().append(
                hyper::header::CONTENT_TYPE,
                hyper::http::HeaderValue::from_str("application/json").unwrap(), // TODO: Remove unwrap
            );
            Ok(response)
        }
        (&hyper::Method::GET, "/socket/file/") => {
            Ok(send_file("web-socket.html").await.expect("can not serve the file"))
        }
        (&hyper::Method::GET, "/socket") => {
            match handle_ws_conn_req(req).await {
                Ok(r) => Ok(r),
                Err(err) => {
                    tracing::error!("error: {}", err);
                    Ok(response("something went wrong".to_string(), hyper::StatusCode::INTERNAL_SERVER_ERROR))
                }
            }
        }
        (&hyper::Method::POST, "/") => {
            let mut response = hyper::Response::new(hyper::Body::empty());
            *response.body_mut() = hyper::Body::from("POST Response");
            *response.status_mut() = hyper::StatusCode::OK;
            response.headers_mut().append(
                hyper::header::CONTENT_TYPE,
                hyper::http::HeaderValue::from_str("application/json").unwrap(), // TODO: Remove unwrap
            );
            Ok(response)
        }
        _ => {
            return Ok(response("NOT-FOUND".to_string(), hyper::StatusCode::NOT_FOUND));
        }
    }
}

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

pub async fn handle_ws_conn_req(mut req: hyper::Request<hyper::Body>) -> Result<hyper::Response<hyper::Body>, Error> {
    if hyper_tungstenite::is_upgrade_request(&req) {
        let (response, ws) = hyper_tungstenite::upgrade(&mut req, None)?;
        tokio::spawn(async move {
            if let Err(e) = serve_websocket(ws).await {
                eprintln!("Error in websocket connection: {}", e);
            }
        });
        Ok(response)
    } else {
        Ok(hyper::Response::new(hyper::Body::from("Hello HTTP!")))
    }
}


pub async fn serve_websocket(websocket: hyper_tungstenite::HyperWebsocket) -> Result<(), Error> {
    let mut websocket = websocket.await?;
    while let Some(msg) = websocket.next().await {
        match msg? {
            hyper_tungstenite::tungstenite::Message::Text(msg) => {
                println!("Received text message: {}", msg);
                websocket.send(hyper_tungstenite::tungstenite::Message::text("Thank you, come again.")).await?;
            }
            hyper_tungstenite::tungstenite::Message::Binary(msg) => {
                println!("Received binary message: {:02X?}", msg);
                websocket.send(hyper_tungstenite::tungstenite::Message::binary(b"Thank you, come again.".to_vec())).await?;
            }
            hyper_tungstenite::tungstenite::Message::Ping(msg) => {
                // No need to send a reply: tungstenite takes care of this for you.
                println!("Received ping message: {:02X?}", msg);
            }
            hyper_tungstenite::tungstenite::Message::Pong(msg) => {
                println!("Received pong message: {:02X?}", msg);
            }
            hyper_tungstenite::tungstenite::Message::Close(msg) => {
                // No need to send a reply: tungstenite takes care of this for you.
                if let Some(msg) = &msg {
                    println!("Received close message with code {} and message: {}", msg.code, msg.reason);
                } else {
                    println!("Received close message");
                }
            }
            hyper_tungstenite::tungstenite::Message::Frame(msg) => {
                unreachable!();
            }
        }
    }

    Ok(())
    // Docs: https://docs.rs/hyper-tungstenite/latest/hyper_tungstenite/
    // Docs: https://gist.github.com/izderadicka/bdc803d38840a15436f1a5ac1b1ca2cd#file-echo-simple-rs-L169
}


pub fn response(body: String, status: hyper::StatusCode) -> hyper::Response<hyper::Body> {
    let mut response = hyper::Response::new(hyper::Body::from(body));
    *response.status_mut() = status;
    response.headers_mut().append(
        hyper::header::CONTENT_TYPE,
        hyper::http::HeaderValue::from_static("application/json"),
    );
    response
}
