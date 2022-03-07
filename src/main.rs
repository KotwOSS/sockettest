use std::path::PathBuf;
use std::str;

use clap::{Parser, Subcommand};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use tokio::io;

#[derive(Parser)]
#[clap(name = "sockettest", about = "A socket tester for udp and tcp.", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Sends packets to a socket.
    #[clap(arg_required_else_help = true)]
    Send {
        /// The type (udp/tcp)
        socket_type: String,

        /// The remote in format host:port
        remote: String,

        /// The data in form of a string
        #[clap(long, short, required = false)]
        data: String,

        /// The file which will be sent
        #[clap(long, short, required = false)]
        file: PathBuf,

        /// Amount of packets that will be sent
        #[clap(long, short, required = false)]
        amount: u16
    },

    /// Connects to a socket (only tcp).
    #[clap(arg_required_else_help = true)]
    Connect {
        /// The remote in format host:port
        remote: String,

        /// The max time for connecting until it will timeout.
        #[clap(long, short, default_value_t = 0, required = false)]
        timeout: u8,
    },

    /// Sends packets to a socket and waits until it gets a response
    #[clap(arg_required_else_help = true)]
    Wait {
        /// The type (udp/tcp)
        socket_type: String,

        /// The remote in format host:port
        remote: String,

        /// The data in form of a string
        #[clap(long, short, required = false)]
        data: String,

        /// The file which will be send
        #[clap(long, short, required = false)]
        file: PathBuf,

        /// The max time for waiting until it will timeout.
        #[clap(long, short, required = false)]
        timeout: u8,

        /// If true, only the response text will be printed.
        #[clap(long, short)]
        silent: bool,

        /// Amount of packets that will be sent
        #[clap(long, short, required = false)]
        amount: u16
    },
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Send { socket_type, remote, data, file, amount } => {
            
        },
        Commands::Wait { socket_type, remote, data, file, amount, timeout, silent } => {

        },
        Commands::Connect { remote, timeout } => {
            let stream = TcpStream::connect(remote).await
                .expect("Couldn't connect to socket!");

            let (mut read_stream, mut write_stream) = stream.into_split();

            println!("Successfully connected to socket {}!", remote);

            tokio::join!(
                async move {
                    loop {
                        let mut buf = [0; 1024];
                        let amount = io::stdin().read(&mut buf).await.expect("Input error:")-1;

                        let mut nbuf = Vec::with_capacity(amount);

                        for i in 0..amount {
                            nbuf.push(buf[i]);
                        };
                        
                        let line =  str::from_utf8(&nbuf).expect("Invalid UTF-8 sequence:");

                        println!("\x1B[1A\x1B[KSENDING: {}", line);
                        write_stream.write(&nbuf).await.expect("Socket write error:");
                    };
                },
                async move {
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
        
                        println!("RECEIVED: {} | {}", amount, line);
                    };
                }
            );
        },
    }
}