use std::{collections::HashMap, sync::Mutex};

use crate::domain::model::posts::Post;

pub trait PostRepository
where
    Self: Send + Sync,
{
    fn find(&self, id: String) -> Result<Option<Post>, String>;
    fn windowed(&self, offset: usize, limit: usize) -> Result<Vec<Post>, String>;
    fn save(&self, post: Post) -> Result<(), String>;
}

pub struct InMemoryPostRepository {
    pub posts: Mutex<HashMap<String, Post>>,
}

impl PostRepository for InMemoryPostRepository {
    fn find(&self, id: String) -> Result<Option<Post>, String> {
        let posts = self.posts.lock().unwrap();
        Ok(posts.get(&id).cloned())
    }

    fn windowed(&self, offset: usize, limit: usize) -> Result<Vec<Post>, String> {
        let posts = self.posts.lock().unwrap();
        let mut posts: Vec<Post> = posts.values().cloned().collect();
        posts.sort_by(|a, b| b.published_at.cmp(&a.published_at));
        Ok(posts
            .into_iter()
            .skip(offset)
            .take(limit)
            .collect::<Vec<Post>>())
    }

    fn save(&self, post: Post) -> Result<(), String> {
        let mut posts = self.posts.lock().unwrap();
        posts.insert(post.id.clone(), post);
        Ok(())
    }
}
