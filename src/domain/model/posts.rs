use chrono::{Utc, DateTime};



pub struct Posts(pub Vec<Post>);

#[derive(Clone)]
pub struct Post {
    pub id: String, 
    pub published_at: DateTime<Utc>,
    pub author: String,
    pub title: String, 
    pub content: String,
}