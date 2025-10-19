use crate::Client;
use anyhow::Result;
#[derive(Clone, Debug)]
pub struct WorkflowActions {
    pub client: Client,
}

impl WorkflowActions {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Perform a `GET` request to `/api/workflow_actions/`.\n\n**Parameters:**\n\n- `page: Option<i64>`: A page number within the paginated result set.\n- `page_size: Option<i64>`: Number of results to return per page.\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_workflow_actions_list_stream() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let mut workflow_actions = client.workflow_actions();\n    let mut stream = workflow_actions.list_stream(Some(4 as i64));\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn list<'a>(
        &'a self,
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> Result<crate::types::PaginatedWorkflowActionList, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "api/workflow_actions/"),
        );
        req = req.header("Authorization", format!("Token {}", &self.client.token));
        let mut query_params = vec![];
        if let Some(p) = page {
            query_params.push(("page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("page_size", format!("{p}")));
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

    #[doc = "Perform a `GET` request to `/api/workflow_actions/`.\n\n**Parameters:**\n\n- `page: Option<i64>`: A page number within the paginated result set.\n- `page_size: Option<i64>`: Number of results to return per page.\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_workflow_actions_list_stream() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let mut workflow_actions = client.workflow_actions();\n    let mut stream = workflow_actions.list_stream(Some(4 as i64));\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    #[allow(non_snake_case)]
    pub fn list_stream<'a>(
        &'a self,
        page_size: Option<i64>,
    ) -> impl futures::Stream<Item = Result<crate::types::WorkflowAction, crate::types::error::Error>>
           + Unpin
           + '_ {
        use crate::types::paginate::Pagination;
        use futures::{StreamExt, TryFutureExt, TryStreamExt};
        self.list(None, page_size)
            .map_ok(move |result| {
                let items = futures::stream::iter(result.items().into_iter().map(Ok));
                let next_pages = futures::stream::try_unfold(
                    (None, result),
                    move |(prev_page_token, new_result)| async move {
                        if new_result.has_more_pages()
                            && !new_result.items().is_empty()
                            && prev_page_token != new_result.next_page_token()
                        {
                            async {
                                let mut req = self.client.client.request(
                                    http::Method::GET,
                                    format!("{}/{}", self.client.base_url, "api/workflow_actions/"),
                                );
                                req = req.header(
                                    "Authorization",
                                    format!("Token {}", &self.client.token),
                                );
                                let mut request = req.build()?;
                                request = new_result.next_page(request)?;
                                let resp = self.client.client.execute(request).await?;
                                let status = resp.status();
                                if status.is_success() {
                                    let text = resp.text().await.unwrap_or_default();
                                    serde_json::from_str(&text).map_err(|err| {
                                        crate::types::error::Error::from_serde_error(
                                            format_serde_error::SerdeError::new(
                                                text.to_string(),
                                                err,
                                            ),
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
                            .map_ok(|result: crate::types::PaginatedWorkflowActionList| {
                                Some((
                                    futures::stream::iter(result.items().into_iter().map(Ok)),
                                    (new_result.next_page_token(), result),
                                ))
                            })
                            .await
                        } else {
                            Ok(None)
                        }
                    },
                )
                .try_flatten();
                items.chain(next_pages)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Perform a `POST` request to `/api/workflow_actions/`.\n\n```rust,no_run\nasync fn example_workflow_actions_create() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::WorkflowAction = client\n        .workflow_actions()\n        .create(&paperless_api_client::types::WorkflowActionRequest {\n            id: Some(4 as i64),\n            type_: Some(4 as i64),\n            assign_title: Some(\"some-string\".to_string()),\n            assign_tags: Some(vec![4 as i64]),\n            assign_correspondent: Some(4 as i64),\n            assign_document_type: Some(4 as i64),\n            assign_storage_path: Some(4 as i64),\n            assign_owner: Some(4 as i64),\n            assign_view_users: Some(vec![4 as i64]),\n            assign_view_groups: Some(vec![4 as i64]),\n            assign_change_users: Some(vec![4 as i64]),\n            assign_change_groups: Some(vec![4 as i64]),\n            assign_custom_fields: Some(vec![4 as i64]),\n            assign_custom_fields_values: Some(serde_json::Value::String(\"some-string\".to_string())),\n            remove_all_tags: Some(true),\n            remove_tags: Some(vec![4 as i64]),\n            remove_all_correspondents: Some(true),\n            remove_correspondents: Some(vec![4 as i64]),\n            remove_all_document_types: Some(true),\n            remove_document_types: Some(vec![4 as i64]),\n            remove_all_storage_paths: Some(true),\n            remove_storage_paths: Some(vec![4 as i64]),\n            remove_custom_fields: Some(vec![4 as i64]),\n            remove_all_custom_fields: Some(true),\n            remove_all_owners: Some(true),\n            remove_owners: Some(vec![4 as i64]),\n            remove_all_permissions: Some(true),\n            remove_view_users: Some(vec![4 as i64]),\n            remove_view_groups: Some(vec![4 as i64]),\n            remove_change_users: Some(vec![4 as i64]),\n            remove_change_groups: Some(vec![4 as i64]),\n            email: Some(paperless_api_client::types::WorkflowActionEmailRequest {\n                id: Some(4 as i64),\n                subject: \"some-string\".to_string(),\n                body: \"some-string\".to_string(),\n                to: \"some-string\".to_string(),\n                include_document: Some(true),\n            }),\n            webhook: Some(paperless_api_client::types::WorkflowActionWebhookRequest {\n                id: Some(4 as i64),\n                url: \"some-string\".to_string(),\n                use_params: Some(true),\n                as_json: Some(true),\n                params: Some(serde_json::Value::String(\"some-string\".to_string())),\n                body: Some(\"some-string\".to_string()),\n                headers: Some(serde_json::Value::String(\"some-string\".to_string())),\n                include_document: Some(true),\n            }),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn create<'a>(
        &'a self,
        body: &crate::types::WorkflowActionRequest,
    ) -> Result<crate::types::WorkflowAction, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "api/workflow_actions/"),
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

    #[doc = "Perform a `GET` request to `/api/workflow_actions/{id}/`.\n\n**Parameters:**\n\n- `id: i64`: A unique integer value identifying this workflow action. (required)\n\n```rust,no_run\nasync fn example_workflow_actions_retrieve() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::WorkflowAction = client.workflow_actions().retrieve(4 as i64).await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn retrieve<'a>(
        &'a self,
        id: i64,
    ) -> Result<crate::types::WorkflowAction, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/workflow_actions/{id}/".replace("{id}", &format!("{id}"))
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

    #[doc = "Perform a `PUT` request to `/api/workflow_actions/{id}/`.\n\n**Parameters:**\n\n- `id: i64`: A unique integer value identifying this workflow action. (required)\n\n```rust,no_run\nasync fn example_workflow_actions_update() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::WorkflowAction = client\n        .workflow_actions()\n        .update(\n            4 as i64,\n            &paperless_api_client::types::WorkflowActionRequest {\n                id: Some(4 as i64),\n                type_: Some(4 as i64),\n                assign_title: Some(\"some-string\".to_string()),\n                assign_tags: Some(vec![4 as i64]),\n                assign_correspondent: Some(4 as i64),\n                assign_document_type: Some(4 as i64),\n                assign_storage_path: Some(4 as i64),\n                assign_owner: Some(4 as i64),\n                assign_view_users: Some(vec![4 as i64]),\n                assign_view_groups: Some(vec![4 as i64]),\n                assign_change_users: Some(vec![4 as i64]),\n                assign_change_groups: Some(vec![4 as i64]),\n                assign_custom_fields: Some(vec![4 as i64]),\n                assign_custom_fields_values: Some(serde_json::Value::String(\n                    \"some-string\".to_string(),\n                )),\n                remove_all_tags: Some(true),\n                remove_tags: Some(vec![4 as i64]),\n                remove_all_correspondents: Some(true),\n                remove_correspondents: Some(vec![4 as i64]),\n                remove_all_document_types: Some(true),\n                remove_document_types: Some(vec![4 as i64]),\n                remove_all_storage_paths: Some(true),\n                remove_storage_paths: Some(vec![4 as i64]),\n                remove_custom_fields: Some(vec![4 as i64]),\n                remove_all_custom_fields: Some(true),\n                remove_all_owners: Some(true),\n                remove_owners: Some(vec![4 as i64]),\n                remove_all_permissions: Some(true),\n                remove_view_users: Some(vec![4 as i64]),\n                remove_view_groups: Some(vec![4 as i64]),\n                remove_change_users: Some(vec![4 as i64]),\n                remove_change_groups: Some(vec![4 as i64]),\n                email: Some(paperless_api_client::types::WorkflowActionEmailRequest {\n                    id: Some(4 as i64),\n                    subject: \"some-string\".to_string(),\n                    body: \"some-string\".to_string(),\n                    to: \"some-string\".to_string(),\n                    include_document: Some(true),\n                }),\n                webhook: Some(paperless_api_client::types::WorkflowActionWebhookRequest {\n                    id: Some(4 as i64),\n                    url: \"some-string\".to_string(),\n                    use_params: Some(true),\n                    as_json: Some(true),\n                    params: Some(serde_json::Value::String(\"some-string\".to_string())),\n                    body: Some(\"some-string\".to_string()),\n                    headers: Some(serde_json::Value::String(\"some-string\".to_string())),\n                    include_document: Some(true),\n                }),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn update<'a>(
        &'a self,
        id: i64,
        body: &crate::types::WorkflowActionRequest,
    ) -> Result<crate::types::WorkflowAction, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/workflow_actions/{id}/".replace("{id}", &format!("{id}"))
            ),
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

    #[doc = "Perform a `DELETE` request to `/api/workflow_actions/{id}/`.\n\n**Parameters:**\n\n- `id: i64`: A unique integer value identifying this workflow action. (required)\n\n```rust,no_run\nasync fn example_workflow_actions_destroy() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    client.workflow_actions().destroy(4 as i64).await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn destroy<'a>(&'a self, id: i64) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/workflow_actions/{id}/".replace("{id}", &format!("{id}"))
            ),
        );
        req = req.header("Authorization", format!("Token {}", &self.client.token));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Perform a `PATCH` request to `/api/workflow_actions/{id}/`.\n\n**Parameters:**\n\n- `id: i64`: A unique integer value identifying this workflow action. (required)\n\n```rust,no_run\nasync fn example_workflow_actions_partial_update() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::WorkflowAction = client\n        .workflow_actions()\n        .partial_update(\n            4 as i64,\n            &paperless_api_client::types::PatchedWorkflowActionRequest {\n                id: Some(4 as i64),\n                type_: Some(4 as i64),\n                assign_title: Some(\"some-string\".to_string()),\n                assign_tags: Some(vec![4 as i64]),\n                assign_correspondent: Some(4 as i64),\n                assign_document_type: Some(4 as i64),\n                assign_storage_path: Some(4 as i64),\n                assign_owner: Some(4 as i64),\n                assign_view_users: Some(vec![4 as i64]),\n                assign_view_groups: Some(vec![4 as i64]),\n                assign_change_users: Some(vec![4 as i64]),\n                assign_change_groups: Some(vec![4 as i64]),\n                assign_custom_fields: Some(vec![4 as i64]),\n                assign_custom_fields_values: Some(serde_json::Value::String(\n                    \"some-string\".to_string(),\n                )),\n                remove_all_tags: Some(true),\n                remove_tags: Some(vec![4 as i64]),\n                remove_all_correspondents: Some(true),\n                remove_correspondents: Some(vec![4 as i64]),\n                remove_all_document_types: Some(true),\n                remove_document_types: Some(vec![4 as i64]),\n                remove_all_storage_paths: Some(true),\n                remove_storage_paths: Some(vec![4 as i64]),\n                remove_custom_fields: Some(vec![4 as i64]),\n                remove_all_custom_fields: Some(true),\n                remove_all_owners: Some(true),\n                remove_owners: Some(vec![4 as i64]),\n                remove_all_permissions: Some(true),\n                remove_view_users: Some(vec![4 as i64]),\n                remove_view_groups: Some(vec![4 as i64]),\n                remove_change_users: Some(vec![4 as i64]),\n                remove_change_groups: Some(vec![4 as i64]),\n                email: Some(paperless_api_client::types::WorkflowActionEmailRequest {\n                    id: Some(4 as i64),\n                    subject: \"some-string\".to_string(),\n                    body: \"some-string\".to_string(),\n                    to: \"some-string\".to_string(),\n                    include_document: Some(true),\n                }),\n                webhook: Some(paperless_api_client::types::WorkflowActionWebhookRequest {\n                    id: Some(4 as i64),\n                    url: \"some-string\".to_string(),\n                    use_params: Some(true),\n                    as_json: Some(true),\n                    params: Some(serde_json::Value::String(\"some-string\".to_string())),\n                    body: Some(\"some-string\".to_string()),\n                    headers: Some(serde_json::Value::String(\"some-string\".to_string())),\n                    include_document: Some(true),\n                }),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn partial_update<'a>(
        &'a self,
        id: i64,
        body: &crate::types::PatchedWorkflowActionRequest,
    ) -> Result<crate::types::WorkflowAction, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/workflow_actions/{id}/".replace("{id}", &format!("{id}"))
            ),
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
