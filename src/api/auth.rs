use gloo_net::http::Request;
use thiserror::Error;
use std::result::Result;

use super::users::User;



#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Please request more than zero cats.")]
    EmptyField(String),

    #[error(transparent)]
    GlooNetError(#[from] gloo_net::Error)
}

#[derive(Error, Debug, Clone)]
pub enum MeError {
    #[error("You need to authorize.")]
    Unauthorized,

    #[error("Gloo error")]
    GlooNetError(String)
}

pub async fn me() -> Result<User, MeError> {
    let url = format!("http://localhost:8080/auth/me");

    let response = Request::get(&url)
        .header("Content-type", "application/json")
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(|err| MeError::GlooNetError(err.to_string()))?;

    if response.status() == 401 {
        Err(MeError::Unauthorized.into())
    } else {
        let user = response.json::<User>().await.map_err(|err| MeError::GlooNetError(err.to_string()))?;
        Ok(user)
    }
}

pub async fn signup(username: String, password: String) -> Result<User, AuthError> {
    if username == "" || password == "" {
        return Err(AuthError::EmptyField("username or password or both fields empty".to_string()).into())
    }

    let url = format!("http://localhost:8080/auth/signup");

    let json_to_send = format!("{{ \"username\": \"{username}\", \"password\": \"{password}\" }}");

    let response = Request::post(&url)
        .header("Content-type", "application/json")
        .body(json_to_send)?
        .send()
        .await?;

    let user = response.json::<User>().await?;

    Ok(user)
}


pub async fn signin(username: String, password: String) -> Result<(), AuthError> {
    if username == "" || password == "" {
        return Err(AuthError::EmptyField("username or password or both fields empty".to_string()).into())
    }

    let url = format!("http://localhost:8080/auth/signin");

    let json_to_send = format!("{{ \"username\": \"{username}\", \"password\": \"{password}\" }}");

    Request::post(&url)
        .header("Content-type", "application/json")
        .credentials(web_sys::RequestCredentials::Include)
        .body(json_to_send)?
        .send()
        .await?;

    Ok(())
}
