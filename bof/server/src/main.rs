use indoc::indoc;
use lazy_static::lazy_static;
use log::info;
use std::net::Shutdown;
use std::ffi::OsString;
use std::net::SocketAddr;
use std::os::unix::ffi::OsStringExt;
use std::process::Stdio;
use tokio::net::{process::Command, TcpListener, TcpStream};
use tokio::prelude::*;

lazy_static! {
    static ref BINARY: String = {
        let binary = std::include_bytes!("vulnerable_program");

        base64::encode(&binary[..])
    };

    static ref OS_ARCH: OsString = {
        use std::process::Command;

        let res = Command::new("uname")
            .arg("-m")
            .output()
            .unwrap()
            .stdout;

        OsString::from_vec(res)
    };
}

macro_rules! awrite {
    ($dst:expr, $($arg:tt)*) => {{
        let val = format!($($arg)*);
        $dst.write_all(val.as_bytes()).await?;
    }}
}

async fn handle_inner(addr: SocketAddr, mut socket: TcpStream) -> Result<TcpStream, std::io::Error> {
    socket.write_all(b"Welcome to the BOF-o-matic, type help or ? for commands!\n").await?;
    let mut buf = [0u8; 1024];
    
    let n = socket.read(&mut buf).await?;

    info!("[{}] Received message: {}", addr, String::from_utf8_lossy(&buf));

    if n == 0 {
        awrite!(socket, "no");
        return Ok(socket);
    }

    let cmd = buf[..(n - 1)].splitn(2, |&c| c == b' ').collect::<Vec<_>>();

    match &cmd[..] {
        [b"?"] | [b"help"] => {
            socket.write_all(indoc!(b"
                    Commands:
                    ?: Show help.
                    help: Show help.
                    dump: Dump the BINARY you need to exploit.
                    exploit <base64 string>: Try to exploit the program.
                    ")).await?;
        },
        [b"dump"] => {
            socket.write_all(BINARY.as_bytes()).await?;
        },
        [b"exploit", program] => {
            let program = base64::decode(program)
                .or(Err(std::io::Error::from(std::io::ErrorKind::InvalidData)))?;

            let mut child = Command::new("/usr/bin/setarch")
                .stdin(Stdio::piped())
                .stderr(Stdio::piped())
                .stdin(Stdio::piped())
                .arg(&*OS_ARCH)
                .arg("-R")
                .arg("/usr/local/bin/vulnerable_program")
                .env("FLAG", std::env::var("FLAG_0").unwrap())
                .spawn()?;

            child.stdin().as_mut().unwrap().write_all(&program).await?;

            let output = child.wait_with_output()
                              .await?;

            awrite!(socket, "Status code: {}\n", output.status);
            awrite!(socket, "Stdout: {:?}\n", String::from_utf8_lossy(&output.stdout));
            awrite!(socket, "Stderr: {:?}\n", String::from_utf8_lossy(&output.stderr));
        },
        _ => {
            awrite!(socket, "I dunno bruh\n");
        }
    }

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

    let mut listener = TcpListener::bind("0.0.0.0:1234").await?;

    loop {
        let (socket, addr) = listener.accept().await?;

        tokio::spawn(handle(addr, socket));
    }
}
