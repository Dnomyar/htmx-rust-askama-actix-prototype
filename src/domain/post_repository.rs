use super::model::posts::Post;

pub trait PostRepository
where
    Self: Send + Sync,
{
    fn find(&self, id: String) -> Result<Option<Post>, String>;
    fn windowed(&self, offset: usize, limit: usize) -> Result<Vec<Post>, String>;
    fn save(&self, post: Post) -> Result<(), String>;
}
