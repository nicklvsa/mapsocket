use crate::types::ResultT;
use warp::{Reply, http::StatusCode};

pub async fn healthcheck() -> ResultT<impl Reply> {
    Ok(StatusCode::OK)
}