extern crate reqwest;
extern crate rand;

use std::env;
use rand::Rng;
use rouille::Request;
use rouille::Response;

fn main() {
    let args: Vec<String> = env::args().collect();
    let port = if (&args[1]).trim().parse::<f64>().is_ok() {
        &args[1]
    } else {
        "8080"
    }.to_string();
    let address = format!("127.0.0.1:{}",port);
    rouille::start_server(address, move |request| {
            Response::text(joke_or_insult())
    });
}


fn joke_or_insult() -> std::string::String {
    let mut rng = rand::thread_rng();
    let rdm = rng.gen_range(0, 2);
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
