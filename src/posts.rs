use std::sync::Mutex;

use crate::{
    domain::model::{author, posts::{Posts, Post}},
    Authors,
};
use actix_web::{
    error::{self, InternalError},
    get,
    http::StatusCode,
    web::{self, Data, Query, Path, Redirect},
    App, HttpServer, Responder, put, post,
};
use askama::Template;
use chrono::{Duration, Utc};
use serde::Deserialize;

struct PostData {
    id: String,
    published_at: Duration,
    author: String,
    title: String,
    content: String,
}

#[derive(Template)]
#[template(path = "posts/post.html")]
struct PostTemplate {
    data: PostData
}

#[derive(Template)]
#[template(path = "posts/post_edit.html")]
struct PostEditTemplate {
    data: PostData
}

#[derive(Template)]
#[template(path = "posts/posts_list.html")]
struct ListPostsTemplate<'a> {
    posts: &'a Vec<PostTemplate>,
    next_page: &'a u16,
}

#[derive(serde::Deserialize)]
struct PaginationQueryParams {
    page: Option<u16>,
}

fn post_to_post_data(post: &Post, authors: &Authors) -> PostData{
    PostData{
        id: post.id.to_string(),
        published_at: Utc::now().signed_duration_since(post.published_at),
        author: authors
                    .get(&post.author)
                    .map(|author| author.name.to_string())
                    .unwrap_or("unknown".to_string())
                    .to_string(),
                title: post.title.to_string(),
                content: post.content.to_string(),
    }

}


#[get("/posts/{id}/edit")]
pub async fn edit_post_endpoint(
    id: Path<String>,
    posts: Data<Mutex<Posts>>,
    authors: Data<Authors>,
) -> std::result::Result<impl Responder, InternalError<String>> {
    match posts.lock().unwrap().0.iter().find(|p| p.id == *id) {
        Some(post) => PostEditTemplate{
            data: post_to_post_data(post, &authors)
        }  .render()
        .map_err(|err| InternalError::new(err.to_string(), StatusCode::INTERNAL_SERVER_ERROR)),
        None =>
        todo!()
    }
  
}

fn get_post_by_id<'a>(id: &str, posts: &Data<Mutex<Posts>>, authors: &Authors) -> Result<String, InternalError<String>> {
    match posts.lock().unwrap().0.iter().find(|p| p.id == *id) {
        Some(post) => PostTemplate{
            data: post_to_post_data(post, &authors)
        }  .render()
        .map_err(|err| InternalError::new(err.to_string(), StatusCode::INTERNAL_SERVER_ERROR)),
        None =>
        todo!()
    }
}


#[get("/posts/{id}")]
pub async fn post_endpoint(
    id: Path<String>,
    posts: Data<Mutex<Posts>>,
    authors: Data<Authors>,
) -> std::result::Result<impl Responder, InternalError<String>> {
    get_post_by_id(&id, &posts, &authors)
  
}

#[derive(Deserialize)]
struct PostUpdateFormData {
    title: String,
    content: String,
}

#[put("/posts/{id}")]
pub async fn post_update_endpoint(
    id: Path<String>,
    form: web::Form<PostUpdateFormData>,
    posts: Data<Mutex<Posts>>,
    authors: Data<Authors>,
) -> std::result::Result<impl Responder, InternalError<String>> {
    let res = posts.lock().unwrap().0.iter_mut().find(|p| p.id == *id).map(|post| {
        post.title = form.title.clone();
        post.content = form.content.clone();
        post
    });
    get_post_by_id(&id, &posts, &authors)
}


#[get("/posts")]
pub async fn list_posts_endpoint(
    pagination: Query<PaginationQueryParams>,
    posts: Data<Mutex<Posts>>,
    authors: Data<Authors>,
) -> std::result::Result<impl Responder, InternalError<String>> {
    let current_page = pagination.page.unwrap_or(0);
    ListPostsTemplate {
        posts: &posts.lock().unwrap()
            .0
            .chunks(3)
            .enumerate()
            .filter(|(idx, _)| *idx as u16 == current_page)
            .flat_map(|(_, posts)| {
                posts.into_iter().map(|post| PostTemplate {
                    data: post_to_post_data(post, &authors)
                })
            })
            .collect(),
        next_page: &(current_page + 1),
    }
    .render()
    .map_err(|err| InternalError::new(err.to_string(), StatusCode::INTERNAL_SERVER_ERROR))
}

#[post("/posts")]
pub async fn create_post_endpoint(
    posts: Data<Mutex<Posts>>,
    authors: Data<Authors>,
) -> std::result::Result<impl Responder, InternalError<String>> {
    let mut posts = posts.lock().unwrap();

    let post = Post {
        id: uuid::Uuid::new_v4().to_string(), 
        published_at: Utc::now(),
        author: "unknown".to_string(),
        title: "New Post".to_string(),
        content: "".to_string(),
    };
    posts.0.insert(0, post.clone());
    PostEditTemplate{
        data: post_to_post_data(&post, &authors)
    }  .render()
    .map_err(|err| InternalError::new(err.to_string(), StatusCode::INTERNAL_SERVER_ERROR))

}
