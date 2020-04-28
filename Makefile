
PROJ_DIR=$(shell pwd)

VERSION=$(shell cat VERSION)
MOBILE_VERSION=$(shell grep 'version:' frontends/$name_snake_case$_mobile/pubspec.yaml | cut -d ' ' -f2 | cut -d '+' -f1)
PUBLIC_API_DOC_OUTPUT=$(PROJ_DIR)/target/api-docs/public-api.html
PRIVATE_API_DOC_OUTPUT=$(PROJ_DIR)/target/api-docs/private-api.html
LIBRARY_DOC_OUTPUT=$(PROJ_DIR)/target/doc/$name_snake_case$/index.html
USER?=postgres
DATABASE_TEST_USER?=$(USER)
DATABASE_TEST_URL?=postgresql://$(DATABASE_TEST_USER)@localhost/$name_snake_case$_test

docs: prepare lib-docs api-docs

prepare:
	@@mkdir -p target/api-docs

lib-docs:
	@@echo generating library documentation...
	@@cargo doc --package $name_snake_case$ --no-deps --lib
	@@echo generated: $(LIBRARY_DOC_OUTPUT)

api-docs: prepare api-docs/public-api.md
	@@echo generating API documentation...
	@@python $(PROJ_DIR)/etc/script/gen_api_docs.py
	@@cd api-docs && aglio -i public-api.md -o $(PUBLIC_API_DOC_OUTPUT)
	@@cd api-docs && aglio -i private-api.md -o $(PRIVATE_API_DOC_OUTPUT)
	@@echo generated: $(PUBLIC_API_DOC_OUTPUT)
	@@echo generated: $(PRIVATE_API_DOC_OUTPUT)

fmt:
	cd testkit && cargo fmt
	cd macros/$name_snake_case$_proc_macro && cargo fmt
	cargo fmt

test:
	@@echo Testing...
	@@DATABASE_URL=$(DATABASE_TEST_URL) cargo test

test-dev:
	@@echo Testing...
	@@DATABASE_URL=$(DATABASE_TEST_URL) cargo test -- --nocapture

lint:
	@@echo Linting...
	@@cargo clippy

audit:
	@@echo Auditing...
	@@cargo audit

commit:
	@@echo Committing...
	@@make fmt
	@@cargo check
	@@git ci -a

version:
	@@sed -i.bak 's/version = ".*" # auto generated do not edit by hand/version = "$(VERSION)" # auto generated do not edit by hand/' Cargo.toml
	@@cat Cargo.toml | grep version | grep "edit by"

release:
	@@echo Build release mode...
	@@cargo build --release

release-linux:
	@@echo ""
	@@echo Ini akan melakukan build menggunakan Docker, 
	@@echo nantinya output binary bisa ditemukan di target/x86_64-unknown-linux-musl/release
	@@echo Building for musl Linux...
	@@docker run -it --rm -v $(PROJ_DIR):/workdir \
					-v /tmp:/root/.cargo/git \
					-v /tmp:/root/.cargo/registry \
					anvie/rust-musl-build:latest \
					cargo build --release --target=x86_64-unknown-linux-musl

<!-- <% if param.with_webapp %> -->
build-web-frontend:
	@@echo Building web frontend...
	cd frontends/$name_snake_case$_web && \
		sed -i .bak s/'dev'/'prod'/ .env && \
		yarn run build && \
		sed -i .bak s/'prod'/'dev'/ .env
	@@echo Web frontend built.
<!-- <% endif %> -->

<!-- <% if param.with_flutter %> -->
BUILD_APK_OUTPUT=$(PROJ_DIR)/frontends/$name_snake_case$_mobile/build/app/outputs/apk/release/$name_snake_case$-$(MOBILE_VERSION).apk

build-apk:
	@@echo Building $name_snake_case$ $(MOBILE_VERSION) for Android ...
	cd frontends/$name_snake_case$_mobile && \
		flutter build apk --build-name=$(MOBILE_VERSION)
	mv $(PROJ_DIR)/frontends/$name_snake_case$_mobile/build/app/outputs/apk/release/app-release.apk $(BUILD_APK_OUTPUT)
	@@echo Done : $(BUILD_APK_OUTPUT)
<!-- <% endif %> -->

test-env:
	diesel database reset --database-url $(DATABASE_TEST_URL)
	diesel migration run --database-url $(DATABASE_TEST_URL)

test-env-redo:
	diesel migration redo --database-url $(DATABASE_TEST_URL)

reset-db:
	diesel database reset
	diesel migration run

.PHONY: prepare docs lib-docs api-docs fmt \
		test test-dev lint audit commit \
		release test-env test-env-redo release-linux \
		reset-db \
		version

