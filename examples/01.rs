
use std::time::Duration;

use ::measured_future_rs::prelude::*;

#[tokio::main]
async fn main() {
    println!("Hai!");

    let () = run().measured("run").await;
}

async fn run() -> () {
    loop {
        let () = ::tokio::time::delay_for(Duration::from_millis(1000)).await;
        println!("run.iteration");
    }
}
