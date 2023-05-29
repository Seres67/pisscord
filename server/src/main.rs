use std::net::SocketAddr;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3001").await?;
    let (tx, _rx) = broadcast::channel::<(SocketAddr, String)>(100);

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("Listening on {}", addr);

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);

            loop {
                let mut buffer = String::new();
                tokio::select! {
                    // Reads from Channel, Sends to Socket
                    msg = rx.recv() => {
                        let (other_addr, msg) = msg.unwrap();
                        println!("{other_addr} {addr}");
                        if other_addr != addr {
                            writer.write_all(format!("{}: {}\r\n", other_addr, msg).as_bytes()).await.unwrap();
                        }
                    }
                    // Reads from Socket, sends to Channel
                    result = reader.read_line(&mut buffer) => {
                        println!("reader {buffer}");
                        if result.is_err() || buffer.trim() == "exit" {
                            println!("Disconnected, {}", addr);
                            break;
                        }
                        tx.send((addr, buffer)).unwrap();
                    }
                }
            }
        });
    }
}

