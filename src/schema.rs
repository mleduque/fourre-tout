table! {
    documents (id) {
        id -> Int8,
        version -> Int8,
        old_version -> Nullable<Int8>,
        body -> Jsonb,
        published -> Timestamptz,
        first_published -> Timestamptz,
    }
}

table! {
    models (id) {
        id -> Int8,
        version -> Int8,
        name -> Varchar,
        old_version -> Nullable<Int8>,
        body -> Jsonb,
        published -> Timestamptz,
        first_published -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    documents,
    models,
);
