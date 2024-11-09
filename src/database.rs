use std::path::Path;

use crate::{
    args::ListType,
    commands::{BookmarkType, DirBookmark, WebBookmark},
    Error,
};
use rusqlite::{params, Connection};

pub fn create_db(db_path: &str) -> Result<(), Error> {
    let conn = Connection::open(db_path)?;

    let _ = conn.execute(CREATE_DIR_BOOKMARKS_TABLE, params![])?;
    let _ = conn.execute(CREATE_WEB_BOOKMARKS_TABLE, params![])?;
    let _ = conn.execute(CREATE_TAGS_TABLE, params![])?;
    let _ = conn.execute(CREATE_BOOKMARK_TAGS_TABLE, params![])?;

    Ok(())
}

pub fn insert_bookmark(db_path: &str, bookmark: BookmarkType) -> Result<(), Error> {
    let mut conn = Connection::open(db_path)?;
    let query = match bookmark {
        BookmarkType::Web(..) => INSERT_WEB_BOOKMARK,
        BookmarkType::Dir(..) => INSERT_DIR_BOOKMARK,
    };
    let tx = conn.transaction()?;
    let _ = tx.execute(query, params![bookmark.key(), bookmark.value()])?;
    let bookmark_id = tx.last_insert_rowid();

    let tags = bookmark.tags();
    if tags.len() > 0 {
        for tag in tags {
            tx.execute(INSERT_TAG, params![tag])?;
            let mut stmt = tx.prepare(SELECT_TAG)?;
            let tag_id: i64 = stmt.query_row(params![tag], |row| row.get(0))?;

            tx.execute(
                INSERT_BOOKMARK_TAG,
                params![bookmark_id, tag_id, bookmark.as_string()],
            )?;
        }
    }
    tx.commit()?;

    Ok(())
}

pub fn select_bookmark(
    db_path: &str,
    key: &str,
    item_type: ListType,
) -> Result<BookmarkType, Error> {
    let conn = Connection::open(db_path)?;
    let bookmark = match item_type {
        ListType::Dir => {
            let bookmark = conn.query_row(SELECT_DIR_BOOKMARK, params![key], |row| {
                Ok(DirBookmark::new(
                    key.to_string(),
                    row.get(1).unwrap(),
                    vec![],
                ))
            })?;
            BookmarkType::Dir(bookmark)
        }
        ListType::Web => {
            let bookmark = conn.query_row(SELECT_WEB_BOOKMARK, params![key], |row| {
                Ok(WebBookmark::new(
                    key.to_string(),
                    row.get(1).unwrap(),
                    vec![],
                ))
            })?;
            BookmarkType::Web(bookmark)
        }
    };

    Ok(bookmark)
}

pub fn insert_tag(db_path: &str, tag: &str) -> Result<i64, Error> {
    let conn = Connection::open(db_path)?;
    let _ = conn.execute(INSERT_TAG, params![tag])?;
    let last_inserted_id = conn.last_insert_rowid();
    Ok(last_inserted_id)
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
    INSERT OR IGNORE INTO tags (name) VALUES (?);
";

const INSERT_BOOKMARK_TAG: &str = "
    INSERT INTO bookmark_tags (bookmark_id, tag_id, bookmark_type) 
    VALUES (?, ?, ?);
";

const INSERT_WEB_BOOKMARK: &str = "
    INSERT OR IGNORE INTO web_bookmarks (key, url) VALUES (?,?);
";

const INSERT_DIR_BOOKMARK: &str = "
    INSERT OR IGNORE INTO dir_bookmarks (key, path) VALUES (?,?);
";

const SELECT_TAG: &str = "
    SELECT id FROM tags WHERE name = ?;
";

const SELECT_WEB_BOOKMARK: &str = "
        SELECT key, url FROM web_bookmarks WHERE key = (?);
";

const SELECT_DIR_BOOKMARK: &str = "
        SELECT key, path FROM dir_bookmarks WHERE key = (?);
";
