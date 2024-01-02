use actix_session::Session;
use actix_web::{error::InternalError, get, http::StatusCode, put, HttpResponse, Responder};
use askama::Template;

static SESSION_NAME: &str = "user_id";

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

pub fn get_auth_info(session: Session) -> Option<AuthInfo> {
    session
        .get::<String>(SESSION_NAME)
        .ok()
        .flatten()
        .map(|user_id| AuthInfo {
            user_id: user_id.to_string(),
            name: user_id.to_string(),
        })
}

pub fn render_auth_status(session: Session) -> std::result::Result<String, InternalError<String>> {
    match session.get::<String>(SESSION_NAME) {
        Ok(Some(value)) => LoggedIn {
            user_id: value.to_string(),
        }
        .render()
        .map_err(|e| InternalError::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR)),
        Ok(None) => NotLoggedIn
            .render()
            .map_err(|e| InternalError::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR)),
        Err(e) => Err(InternalError::new(
            e.to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

#[get("/auth/status")]
pub async fn auth_status(session: Session) -> std::result::Result<String, InternalError<String>> {
    render_auth_status(session)
}

#[put("/auth/login")]
pub async fn login(session: Session) -> std::result::Result<HttpResponse, InternalError<String>> {
    session
        .insert(SESSION_NAME, "123".to_string())
        .map_err(|e| InternalError::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
    render_auth_status(session).map(|body| {
        HttpResponse::Ok()
            .insert_header(("HX-Trigger", "auth-status-changed"))
            .body(body)
    })
}

#[put("/auth/logout")]
pub async fn logout(session: Session) -> std::result::Result<HttpResponse, InternalError<String>> {
    session.remove(SESSION_NAME).ok_or(InternalError::new(
        "unable to remove the session".to_string(),
        StatusCode::INTERNAL_SERVER_ERROR,
    ))?;
    render_auth_status(session).map(|body| {
        HttpResponse::Ok()
            .insert_header(("HX-Trigger", "auth-status-changed"))
            .body(body)
    })
}
