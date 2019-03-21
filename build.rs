extern crate chrono;
extern crate protoc_rust;

use chrono::Local;
use protoc_rust::{Args, Customize};
use std::{env, fs, process};

fn main() {
    let out_dir = env::var("OUT_DIR").expect("Cannot get OUT_DIR");

    protoc_rust::run(Args {
        out_dir: &out_dir,
        input: &["src/protos/$name_snake_case$.proto"],
        includes: &["src/protos"],
        customize: Customize {
            serde_derive: Some(true),
            ..Default::default()
        },
    })
    .expect("Protoc compile");

    let path = format!("{}/$name_snake_case$.rs", out_dir);
    let content = fs::read_to_string(&path).expect("cannot read autogen protobuf $name_snake_case$.rs");
    let mut new_content = vec![];

    for line in content.split("\n") {
        if line.starts_with("#![") {
            continue;
        }
        new_content.push(line);
    }

    let new_content: String = new_content.join("\n");

    let _ = fs::write(&path, new_content);

    let output = process::Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .expect("Cannot get git_rev");

    let git_rev = String::from_utf8_lossy(&output.stdout);
    let git_rev = git_rev.trim();

    println!("cargo:rerun-if-changed={}", "src/protos/$name_snake_case$.proto");
    println!("cargo:rustc-env=GIT_REV={}", git_rev);

    if env::var("BUILD_FOR") == Ok("nightly".to_string()) {
        println!(
            "cargo:rustc-env=BUILD_INFO=ngihtly build {} @ {}",
            env::var("TARGET").unwrap(),
            Local::now()
        );
    } else {
        println!(
            "cargo:rustc-env=BUILD_INFO={} build {} @ {}",
            env::var("PROFILE").unwrap(),
            env::var("TARGET").unwrap(),
            Local::now()
        );
    }
}
