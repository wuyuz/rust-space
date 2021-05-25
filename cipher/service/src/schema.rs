table! {
    code (id) {
        id -> Unsigned<Bigint>,
        email -> Varchar,
        value -> Varchar,
        used_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        expired_at -> Timestamp,
    }
}

table! {
    item (id) {
        id -> Unsigned<Bigint>,
        user_id -> Binary,
        x -> Varbinary,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user (id) {
        id -> Unsigned<Bigint>,
        email -> Varchar,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    code,
    item,
    user,
);
