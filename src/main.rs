use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:6432").await?;

    loop {
        let (client_stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            let rv = handle_client(client_stream).await;
            if let Err(e) = rv {
                eprintln!("Error handling client: {}", e);
            }
        });
    }
}

async fn handle_client(client_stream: TcpStream) -> Result<(), Box<dyn std::error::Error>>{
    let backend_stream = TcpStream::connect("127.0.0.1:5432").await?;

    let (mut client_reader, mut client_writer) = client_stream.into_split();
    let (mut backend_reader, mut backend_writer) = backend_stream.into_split();

    let client_to_backend = tokio::io::copy(&mut client_reader, &mut backend_writer);
    let backend_to_client = tokio::io::copy(&mut backend_reader, &mut client_writer);

    tokio::select! {
        result = client_to_backend => {
            if let Err(e) = result {
                eprintln!("Error forwarding client->backend: {}", e);
            }
        }
        result = backend_to_client => {
            if let Err(e) = result {
                eprintln!("Error forwarding backend->client: {}", e);
            }
        }
    }
    
    Ok(())
}