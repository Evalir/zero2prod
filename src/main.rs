use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("Could not read configuration");
    let connection = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Could not connect to database");

    let address = format!("127.0.0.1:{}", config.application_port);

    run(TcpListener::bind(address)?, connection)?.await
}
