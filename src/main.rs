mod network;

use tokio::net::TcpListener;
use tracing::info;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:25566";
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on {addr}");

    loop {
        let (mut stream, _) = listener.accept().await?;
        info!("Accepted stream");

        tokio::spawn(async move {
            network::stream_handler::handle_stream(&mut stream)
                .await
                .unwrap();
        });
    }
}
