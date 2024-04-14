use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:4040").await?;

    println!("Server is running on port 4040");

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            println!("Accepted new connection");

            let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 13\r\n\r\nHello, World!";
            println!("Sending response: {}", response);

            if let Err(e) = socket.write_all(response.as_bytes()).await {
                eprintln!("Failed to send response: {}", e);
            }
        });
    }
}
