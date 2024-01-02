pub mod author_repository;
pub mod post_repository;
pub mod templates;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    auth::{self, get_auth_info, get_auth_info_option, AuthInfo},
    domain::model::{
        author::{self, Author},
        posts::{Post, Posts},
    },
    posts::{author_repository::AuthorRepository, post_repository::PostRepository, templates::*},
    Authors,
};
use actix_identity::Identity;
use actix_session::Session;
use actix_web::{
    error::{self, InternalError},
    get,
    http::StatusCode,
    post, put,
    web::{self, Data, Path, Query, Redirect},
    App, HttpResponse, HttpServer, Responder,
};
use askama::Template;
use chrono::{Duration, Utc};
use serde::Deserialize;

pub struct PostData {
    id: String,
    published_at: Duration,
    author: String,
    title: String,
    content: String,
}

#[derive(serde::Deserialize)]
struct PaginationQueryParams {
    page: Option<u16>,
}

#[get("/posts/ui/add")]
pub async fn add_post_button_endpoint(
    user: Option<Identity>,
) -> std::result::Result<HttpResponse, InternalError<String>> {
    match user {
        Some(_) => AddPostButton
            .render()
            .map(|body| HttpResponse::Ok().body(body))
            .map_err(|err| InternalError::new(err.to_string(), StatusCode::INTERNAL_SERVER_ERROR)),
        None => Ok(HttpResponse::Ok().body("")),
    }
}

fn post_to_post_data(post: Post, author: &Author) -> PostData {
    PostData {
        id: post.id.to_string(),
        published_at: Utc::now().signed_duration_since(post.published_at),
        author: author.name.to_string(),
        title: post.title.to_string(),
        content: post.content.to_string(),
    }
}

fn create_post_data_from_repository(
    id: &str,
    post_repository: Data<Box<dyn PostRepository>>,
    author_repository: Data<Box<dyn AuthorRepository>>,
) -> Result<Option<PostData>, String> {
    match post_repository.find(id.to_string())? {
        Some(post) => match author_repository.find(post.author.to_string())? {
            Some(author) => Ok(Some(post_to_post_data(post, &author))),
            None => Ok(None),
        },
        None => Ok(None),
    }
}

#[get("/posts/ui/posts/{id}/edit")]
pub async fn edit_post_endpoint(
    id: Path<String>,
    post_respository: Data<Box<dyn PostRepository>>,
    author_repository: Data<Box<dyn AuthorRepository>>,
) -> HttpResponse {
    match create_post_data_from_repository(&id, post_respository, author_repository) {
        Ok(Some(post)) => PostEditTemplate { data: post }
            .render()
            .map(|body| HttpResponse::Ok().body(body))
            .unwrap_or_else(|e| HttpResponse::InternalServerError().body("Internal Server Error")),
        Ok(None) => HttpResponse::NotFound().body("Post not found"),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

fn get_post_by_id<'a>(
    id: &str,
    post_respository: Data<Box<dyn PostRepository>>,
    author_repository: Data<Box<dyn AuthorRepository>>,
    auth: &'a Option<AuthInfo>,
) -> HttpResponse {
    match create_post_data_from_repository(id, post_respository, author_repository) {
        Ok(Some(post)) => PostTemplate { data: post, auth }
            .render()
            .map(|body| HttpResponse::Ok().body(body))
            .unwrap_or_else(|err| {
                HttpResponse::InternalServerError().body(format!("Internal Server Error: {}", err))
            }),
        Ok(None) => HttpResponse::NotFound().body("Post not found"),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[get("/posts/ui/posts/{id}")]
pub async fn post_endpoint(
    id: Path<String>,
    post_respository: Data<Box<dyn PostRepository>>,
    author_repository: Data<Box<dyn AuthorRepository>>,
    identify: Option<Identity>,
) -> HttpResponse {
    let auth = get_auth_info_option(identify);
    get_post_by_id(&id, post_respository, author_repository, &auth)
}

#[derive(Deserialize)]
struct PostUpdateFormData {
    title: String,
    content: String,
}

fn update_post(
    id: &str,
    form: &PostUpdateFormData,
    post_repository: Data<Box<dyn PostRepository>>,
) -> Result<(), String> {
    let maybe_post = post_repository.find(id.to_string())?;
    match maybe_post {
        Some(mut post) => {
            post.title = form.title.clone();
            post.content = form.content.clone();
            post_repository.save(post)?;
            Ok(())
        }
        None => Err("Post not found".to_string()),
    }
}

#[put("/posts/{id}")]
pub async fn post_update_endpoint(
    id: Path<String>,
    form: web::Form<PostUpdateFormData>,
    post_repository: Data<Box<dyn PostRepository>>,
    identify: Identity,
) -> HttpResponse {
    match update_post(&id, &form, post_repository.clone()) {
        Ok(_) => HttpResponse::Ok().body(""),
        Err(_) => HttpResponse::NotFound().body("Post not found"),
    }
}

fn windowed_posts<'a>(
    page: u16,
    post_repository: Data<Box<dyn PostRepository>>,
    author_repository: Data<Box<dyn AuthorRepository>>,
    auth: &'a Option<AuthInfo>,
) -> Result<Vec<PostTemplate<'a>>, String> {
    let posts = post_repository.windowed(page as usize * 3, 3)?;
    let author_ids = posts
        .iter()
        .map(|post| post.author.clone())
        .collect::<Vec<String>>();
    let authors = author_repository
        .find_by_ids(author_ids)?
        .into_iter()
        .map(|author| (author.id.clone(), author))
        .collect::<HashMap<String, Author>>();
    Ok(posts
        .into_iter()
        .map(|post| {
            let author_id = post.author.to_string();
            PostTemplate {
                data: post_to_post_data(post, authors.get(&author_id).unwrap()),
                auth: auth,
            }
        })
        .collect())
}

#[get("/posts")]
pub async fn list_posts_endpoint(
    pagination: Query<PaginationQueryParams>,
    post_repository: Data<Box<dyn PostRepository>>,
    author_repository: Data<Box<dyn AuthorRepository>>,
    identify: Option<Identity>,
) -> HttpResponse {
    let current_page = pagination.page.unwrap_or(0);
    let auth = get_auth_info_option(identify);
    match windowed_posts(current_page, post_repository, author_repository, &auth) {
        Ok(posts) => ListPostsTemplate { posts: &posts }
            .render()
            .map(|body| HttpResponse::Ok().body(body))
            .unwrap_or_else(|err| {
                HttpResponse::InternalServerError().body(format!("Internal Server Error: {}", err))
            }),
        Err(err) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[post("/posts")]
pub async fn create_post_endpoint(
    post_repository: Data<Box<dyn PostRepository>>,
    identify: Identity,
) -> HttpResponse {
    let create_post = || -> Result<PostData, String> {
        let post_id = uuid::Uuid::new_v4().to_string();
        let auth = get_auth_info(identify);
        let post = Post {
            id: post_id,
            published_at: Utc::now(),
            author: "1".to_string(),
            title: "New Post".to_string(),
            content: "".to_string(),
        };
        post_repository.save(post.clone())?;
        Ok(post_to_post_data(
            post,
            &Author {
                id: auth.user_id,
                name: auth.name,
            },
        ))
    };
    match create_post() {
        Ok(post) => PostEditTemplate { data: post }
            .render()
            .map(|body| HttpResponse::Ok().body(body))
            .unwrap_or_else(|err| {
                HttpResponse::InternalServerError().body(format!("Internal Server Error: {}", err))
            }),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}
