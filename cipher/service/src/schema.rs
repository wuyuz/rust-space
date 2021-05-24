table! {
    code (id) {
        id -> Binary,
        email -> Varchar,
        value -> Mediumtext,
        #[sql_name = "type"]
        type_ -> Tinyint,
        ip -> Nullable<Varbinary>,
        used_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        expired_at -> Timestamp,
    }
}

table! {
    feedback (id) {
        id -> Binary,
        user_id -> Binary,
        content -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    item (id) {
        id -> Binary,
        user_id -> Binary,
        #[sql_name = "type"]
        type_ -> Tinyint,
        title -> Nullable<Varchar>,
        website -> Nullable<Varchar>,
        x_alg -> Tinyint,
        x -> Varbinary,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    kv (key) {
        key -> Varchar,
        value -> Varbinary,
        flag -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    login (id) {
        id -> Binary,
        user_id -> Binary,
        platform -> Tinyint,
        ip -> Nullable<Varbinary>,
        device_id -> Binary,
        client_name -> Nullable<Varchar>,
        client_version_name -> Nullable<Varchar>,
        client_version_code -> Nullable<Integer>,
        host_name -> Nullable<Varchar>,
        host_version -> Nullable<Varchar>,
        os_name -> Nullable<Varchar>,
        os_version -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

table! {
    order (id) {
        id -> Binary,
        user_id -> Binary,
        level -> Tinyint,
        plan -> Varbinary,
        quantity -> Smallint,
        total_price -> Integer,
        platform -> Tinyint,
        payment_type -> Tinyint,
        payment_id -> Nullable<Varchar>,
        payment_at -> Nullable<Timestamp>,
        expired_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        status -> Tinyint,
    }
}

table! {
    token (id) {
        id -> Binary,
        user_id -> Binary,
        a -> Binary,
        r -> Binary,
        platform -> Tinyint,
        device_id -> Binary,
        expired_at -> Timestamp,
        created_at -> Timestamp,
        enabled -> Tinyint,
    }
}

table! {
    user (id) {
        id -> Binary,
        email -> Varchar,
        level -> Tinyint,
        expired_at -> Timestamp,
        settings -> Varbinary,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        enabled -> Tinyint,
    }
}

table! {
    version (id) {
        id -> Binary,
        platform -> Tinyint,
        name -> Nullable<Varchar>,
        version_name -> Nullable<Varchar>,
        version_code -> Nullable<Integer>,
        file_path -> Nullable<Varchar>,
        changelog -> Nullable<Varchar>,
        optional -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    code,
    feedback,
    item,
    kv,
    login,
    order,
    token,
    user,
    version,
);
