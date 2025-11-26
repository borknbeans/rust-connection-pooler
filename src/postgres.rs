use tokio::net::TcpStream;
use std::sync::Mutex;

pub(crate) static CONNECTION_POOL: Mutex<Vec<PostgresConnection>> = Mutex::new(Vec::new());


#[derive(PartialEq)]
pub(crate) enum ConnectionState {
    Idle,
    Active,
    Broken,
}

pub(crate) struct PostgresConnection {
    pub(crate) stream: TcpStream,
    pub(crate) state: ConnectionState,
}

pub(crate) async fn connect_to_postgres() -> Result<PostgresConnection, Box<dyn std::error::Error>> {
    let connection = {
        let mut pool = CONNECTION_POOL.lock().unwrap();
        if let Some(idx) = pool.iter().position(|c| c.state == ConnectionState::Idle) {
            let mut connection = pool.remove(idx);
            connection.state = ConnectionState::Active;
            Some(connection)
        } else {
            None
        }
    };
    
    if let Some(conn) = connection {
        println!("Reusing connection, state");
        Ok(conn)
    } else {
        println!("Creating new connection");
        let stream = TcpStream::connect("127.0.0.1:5432").await?;
        Ok(PostgresConnection {
            stream,
            state: ConnectionState::Active,
        })
    }
}

pub(crate) async fn release_connection(mut connection: PostgresConnection) {
    println!("Releasing connection");
    let mut pool = CONNECTION_POOL.lock().unwrap();
    connection.state = ConnectionState::Idle;
    pool.push(connection);
}