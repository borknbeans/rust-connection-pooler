use tokio::{io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf}, net::{TcpListener, TcpStream}};
use crate::errors::ConnectionPoolerError;

mod errors;

#[tokio::main]
async fn main() -> Result<(), ConnectionPoolerError> {
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

async fn handle_client(client_stream: TcpStream) -> Result<(), ConnectionPoolerError>{
    let backend_stream = TcpStream::connect("127.0.0.1:5432").await?;

    let (mut client_reader, mut client_writer) = tokio::io::split(client_stream);
    let (mut backend_reader, mut backend_writer) = tokio::io::split(backend_stream);

    let client_to_backend = forward_traffic(&mut client_reader, &mut backend_writer);
    let backend_to_client = forward_traffic(&mut backend_reader, &mut client_writer);

    let (client_result, backend_result) = tokio::join!(client_to_backend, backend_to_client);

    if let Err(e) = client_result {
        eprintln!("Error forwarding client->backend: {}", e);
    }

    if let Err(e) = backend_result {
        eprintln!("Error forwarding backend->client: {}", e);
    }

    Ok(())
}

async fn forward_traffic(reader: &mut ReadHalf<TcpStream>, writer: &mut WriteHalf<TcpStream>) -> Result<(), ConnectionPoolerError> {
    //tokio::io::copy(reader, writer).await?;
    loop {
        let mut buffer = [0; 1024];
        let n = reader.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        writer.write_all(&buffer[..n]).await?;
    }
    Ok(())
}