table! {
    access_tokens (token) {
        token -> Text,
        $param.service_name_snake_case$_id -> Int8,
        created -> Timestamp,
        valid_thru -> Timestamp,
    }
}

table! {
    $param.service_name_snake_case$_keys (id) {
        id -> Int8,
        $param.service_name_snake_case$_id -> Int8,
        pub_key -> Text,
        secret_key -> Text,
        created -> Timestamp,
        active -> Bool,
    }
}

table! {
    $param.service_name_snake_case$_passhash ($param.service_name_snake_case$_id) {
        $param.service_name_snake_case$_id -> Int8,
        passhash -> Varchar,
        deprecated -> Bool,
        ver -> Int4,
        created -> Timestamp,
    }
}

table! {
    $param.service_name_snake_case$s (id) {
        id -> Int8,
        full_name -> Varchar,
        email -> Varchar,
        phone_num -> Varchar,
        active -> Bool,
        register_time -> Timestamp,
    }
}

table! {
    addresses (id) {
        id -> Int8,
        $param.service_name_snake_case$_id -> Int8,
        kind -> Int4,
        address -> Text,
        regency -> Varchar,
        province -> Varchar,
        country -> Varchar,
        phone_num -> Varchar,
        active -> Bool,
        notes -> Text,
    }
}

table! {
    register_$param.service_name_snake_case$s (token) {
        token -> Varchar,
        full_name -> Varchar,
        email -> Varchar,
        phone_num -> Varchar,
        register_time -> Timestamp,
        code -> Varchar,
    }
}

joinable!(access_tokens -> $param.service_name_snake_case$s ($param.service_name_snake_case$_id));
joinable!($param.service_name_snake_case$_keys -> $param.service_name_snake_case$s ($param.service_name_snake_case$_id));
joinable!($param.service_name_snake_case$_passhash -> $param.service_name_snake_case$s ($param.service_name_snake_case$_id));
joinable!(addresses -> $param.service_name_snake_case$s ($param.service_name_snake_case$_id));

allow_tables_to_appear_in_same_query!(
    access_tokens,
    $param.service_name_snake_case$_keys,
    $param.service_name_snake_case$_passhash,
    $param.service_name_snake_case$s,
    addresses,
    register_$param.service_name_snake_case$s,
);
