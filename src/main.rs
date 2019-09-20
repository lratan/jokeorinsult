extern crate reqwest;
extern crate rand;

use std::env;
use rand::Rng;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Write;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    let port = if (&args[1]).trim().parse::<f64>().is_ok() {
        &args[1]
    } else {
        "8080"
    }.to_string();
    let address = format!("127.0.0.1:{}",port);
    let listener = match TcpListener::bind(address) {
        Ok(u) => u,
        _ => {
            println!("Unable to open the connection on port {}, abort.", port);
            return;
        }
    };

    println!("Web server listening on :{}", port);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let contents = joke_or_insult();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}\n", contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn joke_or_insult() -> std::string::String {
    let mut rng = rand::thread_rng();
    let rdm = rng.gen_range(0, 2);
    println!("Random number : {}", rdm);
    let res = if rdm == 0 {
        let client = reqwest::Client::new();
        let body = client.get("https://icanhazdadjoke.com/")
                .header("Accept", "text/plain")
                    .send().unwrap().text().unwrap();
        body
    } else {
        let body = reqwest::get("https://insult.mattbas.org/api/insult").unwrap().text().unwrap();
        body
    }.to_string();
    
    return res;
}
