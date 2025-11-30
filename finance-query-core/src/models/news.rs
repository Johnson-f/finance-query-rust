use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct News {
    pub title: String,
    pub link: String,
    pub source: String,
    pub img: String,
    pub time: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // **Feature: crate-extraction, Property 1: Model Serialization Round-Trip**
    // **Validates: Requirements 2.2**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn news_roundtrip(
            title in "[A-Za-z0-9 ]{1,50}",
            link in "https://[a-z]{5,10}\\.com/[a-z]{5,10}",
            source in "[A-Za-z ]{1,20}",
            img in "https://[a-z]{5,10}\\.com/[a-z]{5,10}\\.jpg",
            time in "[0-9]{1,2}h ago",
        ) {
            let news = News {
                title: title.clone(),
                link: link.clone(),
                source: source.clone(),
                img: img.clone(),
                time: time.clone(),
            };

            let json = serde_json::to_string(&news).unwrap();
            let parsed: News = serde_json::from_str(&json).unwrap();

            prop_assert_eq!(news.title, parsed.title);
            prop_assert_eq!(news.link, parsed.link);
            prop_assert_eq!(news.source, parsed.source);
            prop_assert_eq!(news.img, parsed.img);
            prop_assert_eq!(news.time, parsed.time);
        }
    }
}
