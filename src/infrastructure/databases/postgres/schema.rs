// @generated automatically by Diesel CLI.

diesel::table! {
    game_events (id) {
        id -> Int8,
        session_id -> Nullable<Int8>,
        provider_id -> Nullable<Int8>,
        game_id -> Nullable<Int8>,
        user_id -> Nullable<Int8>,
        currency -> Nullable<Int8>,
        spin_id -> Nullable<Int8>,
        balance_before -> Nullable<Numeric>,
        balance_after -> Nullable<Numeric>,
        bet -> Nullable<Numeric>,
        win -> Nullable<Numeric>,
        profit -> Nullable<Numeric>,
        transaction_create_time -> Timestamp,
        transaction_finished_time -> Timestamp,
    }
}

diesel::table! {
    game_sessions (id) {
        id -> Int8,
        session_id -> Nullable<Int8>,
        user_id -> Nullable<Int8>,
        game_id -> Nullable<Int8>,
        currency -> Nullable<Int8>,
        start_time -> Timestamp,
    }
}

diesel::table! {
    games (id) {
        id -> Int4,
        provider_id -> Int4,
        code -> Varchar,
        name -> Varchar,
        image -> Varchar,
        is_mobile -> Bool,
        is_desktop -> Bool,
        is_tablet -> Bool,
        is_demo -> Bool,
        is_embedded -> Bool,
        bonus -> Int4,
        free_spins -> Int4,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    providers (id) {
        id -> Int4,
        name -> Varchar,
        aggregator -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    game_events,
    game_sessions,
    games,
    providers,
);
