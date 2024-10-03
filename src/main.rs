mod entity;
mod item;
mod map;
mod network;
mod player;

use tokio::net::TcpListener;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:25566";
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (mut stream, _) = listener.accept().await?;

        tokio::spawn(async move {
            network::stream_handler::handle_stream(&mut stream)
                .await
                .unwrap();
        });
    }
}
