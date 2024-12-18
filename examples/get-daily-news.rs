use log::{info, LevelFilter};

#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .filter(None, LevelFilter::Info)
        .init();
    info!("teset");
    let news = pts_news::daily_news::get_daily_news().await;
    info!("{:#?}", news);
}
