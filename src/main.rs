use email_newsletter::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};
use sqlx::Pool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("email_newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");

    let connection_pool = Pool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
