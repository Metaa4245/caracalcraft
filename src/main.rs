#![deny(clippy::cargo)]
#![deny(clippy::complexity)]
#![deny(clippy::correctness)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]
#![allow(clippy::no_effect_underscore_binding)]
#![allow(clippy::module_name_repetitions)]

mod block;
mod entity;
mod item;
mod map;
mod network;

use network::stream_handler::handle_stream;
use tokio::net::TcpListener;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:25566";
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (mut stream, _) = listener.accept().await?;

        tokio::spawn(async move {
            handle_stream(&mut stream).await.unwrap();
        });
    }
}
