use redis_async::{client, resp::FromResp};
use futures::StreamExt;
use std::env;
use tokio::task;
use std::process::Command;

fn monitor(message : String) {
    task::spawn(async move {
        if message.parse::<i32>().unwrap() < 3 { //hardcoded for demo; we can set threshold separately
            Command::new("cmd")
            .args(&["/C","start cmd /K dotnet run --project projectpathaddhere --configuration Release"]) //hardcoded for demo; we can place this in separate file.
            .status()
            .expect("failed to execute process");   
        }
    });
}


#[tokio::main]
async fn main() {
    let addr = env::args().nth(2).unwrap_or_else(|| "127.0.0.1:6379".to_string()).parse().unwrap(); //hardcoded for demo; we can place this in separate file.
    let conn = client::pubsub_connect(addr).await.unwrap();

    let mut events = conn.psubscribe("workers").await.unwrap(); // hardcoded for demo; we can place this in separate file.

    while let Some(event) = events.next().await {
        match event {
            Ok(event) => monitor(String::from_resp(event).unwrap()),
            Err(e) => {
                eprintln!("ERROR: {}", e);
                break;
            }
        }
    }
}