use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use super::schema::bookmarks;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = bookmarks)]
pub struct Bookmark {
    pub id: String,
    pub url: String,
    pub title: String,
    pub description: String,
    // pub created_at: chrono::NaiveDateTime,
    // pub tags: Vec<i32>,
}
