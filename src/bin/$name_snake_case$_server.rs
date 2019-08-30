
#[macro_use]
extern crate log;
extern crate dotenv;
extern crate env_logger;

use $name_snake_case$::prelude::*;
use $name_snake_case$::service::load_services;

use std::env;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    println!(
        r#"

____________    __      ____________,
\_____     /   /_ \     \     _____/
 \_____    \____/__\____/    _____/
  \_____      |  M  |       _____/
     \________\__|__/_________/
               /___\
            ._//___\\_.
    
             MAINFRAME
    "#
    );

    println!(
        "\n$name$ server {}\n_______________________________________\n{}\ngit: {}\n",
        env!("CARGO_PKG_VERSION"),
        env!("BUILD_INFO"),
        env!("GIT_REV")
    );

    trace!("starting up...");

    let services = load_services();

    let public_listening_address =
        env::var("$name_shout_snake_case$_PUBLIC_LISTENING").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    let private_listening_address =
        env::var("$name_shout_snake_case$_PRIVATE_LISTENING").unwrap_or_else(|_| "127.0.0.1:9090".to_string());

    let config = ServiceApiConfig::new(vec![
        ApiServer::new(ApiAccess::Public, public_listening_address),
        ApiServer::new(ApiAccess::Private, private_listening_address),
    ]);

    api::start(ApiAggregator::new(services), config);
}
