use super::model::author::Author;

pub trait AuthorRepository
where
    Self: Send + Sync,
{
    fn find(&self, id: String) -> Result<Option<Author>, String>;
    fn find_by_ids(&self, ids: Vec<String>) -> Result<Vec<Author>, String>;
    fn save(&self, author: Author) -> Result<(), String>;
}
