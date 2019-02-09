use colored::*;

use std::{
    env::args,
    io::{
        prelude::*,
        stdin,
    },
    net::TcpStream,
};

fn read_input() -> String {
    let mut input = String::new();
    if let Err(error) = stdin().read_line(&mut input) {
        println!("{:?} {:?}", "Failed to read input.".bright_red(), error)
    }
    input
}

fn send_msg(msg: String, address: &str) -> String {
    match TcpStream::connect(address) {
        Ok(mut stream) => {
            let _ = stream.write(msg.as_bytes());
            let mut response = [0; 512];
            let _ = stream.read(&mut response);
            let trimmed: Vec<u8> = response.iter().take_while(|n| *n > &0u8).map(|x| *x).collect();
            String::from_utf8(trimmed).unwrap()
        }
        Err(err) => format!("Could not connect. {}", err)
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    match args.get(1) {
        Some(address) => {
            loop {
                println!(
                    "{}{}{}{}{}{}",
                    "a".green(), "s".yellow(), "k".blue(), "k".magenta(), "k".cyan(), ":"
                );

                println!(
                    "{}", send_msg(read_input(), address).cyan()
                )
            }
        },
        _ => println!("{}", "Can't connect without an address.".bright_red())
    }
}