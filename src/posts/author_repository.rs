use std::{collections::HashMap, sync::Mutex};

use crate::domain::model::author::Author;

pub trait AuthorRepository
where
    Self: Send + Sync,
{
    fn find(&self, id: String) -> Result<Option<Author>, String>;
    fn find_by_ids(&self, ids: Vec<String>) -> Result<Vec<Author>, String>;
    fn save(&self, author: Author) -> Result<(), String>;
}

pub struct InMemoryAuthorRepository {
    pub authors: Mutex<HashMap<String, Author>>,
}

impl AuthorRepository for InMemoryAuthorRepository {
    fn find(&self, id: String) -> Result<Option<Author>, String> {
        let authors = self.authors.lock().unwrap();
        Ok(authors.get(&id).cloned())
    }

    fn find_by_ids(&self, ids: Vec<String>) -> Result<Vec<Author>, String> {
        let authors = self.authors.lock().unwrap();
        Ok(ids
            .iter()
            .filter_map(|id| authors.get(id).cloned())
            .collect())
    }

    fn save(&self, author: Author) -> Result<(), String> {
        let mut authors = self.authors.lock().unwrap();
        authors.insert(author.id.clone(), author);
        Ok(())
    }
}
