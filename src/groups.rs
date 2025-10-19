use crate::Client;
use anyhow::Result;
#[derive(Clone, Debug)]
pub struct Groups {
    pub client: Client,
}

impl Groups {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Perform a `GET` request to `/api/groups/`.\n\n**Parameters:**\n\n- `name_icontains: Option<String>`\n- `name_iendswith: Option<String>`\n- `name_iexact: Option<String>`\n- `name_istartswith: Option<String>`\n- `ordering: Option<String>`: Which field to use when ordering the results.\n- `page: Option<i64>`: A page number within the paginated result set.\n- `page_size: Option<i64>`: Number of results to return per page.\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_groups_list_stream() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let mut groups = client.groups();\n    let mut stream = groups.list_stream(\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(4 as i64),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn list<'a>(
        &'a self,
        name_icontains: Option<String>,
        name_iendswith: Option<String>,
        name_iexact: Option<String>,
        name_istartswith: Option<String>,
        ordering: Option<String>,
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> Result<crate::types::PaginatedGroupList, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "api/groups/"),
        );
        req = req.header("Authorization", format!("Token {}", &self.client.token));
        let mut query_params = vec![];
        if let Some(p) = name_icontains {
            query_params.push(("name__icontains", p));
        }

        if let Some(p) = name_iendswith {
            query_params.push(("name__iendswith", p));
        }

        if let Some(p) = name_iexact {
            query_params.push(("name__iexact", p));
        }

        if let Some(p) = name_istartswith {
            query_params.push(("name__istartswith", p));
        }

        if let Some(p) = ordering {
            query_params.push(("ordering", p));
        }

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

    #[doc = "Perform a `GET` request to `/api/groups/`.\n\n**Parameters:**\n\n- `name_icontains: Option<String>`\n- `name_iendswith: Option<String>`\n- `name_iexact: Option<String>`\n- `name_istartswith: Option<String>`\n- `ordering: Option<String>`: Which field to use when ordering the results.\n- `page: Option<i64>`: A page number within the paginated result set.\n- `page_size: Option<i64>`: Number of results to return per page.\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_groups_list_stream() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let mut groups = client.groups();\n    let mut stream = groups.list_stream(\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(4 as i64),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    #[allow(non_snake_case)]
    pub fn list_stream<'a>(
        &'a self,
        name__icontains: Option<String>,
        name__iendswith: Option<String>,
        name__iexact: Option<String>,
        name__istartswith: Option<String>,
        ordering: Option<String>,
        page_size: Option<i64>,
    ) -> impl futures::Stream<Item = Result<crate::types::Group, crate::types::error::Error>> + Unpin + '_
    {
        use crate::types::paginate::Pagination;
        use futures::{StreamExt, TryFutureExt, TryStreamExt};
        self.list(
            name__icontains,
            name__iendswith,
            name__iexact,
            name__istartswith,
            ordering,
            None,
            page_size,
        )
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
                                format!("{}/{}", self.client.base_url, "api/groups/"),
                            );
                            req = req
                                .header("Authorization", format!("Token {}", &self.client.token));
                            let mut request = req.build()?;
                            request = new_result.next_page(request)?;
                            let resp = self.client.client.execute(request).await?;
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
                        .map_ok(|result: crate::types::PaginatedGroupList| {
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

    #[doc = "Perform a `POST` request to `/api/groups/`.\n\n```rust,no_run\nasync fn example_groups_create() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::Group = client\n        .groups()\n        .create(&paperless_api_client::types::GroupRequest {\n            name: \"some-string\".to_string(),\n            permissions: vec![\"some-string\".to_string()],\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn create<'a>(
        &'a self,
        body: &crate::types::GroupRequest,
    ) -> Result<crate::types::Group, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "api/groups/"),
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

    #[doc = "Perform a `GET` request to `/api/groups/{id}/`.\n\n**Parameters:**\n\n- `id: i64`: A unique integer value identifying this group. (required)\n\n```rust,no_run\nasync fn example_groups_retrieve() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::Group = client.groups().retrieve(4 as i64).await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn retrieve<'a>(
        &'a self,
        id: i64,
    ) -> Result<crate::types::Group, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/groups/{id}/".replace("{id}", &format!("{id}"))
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

    #[doc = "Perform a `PUT` request to `/api/groups/{id}/`.\n\n**Parameters:**\n\n- `id: i64`: A unique integer value identifying this group. (required)\n\n```rust,no_run\nasync fn example_groups_update() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::Group = client\n        .groups()\n        .update(\n            4 as i64,\n            &paperless_api_client::types::GroupRequest {\n                name: \"some-string\".to_string(),\n                permissions: vec![\"some-string\".to_string()],\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn update<'a>(
        &'a self,
        id: i64,
        body: &crate::types::GroupRequest,
    ) -> Result<crate::types::Group, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/groups/{id}/".replace("{id}", &format!("{id}"))
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

    #[doc = "Perform a `DELETE` request to `/api/groups/{id}/`.\n\n**Parameters:**\n\n- `id: i64`: A unique integer value identifying this group. (required)\n\n```rust,no_run\nasync fn example_groups_destroy() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    client.groups().destroy(4 as i64).await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn destroy<'a>(&'a self, id: i64) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/groups/{id}/".replace("{id}", &format!("{id}"))
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

    #[doc = "Perform a `PATCH` request to `/api/groups/{id}/`.\n\n**Parameters:**\n\n- `id: i64`: A unique integer value identifying this group. (required)\n\n```rust,no_run\nasync fn example_groups_partial_update() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::Group = client\n        .groups()\n        .partial_update(\n            4 as i64,\n            &paperless_api_client::types::PatchedGroupRequest {\n                name: Some(\"some-string\".to_string()),\n                permissions: Some(vec![\"some-string\".to_string()]),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn partial_update<'a>(
        &'a self,
        id: i64,
        body: &crate::types::PatchedGroupRequest,
    ) -> Result<crate::types::Group, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/groups/{id}/".replace("{id}", &format!("{id}"))
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
