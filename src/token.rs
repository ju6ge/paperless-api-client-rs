use crate::Client;
use anyhow::Result;
#[derive(Clone, Debug)]
pub struct Token {
    pub client: Client,
}

impl Token {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Perform a `POST` request to `/api/token/`.\n\n```rust,no_run\nasync fn example_token_create() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::PaperlessAuthToken = client\n        .token()\n        .create(&paperless_api_client::types::PaperlessAuthTokenRequest {\n            username: \"some-string\".to_string(),\n            password: \"some-string\".to_string(),\n            code: Some(\"some-string\".to_string()),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn create<'a>(
        &'a self,
        body: &crate::types::PaperlessAuthTokenRequest,
    ) -> Result<crate::types::PaperlessAuthToken, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "api/token/"),
        );
        req = req.header("Authorization", format!("Token {}", &self.client.token));
        req = req.form(body);
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
