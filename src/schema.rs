
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
