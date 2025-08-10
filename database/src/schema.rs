// @generated automatically by Diesel CLI.

diesel::table! {
    cates (id) {
        id -> Integer,
        icon_url -> Text,
        img_url -> Text,
        cate_name -> Text,
        live_total -> Integer,
    }
}

diesel::table! {
    rooms (id) {
        id -> Integer,
        title -> Text,
        is_live -> Bool,
        img_url -> Text,
        hot -> Integer,
        user_id -> Integer,
        cate_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    rooms_tags (room_id, tag_id) {
        room_id -> Integer,
        tag_id -> Integer,
    }
}

diesel::table! {
    tags (id) {
        id -> Integer,
        title -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        user_name -> Text,
        avatar -> Text,
        created_at -> Timestamp,
    }
}

diesel::joinable!(rooms -> cates (cate_id));
diesel::joinable!(rooms -> users (user_id));
diesel::joinable!(rooms_tags -> rooms (room_id));
diesel::joinable!(rooms_tags -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    cates,
    rooms,
    rooms_tags,
    tags,
    users,
);
