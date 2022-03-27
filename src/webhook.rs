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
    markdown: Option<MarkdownMessage<'a>>,
}

#[derive(Serialize, Debug)]
pub struct TextMessage<'a> {
    content: &'a str,
}

#[derive(Serialize, Debug)]
pub struct MarkdownMessage<'a> {
    content: &'a str,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
enum MessageType {
    Text,
    Markdown,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct Response {
    #[serde(rename = "errcode")]
    code: u32,
    #[serde(rename = "errmsg")]
    msg: String,
}

const WECOM_WEBHOOK_URL: &'static str = "https://qyapi.weixin.qq.com/cgi-bin/webhook/send";

pub fn new(key: &str) -> Client {
    Client {
        key: key,
        client: reqwest::Client::new(),
    }
}

use crate::error;
impl Client<'_> {
    pub async fn notify<'a>(&self, body: &Request<'a>) -> Result<()> {
        let params = [("key", &self.key)];
        let res: Response = self
            .client
            .post(WECOM_WEBHOOK_URL)
            .header("User-Agent", "wecom-rs")
            .query(&params)
            .json(body)
            .send()
            .await?
            .json()
            .await?;
        println!("wx: {:?}", res);
        if res.code == 0 {
            Ok(())
        } else {
            Err(error::Error::WC(res.msg))
        }
    }

    pub async fn notify_text(&self, text: &str) -> Result<()> {
        self.notify(&Request {
            kind: MessageType::Text,
            text: Some(TextMessage { content: text }),
            markdown: None,
        })
        .await
    }
    pub async fn notify_markdown(&self, md: &str) -> Result<()> {
        self.notify(&Request {
            kind: MessageType::Markdown,
            markdown: Some(MarkdownMessage { content: md }),
            text: None,
        })
        .await
    }
}
