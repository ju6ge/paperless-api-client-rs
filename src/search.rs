use crate::Client;
use anyhow::Result;
#[derive(Clone, Debug)]
pub struct Search {
    pub client: Client,
}

impl Search {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Perform a `GET` request to `/api/search/`.\n\nGlobal search\n\n**Parameters:**\n\n- `db_only: Option<bool>`: Search only the database\n- `query: &'astr`: Query to search for (required)\n\n```rust,no_run\nasync fn example_search_retrieve() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::SearchResult =\n        client.search().retrieve(Some(true), \"some-string\").await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn retrieve<'a>(
        &'a self,
        db_only: Option<bool>,
        query: &'a str,
    ) -> Result<crate::types::SearchResult, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "api/search/"),
        );
        req = req.header("Authorization", format!("Token {}", &self.client.token));
        let mut query_params = vec![("query", query.to_string())];
        if let Some(p) = db_only {
            query_params.push(("db_only", format!("{p}")));
        }

        req = req.query(&query_params);
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

    #[doc = "Perform a `GET` request to `/api/search/autocomplete/`.\n\nGet a list of all available tags\n\n**Parameters:**\n\n- `limit: Option<i64>`: Number of completions to return\n- `term: Option<String>`: Term to search for\n\n```rust,no_run\nasync fn example_search_autocomplete_list() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: Vec<String> = client\n        .search()\n        .autocomplete_list(Some(4 as i64), Some(\"some-string\".to_string()))\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn autocomplete_list<'a>(
        &'a self,
        limit: Option<i64>,
        term: Option<String>,
    ) -> Result<Vec<String>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "api/search/autocomplete/"),
        );
        req = req.header("Authorization", format!("Token {}", &self.client.token));
        let mut query_params = vec![];
        if let Some(p) = limit {
            query_params.push(("limit", format!("{p}")));
        }

        if let Some(p) = term {
            query_params.push(("term", p));
        }

        req = req.query(&query_params);
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
