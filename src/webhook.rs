use reqwest;
use serde::{Deserialize, Serialize};

use crate::Result;

pub struct Client<'a> {
    key: &'a str,
    client: reqwest::Client,
}

#[derive(Serialize, Debug)]
pub struct Request<'a> {
    #[serde(rename = "msgtype")]
    kind: MessageType,
    text: Option<TextMessage<'a>>,
}

#[derive(Serialize, Debug)]
pub struct TextMessage<'a> {
    content: &'a str,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
enum MessageType {
    Text,
    Markdown,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct Response {}

const WECOM_WEBHOOK_URL: &'static str = "https://qyapi.weixin.qq.com/cgi-bin/webhook/send";

pub fn new(key: &str) -> Client {
    Client {
        key: key,
        client: reqwest::Client::new(),
    }
}

impl Client<'_> {
    pub async fn notify<'a>(&self, body: &Request<'a>) -> Result<()> {
        let params = [("key", &self.key)];
        let res = self
            .client
            .post(WECOM_WEBHOOK_URL)
            .header("User-Agent", "wecom-rs")
            .query(&params)
            .json(body)
            .send()
            .await?;
        println!("wx: {:?}", res);
        Ok(())
    }

    pub async fn notify_text(&self, text: &str) -> Result<()> {
        let request = Request {
            kind: MessageType::Text,
            text: Some(TextMessage { content: text }),
        };
        let _res = self.notify(&request).await?;
        Ok(())
    }
}
