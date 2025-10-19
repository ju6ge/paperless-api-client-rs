use crate::Client;
use anyhow::Result;
#[derive(Clone, Debug)]
pub struct Logs {
    pub client: Client,
}

impl Logs {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Perform a `GET` request to `/api/logs/`.\n\nLogs view\n\n```rust,no_run\nasync fn example_logs_list() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: Vec<String> = client.logs().list().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn list<'a>(&'a self) -> Result<Vec<String>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "api/logs/"),
        );
        req = req.header("Authorization", format!("Token {}", &self.client.token));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
            })
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Perform a `GET` request to `/api/logs/{id}/`.\n\nSingle log view\n\n**Parameters:**\n\n- `id: &'astr` (required)\n\n```rust,no_run\nasync fn example_logs_retrieve() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: Vec<String> = client.logs().retrieve(\"some-string\").await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn retrieve<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<Vec<String>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/logs/{id}/".replace("{id}", id)
            ),
        );
        req = req.header("Authorization", format!("Token {}", &self.client.token));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
            })
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }
}
