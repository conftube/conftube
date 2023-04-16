use crate::schemas::project_schemas::Video;
use google_youtube3::client::NoToken;
use google_youtube3::hyper::client::HttpConnector;
use google_youtube3::{hyper, Error, YouTube};
use hyper_rustls::HttpsConnector;
use std::sync::Arc;

pub type Youtube = Arc<YoutubeClient>;

#[derive(Debug)]
pub enum YoutubeError {
    UNKNOWN(Error),
}

pub struct YoutubeClient {
    key: String,
    hub: YouTube<HttpsConnector<HttpConnector>>,
}

impl YoutubeClient {
    pub fn new(key: String) -> Self {
        let hub = YouTube::new(
            hyper::Client::builder().build(
                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .https_or_http()
                    .enable_http1()
                    .enable_http2()
                    .build(),
            ),
            NoToken,
        );

        Self { key, hub }
    }

    pub async fn query(&self, term: String, max_results: u32) -> Result<Vec<Video>, YoutubeError> {
        let results = self
            .hub
            .search()
            .list(&vec!["snippet".into()])
            .q(format!("{} conference", term).as_str())
            .safe_search("strict")
            .max_results(max_results)
            .param("key", self.key.as_str())
            .doit()
            .await
            .map_err(YoutubeError::UNKNOWN)?
            .1
            .items;

        match results {
            None => Ok(Vec::new()),
            Some(e) => Ok(e
                .into_iter()
                .filter(|e| e.id.clone().unwrap().kind.unwrap() == "youtube#video")
                .map(|e| {
                    let snippet = e.snippet.unwrap();

                    Video {
                        id: e.id.unwrap().video_id.unwrap(),
                        platform: "youtube".to_string(),
                        title: snippet.title.unwrap(),
                        description: snippet.description.unwrap(),
                        published_at: snippet.published_at.unwrap().naive_utc(),
                        thumbnail_url: snippet.thumbnails.unwrap().medium.unwrap().url.unwrap(),
                        rating: None,
                    }
                })
                .collect()),
        }
    }
}
