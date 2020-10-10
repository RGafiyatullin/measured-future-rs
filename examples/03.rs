use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use ::futures::channel::mpsc;
use ::futures::channel::oneshot;
use ::futures::prelude::*;

use ::measured_future_rs::prelude::*;
use ::measured_future_rs::report::Report;

#[tokio::main]
async fn main() {
    println!("num_cpus: {}", ::num_cpus::get());

    let iterations = 16 * 1024_00;
    let channels_count = 64;

    for producers_count in &[/*1, 2, 4, 8, 16, 32, */ 64] {
        run(channels_count, *producers_count, iterations).await;
    }
}

async fn run(channels_count: usize, producers_count: usize, iterations: usize) {
    let (report_tx, mut report_rx) = mpsc::channel(256);
    let report_aggregator_running = async move {
        let mut report = Report::empty();

        while let Some(r) = report_rx.next().await {
            report = report.merge(r);
        }

        println!("{:#?}", report);
    };
    let report_aggregator_done = ::tokio::spawn(report_aggregator_running);

    // ================

    let iterations_per_worker = iterations / producers_count;

    let (_join_producer_times, join_consumer_time) = {
        let (txs, rxs) = (0..channels_count)
            .map(|_| mpsc::channel::<Message>(0))
            .unzip();
        let tx_roulette = TxRoulette::new(txs);

        let producers_running = run_producers(producers_count, tx_roulette, iterations_per_worker);
        let consumer_running = time_it(run_consumer(rxs).measured("consumer"));

        let (producer_times, (consumer_time, ())) =
            future::join(producers_running, consumer_running)
                .measured("join")
                .report(report_tx.clone())
                .with_flush_interval(Duration::from_millis(100))
                .await;
        (producer_times, consumer_time)
    };

    let join_consumer_rate =
        iterations as f64 / join_consumer_time.as_nanos() as f64 * 1_000_000_000 as f64;

    // ================

    let (_spawn_producer_times, spawn_consumer_time) = {
        let (txs, rxs) = (0..channels_count)
            .map(|_| mpsc::channel::<Message>(0))
            .unzip();

        let tx_roulette = TxRoulette::new(txs);

        let producer_running = ::tokio::spawn(
            run_producers(producers_count, tx_roulette, iterations_per_worker)
                .measured("spawn")
                .report(report_tx.clone()),
        );
        let consumer_running = ::tokio::spawn(time_it(
            run_consumer(rxs)
                .measured("consumer")
                .measured("spawn")
                .report(report_tx.clone()),
        ));

        let (producer_result, consumer_result) =
            future::join(producer_running, consumer_running).await;

        let (producer_times, (consumer_time, ())) =
            (producer_result.unwrap(), consumer_result.unwrap());
        (producer_times, consumer_time)
    };

    let spawn_consumer_rate =
        iterations as f64 / spawn_consumer_time.as_nanos() as f64 * 1_000_000_000 as f64;

    println!("PRODUCER-CONCURRENCY: {}", producers_count);
    println!(" JOIN");
    // println!("  producer: {:?}", join_producer_times);
    println!("  consumer: {:?}", join_consumer_time);
    println!("   rate: {:.2}hz", join_consumer_rate);
    println!(" SPAWN");
    // println!("  producer: {:?}", spawn_producer_times);
    println!("  consumer: {:?}", spawn_consumer_time);
    println!("   rate: {:.2}hz", spawn_consumer_rate);
    println!("");

    let () = std::mem::drop(report_tx);

    let _ = report_aggregator_done.await;
}

struct Reply {
    id: usize,
    arc: Arc<()>,
    ballast: [[u8; 16]; 0],
}

struct Message {
    id: usize,
    reply_tx: oneshot::Sender<Reply>,
}

#[derive(Debug)]
struct TxRoulette<T> {
    txs: Vec<mpsc::Sender<T>>,
    counter: usize,
}
impl<T> Clone for TxRoulette<T> {
    fn clone(&self) -> Self {
        let counter = (self.counter + 1) % self.txs.len();
        Self {
            txs: self.txs.clone(),
            counter,
        }
    }
}
impl<T> TxRoulette<T> {
    fn new(txs: Vec<mpsc::Sender<T>>) -> Self {
        Self { txs, counter: 0 }
    }

    async fn send(&mut self, item: T) -> Result<(), mpsc::SendError> {
        self.counter = (self.counter + 1) % self.txs.len();
        self.txs[self.counter].send(item).await
    }
}

async fn run_producers(
    producers_count: usize,
    tx: TxRoulette<Message>,
    iterations: usize,
) -> Vec<Duration> {
    future::join_all(
        (0..producers_count)
            .into_iter()
            .map(|_| time_it(run_producer(tx.clone(), iterations).measured("producer"))),
    )
    .measured("producers")
    .await
    .into_iter()
    .map(|(t, _)| t)
    .collect()
}

fn create_message(id: usize) -> (Message, oneshot::Receiver<Reply>) {
    let (reply_tx, reply_rx) = oneshot::channel();
    (Message { id, reply_tx }, reply_rx)
}

async fn run_producer(mut tx: TxRoulette<Message>, iterations: usize) {
    for i in 0..iterations {
        let (message, result) = create_message(i);
        let () = tx.send(message).measured("send").await.unwrap();
        assert_eq!(result.measured("recv").await.unwrap().id, i);
    }
}

async fn run_consumer(rxs: Vec<mpsc::Receiver<Message>>) {
    let arc = Arc::new(());
    let mut rx = stream::select_all(rxs);
    while let Some(message) = rx.next().measured("fetch").await {
        let _ = message.reply_tx.send(Reply {
            id: message.id,
            arc: arc.clone(),
            ballast: Default::default(),
        });
    }
}

async fn time_it<F: Future>(f: F) -> (Duration, F::Output) {
    let start = Instant::now();
    let output = f.await;
    (start.elapsed(), output)
}
