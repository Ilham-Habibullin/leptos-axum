use leptos::error::Result;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

use crate::common_types::{Pagination, VecOfMaps};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Note {
    pub id: i32,
    pub text: String
}

pub async fn fetch_notes(limit: usize, offset: usize) -> Result<(VecOfMaps, String)> {
    let url = format!("http://localhost:8080/notes?limit={limit}&offset={offset}");

    let resp = Request::get(&url)
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await?;

    let notes_count = resp.headers().get("x-total-count");

    match notes_count {
        None => panic!("notes count was not set"), 
        Some(count) => {
            let response_as_text = resp.text().await?;
            let notes_as_map: VecOfMaps  = serde_json::from_str(&response_as_text)?;
            Ok((notes_as_map, count))
        }
    }
}

pub async fn get_notes(Pagination(limit, offset): Pagination, mut initial_vec: VecOfMaps) -> Result<(VecOfMaps, String)> {
    let (mut new_notes, count) = fetch_notes(limit, offset).await?;
    initial_vec.append(&mut new_notes);
    Ok((initial_vec, count))
}

pub async fn delete_note() {
    todo!()
}