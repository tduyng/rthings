use bytes::BytesMut;
use mini_http::response::Response;
use tokio::{io::AsyncWriteExt, net::TcpListener};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:4040").await?;

    println!("Server is running on port 4040");

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let response = Response::new()
                .status_code(200, "OK")
                .header("Content-Type", "text/plain")
                .body_str("Hello, World!");

            let mut buf = BytesMut::new();
            response.encode(&mut buf).unwrap();

            if let Err(e) = socket.write_all(&buf).await {
                eprintln!("Failed to write to socket: {}", e);
            }
        });
    }
}
