// @generated automatically by Diesel CLI.

// Define the Diesel prelude to include common traits and types.
use diesel::prelude::*;

diesel::table! {
    conversations (id) {
        id -> Text,
        room_id -> Text,
        user_id -> Text,
        content -> Text,
        created_at -> Text,
    }
}

diesel::table! {
    rooms (id) {
        id -> Text,
        name -> Text,
        last_message -> Text,
        participant_ids -> Text,
        created_at -> Text,
    }
}

pub mod users {
    diesel::table! {
        users (id) {
            id -> Text,
            username -> Text,
            phone -> Text,
            created_at -> Text,
        }
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    conversations,
    rooms,
    users,
);
