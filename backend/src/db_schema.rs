// @generated automatically by Diesel CLI.

diesel::table! {
    ratings (id) {
        id -> Int4,
        video_id -> Varchar,
        user_id -> Int4,
        rating -> Float8,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        family_name -> Varchar,
        given_name -> Varchar,
        picture -> Varchar,
    }
}

diesel::table! {
    videos (id) {
        id -> Varchar,
        platform -> Varchar,
        title -> Varchar,
        description -> Text,
        published_at -> Timestamptz,
        thumbnail_url -> Varchar,
        rating -> Nullable<Float8>,
    }
}

diesel::joinable!(ratings -> users (user_id));
diesel::joinable!(ratings -> videos (video_id));

diesel::allow_tables_to_appear_in_same_query!(
    ratings,
    users,
    videos,
);
