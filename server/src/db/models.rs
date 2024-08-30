use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

use super::schema::{bookmarks, tags};

#[derive(Debug, Clone, Serialize, Deserialize, Selectable, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = bookmarks)]
pub struct Bookmark {
    pub id: String,
    pub url: String,
    pub title: String,
    pub description: String,
    pub tags: String,
    // pub created_at: chrono::NaiveDateTime,
    // pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Selectable, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = bookmarks)]
pub struct NewBookmark {
    pub url: String,
    pub title: String,
    pub description: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Selectable, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = tags)]
pub struct Tag {
    pub id: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Selectable, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = tags)]
pub struct NewTag {
    pub title: String,
}