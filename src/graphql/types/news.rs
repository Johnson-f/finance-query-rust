use async_graphql::*;
use finance_query_core::models::news::News as NewsModel;

#[derive(SimpleObject, Clone)]
pub struct News {
    pub title: String,
    pub link: String,
    pub source: String,
    pub img: String,
    pub time: String,
}

impl From<NewsModel> for News {
    fn from(news: NewsModel) -> Self {
        News {
            title: news.title,
            link: news.link,
            source: news.source,
            img: news.img,
            time: news.time,
        }
    }
}
