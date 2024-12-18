use log::debug;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use scraper::{ElementRef, Html, Selector};

use crate::PTS_NEWS_URL;
#[derive(Debug)]
/// Represent an article
pub struct DailyNews {
    pub title: String,
    pub id: u32,
    pub link: String,
    pub cover_img_link: String,
    pub time: String,
    pub category: NewsCategory,
    pub hashtags: Vec<NewsHashTag>,
}

impl<'a> From<ElementRef<'a>> for DailyNews {
    /// Converts an HTML element reference to a `DailyNews` instance.
    fn from(element_ref: ElementRef<'a>) -> Self {
        debug!("DailyNews from ElementRef: {}", element_ref.html());
        let title_selector = Selector::parse(r#"h2"#).unwrap();
        let title = match element_ref.select(&title_selector).next() {
            Some(title) => title.attr("title").unwrap(),
            None => {
                let title_selector = Selector::parse(r#"h1"#).unwrap();
                let title = element_ref
                    .select(&title_selector)
                    .next()
                    .unwrap()
                    .first_child()
                    .unwrap()
                    .first_child()
                    .unwrap();
                title.value().as_text().unwrap()
            }
        };
        let a_selector = Selector::parse("a").unwrap();
        let link = element_ref
            .select(&a_selector)
            .next()
            .unwrap()
            .attr("href")
            .unwrap()
            .to_string();
        let id = link.split("/").last().unwrap().parse::<u32>().unwrap();
        let img_selector = Selector::parse("img").unwrap();
        let img_link = element_ref
            .select(&img_selector)
            .next()
            .unwrap()
            .attr("src")
            .unwrap()
            .to_string();
        let info_selector = Selector::parse(r#"div[class="news-info"]"#).unwrap();
        let news_info = match element_ref.select(&info_selector).next() {
            Some(news_info) => news_info,
            None => {
                let update_info_selector =
                    Selector::parse(r#"div[class="news-info news-info-update"]"#).unwrap();
                element_ref.select(&update_info_selector).next().unwrap()
            }
        };
        let time_selector = Selector::parse("time").unwrap();
        let time = news_info
            .select(&time_selector)
            .next()
            .unwrap()
            .attr("datetime")
            .unwrap()
            .to_string();
        let category = NewsCategory::from(news_info.select(&a_selector).next().unwrap());
        let hashtags_selector = Selector::parse(r#"li[class="gray-tag hashList"]"#).unwrap();
        let hidden_hashtags_selector =
            Selector::parse(r#"li[class="gray-tag hide-tag hashList"]"#).unwrap();
        let mut hashtags: Vec<NewsHashTag> = element_ref
            .select(&hashtags_selector)
            .map(NewsHashTag::from)
            .collect();
        let mut hidden_hashtags: Vec<NewsHashTag> = element_ref
            .select(&hidden_hashtags_selector)
            .map(NewsHashTag::from)
            .collect();
        hashtags.append(&mut hidden_hashtags);

        Self {
            title: title.to_string(),
            id,
            link,
            cover_img_link: img_link,
            time,
            category,
            hashtags,
        }
    }
}
#[derive(Debug)]
/// Represents a news category.
pub struct NewsCategory {
    pub id: u32,
    pub name: String,
    pub link: String,
}
impl<'a> From<ElementRef<'a>> for NewsCategory {
    /// Converts an HTML element reference to a `NewsCategory` instance.
    fn from(value: ElementRef<'a>) -> Self {
        debug!("NewsCategory from ElementRef: {}", value.html());
        Self {
            id: value
                .attr("href")
                .unwrap()
                .split("/")
                .last()
                .unwrap()
                .parse::<u32>()
                .unwrap(),
            name: value
                .first_child()
                .unwrap()
                .value()
                .as_text()
                .unwrap()
                .to_string(),
            link: value.attr("href").unwrap().to_string(),
        }
    }
}

#[derive(Debug)]
/// Represents a news hashtag.
pub struct NewsHashTag {
    pub id: u32,
    pub name: String,
    pub link: String,
}
/// Converts an HTML element reference to a `NewsHashTag` instance.
impl<'a> From<ElementRef<'a>> for NewsHashTag {
    fn from(value: ElementRef<'a>) -> Self {
        debug!("NewsHashtag from ElementRef: {}", value.html());
        let a_selector = Selector::parse("a").unwrap();
        let a = value.select(&a_selector).next().unwrap();
        let link = a.attr("href").unwrap().to_string();
        let name = a
            .first_child()
            .unwrap()
            .value()
            .as_text()
            .unwrap()
            .to_string();
        Self {
            id: link.split("/").nth(2).unwrap().parse().unwrap(),
            name,
            link,
        }
    }
}

/// Fetches the daily news from the PTS News website.
pub async fn get_daily_news() -> Vec<DailyNews> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static(
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"));

    let client = reqwest::Client::new();

    let url = format!("{}/dailynews", PTS_NEWS_URL);
    let response = client.get(&url).headers(headers).send().await.unwrap();

    if !response.status().is_success() {
        panic!("Failed to get daily news");
    }
    let text = response.text().await.unwrap();
    let doc = Html::parse_document(&text);
    let breaking_news_selector = Selector::parse(r#"div[class="breakingnews"]"#).unwrap();
    let mut news: Vec<DailyNews> = Vec::new();
    news.push(DailyNews::from(
        doc.select(&breaking_news_selector).next().unwrap(),
    ));

    let selector = Selector::parse(r#"li[class="d-flex"]"#).unwrap();
    for element in doc.select(&selector) {
        news.push(DailyNews::from(element));
    }
    news
}
