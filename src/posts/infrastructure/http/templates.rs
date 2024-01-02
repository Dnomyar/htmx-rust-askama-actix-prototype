use askama::Template;

use crate::auth::AuthInfo;

use super::routes::PostData;

#[derive(Template)]
#[template(path = "posts/post.html")]
pub struct PostTemplate<'a> {
    pub data: PostData,
    pub auth: &'a Option<AuthInfo>,
}

#[derive(Template)]
#[template(path = "posts/add_post_button.html")]
pub struct AddPostButton;

#[derive(Template)]
#[template(path = "posts/post_edit.html")]
pub struct PostEditTemplate {
    pub data: PostData,
}

#[derive(Template)]
#[template(path = "posts/posts_list.html")]
pub struct ListPostsTemplate<'a> {
    pub posts: &'a Vec<PostTemplate<'a>>,
}
