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
    bank (id) {
        id -> Int8,
        name -> Varchar,
    }
}

table! {
    external_transaction_histories (id) {
        id -> Int8,
        internal_tx_id -> Int8,
        ttype -> Int4,
        subttype -> Int4,
        amount -> Float8,
        status -> Int4,
        created -> Timestamp,
        invoice_id -> Nullable<Int8>,
        from_$param.service_name_snake_case$_id -> Nullable<Int8>,
        to_$param.service_name_snake_case$_id -> Nullable<Int8>,
        merchant_id -> Nullable<Int8>,
        error_code -> Int4,
        error_info -> Text,
        notes -> Nullable<Text>,
    }
}

table! {
    invoice_items (id) {
        id -> Int8,
        invoice_id -> Int8,
        name -> Varchar,
        price -> Float8,
    }
}

table! {
    invoices (id) {
        id -> Int8,
        id_ref -> Text,
        issuer_$param.service_name_snake_case$ -> Int8,
        to_$param.service_name_snake_case$ -> Int8,
        discount -> Float8,
        amount -> Float8,
        notes -> Text,
        created -> Timestamp,
        paid -> Bool,
        paid_by -> Int8,
        paid_at -> Nullable<Timestamp>,
    }
}

table! {
    merchant (id) {
        id -> Int8,
        name -> Varchar,
        $param.service_name_snake_case$_inst_id -> Nullable<Int8>,
        $param.service_name_snake_case$_no -> Nullable<Varchar>,
        $param.service_name_snake_case$_id -> Int8,
    }
}

table! {
    payment_history (id) {
        id -> Int8,
        invoice_id -> Int8,
        payer -> Int8,
        via -> Varchar,
        ts -> Timestamp,
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

table! {
    transaction_histories (id) {
        id -> Int8,
        dbcr_flag -> Int4,
        ttype -> Int4,
        amount -> Float8,
        status -> Int4,
        created -> Timestamp,
        last_updated -> Timestamp,
        invoice_id -> Nullable<Int8>,
        from_$param.service_name_snake_case$_id -> Nullable<Int8>,
        to_$param.service_name_snake_case$_id -> Nullable<Int8>,
        merchant_id -> Nullable<Int8>,
        notes -> Nullable<Text>,
    }
}

joinable!(access_tokens -> $param.service_name_snake_case$s ($param.service_name_snake_case$_id));
joinable!($param.service_name_snake_case$_keys -> $param.service_name_snake_case$s ($param.service_name_snake_case$_id));
joinable!($param.service_name_snake_case$_passhash -> $param.service_name_snake_case$s ($param.service_name_snake_case$_id));
joinable!(addresses -> $param.service_name_snake_case$s ($param.service_name_snake_case$_id));
joinable!(external_transaction_histories -> transaction_histories (internal_tx_id));
joinable!(invoice_items -> invoices (invoice_id));
joinable!(merchant -> $param.service_name_snake_case$s ($param.service_name_snake_case$_id));
joinable!(merchant -> bank ($param.service_name_snake_case$_inst_id));
joinable!(payment_history -> $param.service_name_snake_case$s (payer));
joinable!(payment_history -> invoices (invoice_id));

allow_tables_to_appear_in_same_query!(
    access_tokens,
    $param.service_name_snake_case$_keys,
    $param.service_name_snake_case$_passhash,
    $param.service_name_snake_case$s,
    addresses,
    bank,
    external_transaction_histories,
    invoice_items,
    invoices,
    merchant,
    payment_history,
    register_$param.service_name_snake_case$s,
    transaction_histories,
);
