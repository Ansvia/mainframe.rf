
CREATE TABLE accounts (
    id BIGSERIAL PRIMARY KEY,
    full_name VARCHAR NOT NULL,
    email VARCHAR NOT NULL, -- bisa digunakan untuk login
    phone_num VARCHAR NOT NULL, -- bisa digunakan untuk login
    active BOOLEAN NOT NULL,
    register_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- create nobody account
INSERT INTO accounts (id, full_name, email, phone_num, active)
VALUES
(0, 'nobody', 'nobody@nowhere.net', '', TRUE);


CREATE UNIQUE INDEX accounts_email ON accounts (
    (lower(email))
);
CREATE UNIQUE INDEX accounts_phone_num ON accounts (
    (lower(phone_num))
);


-- Berisi koleksi passhash dari akun
-- dibuat one-to-many agar ada history-nya setiap user merubah password.
CREATE TABLE account_passhash (
    account_id BIGINT PRIMARY KEY REFERENCES accounts(id) ON DELETE CASCADE,
    passhash VARCHAR NOT NULL,
    deprecated BOOLEAN NOT NULL,
    ver INT NOT NULL, -- passhash versioning, dibutuhkan apabila ingin merubah algo passhash ketika sudah jalan.
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Tabel untuk menampung user-user yang baru mendaftar tapi belum melakukan aktifasi
CREATE TABLE register_accounts (
    -- id BIGSERIAL PRIMARY KEY,
    token VARCHAR(100) PRIMARY KEY,
    full_name VARCHAR NOT NULL,
    email VARCHAR NOT NULL, -- untuk melakukan aktivasi via email
    phone_num VARCHAR NOT NULL, -- untuk melakukan aktivasi via phone (kalau tidak email)
    register_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    code VARCHAR(10) NOT NULL -- activation code bisa digunakan untuk aktivasi via SMS misalnya.
);

CREATE UNIQUE INDEX register_accounts_email ON register_accounts (
    (lower(email))
);
CREATE UNIQUE INDEX register_accounts_phone_num ON register_accounts (
    (lower(phone_num))
);

-- Tabel untuk alamat akun
CREATE TABLE addresses (
    id BIGSERIAL PRIMARY KEY,
    account_id BIGINT NOT NULL DEFAULT 0 REFERENCES accounts (id) ON DELETE SET DEFAULT,
    kind INT NOT NULL DEFAULT 0, -- 0=Domisili, 1=Asli
    "address" TEXT NOT NULL,
    regency VARCHAR NOT NULL,
    province VARCHAR NOT NULL,
    country VARCHAR NOT NULL,
    phone_num VARCHAR NOT NULL,
    active BOOLEAN NOT NULL,
    notes TEXT NOT NULL DEFAULT ''
);

-- Koleksi key pair untuk akun.
CREATE TABLE account_keys (
    id BIGSERIAL PRIMARY KEY,
    account_id BIGINT NOT NULL DEFAULT 0 REFERENCES accounts (id) ON DELETE CASCADE,
    pub_key TEXT NOT NULL,
    secret_key TEXT NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    active BOOLEAN NOT NULL DEFAULT FALSE
);



