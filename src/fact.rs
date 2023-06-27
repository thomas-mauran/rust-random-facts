use reqwest::{Error, Response};
use serde::{Deserialize, Serialize};
use crate::errors::ApiError;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Fact {
    pub id: String,
    pub text: String,
    pub source: String,
    pub source_url: String,
    pub language: String,
    pub permalink: String,
}

impl Fact {
    pub fn new(
        id: String,
        text: String,
        source: String,
        source_url: String,
        language: String,
        permalink: String,
    ) -> Fact {
        Fact {
            id,
            text,
            source,
            source_url,
            language,
            permalink,
        }
    }

    pub async fn fetch_random(&mut self) -> Result<(), ApiError> {
        let url = "https://uselessfacts.jsph.pl/api/v2/facts/random";
        let response = Self::fetch_api(url).await?;
        let fact = serde_json::from_str::<Fact>(&response)?;
        self.id = fact.id;
        self.text = fact.text;
        self.source = fact.source;
        self.source_url = fact.source_url;
        self.language = fact.language;
        self.permalink = fact.permalink;
        Ok(())
    }

    async fn fetch_api(url: &str) -> Result<String, ApiError> {
        let client = reqwest::Client::new();
        let response = client.get(url).send().await?;
        let response_text = response.text().await?;

        Ok(response_text)
    }
}
