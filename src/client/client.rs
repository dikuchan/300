use super::Summary;
use super::{ClientError, ClientResult};

use select::{
    document::Document,
    predicate::{Class, Name},
};
use serde::Deserialize;
use std::collections::HashMap;

const API_URL: &str = "https://300.ya.ru/api/sharing-url";
const SUCCESS_STATUS: &str = "success";

pub struct Client {
    http_client: reqwest::Client,
    auth_header: String,
}

impl Client {
    pub fn from_token(token: &str) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            auth_header: format!("OAuth {}", token),
        }
    }

    pub async fn get_summary(&self, article_url: &str) -> ClientResult<Summary> {
        let mut body = HashMap::new();
        body.insert("article_url", article_url);

        let response = self
            .http_client
            .post(API_URL)
            .header("Authorization", self.auth_header.as_str())
            .json(&body)
            .send()
            .await
            .unwrap();

        let body = response.json::<GetSummaryBody>().await.unwrap();
        if body.status != SUCCESS_STATUS {
            return Err(ClientError::ErrorStatus(body.status));
        }
        let summary_url = body.sharing_url.unwrap();

        let response = self
            .http_client
            .get(summary_url.as_str())
            .send()
            .await
            .unwrap();
        let body = response.text().await.unwrap();

        let doc = Document::from(body.as_str());

        let title = doc
            .find(Class("content-title"))
            .next()
            .ok_or(ClientError::TitleNotFound)?
            .find(Name("span"))
            .next()
            .ok_or(ClientError::TitleNotFound)?
            .text();

        let theses = doc
            .find(Class("content-theses"))
            .flat_map(|node| node.find(Name("li")).map(|node| node.text()))
            .collect::<Vec<_>>();
        if theses.is_empty() {
            return Err(ClientError::ThesesNotFound);
        }

        Ok(Summary::new(title, theses))
    }
}

#[derive(Debug, Deserialize)]
struct GetSummaryBody {
    status: String,
    sharing_url: Option<String>,
}
