
CREATE TABLE $param.service_name_snake_case$s (
    id BIGSERIAL PRIMARY KEY,
    full_name VARCHAR NOT NULL,
    email VARCHAR NOT NULL, -- bisa digunakan untuk login
    phone_num VARCHAR NOT NULL, -- bisa digunakan untuk login
    active BOOLEAN NOT NULL,
    register_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- create nobody $param.service_name_snake_case$
INSERT INTO $param.service_name_snake_case$s (id, full_name, email, phone_num, active)
VALUES
(0, 'nobody', 'nobody@nowhere.net', '+628512345', TRUE),
(1, 'Dummy User 1', 'dummy1@nowhere.net', '+62856789', TRUE),
(2, 'Dummy User 2', 'dummy2@nowhere.net', '+62856789124', TRUE),
(3, 'Dummy User 3', 'dummy3@nowhere.net', '+62856789124', TRUE)
;

SELECT SETVAL('$param.service_name_snake_case$s_id_seq', 3);

CREATE UNIQUE INDEX $param.service_name_snake_case$s_email ON $param.service_name_snake_case$s (
    (lower(email))
);
CREATE UNIQUE INDEX $param.service_name_snake_case$s_phone_num ON $param.service_name_snake_case$s (
    (lower(phone_num))
);


-- Berisi koleksi passhash dari akun
-- dibuat one-to-many agar ada history-nya setiap user merubah password.
CREATE TABLE $param.service_name_snake_case$_passhash (
    $param.service_name_snake_case$_id BIGINT PRIMARY KEY REFERENCES $param.service_name_snake_case$s(id) ON DELETE CASCADE,
    passhash VARCHAR NOT NULL,
    deprecated BOOLEAN NOT NULL,
    ver INT NOT NULL, -- passhash versioning, dibutuhkan apabila ingin merubah algo passhash ketika sudah jalan.
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Insert preset password for dummy user using password `123123` by default.
INSERT INTO $param.service_name_snake_case$_passhash ($param.service_name_snake_case$_id, passhash, deprecated, ver)
VALUES
(1, '$2y$05$mw56Wls35HoufQH7QipJnOzqzVmZuwcVUojcqQxKZ5hcG8aBdZRo.', FALSE, 1),
(2, '$2y$05$mw56Wls35HoufQH7QipJnOzqzVmZuwcVUojcqQxKZ5hcG8aBdZRo.', FALSE, 1),
(3, '$2y$05$mw56Wls35HoufQH7QipJnOzqzVmZuwcVUojcqQxKZ5hcG8aBdZRo.', FALSE, 1)
;

-- Tabel untuk menampung user-user yang baru mendaftar tapi belum melakukan aktifasi
CREATE TABLE register_$param.service_name_snake_case$s (
    -- id BIGSERIAL PRIMARY KEY,
    token VARCHAR(100) PRIMARY KEY,
    full_name VARCHAR NOT NULL,
    email VARCHAR NOT NULL, -- untuk melakukan aktivasi via email
    phone_num VARCHAR NOT NULL, -- untuk melakukan aktivasi via phone (kalau tidak email)
    register_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    code VARCHAR(10) NOT NULL -- activation code bisa digunakan untuk aktivasi via SMS misalnya.
);

CREATE UNIQUE INDEX register_$param.service_name_snake_case$s_email ON register_$param.service_name_snake_case$s (
    (lower(email))
);
CREATE UNIQUE INDEX register_$param.service_name_snake_case$s_phone_num ON register_$param.service_name_snake_case$s (
    (lower(phone_num))
);

-- Tabel untuk alamat akun
CREATE TABLE addresses (
    id BIGSERIAL PRIMARY KEY,
    $param.service_name_snake_case$_id BIGINT NOT NULL DEFAULT 0 REFERENCES $param.service_name_snake_case$s (id) ON DELETE SET DEFAULT,
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
CREATE TABLE $param.service_name_snake_case$_keys (
    id BIGSERIAL PRIMARY KEY,
    $param.service_name_snake_case$_id BIGINT NOT NULL DEFAULT 0 REFERENCES $param.service_name_snake_case$s (id) ON DELETE CASCADE,
    pub_key TEXT NOT NULL,
    secret_key TEXT NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    active BOOLEAN NOT NULL DEFAULT FALSE
);



