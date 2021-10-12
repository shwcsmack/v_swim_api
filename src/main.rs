use std::net::TcpListener;
use v_swim_api::configuration::get_configuration;
use v_swim_api::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).unwrap();
    run(listener)?.await
}
