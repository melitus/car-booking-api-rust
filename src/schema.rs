// @generated automatically by Diesel CLI.

diesel::table! {
    cars (id) {
        id -> Uuid,
        name -> Text,
        price -> Text,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(cars -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    cars,
    users,
);
