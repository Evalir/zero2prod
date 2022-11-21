use std::net::TcpListener;

use secrecy::ExposeSecret;
use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let config = get_configuration().expect("Could not read configuration");
    let connection = PgPool::connect_lazy(config.database.connection_string().expose_secret())
        .expect("Could not connect to database");

    let address = format!("{}:{}", config.application.host, config.application.port);

    run(TcpListener::bind(address)?, connection)?.await
}
