use indicatif_log_bridge::LogWrapper;
use tokio::time::{sleep, Duration};

use indicatif::{MultiProgress, ProgressBar};


#[tokio::main]
async fn main() {
    let logger =
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
            .format_timestamp(None)
            .format_target(false)
            .build();

    let multibar = MultiProgress::new();

    LogWrapper::new(multibar.clone(), logger)
        .try_init()
        .unwrap();

    let bar1 = multibar.add(ProgressBar::new(3));
    let bar2 = multibar.add(ProgressBar::new(3));
    
    for _ in 0..3 {
        bar1.inc(1);
        sleep(Duration::from_millis(1000)).await;
    }
    
    bar1.finish();

    
    for _ in 0..3 {
        bar2.inc(1);
        sleep(Duration::from_millis(1000)).await;
    }


    bar2.finish();
}
