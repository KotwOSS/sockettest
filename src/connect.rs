use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use tokio::io;

use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

use std::process::exit;
use std::str;

use crate::colors;

async fn send_loop(mut write_stream: OwnedWriteHalf) {
    loop {
        let mut buf = Vec::new();

        io::stdin().read_buf(&mut buf).await.expect("Input error:");

        // Remove the aditional \n
        buf.pop();

        let amount = buf.len();

        let line = str::from_utf8(&buf).expect("Encoding error:");

        println!(
            "{}{}\r{}SEND      {}{}{}'{}'{}", 
            colors::CURSOR_UP, 
            colors::CLEAR_LINE,
            colors::YELLOW,
            colors::LIGHT_ORANGE,
            amount, 
            colors::ORANGE,
            line,
            colors::RESET
        );
        write_stream.write(&buf).await.expect("Socket write error:");
    };
} 

async fn receive_loop(mut read_stream: OwnedReadHalf) {
    loop {
        let mut buf = [0; 1024];
        let amount = match read_stream.read(&mut buf).await {
            Ok(n) if n == 0 => return,
            Ok(n) => n,
            Err(e) => {
                eprintln!("Failed to read from socket; err = {:?}", e);
                return;
            }
        };

        let line =  str::from_utf8(&buf).expect("Invalid UTF-8 sequence:");

        println!(
            "{}RECEIVE   {}{}{}'{}'{}",
            colors::BLUE,
            colors::LIGHT_ORANGE,
            amount, 
            colors::ORANGE,
            line,
            colors::RESET
        );
    };
}

async fn connect(remote: &String) -> TcpStream {
    match TcpStream::connect(remote).await {
        Err(e)=>{
            println!(
                "{}ERROR{} while connecting to socket {}'{}'{}: {}{}",
                colors::RED,
                colors::LIGHT_RED,
                colors::ORANGE, 
                remote,
                colors::LIGHT_RED,
                colors::RED,
                e
            );
            exit(1);
        },
        Ok(r)=>{r}
    }
}

pub async fn main(remote: &String, timeout: &u64) {
    let stream = match tokio::time::timeout(std::time::Duration::from_millis(*timeout), connect(remote)).await {
        Err(_e) => { 
            println!(
                "{}ERROR{} while connecting to socket {}'{}'{}: {}Timeout after {}ms!",
                colors::RED,
                colors::LIGHT_RED,
                colors::ORANGE, 
                remote,
                colors::LIGHT_RED,
                colors::RED,
                timeout
            );
            exit(1);
        },
        Ok(r) => { r }
    };

    let (read_stream, write_stream) = stream.into_split();

    println!(
        "{}CONNECTED {}'{}'{}", 
        colors::GREEN, 
        colors::ORANGE, 
        remote, 
        colors::RESET
    );

    tokio::join!(send_loop(write_stream), receive_loop(read_stream));
}