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
        _ => todo!(),
    }
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
