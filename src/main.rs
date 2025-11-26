use tokio::net::{TcpListener, TcpStream};

mod postgres;

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
    let mut pg_connection = postgres::connect_to_postgres().await?;

    let (mut client_reader, mut client_writer) = tokio::io::split(client_stream);
    let (mut backend_reader, mut backend_writer) = tokio::io::split(pg_connection.stream);

    let client_to_backend = tokio::io::copy(&mut client_reader, &mut backend_writer);
    let backend_to_client = tokio::io::copy(&mut backend_reader, &mut client_writer);

    let (client_result, backend_result) = tokio::join!(client_to_backend, backend_to_client);

    if let Err(e) = client_result {
        eprintln!("Error forwarding client->backend: {}", e);
    }

    if let Err(e) = backend_result {
        eprintln!("Error forwarding backend->client: {}", e);
        pg_connection.state = postgres::ConnectionState::Broken;
    }

    pg_connection.stream = backend_reader.unsplit(backend_writer);
    postgres::release_connection(pg_connection).await;

    Ok(())
}