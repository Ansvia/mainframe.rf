#!/usr/bin/env bash
set -e

echo "Compiling $name$..."

if [ "$1" = "compile" ]; then

        pushd ..

        docker run -it --rm -v $(pwd):/workdir \
                -v /tmp:/root/.cargo/git \
                -v /tmp:/root/.cargo/registry \
                anvie/rust-musl-build:latest \
                cargo build --release --target=x86_64-unknown-linux-musl

        cp target/x86_64-unknown-linux-musl/release/$name_snake_case$_server docker

        popd

else
        echo "Downloading..."
        wget -O ./$name_snake_case$_server "http://178.128.219.222/linux-x86_64-musl/$name_snake_case$_server-nightly-latest?"$(date +"%s")
fi

echo "Build SQL init script..."
# Build init.sql 
python build_init_sql.py

echo "Build docker image..."
docker build . -t $name_snake_case$_server

echo ""
echo ""
echo "Sekarang kamu bisa menjalankan docker compose:"
echo ""
echo "   $ docker-compose up"
echo ""
