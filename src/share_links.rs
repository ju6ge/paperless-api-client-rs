use crate::Client;
use anyhow::Result;
#[derive(Clone, Debug)]
pub struct ShareLinks {
    pub client: Client,
}

impl ShareLinks {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Perform a `GET` request to `/api/share_links/`.\n\n**Parameters:**\n\n- `created_date_gt: Option<chrono::NaiveDate>`\n- `created_date_gte: Option<chrono::NaiveDate>`\n- `created_date_lt: Option<chrono::NaiveDate>`\n- `created_date_lte: Option<chrono::NaiveDate>`\n- `created_day: Option<f64>`\n- `created_gt: Option<chrono::DateTime<chrono::Utc>>`\n- `created_gte: Option<chrono::DateTime<chrono::Utc>>`\n- `created_lt: Option<chrono::DateTime<chrono::Utc>>`\n- `created_lte: Option<chrono::DateTime<chrono::Utc>>`\n- `created_month: Option<f64>`\n- `created_year: Option<f64>`\n- `expiration_date_gt: Option<chrono::NaiveDate>`\n- `expiration_date_gte: Option<chrono::NaiveDate>`\n- `expiration_date_lt: Option<chrono::NaiveDate>`\n- `expiration_date_lte: Option<chrono::NaiveDate>`\n- `expiration_day: Option<f64>`\n- `expiration_gt: Option<chrono::DateTime<chrono::Utc>>`\n- `expiration_gte: Option<chrono::DateTime<chrono::Utc>>`\n- `expiration_lt: Option<chrono::DateTime<chrono::Utc>>`\n- `expiration_lte: Option<chrono::DateTime<chrono::Utc>>`\n- `expiration_month: Option<f64>`\n- `expiration_year: Option<f64>`\n- `ordering: Option<String>`: Which field to use when ordering the results.\n- `page: Option<i64>`: A page number within the paginated result set.\n- `page_size: Option<i64>`: Number of results to return per page.\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_share_links_list_stream() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let mut share_links = client.share_links();\n    let mut stream = share_links.list_stream(\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(3.14 as f64),\n        Some(chrono::Utc::now()),\n        Some(chrono::Utc::now()),\n        Some(chrono::Utc::now()),\n        Some(chrono::Utc::now()),\n        Some(3.14 as f64),\n        Some(3.14 as f64),\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(3.14 as f64),\n        Some(chrono::Utc::now()),\n        Some(chrono::Utc::now()),\n        Some(chrono::Utc::now()),\n        Some(chrono::Utc::now()),\n        Some(3.14 as f64),\n        Some(3.14 as f64),\n        Some(\"some-string\".to_string()),\n        Some(4 as i64),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn list<'a>(
        &'a self,
        created_date_gt: Option<chrono::NaiveDate>,
        created_date_gte: Option<chrono::NaiveDate>,
        created_date_lt: Option<chrono::NaiveDate>,
        created_date_lte: Option<chrono::NaiveDate>,
        created_day: Option<f64>,
        created_gt: Option<chrono::DateTime<chrono::Utc>>,
        created_gte: Option<chrono::DateTime<chrono::Utc>>,
        created_lt: Option<chrono::DateTime<chrono::Utc>>,
        created_lte: Option<chrono::DateTime<chrono::Utc>>,
        created_month: Option<f64>,
        created_year: Option<f64>,
        expiration_date_gt: Option<chrono::NaiveDate>,
        expiration_date_gte: Option<chrono::NaiveDate>,
        expiration_date_lt: Option<chrono::NaiveDate>,
        expiration_date_lte: Option<chrono::NaiveDate>,
        expiration_day: Option<f64>,
        expiration_gt: Option<chrono::DateTime<chrono::Utc>>,
        expiration_gte: Option<chrono::DateTime<chrono::Utc>>,
        expiration_lt: Option<chrono::DateTime<chrono::Utc>>,
        expiration_lte: Option<chrono::DateTime<chrono::Utc>>,
        expiration_month: Option<f64>,
        expiration_year: Option<f64>,
        ordering: Option<String>,
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> Result<crate::types::PaginatedShareLinkList, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "api/share_links/"),
        );
        req = req.header("Authorization", format!("Token {}", &self.client.token));
        let mut query_params = vec![];
        if let Some(p) = created_date_gt {
            query_params.push(("created__date__gt", format!("{p}")));
        }

        if let Some(p) = created_date_gte {
            query_params.push(("created__date__gte", format!("{p}")));
        }

        if let Some(p) = created_date_lt {
            query_params.push(("created__date__lt", format!("{p}")));
        }

        if let Some(p) = created_date_lte {
            query_params.push(("created__date__lte", format!("{p}")));
        }

        if let Some(p) = created_day {
            query_params.push(("created__day", format!("{p}")));
        }

        if let Some(p) = created_gt {
            query_params.push(("created__gt", format!("{p}")));
        }

