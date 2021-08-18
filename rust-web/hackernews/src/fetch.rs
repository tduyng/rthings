use super::rss;
use rss::{Channel, Item};

pub type FetchResult<T> = Result<T, rss::Error>;

pub fn fetch_from(url: &str) -> FetchResult<Vec<Item>> {
  Ok(Channel::from_str(url)?.items().to_vec())
}
