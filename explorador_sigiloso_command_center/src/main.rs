use command_center::CommandCenter;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let mut center = CommandCenter::default();

    if let Err(e) = center.launch_all().await {
        eprintln!("🔥 CommandCenter failed: {:#}", e);
    }
}
