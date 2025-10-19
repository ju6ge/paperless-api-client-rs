use crate::Client;
use anyhow::Result;
#[derive(Clone, Debug)]
pub struct MailAccounts {
    pub client: Client,
}

impl MailAccounts {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Perform a `GET` request to `/api/mail_accounts/`.\n\n**Parameters:**\n\n- `page: Option<i64>`: A page number within the paginated result set.\n- `page_size: Option<i64>`: Number of results to return per page.\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_mail_accounts_list_stream() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let mut mail_accounts = client.mail_accounts();\n    let mut stream = mail_accounts.list_stream(Some(4 as i64));\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn list<'a>(
        &'a self,
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> Result<crate::types::PaginatedMailAccountList, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "api/mail_accounts/"),
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

    #[doc = "Perform a `GET` request to `/api/mail_accounts/`.\n\n**Parameters:**\n\n- `page: Option<i64>`: A page number within the paginated result set.\n- `page_size: Option<i64>`: Number of results to return per page.\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_mail_accounts_list_stream() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let mut mail_accounts = client.mail_accounts();\n    let mut stream = mail_accounts.list_stream(Some(4 as i64));\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    #[allow(non_snake_case)]
    pub fn list_stream<'a>(
        &'a self,
        page_size: Option<i64>,
    ) -> impl futures::Stream<Item = Result<crate::types::MailAccount, crate::types::error::Error>>
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
                                    format!("{}/{}", self.client.base_url, "api/mail_accounts/"),
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
                            .map_ok(|result: crate::types::PaginatedMailAccountList| {
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

    #[doc = "Perform a `POST` request to `/api/mail_accounts/`.\n\n```rust,no_run\nasync fn example_mail_accounts_create() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::MailAccount = client\n        .mail_accounts()\n        .create(&paperless_api_client::types::MailAccountRequest {\n            name: \"some-string\".to_string(),\n            imap_server: \"some-string\".to_string(),\n            imap_port: Some(4 as i64),\n            imap_security: Some(4 as i64),\n            username: \"some-string\".to_string(),\n            password: \"some-string\".to_string(),\n            character_set: Some(\"some-string\".to_string()),\n            is_token: Some(true),\n            owner: Some(4 as i64),\n            set_permissions: Some(paperless_api_client::types::SetPermissions {\n                view: Some(paperless_api_client::types::View {\n                    users: Some(vec![4 as i64]),\n                    groups: Some(vec![4 as i64]),\n                }),\n                change: Some(paperless_api_client::types::Change {\n                    users: Some(vec![4 as i64]),\n                    groups: Some(vec![4 as i64]),\n                }),\n            }),\n            account_type: Some(4 as i64),\n            expiration: Some(chrono::Utc::now()),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn create<'a>(
        &'a self,
        body: &crate::types::MailAccountRequest,
    ) -> Result<crate::types::MailAccount, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "api/mail_accounts/"),
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

    #[doc = "Perform a `GET` request to `/api/mail_accounts/{id}/`.\n\n**Parameters:**\n\n- `id: i64`: A unique integer value identifying this mail account. (required)\n\n```rust,no_run\nasync fn example_mail_accounts_retrieve() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::MailAccount = client.mail_accounts().retrieve(4 as i64).await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn retrieve<'a>(
        &'a self,
        id: i64,
    ) -> Result<crate::types::MailAccount, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/mail_accounts/{id}/".replace("{id}", &format!("{id}"))
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

    #[doc = "Perform a `PUT` request to `/api/mail_accounts/{id}/`.\n\n**Parameters:**\n\n- `id: i64`: A unique integer value identifying this mail account. (required)\n\n```rust,no_run\nasync fn example_mail_accounts_update() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::MailAccount = client\n        .mail_accounts()\n        .update(\n            4 as i64,\n            &paperless_api_client::types::MailAccountRequest {\n                name: \"some-string\".to_string(),\n                imap_server: \"some-string\".to_string(),\n                imap_port: Some(4 as i64),\n                imap_security: Some(4 as i64),\n                username: \"some-string\".to_string(),\n                password: \"some-string\".to_string(),\n                character_set: Some(\"some-string\".to_string()),\n                is_token: Some(true),\n                owner: Some(4 as i64),\n                set_permissions: Some(paperless_api_client::types::SetPermissions {\n                    view: Some(paperless_api_client::types::View {\n                        users: Some(vec![4 as i64]),\n                        groups: Some(vec![4 as i64]),\n                    }),\n                    change: Some(paperless_api_client::types::Change {\n                        users: Some(vec![4 as i64]),\n                        groups: Some(vec![4 as i64]),\n                    }),\n                }),\n                account_type: Some(4 as i64),\n                expiration: Some(chrono::Utc::now()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn update<'a>(
        &'a self,
        id: i64,
        body: &crate::types::MailAccountRequest,
    ) -> Result<crate::types::MailAccount, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/mail_accounts/{id}/".replace("{id}", &format!("{id}"))
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

    #[doc = "Perform a `DELETE` request to `/api/mail_accounts/{id}/`.\n\n**Parameters:**\n\n- `id: i64`: A unique integer value identifying this mail account. (required)\n\n```rust,no_run\nasync fn example_mail_accounts_destroy() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    client.mail_accounts().destroy(4 as i64).await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn destroy<'a>(&'a self, id: i64) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/mail_accounts/{id}/".replace("{id}", &format!("{id}"))
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

    #[doc = "Perform a `PATCH` request to `/api/mail_accounts/{id}/`.\n\n**Parameters:**\n\n- `id: i64`: A unique integer value identifying this mail account. (required)\n\n```rust,no_run\nasync fn example_mail_accounts_partial_update() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::MailAccount = client\n        .mail_accounts()\n        .partial_update(\n            4 as i64,\n            &paperless_api_client::types::PatchedMailAccountRequest {\n                name: Some(\"some-string\".to_string()),\n                imap_server: Some(\"some-string\".to_string()),\n                imap_port: Some(4 as i64),\n                imap_security: Some(4 as i64),\n                username: Some(\"some-string\".to_string()),\n                password: Some(\"some-string\".to_string()),\n                character_set: Some(\"some-string\".to_string()),\n                is_token: Some(true),\n                owner: Some(4 as i64),\n                set_permissions: Some(paperless_api_client::types::SetPermissions {\n                    view: Some(paperless_api_client::types::View {\n                        users: Some(vec![4 as i64]),\n                        groups: Some(vec![4 as i64]),\n                    }),\n                    change: Some(paperless_api_client::types::Change {\n                        users: Some(vec![4 as i64]),\n                        groups: Some(vec![4 as i64]),\n                    }),\n                }),\n                account_type: Some(4 as i64),\n                expiration: Some(chrono::Utc::now()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn partial_update<'a>(
        &'a self,
        id: i64,
        body: &crate::types::PatchedMailAccountRequest,
    ) -> Result<crate::types::MailAccount, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/mail_accounts/{id}/".replace("{id}", &format!("{id}"))
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

    #[doc = "Perform a `POST` request to `/api/mail_accounts/{id}/process/`.\n\nManually process the selected mail account for new messages.\n\n**Parameters:**\n\n- `id: i64`: A unique integer value identifying this mail account. (required)\n\n```rust,no_run\nasync fn example_mail_accounts_process() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::MailAccountProcessResponse = client\n        .mail_accounts()\n        .process(\n            4 as i64,\n            &paperless_api_client::types::MailAccountRequest {\n                name: \"some-string\".to_string(),\n                imap_server: \"some-string\".to_string(),\n                imap_port: Some(4 as i64),\n                imap_security: Some(4 as i64),\n                username: \"some-string\".to_string(),\n                password: \"some-string\".to_string(),\n                character_set: Some(\"some-string\".to_string()),\n                is_token: Some(true),\n                owner: Some(4 as i64),\n                set_permissions: Some(paperless_api_client::types::SetPermissions {\n                    view: Some(paperless_api_client::types::View {\n                        users: Some(vec![4 as i64]),\n                        groups: Some(vec![4 as i64]),\n                    }),\n                    change: Some(paperless_api_client::types::Change {\n                        users: Some(vec![4 as i64]),\n                        groups: Some(vec![4 as i64]),\n                    }),\n                }),\n                account_type: Some(4 as i64),\n                expiration: Some(chrono::Utc::now()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn process<'a>(
        &'a self,
        id: i64,
        body: &crate::types::MailAccountRequest,
    ) -> Result<crate::types::MailAccountProcessResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/mail_accounts/{id}/process/".replace("{id}", &format!("{id}"))
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

    #[doc = "Perform a `POST` request to `/api/mail_accounts/test/`.\n\nTest a mail account\n\n```rust,no_run\nasync fn example_mail_accounts_test() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::MailAccountTestResponse = client\n        .mail_accounts()\n        .test(&paperless_api_client::types::MailAccountRequest {\n            name: \"some-string\".to_string(),\n            imap_server: \"some-string\".to_string(),\n            imap_port: Some(4 as i64),\n            imap_security: Some(4 as i64),\n            username: \"some-string\".to_string(),\n            password: \"some-string\".to_string(),\n            character_set: Some(\"some-string\".to_string()),\n            is_token: Some(true),\n            owner: Some(4 as i64),\n            set_permissions: Some(paperless_api_client::types::SetPermissions {\n                view: Some(paperless_api_client::types::View {\n                    users: Some(vec![4 as i64]),\n                    groups: Some(vec![4 as i64]),\n                }),\n                change: Some(paperless_api_client::types::Change {\n                    users: Some(vec![4 as i64]),\n                    groups: Some(vec![4 as i64]),\n                }),\n            }),\n            account_type: Some(4 as i64),\n            expiration: Some(chrono::Utc::now()),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn test<'a>(
        &'a self,
        body: &crate::types::MailAccountRequest,
    ) -> Result<crate::types::MailAccountTestResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "api/mail_accounts/test/"),
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
