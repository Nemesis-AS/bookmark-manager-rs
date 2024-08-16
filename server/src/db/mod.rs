use diesel::{r2d2, SqliteConnection};

pub mod models;
pub mod schema;

pub fn init_db() -> r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>> {
    let db_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager: r2d2::ConnectionManager<SqliteConnection> =
        r2d2::ConnectionManager::<SqliteConnection>::new(db_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Database URL should be a valid path for SQLite database file!")
}
