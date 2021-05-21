use std::time::Duration;

use ::measured_future_rs::meters::default_meter::DefaultMeter;
use ::measured_future_rs::meters::default_meter::DefaultMeterReport;
use ::measured_future_rs::prelude::*;
use ::measured_future_rs::report_sinks::DefaultSink;

#[tokio::test]
async fn should_compile() {
    let _ = ::pretty_env_logger::formatted_timed_builder()
        .filter_level("TRACE".parse().unwrap())
        .try_init();

    let () = DefaultSink::install();

    let future = async {
        println!("actually do something...");
        let () = ::tokio::time::sleep(Duration::from_secs(1))
            .measure_with(DefaultMeter::new("sleep-1"))
            .await;

        let () = ::tokio::time::sleep(Duration::from_secs(1))
            .measure_with(DefaultMeter::new("sleep-2"))
            .await;

        println!("actually done something!");
    };
    let measured = future.measure_with(DefaultMeter::new("future"));
    let reporing = measured.report_to_current::<DefaultMeterReport>();

    let () = reporing.await;

    let () = ::tokio::time::sleep(Duration::from_secs(5)).await;
}
