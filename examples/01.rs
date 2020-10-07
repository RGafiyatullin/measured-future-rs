use std::time::Duration;
use std::time::Instant;

use ::measured_future_rs::prelude::*;

#[tokio::main]
async fn main() {
    println!("Hai!");

    let () = run()
        .measured("run")
        .report(::measured_future_rs::DumpToStdout)
        .with_flush_interval(Duration::from_secs(3))
        .await;
}

async fn run() -> () {
    let started_at = Instant::now();
    loop {
        let () = ::tokio::time::delay_for(Duration::from_millis(100)).await;
        doit().measured("a").await;
        doit().measured("b").await;
        quick().measured("quick").await;

        if started_at.elapsed().as_secs() > 7 {
            println!("Ciao!");
            return ();
        }
    }
}

async fn doit() -> () {
    doit_1().measured("1").await;
    let () = std::thread::sleep(Duration::from_millis(10));
    doit_2().measured("2").await;
}

async fn doit_1() -> () {
    let () = std::thread::sleep(Duration::from_millis(10));
}

async fn doit_2() -> () {
    let () = std::thread::sleep(Duration::from_millis(20));
}

async fn quick() -> () {
    quick_1().measured("1").await;
    quick_2().measured("2").await;
}

async fn quick_1() -> () {}

async fn quick_2() -> () {}
