use sqlx::postgres::PgPool;
use std::net::TcpListener;
use v_swim_api::configuration::get_configuration;
use v_swim_api::startup::run;
use v_swim_api::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("v_swim".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).unwrap();
    run(listener, connection_pool)?.await
}
