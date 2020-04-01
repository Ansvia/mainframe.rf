$name$
============================

[![pipeline status](https://gitlab.com/$param.author_name_kebab_case$/$name_snake_case$/badges/master/pipeline.svg)](https://gitlab.com/$param.author_name_kebab_case$/$name_snake_case$/commits/master)

$param.desc$

This project based on Mainframe 0.2

Kebutuhan
----------------

Daftar kebutuhan berikut dibutuhkan apabila kita ingin melakukan build di mesin environment lokal, kamu bisa juga melakukan build menggunakan Docker sehingga tidak perlu menginstall satu-per-satu kebutuhan ini. Untuk build menggunakan Docker lihat bagian *Build menngunakan Docker*.
Berikut kebutuhan pokok untuk bisa melakukan build di mesin lokal:


**Backend Server:**

1. [Rust](https://www.rust-lang.org/)
2. PostgreSQL >= 9.x
3. [diesel](http://diesel.rs)
4. [Aglio](https://www.npmjs.com/package/aglio) (optional, untuk dokumentasi)
5. [Rustfmt](https://github.com/rust-lang/rustfmt)
5. [Cargo clippy](https://github.com/rust-lang/rust-clippy)
6. [Cargo audit](https://github.com/RustSec/cargo-audit)
7. [Protocol Buffer](https://developers.google.com/protocol-buffers/)

**Web Frontend:**

1. [Node JS](https://nodejs.org)
2. [NPM](https://www.npmjs.com/)
3. [Yarn](https://yarnpkg.com/)

**Mobile Frontend:**

1. [Flutter](https://flutter.dev/)
2. [Xcode](https://developer.apple.com/xcode/) <-- untuk iOS
3. [Android SDK](https://developer.android.com/studio/releases/sdk-tools) <-- untuk Android


Build
-----------

Sebelum melakukan build pastikan dulu Libpq (Library-nya PostgreSQL) telah tersedia, di Ubuntu bisa menggunakan perintah `apt install libpq-dev` atau di Debian `apt install libpq-devel`, di OSX bisa menggunakan perintah: `brew install libpq`.

Setelah semua dependensi tersedia, ketikkan:

    $ cargo build

Build menggunakan Docker
----------------------------

Cara paling mudah untuk melakukan build adalah menggunakan Docker:

    $ docker run -it --rm -v $(pwd):/workdir \
        -v /tmp:/root/.cargo/git \
        -v /tmp:/root/.cargo/registry \
        anvie/rust-musl-build:latest \
        cargo build --release --target=x86_64-unknown-linux-musl

Docker image `anvie/rust-musl-build` adalah container berbasis Linux dan sudah berisi semua kebutuhan development untuk build project ini, setelah build selesai
output bisa didapatkan di `target/x86_64-unknown-linux-musl`.

Kamu bisa juga menjalankan perintah tersebut menggunakan make:

    $ make release-linux

Testing
----------

Testing kebanyakan ditulis terintegrasi (integration testing), untuk itu perlu menjalankan database
dan mempersiapkan environment-nya, ini hanya perlu dijalankan sekali, ketikkan:

    $ make test-env

**CATATAN**: Perintah `test-env` akan membuat database baru dengan nama `$name_snake_case$_test` dimana database ini akan digunakan
sebagai storage ketika proses testing terjadi.
Perintah `make test-env` ini juga perlu dijalankan ulang apabila ada perubahan schema untuk memastikan schema
dalam database selalu up-to-date.

Untuk melakukan test ketikkan:

    $ make test

Menjalankan
-------------

Untuk menjalankan service $name_snake_case$ perlu dipastikan service PostgreSQL sudah jalan terlebih dahulu, dan telah disetup database-nya.

Untuk men-setup database bisa menggunakan perintah:

    $ make reset-db

Selanjutkan jalankan $name$ servernya:

    $ cargo run --bin $name_snake_case$_server


Frontend
------------

<!-- <% if param.with_webapp %> -->
Untuk frontend web menggunakan Vue.js, base ada di direktori `/frontends`.

Apabila ingin mencoba menjalankannya bisa check frontend web dengan langkah-langkah berikut:

    $ cd frontends/$name_snake_case$_web
    $ npm install
    $ npm start

Apabila menggunakan Yarn bisa:

    $ cd frontends/$name_snake_case$_web
    $ yarn install
    $ yarn serve

Buka http://localhost:8080/ atau apabila server juga jalan di lokal bisa port-nya berubah menjadi http://localhost:8081/

Untuk Vue.js ada di `/frontends/$name_snake_case$_web`:

    $ cd frontends/$name_snake_case$_web
    $ yarn install
    $ yarn serve
<!-- <% endif %> -->

<!-- <% if param.with_flutter %> -->
Untuk frontend mobile menggunakan Flutter, bisa ditemukan di direktori `/frontends/$name_snake_case$_mobile`.
Contoh cara menjalankan:

    $ cd frontends/$name_snake_case$_mobile
    $ flutter pub get
    $ flutter run

<!-- <% endif %> -->

**CATATAN**: Kamu bisa menggunakan npm maupun yarn, tapi direkomendasikan menggunakan yarn.

Dokumentasi
-------------

Dokumentasi dibagikan menjadi beberapa bagian:

1. Dokumentasi pustaka (library).
2. Dokumentasi Rest API.

Untuk menggenerasikan dokumentasi pustaka cukup ketikkan:

    $ make lib-docs

Untuk menggenerasikan dokumentasi rest API:

    $ make api-docs

**CATATAN**: Penggenerasian dokumentasi untuk rest API membutuhkan tool [Aglio](https://www.npmjs.com/package/aglio).


Konvensi
------------

Setiap perubahan pada project ini harus mengikuti konvensi ini.

* Setiap menambahkan parameter melalui environment variable yang wajib (yang menyebabkan aplikasi error apabila tidak diset) contohnya seperti yang diakses menggunakan macro `env!` maka perlu mengupdate juga file `.env.example` karena sebagai contoh konfigurasi untuk team.

Sebelum melakukan commit harus:

* Memastikan kodenya telah diformat menggunakan perintah: `make fmt`.
* Memastikan kodenya telah layak sesuai standar dengan cara menjalankan perintah: `make lint`.
* Memastikan kodenya telah lolos unittest dengan cara menjalankan perintah: `make test`.
* Memastikan kodenya telah aman dari dependensi yang bermasalah dengan menjalankan perintah: `make audit`.
* Menggunakan tata bahasa yang mudah dipahami dan menjelaskan perubahan mendasar pada commit message-nya.


Troubleshooting
-----------------

*Case*

    $ docker-compose up
    ERROR: Version in "./docker-compose.yml" is unsupported.

Itu artinya versi `docker-compose` yang ada di sistem mu tidak support, maka perlu dilakukan install manual:

*Fix (ubuntu 16.04)* 

    $ sudo curl -L sudo curl -L "https://github.com/docker/compose/releases/download/1.22.0/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
    $ sudo chmod +x /usr/local/bin/docker-compose

*Case*
 
Ketika install diesel muncul error seperti berikut:

    $ cargo install diesel_cli --no-default-features --features postgres 
     Error : cannot find lpg

Itu artinya library postgres-devel belum diinstall, maka perlu intsall dulu libpq:

*Fix (ubuntu 16.04)* 
    sudo apt install libpq-dev   

*Case*

Ketika sedang compile/test gagal dengan error kurang lebih seperti ini:
    
    ERROR $name_snake_case$::api::error] error: "relation \"transactions\" does not exist"

Itu artinya table schema mu yang digunakan untuk test belum up-to-date dengan schema terbaru, maka perlu dilakukan migration untuk apply patch-nya:

    $ diesel migration run --database-url postgresql://localhost/$name_snake_case$_test?sslmode=disable

Atau reset database untuk test-nya agar di-rebuild schema-nya dari pertama:

    $ make test-env


