table! {
    comments (id) {
        id -> Int8,
        event_id -> Int8,
        username -> Varchar,
        message -> Text,
        timestamp -> Int8,
    }
}

