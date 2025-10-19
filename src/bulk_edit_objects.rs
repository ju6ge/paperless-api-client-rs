use crate::Client;
use anyhow::Result;
#[derive(Clone, Debug)]
pub struct BulkEditObjects {
    pub client: Client,
}

impl BulkEditObjects {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Perform a `POST` request to `/api/bulk_edit_objects/`.\n\nPerform a bulk edit operation on a list of objects\n\nSee <https://docs.paperless-ngx.com/api/#objects|Further documentation> for more information.\n\n```rust,no_run\nasync fn example_bulk_edit_objects_bulk_edit_objects() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::BulkEditResult = client\n        .bulk_edit_objects()\n        .bulk_edit_objects(&paperless_api_client::types::BulkEditObjectsRequest {\n            objects: vec![4 as i64],\n            object_type: paperless_api_client::types::ObjectTypeEnum::DocumentTypes,\n            operation: paperless_api_client::types::OperationEnum::Delete,\n            owner: Some(4 as i64),\n            permissions: Some(std::collections::HashMap::from([(\n                \"some-key\".to_string(),\n                serde_json::Value::String(\"some-string\".to_string()),\n            )])),\n            merge: true,\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn bulk_edit_objects<'a>(
        &'a self,
        body: &crate::types::BulkEditObjectsRequest,
    ) -> Result<crate::types::BulkEditResult, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "api/bulk_edit_objects/"),
        );
        req = req.header("Authorization", format!("Token {}", &self.client.token));
        req = req.json(body);
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
