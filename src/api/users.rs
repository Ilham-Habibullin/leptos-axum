use leptos::error::Result;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

use crate::components::admin_components::{Pagination, VecOfMaps};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub role: String
}

pub async fn fetch_users(limit: usize, offset: usize, role: Option<i16>) -> Result<(VecOfMaps, String)> {
    let url = match role {
        None => format!("http://localhost:8080/users?limit={limit}&offset={offset}"),
        Some(role) => format!("http://localhost:8080/users?limit={limit}&offset={offset}&role={role}")
    };

    let resp = Request::get(&url)
        .send()
        .await?;

    let users_count = resp.headers().get("x-total-count");

    match users_count {
        None => panic!("users count was not set"),
        Some(count) => {
            let response_as_text = resp.text().await?;
            let users_as_map: VecOfMaps  = serde_json::from_str(&response_as_text)?;
        
            Ok((users_as_map, count))
        }
    }
}

pub async fn get_users(Pagination(limit, offset): Pagination, mut initial_vec: VecOfMaps, role: Option<i16>) -> Result<(VecOfMaps, String)> {
    let (mut new_users, count) = fetch_users(limit, offset, role).await?;
    initial_vec.append(&mut new_users);
    Ok((initial_vec, count))
}

pub async fn delete_user() {
    todo!()
}

pub async fn promote_user() {
    todo!()
}