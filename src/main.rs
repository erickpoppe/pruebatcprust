use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Servidor arrancado. Escuchando en 127.0.0.1:8080");

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            println!("Se conecto un nuevo cliente.");

            let mut buf = [0; 1024];
            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => {
                        println!("Cliente deconectado.");
                        return;
                    }
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Error leyendo del socket: {}", e);
                        return;
                    }
                };

                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("Error escribiendo al socket: {}", e);
                    return;
                }
            }
        });
    }
}