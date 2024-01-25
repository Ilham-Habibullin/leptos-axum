use leptos::error::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cat {
    url: String,
}   

#[derive(Error, Debug)]
pub enum CatError {
    #[error("Please request more than zero cats.")]
    NonZeroCats,
}

pub async fn fetch_cats(count: usize) -> Result<Vec<String>> {
    if count <= 0 {
        return Err(CatError::NonZeroCats.into())
    }

    let url = &format!("https://api.thecatapi.com/v1/images/search?limit={count}");

    let response = gloo_net::http::Request
        ::get(url)
        .send()
        .await?
        .json::<Vec<Cat>>()
        .await?
        .into_iter()
        .take(count)
        .map(|cat| cat.url)
        .collect::<Vec<_>>();

    Ok(response)
}
