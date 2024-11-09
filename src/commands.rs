use std::env;

use crate::{database::insert_bookmark, Error};

pub struct WebBookmark {
    pub key: String,
    pub value: String,
    pub tags: Vec<String>,
}

pub struct DirBookmark {
    pub key: String,
    pub value: String,
    pub tags: Vec<String>,
}

pub enum BookmarkType {
    Web(WebBookmark),
    Dir(DirBookmark),
}

impl BookmarkType {
    pub fn key(&self) -> &String {
        match self {
            BookmarkType::Web(bookmark) => &bookmark.key,
            BookmarkType::Dir(bookmark) => &bookmark.key,
        }
    }

    pub fn value(&self) -> &String {
        match self {
            BookmarkType::Web(bookmark) => &bookmark.value,
            BookmarkType::Dir(bookmark) => &bookmark.value,
        }
    }

    pub fn tags(&self) -> &Vec<String> {
        match self {
            BookmarkType::Web(bookmark) => &bookmark.tags,
            BookmarkType::Dir(bookmark) => &bookmark.tags,
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            BookmarkType::Web(..) => "web".to_string(),
            BookmarkType::Dir(..) => "dir".to_string(),
        }
    }
}

impl DirBookmark {
    pub fn new(key: String, value: String, tags: Vec<String>) -> Self {
        Self { key, value, tags }
    }

    pub fn get_full_path(path: &str) -> Result<String, Error> {
        let curr_dir = env::current_dir()?;
        let absolute_path = curr_dir.join(path).canonicalize()?;

        if let Some(absolute_path_str) = absolute_path.to_str() {
            Ok(absolute_path_str.to_string())
        } else {
            Err("No path found".into())
        }
    }
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
    let bookmark = WebBookmark::new(key, value, tags);

    insert_bookmark(db_path, BookmarkType::Web(bookmark))
}

pub fn add_dir_bookmark(
    db_path: &str,
    key: String,
    value: String,
    tags: Vec<String>,
) -> Result<(), Error> {
    let mut bookmark = DirBookmark::new(key, value, tags);
    bookmark.value = DirBookmark::get_full_path(&bookmark.value)?;
    insert_bookmark(db_path, BookmarkType::Dir(bookmark))
}
