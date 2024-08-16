use diesel::table;

table! {
    bookmarks (id) {
        id -> Text,
        url -> Text,
        title -> Text,
        description -> Text,
        // created_at -> Timestamp,
        // tags -> Array<Int4>,
    }
}