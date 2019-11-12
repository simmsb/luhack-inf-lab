use log::info;
use std::net::Shutdown;
use std::ffi::OsStr;
use std::net::SocketAddr;
use std::os::unix::ffi::OsStrExt;
use tokio::net::{process::Command, TcpListener, TcpStream};
use tokio::prelude::*;

macro_rules! awrite {
    ($dst:expr, $($arg:tt)*) => {{
        let val = format!($($arg)*);
        $dst.write_all(val.as_bytes()).await?;
    }}
}

async fn handle_inner(addr: SocketAddr, mut socket: TcpStream) -> Result<TcpStream, std::io::Error> {
    let mut buf = [0u8; 1024];

    let n = socket.read(&mut buf).await?;

    info!(
        "[{}] Received message: {}",
        addr,
        String::from_utf8_lossy(&buf)
    );

    if n == 0 {
        return Ok(socket);
    }

    let output = Command::new("/usr/bin/python3")
        .arg("-c")
        .arg(OsStr::from_bytes(&buf[..(n - 1)]))
        .output()
        .await?;

    awrite!(socket, "Status code: {}\n", output.status);
    awrite!(socket, "Stdout: {:?}\n", String::from_utf8_lossy(&output.stdout));
    awrite!(socket, "Stderr: {:?}\n", String::from_utf8_lossy(&output.stderr));

    Ok(socket)
}

async fn handle(addr: SocketAddr, socket: TcpStream) {
    info!("[{}] User connected", addr);
    match handle_inner(addr, socket).await {
        Ok(socket) => {
            let _ = socket.shutdown(Shutdown::Write);
        },
        Err(e) => {
            info!("[{}] Returned: {:?}", addr, e);
        }
    }
    info!("[{}] User disconnected", addr);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init().unwrap();

    let mut listener = TcpListener::bind("0.0.0.0:4321").await?;

    loop {
        let (socket, addr) = listener.accept().await?;

        tokio::spawn(handle(addr, socket));
    }
}
