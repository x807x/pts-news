use pts_news::daily_news::get_daily_news;

#[tokio::test]
async fn test_daily_news() {
    let news = get_daily_news().await;
    assert!(news.len() > 0);
}
