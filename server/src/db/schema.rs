// @generated automatically by Diesel CLI.

diesel::table! {
    bookmarks (id) {
        id -> Text,
        url -> Text,
        title -> Text,
        description -> Text,
        tags -> Text,
    }
}

diesel::table! {
    tags (id) {
        id -> Text,
        title -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    bookmarks,
    tags,
);
