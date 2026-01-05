use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use crate::services::storage::{get_token, set_token};

const API_BASE_URL: &str = "http://127.0.0.1:8080/api";

pub async fn post<T, R>(path: &str, body: &T) -> Result<R, String>
where
    T: Serialize,
    R: for<'de> Deserialize<'de>,
{
    let url = format!("{}{}", API_BASE_URL, path);
    let mut request = Request::post(&url);

    if let Some(token) = get_token() {
        request = request.header("Authorization", &format!("Bearer {}", token));
    }

    let response = request
        .json(body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.ok() {
        response.json::<R>().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Error: {}", response.status()))
    }
}

pub async fn get<R>(path: &str) -> Result<R, String>
where
    R: for<'de> Deserialize<'de>,
{
    let url = format!("{}{}", API_BASE_URL, path);
    let mut request = Request::get(&url);

    if let Some(token) = get_token() {
        request = request.header("Authorization", &format!("Bearer {}", token));
    }

    let response = request
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.ok() {
        response.json::<R>().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Error: {}", response.status()))
    }
}
