use tokio::{io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf}, net::{TcpListener, TcpStream}};
use tracing::{debug, error, info};
use crate::errors::ConnectionPoolerError;

mod errors;
mod postgres;

#[tokio::main]
async fn main() -> Result<(), ConnectionPoolerError> {
    let listener = TcpListener::bind("127.0.0.1:6432").await?;

    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();

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

    let client_to_backend = forward_traffic_to_backend(&mut client_reader, &mut backend_writer);
    let backend_to_client = forward_traffic_to_client(&mut backend_reader, &mut client_writer);

    let (client_result, backend_result) = tokio::join!(client_to_backend, backend_to_client);

    if let Err(e) = client_result {
        error!("Error forwarding client->backend: {}", e);
    }

    if let Err(e) = backend_result {
        error!("Error forwarding backend->client: {}", e);
    }

    Ok(())
}

async fn forward_traffic_to_client(reader: &mut ReadHalf<TcpStream>, writer: &mut WriteHalf<TcpStream>) -> Result<(), ConnectionPoolerError> {
    //tokio::io::copy(reader, writer).await?;
    loop {
        let mut buffer = [0; 1024];
        let n = reader.read(&mut buffer).await?;

        if n == 0 {
            break;
        }

        let wire_messages = postgres::try_parse_wire_messages(&buffer[..n])?;
        for wire_message in wire_messages {
            debug!("Backend sent wire message: {:?}", wire_message);
        }

        writer.write_all(&buffer[..n]).await?;
    }
    Ok(())
}

async fn forward_traffic_to_backend(reader: &mut ReadHalf<TcpStream>, writer: &mut WriteHalf<TcpStream>) -> Result<(), ConnectionPoolerError> {
    //tokio::io::copy(reader, writer).await?;
    loop {
        let mut buffer = [0; 1024];
        let n = reader.read(&mut buffer).await?;

        if n == 0 {
            break;
        }

        let wire_messages = postgres::try_parse_wire_messages(&buffer[..n])?;
        for wire_message in wire_messages {
            debug!("Client sent wire message: {:?}", wire_message);
        }
        
        writer.write_all(&buffer[..n]).await?;
    }
    Ok(())
}