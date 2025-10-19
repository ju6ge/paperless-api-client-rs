use crate::Client;
use anyhow::Result;
#[derive(Clone, Debug)]
pub struct RemoteVersion {
    pub client: Client,
}

impl RemoteVersion {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Perform a `GET` request to `/api/remote_version/`.\n\nGet the current version of the Paperless-NGX server\n\n```rust,no_run\nasync fn example_remote_version_retrieve() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: std::collections::HashMap<String, serde_json::Value> =\n        client.remote_version().retrieve().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn retrieve<'a>(
        &'a self,
    ) -> Result<std::collections::HashMap<String, serde_json::Value>, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "api/remote_version/"),
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
