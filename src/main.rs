use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic if we don't read configuration
    let configuration = get_configuration()
        .expect("Failed to read configuration.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let tcp_listener = TcpListener::bind(address)?;
    run(tcp_listener)?.await
}