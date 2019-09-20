extern crate reqwest;
extern crate rand;

use std::env;
use rand::Rng;
use rouille::Request;
use rouille::Response;

type Unwraper<T> = Result<T, Box<std::error::Error>>;

fn main() {
    let args: Vec<String> = env::args().collect();
    let port = if (&args[1]).trim().parse::<f64>().is_ok() {
        &args[1]
    } else {
        "8080"
    }.to_string();
    let address = format!("127.0.0.1:{}",port);
    rouille::start_server(address, move |request| {
            Response::text(joke_or_insult().unwrap_or("".to_string()))
    });
}


fn joke_or_insult() -> Unwraper<String> {
    let mut rng = rand::thread_rng();
    let rdm = rng.gen_range(0, 2);
    let res = if rdm == 0 {
        let client = reqwest::Client::new();
        client.get("https://icanhazdadjoke.com/")
              .header("Accept", "text/plain")
              .send()?
              .text()?
    } else {
        reqwest::get("https://insult.mattbas.org/api/insult")?.text()?
    }
    .to_string();

    Ok(res)
}
