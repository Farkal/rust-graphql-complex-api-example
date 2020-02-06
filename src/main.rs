use env_logger;
use std::env;

fn main() {
    if env::var("RUST_LOG").is_err() {
        std::env::set_var(
            "RUST_LOG",
            "actix_web=info,rust_graphql_complex_api_example=info",
        );
    }
    env_logger::init();
    if let Err(e) = rust_graphql_complex_api_example::run(config) {
        error!("Application error: {}", e);
        process::exit(1);
    };
}
