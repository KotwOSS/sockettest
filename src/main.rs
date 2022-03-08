#![allow(unused_variables)]

use std::path::PathBuf;
use std::str;

use clap::{Parser, Subcommand};

pub mod connect;
pub mod colors;

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
        timeout: u64,
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
            connect::main(remote, timeout).await;
        },
    }
}