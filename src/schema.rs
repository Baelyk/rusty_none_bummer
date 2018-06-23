table! {
    activities (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        points -> Int4,
    }
}

table! {
    friends (id) {
        id -> Int4,
        name -> Text,
        points -> Int4,
        activities -> Nullable<Jsonb>,
    }
}

allow_tables_to_appear_in_same_query!(
    activities,
    friends,
);
