use std::path::Path;

use crate::{commands::WebBookmark, Error};
use rusqlite::{params, Connection};

pub fn create_db(db_path: &str) -> Result<(), Error> {
    let conn = Connection::open(db_path)?;

    let _ = conn.execute(CREATE_DIR_BOOKMARKS_TABLE, params![])?;
    let _ = conn.execute(CREATE_WEB_BOOKMARKS_TABLE, params![])?;
    let _ = conn.execute(CREATE_TAGS_TABLE, params![])?;
    let _ = conn.execute(CREATE_BOOKMARK_TAGS_TABLE, params![])?;

    Ok(())
}

pub fn insert_web_bookmark(db_path: &str, bookmark: &WebBookmark) -> Result<(), Error> {
    let conn = Connection::open(db_path)?;
    let _ = conn.execute(INSERT_WEB_BOOKMARK, params![bookmark.key, bookmark.value])?;

    Ok(())
}

pub fn check_db_exist(db_path: &str) -> bool {
    Path::new(db_path).exists()
}

const CREATE_DIR_BOOKMARKS_TABLE: &str = "
    CREATE TABLE dir_bookmarks (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        key TEXT UNIQUE NOT NULL,
        path TEXT NOT NULL
);";

const CREATE_WEB_BOOKMARKS_TABLE: &str = "
    CREATE TABLE IF NOT EXISTS web_bookmarks (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        key TEXT UNIQUE NOT NULL,
        url TEXT NOT NULL
);";

const CREATE_BOOKMARK_TAGS_TABLE: &str = "
    CREATE TABLE bookmark_tags (
        bookmark_id INTEGER NOT NULL,
        tag_id INTEGER NOT NULL,
        bookmark_type TEXT NOT NULL,
        FOREIGN KEY (bookmark_id) REFERENCES web_bookmarks(id),
        FOREIGN KEY (tag_id) REFERENCES tags(id)
);";

const CREATE_TAGS_TABLE: &str = "
    CREATE TABLE tags (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT UNIQUE NOT NULL
);";

const INSERT_TAG: &str = "
    INSERT INTO tags (name) VALUES (?);
";

const INSERT_WEB_BOOKMARK: &str = "
    INSERT INTO web_bookmarks (key, url) VALUES (?,?)
";
