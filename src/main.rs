extern crate discord_rpc_client;

use discord_rpc_client::Client;
use std::{env, fs, process, time};

const CLIENT_ID: u64 = 561241836451004449;

fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Expected a filename to read from.");
        process::exit(1);
    });
    let mut client = Client::new(CLIENT_ID);
    client.start();

    loop {
        let info_bytes = fs::read(&filename).unwrap_or_else(|err| {
            eprintln!("Something went wrong with reading the fifo: {}", err);
            process::exit(1);
        });
        let info = String::from_utf8(info_bytes).unwrap_or_else(|err| {
            eprintln!("Something went wrong with parsing the bytes: {}", err);
            process::exit(1);
        });

        if info.contains("exit") {
            break;
        }
        let now = time::SystemTime::now();
        let epoc_secs = now
            .duration_since(time::UNIX_EPOCH)
            .expect("Epoch is after now?")
            .as_secs();
        client
            .set_activity(|act| {
                act.details(format!("Editing {}", info.replace("'", "")))
                    .timestamps(|timestamp| timestamp.start(epoc_secs))
                    .assets(|ass| ass.large_image("default"))
            })
            .expect("Failed to set activity");
    }
}
