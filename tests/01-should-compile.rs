use std::time::Duration;

use ::futures::prelude::*;
use ::measured_future_rs::prelude::*;
use ::measured_future_rs::report_sinks::DefaultSink;

#[tokio::test]
async fn should_compile() {
    let _ = ::pretty_env_logger::formatted_timed_builder()
        .filter_level("TRACE".parse().unwrap())
        .try_init();

    let () = DefaultSink::install(
        sink::unfold((), |(), report| async move {
            println!("{:#?}", report);
            Ok(())
        }),
        |_: &std::convert::Infallible| true,
    );

    let future = async {
        println!("actually do something...");
        let () = ::tokio::time::sleep(Duration::from_secs(1))
            .measure_with("sleep-1")
            .await;

        let () = ::tokio::time::sleep(Duration::from_secs(1))
            .measure_with("sleep-2")
            .await;

        println!("actually done something!");
    };
    let measured = future.measure_with("future");
    let reporing = measured.report_to_current();

    let () = reporing.await;

    let () = ::tokio::time::sleep(Duration::from_secs(5)).await;
}
