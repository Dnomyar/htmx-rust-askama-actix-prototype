use actix_identity::Identity;

use actix_web::{
    error::InternalError,
    get,
    http::{header::TryIntoHeaderPair, StatusCode},
    put, HttpMessage, HttpRequest, HttpResponse,
};
use askama::Template;

#[derive(Template)]
#[template(path = "auth/not_logged_in.html")]
struct NotLoggedIn;

#[derive(Template)]
#[template(path = "auth/logged_in.html")]
struct LoggedIn {
    user_id: String,
}

pub struct AuthInfo {
    pub user_id: String,
    pub name: String,
}

pub fn get_auth_info(identity: Identity) -> AuthInfo {
    AuthInfo {
        user_id: identity.id().unwrap_or("".to_string()),
        name: identity.id().unwrap_or("".to_string()),
    }
}

pub fn get_auth_info_option(identity: Option<Identity>) -> Option<AuthInfo> {
    identity.map(|id| get_auth_info(id))
}

pub fn render_auth_status(
    user: Option<Identity>,
    headers: Option<impl TryIntoHeaderPair>,
) -> HttpResponse {
    match user {
        Some(value) => value
            .id()
            .map(|user_id| {
                LoggedIn { user_id: user_id }
                    .render()
                    .map(|body| {
                        let mut ok = HttpResponse::Ok();
                        match headers {
                            Some(headers) => {
                                ok.insert_header(headers);
                            }
                            None => {}
                        }
                        ok.body(body)
                    })
                    .unwrap_or_else(|_e| {
                        HttpResponse::InternalServerError().body("Internal Server Error")
                    })
            })
            .unwrap_or_else(|_e| HttpResponse::InternalServerError().body("Internal Server Error")),
        None => NotLoggedIn
            .render()
            .map(|body| {
                let mut ok = HttpResponse::Ok();
                match headers {
                    Some(headers) => {
                        ok.insert_header(headers);
                    }
                    None => {}
                }
                ok.body(body)
            })
            .unwrap_or_else(|_e| HttpResponse::InternalServerError().body("Internal Server Error")),
    }
}

#[get("/auth/status")]
pub async fn auth_status(user: Option<Identity>) -> HttpResponse {
    render_auth_status(user, None::<(&str, &str)>)
}

#[put("/auth/login")]
pub async fn login(
    request: HttpRequest,
) -> std::result::Result<HttpResponse, InternalError<String>> {
    match Identity::login(&request.extensions(), "User1".into()) {
        Ok(identity) => Ok(render_auth_status(
            Some(identity),
            Some(("HX-Trigger", "auth-status-changed")),
        )),
        Err(e) => Err(InternalError::new(
            e.to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

#[put("/auth/logout")]
pub async fn logout(user: Identity) -> HttpResponse {
    user.logout();
    render_auth_status(None, Some(("HX-Trigger", "auth-status-changed")))
}
