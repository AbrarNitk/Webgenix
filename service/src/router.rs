use futures::{SinkExt, StreamExt};
use hyper::Body;
use std::collections::HashMap;

#[derive(thiserror::Error, Debug)]
pub enum BodyError {
    #[error("HyperBodyReadError: {}", _0)]
    HyperBodyRead(#[from] hyper::Error),
    #[error("SerdeDeserialize: {}", _0)]
    SerdeDeserialize(#[from] serde_json::Error),
}

async fn from_body<T: serde::de::DeserializeOwned>(b: Body) -> Result<T, BodyError> {
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
    tracing::info!(
        target = "request",
        method = req.method().as_str(),
        path = req.uri().path()
    );
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
        (&hyper::Method::GET, "/socket/file/") => Ok(send_file("web-socket.html")
            .await
            .expect("can not serve the file")),
        (&hyper::Method::GET, "/socket") => match handle_ws_conn_req(req).await {
            Ok(r) => Ok(r),
            Err(err) => {
                tracing::error!("error: {}", err);
                Ok(response(
                    "something went wrong".to_string(),
                    hyper::StatusCode::INTERNAL_SERVER_ERROR,
                ))
            }
        },
        (&hyper::Method::POST, "/api/post") => {
            let query = req.uri().query().map(|x| x.to_owned());
            let headers = req.headers().clone();

            let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
            let body: serde_json::Value = serde_json::from_slice(&body_bytes)?;

            let mut req_headers = HashMap::new();
            for (header_name, v) in headers {
                if let Some(name) = header_name {
                    req_headers.insert(name.to_string(), v.to_str().unwrap().to_string());
                }
            }
            let body = serde_json::json!({
                "body": body,
                "query": query,
                "headers": req_headers
            });

            println!("{:#?}", body);

            let response_body = serde_json::to_vec(&body)?;
            let mut response = hyper::Response::new(Body::from(response_body).into());
            *response.status_mut() = hyper::StatusCode::OK;
            response.headers_mut().append(
                hyper::header::CONTENT_TYPE,
                hyper::http::HeaderValue::from_str("application/json").unwrap(), // TODO: Remove unwrap
            );

            Ok(response)
        }
        (&hyper::Method::GET, "/api/get") => {
            println!("this is get call");
            let mut response = hyper::Response::new(hyper::Body::empty());
            *response.body_mut() = req.into_body();
            *response.status_mut() = hyper::StatusCode::OK;
            response.headers_mut().append(
                hyper::header::CONTENT_TYPE,
                hyper::http::HeaderValue::from_str("application/json").unwrap(), // TODO: Remove unwrap
            );
            Ok(response)
        }
        (&hyper::Method::GET, "/conver/settings/") => {
            let mut response = hyper::Response::new(hyper::Body::empty());
            *response.body_mut() = hyper::Body::from(conver_settings());
            *response.status_mut() = hyper::StatusCode::OK;
            response.headers_mut().append(
                hyper::header::CONTENT_TYPE,
                hyper::http::HeaderValue::from_str("text/plain").unwrap(), // TODO: Remove unwrap
            );
            Ok(response)
        }
        _ => {
            let apis = crate::utils::apis().expect("no api file found");
            let (parts, body) = req.into_parts();
            let req_body: serde_json::Value = from_body(body).await.unwrap();
            tracing::info!(body = serde_json::to_string(&req_body).unwrap());
            match apis.response(parts.method.as_str(), parts.uri.path()) {
                Some(r) => Ok(response(
                    serde_json::to_string(&r).unwrap(),
                    hyper::StatusCode::OK,
                )),
                None => {
                    return Ok(response(
                        "NOT-FOUND".to_string(),
                        hyper::StatusCode::NOT_FOUND,
                    ));
                }
            }
        }
    }
}

pub fn conver_settings() -> String {
    use std::io::Read;
    let mut file = std::fs::File::options()
        .read(true)
        .open("conver-settings.toml")
        .unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    buffer
}

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

pub async fn handle_ws_conn_req(
    mut req: hyper::Request<hyper::Body>,
) -> Result<hyper::Response<hyper::Body>, Error> {
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
                // websocket.send(hyper_tungstenite::tungstenite::Message::text("Thank you, come again.")).await?;
                // loop {
                let msg = serde_json::json!({
                    "type": "requested-sign",
                    "requestId": "12345",
                    "request": {
                        "type": "SignData",
                        "requestId": "req-12345",
                        "socketId": "socket-12345",
                        "hash": "hash-string",
                        "data": {
                            "purpose": "i have some purpose",
                            "amount": 12345,
                            "did": "my-did"
                        }
                    }
                });
                websocket
                    .send(hyper_tungstenite::tungstenite::Message::text(
                        serde_json::to_string(&msg).unwrap().as_str(),
                    ))
                    .await?;
                // std::thread::sleep(std::time::Duration::from_secs(2))
                // }
            }
            hyper_tungstenite::tungstenite::Message::Binary(msg) => {
                println!("Received binary message: {:02X?}", msg);
                websocket
                    .send(hyper_tungstenite::tungstenite::Message::binary(
                        b"Thank you, come again.".to_vec(),
                    ))
                    .await?;
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
                    println!(
                        "Received close message with code {} and message: {}",
                        msg.code, msg.reason
                    );
                } else {
                    println!("Received close message");
                }
            }
            hyper_tungstenite::tungstenite::Message::Frame(_msg) => {
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