        if let Some(p) = created_gte {
            query_params.push(("created__gte", format!("{p}")));
        }

        if let Some(p) = created_lt {
            query_params.push(("created__lt", format!("{p}")));
        }

        if let Some(p) = created_lte {
            query_params.push(("created__lte", format!("{p}")));
        }

        if let Some(p) = created_month {
            query_params.push(("created__month", format!("{p}")));
        }

        if let Some(p) = created_year {
            query_params.push(("created__year", format!("{p}")));
        }

        if let Some(p) = expiration_date_gt {
            query_params.push(("expiration__date__gt", format!("{p}")));
        }

        if let Some(p) = expiration_date_gte {
            query_params.push(("expiration__date__gte", format!("{p}")));
        }

        if let Some(p) = expiration_date_lt {
            query_params.push(("expiration__date__lt", format!("{p}")));
        }

        if let Some(p) = expiration_date_lte {
            query_params.push(("expiration__date__lte", format!("{p}")));
        }

        if let Some(p) = expiration_day {
            query_params.push(("expiration__day", format!("{p}")));
        }

        if let Some(p) = expiration_gt {
            query_params.push(("expiration__gt", format!("{p}")));
        }

        if let Some(p) = expiration_gte {
            query_params.push(("expiration__gte", format!("{p}")));
        }

        if let Some(p) = expiration_lt {
            query_params.push(("expiration__lt", format!("{p}")));
        }

        if let Some(p) = expiration_lte {
            query_params.push(("expiration__lte", format!("{p}")));
        }

        if let Some(p) = expiration_month {
            query_params.push(("expiration__month", format!("{p}")));
        }

        if let Some(p) = expiration_year {
            query_params.push(("expiration__year", format!("{p}")));
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

    #[doc = "Perform a `GET` request to `/api/share_links/`.\n\n**Parameters:**\n\n- `created_date_gt: Option<chrono::NaiveDate>`\n- `created_date_gte: Option<chrono::NaiveDate>`\n- `created_date_lt: Option<chrono::NaiveDate>`\n- `created_date_lte: Option<chrono::NaiveDate>`\n- `created_day: Option<f64>`\n- `created_gt: Option<chrono::DateTime<chrono::Utc>>`\n- `created_gte: Option<chrono::DateTime<chrono::Utc>>`\n- `created_lt: Option<chrono::DateTime<chrono::Utc>>`\n- `created_lte: Option<chrono::DateTime<chrono::Utc>>`\n- `created_month: Option<f64>`\n- `created_year: Option<f64>`\n- `expiration_date_gt: Option<chrono::NaiveDate>`\n- `expiration_date_gte: Option<chrono::NaiveDate>`\n- `expiration_date_lt: Option<chrono::NaiveDate>`\n- `expiration_date_lte: Option<chrono::NaiveDate>`\n- `expiration_day: Option<f64>`\n- `expiration_gt: Option<chrono::DateTime<chrono::Utc>>`\n- `expiration_gte: Option<chrono::DateTime<chrono::Utc>>`\n- `expiration_lt: Option<chrono::DateTime<chrono::Utc>>`\n- `expiration_lte: Option<chrono::DateTime<chrono::Utc>>`\n- `expiration_month: Option<f64>`\n- `expiration_year: Option<f64>`\n- `ordering: Option<String>`: Which field to use when ordering the results.\n- `page: Option<i64>`: A page number within the paginated result set.\n- `page_size: Option<i64>`: Number of results to return per page.\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_share_links_list_stream() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let mut share_links = client.share_links();\n    let mut stream = share_links.list_stream(\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(3.14 as f64),\n        Some(chrono::Utc::now()),\n        Some(chrono::Utc::now()),\n        Some(chrono::Utc::now()),\n        Some(chrono::Utc::now()),\n        Some(3.14 as f64),\n        Some(3.14 as f64),\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(3.14 as f64),\n        Some(chrono::Utc::now()),\n        Some(chrono::Utc::now()),\n        Some(chrono::Utc::now()),\n        Some(chrono::Utc::now()),\n        Some(3.14 as f64),\n        Some(3.14 as f64),\n        Some(\"some-string\".to_string()),\n        Some(4 as i64),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    #[allow(non_snake_case)]
    pub fn list_stream<'a>(
        &'a self,
        created__date__gt: Option<chrono::NaiveDate>,
        created__date__gte: Option<chrono::NaiveDate>,
        created__date__lt: Option<chrono::NaiveDate>,
        created__date__lte: Option<chrono::NaiveDate>,
        created__day: Option<f64>,
        created__gt: Option<chrono::DateTime<chrono::Utc>>,
        created__gte: Option<chrono::DateTime<chrono::Utc>>,
        created__lt: Option<chrono::DateTime<chrono::Utc>>,
        created__lte: Option<chrono::DateTime<chrono::Utc>>,
        created__month: Option<f64>,
        created__year: Option<f64>,
        expiration__date__gt: Option<chrono::NaiveDate>,
        expiration__date__gte: Option<chrono::NaiveDate>,
        expiration__date__lt: Option<chrono::NaiveDate>,
        expiration__date__lte: Option<chrono::NaiveDate>,
        expiration__day: Option<f64>,
        expiration__gt: Option<chrono::DateTime<chrono::Utc>>,
        expiration__gte: Option<chrono::DateTime<chrono::Utc>>,
        expiration__lt: Option<chrono::DateTime<chrono::Utc>>,
        expiration__lte: Option<chrono::DateTime<chrono::Utc>>,
        expiration__month: Option<f64>,
        expiration__year: Option<f64>,
        ordering: Option<String>,
        page_size: Option<i64>,
    ) -> impl futures::Stream<Item = Result<crate::types::ShareLink, crate::types::error::Error>>
           + Unpin
           + '_ {
        use crate::types::paginate::Pagination;
        use futures::{StreamExt, TryFutureExt, TryStreamExt};
        self.list(
            created__date__gt,
            created__date__gte,
            created__date__lt,
            created__date__lte,
            created__day,
            created__gt,
            created__gte,
            created__lt,
            created__lte,
            created__month,
            created__year,
            expiration__date__gt,
            expiration__date__gte,
            expiration__date__lt,
            expiration__date__lte,
            expiration__day,
            expiration__gt,
            expiration__gte,
            expiration__lt,
            expiration__lte,
            expiration__month,
            expiration__year,
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
                                format!("{}/{}", self.client.base_url, "api/share_links/"),
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
                        .map_ok(|result: crate::types::PaginatedShareLinkList| {
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

    #[doc = "Perform a `POST` request to `/api/share_links/`.\n\n```rust,no_run\nasync fn example_share_links_create() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::ShareLink = client\n        .share_links()\n        .create(&paperless_api_client::types::ShareLinkRequest {\n            expiration: Some(chrono::Utc::now()),\n            document: Some(4 as i64),\n            file_version: Some(paperless_api_client::types::FileVersionEnum::Original),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn create<'a>(
        &'a self,
        body: &crate::types::ShareLinkRequest,
    ) -> Result<crate::types::ShareLink, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "api/share_links/"),
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

    #[doc = "Perform a `GET` request to `/api/share_links/{id}/`.\n\n**Parameters:**\n\n- `id: i64`: A unique integer value identifying this share link. (required)\n\n```rust,no_run\nasync fn example_share_links_retrieve() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::ShareLink = client.share_links().retrieve(4 as i64).await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn retrieve<'a>(
        &'a self,
        id: i64,
    ) -> Result<crate::types::ShareLink, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/share_links/{id}/".replace("{id}", &format!("{id}"))
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

