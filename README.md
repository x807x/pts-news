# PTS News
[![Rust](https://github.com/x807x/pts-news/actions/workflows/rust.yml/badge.svg)](https://github.com/x807x/pts-news/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/pts-news.svg)](https://crates.io/crates/pts-news)
[![Documentation](https://docs.rs/pts-news/badge.svg)](https://docs.rs/pts-news)
[![License](https://img.shields.io/crates/l/pts-news?labelColor=green
)](https://github.com/x807x/pts-news/blob/master/LICENSE)

PTS News is a Rust project that fetches and parses daily news from the [PTS News website](https://news.pts.org.tw).

## Getting Started
5
To use this in your project, run the following command:

```sh
cargo add pts-news
```

### Building the Project

To build the project, run the following command:

```sh
cargo build
```

### Running the Example

To run the example that fetches and prints the daily news, use the following command:

```sh
cargo run --example get-daily-news
```

## Project Modules

This file contains the main module definitions and constants for the project.

```rust
pub const PTS_NEWS_URL: &str = "https://news.pts.org.tw";

pub mod daily_news;
```

### Example

This file contains an example of how to use the `get_daily_news` function to fetch and print the daily news.

```rust
use log::{info, LevelFilter};

#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .filter(None, LevelFilter::Info)
        .init();
    info!("Fetching daily news...");
    let news = pts_news::daily_news::get_daily_news().await;
    info!("{:#?}", news);
}
```

## Dependencies

- log: For logging.
- reqwest: For making HTTP requests.
- scraper: For parsing HTML.

## License

This project is licensed under the MIT License. See the LICENSE file for details.
