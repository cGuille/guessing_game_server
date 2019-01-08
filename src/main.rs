extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::thread;

fn main() {
    const SOCKET_PATH: &str = "/tmp/guessing_game_server.sock";

    let socket = Path::new(SOCKET_PATH);

    if socket.exists() {
        fs::remove_file(&socket).unwrap();
    }

    let listener = UnixListener::bind(SOCKET_PATH).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            }
            Err(err) => {
                println!("Connection error: {}", err);
                break;
            }
        }
    }
}

fn handle_client(stream: UnixStream) {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("New game! Secret number is {}", secret_number);

    let mut stream = BufReader::new(stream);
    loop {
        stream
            .get_ref()
            .write(b"Please input your guess.\n")
            .unwrap();

        let mut guess = String::new();
        if stream.read_line(&mut guess).is_err() {
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Read guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => stream.get_ref().write(b"Too small!\n").unwrap(),
            Ordering::Greater => stream.get_ref().write(b"Too big!\n").unwrap(),
            Ordering::Equal => {
                stream.get_ref().write(b"You win!\n").unwrap();
                break;
            }
        };
    }
}