    #[doc = "Perform a `PUT` request to `/api/share_links/{id}/`.\n\n**Parameters:**\n\n- `id: i64`: A unique integer value identifying this share link. (required)\n\n```rust,no_run\nasync fn example_share_links_update() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::ShareLink = client\n        .share_links()\n        .update(\n            4 as i64,\n            &paperless_api_client::types::ShareLinkRequest {\n                expiration: Some(chrono::Utc::now()),\n                document: Some(4 as i64),\n                file_version: Some(paperless_api_client::types::FileVersionEnum::Original),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn update<'a>(
        &'a self,
        id: i64,
        body: &crate::types::ShareLinkRequest,
    ) -> Result<crate::types::ShareLink, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/share_links/{id}/".replace("{id}", &format!("{id}"))
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

    #[doc = "Perform a `DELETE` request to `/api/share_links/{id}/`.\n\n**Parameters:**\n\n- `id: i64`: A unique integer value identifying this share link. (required)\n\n```rust,no_run\nasync fn example_share_links_destroy() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    client.share_links().destroy(4 as i64).await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn destroy<'a>(&'a self, id: i64) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/share_links/{id}/".replace("{id}", &format!("{id}"))
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

    #[doc = "Perform a `PATCH` request to `/api/share_links/{id}/`.\n\n**Parameters:**\n\n- `id: i64`: A unique integer value identifying this share link. (required)\n\n```rust,no_run\nasync fn example_share_links_partial_update() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::ShareLink = client\n        .share_links()\n        .partial_update(\n            4 as i64,\n            &paperless_api_client::types::PatchedShareLinkRequest {\n                expiration: Some(chrono::Utc::now()),\n                document: Some(4 as i64),\n                file_version: Some(paperless_api_client::types::FileVersionEnum::Original),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn partial_update<'a>(
        &'a self,
        id: i64,
        body: &crate::types::PatchedShareLinkRequest,
    ) -> Result<crate::types::ShareLink, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/share_links/{id}/".replace("{id}", &format!("{id}"))
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
