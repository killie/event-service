// That's great love, but where does all this end up and how can I use it in the same module but in another file?

table! {
    events (id) {
        id -> Int8,
        from -> Int8,
        to -> Int8,
        origin_id -> Int4,
        event_type -> Int4,
        message -> Text,
    }
}

table! {
    origins (id) {
        id -> Int8,
        name -> Varchar,
    }
}

table! {
    sources (id) {
        id -> Int8,
        name -> Varchar,
        origin_id -> Int8,
    }
}

table! {
    event_sources (id) {
        id -> Int8,
        event_id -> Int8,
        source_id -> Int8,
    }
}

table! {
    event_types (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
    }
}

table! {
    comments (id) {
        id -> Int8,
        event_id -> Int8,
        username -> Varchar,
        message -> Text,
        timestamp -> Int8,
    }
}
