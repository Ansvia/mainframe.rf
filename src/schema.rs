table! {
    access_tokens (token) {
        token -> Text,
        account_id -> Int8,
        created -> Timestamp,
        valid_thru -> Timestamp,
    }
}

table! {
    account_keys (id) {
        id -> Int8,
        account_id -> Int8,
        pub_key -> Text,
        secret_key -> Text,
        created -> Timestamp,
        active -> Bool,
    }
}

table! {
    account_passhash (account_id) {
        account_id -> Int8,
        passhash -> Varchar,
        deprecated -> Bool,
        ver -> Int4,
        created -> Timestamp,
    }
}

table! {
    accounts (id) {
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
        account_id -> Int8,
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
        from_account_id -> Nullable<Int8>,
        to_account_id -> Nullable<Int8>,
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
        issuer_account -> Int8,
        to_account -> Int8,
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
        account_inst_id -> Nullable<Int8>,
        account_no -> Nullable<Varchar>,
        account_id -> Int8,
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
    register_accounts (token) {
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
        from_account_id -> Nullable<Int8>,
        to_account_id -> Nullable<Int8>,
        merchant_id -> Nullable<Int8>,
        notes -> Nullable<Text>,
    }
}

joinable!(access_tokens -> accounts (account_id));
joinable!(account_keys -> accounts (account_id));
joinable!(account_passhash -> accounts (account_id));
joinable!(addresses -> accounts (account_id));
joinable!(external_transaction_histories -> transaction_histories (internal_tx_id));
joinable!(invoice_items -> invoices (invoice_id));
joinable!(merchant -> accounts (account_id));
joinable!(merchant -> bank (account_inst_id));
joinable!(payment_history -> accounts (payer));
joinable!(payment_history -> invoices (invoice_id));

allow_tables_to_appear_in_same_query!(
    access_tokens,
    account_keys,
    account_passhash,
    accounts,
    addresses,
    bank,
    external_transaction_histories,
    invoice_items,
    invoices,
    merchant,
    payment_history,
    register_accounts,
    transaction_histories,
);
