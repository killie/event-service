table! {
    comments (id) {
        id -> Int4,
        event_id -> Int4,
        username -> Varchar,
        message -> Text,
        timestamp -> Int8,
    }
}

table! {
    events (id) {
        id -> Int4,
        from -> Int8,
        to -> Int8,
        origin_id -> Nullable<Int4>,
        event_type -> Nullable<Int4>,
        message -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    comments,
    events,
);
