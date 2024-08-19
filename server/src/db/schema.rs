// @generated automatically by Diesel CLI.

diesel::table! {
    bookmarks {
        id -> Text,
        url -> Text,
        title -> Text,
        description -> Text,
        tags -> Text,
    }
}
