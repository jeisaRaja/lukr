use crate::{database::insert_web_bookmark, Error};

pub struct WebBookmark {
    pub key: String,
    pub value: String,
    pub tags: Vec<String>,
}

impl WebBookmark {
    pub fn new(key: String, value: String, tags: Vec<String>) -> Self {
        Self { key, value, tags }
    }
}

pub fn add_web_bookmark(
    db_path: &str,
    key: String,
    value: String,
    tags: Vec<String>,
) -> Result<(), Error> {
    let wb = WebBookmark::new(key, value, tags);

    insert_web_bookmark(db_path, &wb)
}
