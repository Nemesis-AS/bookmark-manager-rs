// @generated automatically by Diesel CLI.

diesel::table! {
    bookmarks (id) {
        id -> Text,
        url -> Text,
        title -> Text,
        description -> Text,
        tags -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tags (id) {
        id -> Text,
        title -> Text,
        color -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    bookmarks,
    tags,
);
