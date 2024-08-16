// @generated automatically by Diesel CLI.

diesel::table! {
    bookmarks (id) {
        id -> Nullable<Text>,
        url -> Text,
        title -> Text,
        description -> Text,
    }
}
