use std::net::TcpListener;

use newsletter_server::configuration::get_configuration;
use newsletter_server::startup::run;
use newsletter_server::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber(
        "newsletter-server".into(),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    let configuration =
        get_configuration().expect("Failed to read configuration.");
    let connection = PgPool::connect(
        configuration.database.connection_string().expose_secret(),
    )
    .await
    .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection)?.await
}
