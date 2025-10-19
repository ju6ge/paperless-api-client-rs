use crate::Client;
use anyhow::Result;
#[derive(Clone, Debug)]
pub struct Config {
    pub client: Client,
}

impl Config {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Perform a `GET` request to `/api/config/`.\n\nGet the application configuration\n\nSee <https://docs.paperless-ngx.com/configuration/|Application Configuration> for more information.\n\n```rust,no_run\nasync fn example_config_list() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: Vec<paperless_api_client::types::ApplicationConfiguration> = client.config().list().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn list<'a>(
        &'a self,
    ) -> Result<Vec<crate::types::ApplicationConfiguration>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "api/config/"),
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

    #[doc = "Perform a `GET` request to `/api/config/{id}/`.\n\n**Parameters:**\n\n- `id: i64`: A unique integer value identifying this paperless application settings. (required)\n\n```rust,no_run\nasync fn example_config_retrieve() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::ApplicationConfiguration = client.config().retrieve(4 as i64).await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn retrieve<'a>(
        &'a self,
        id: i64,
    ) -> Result<crate::types::ApplicationConfiguration, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/config/{id}/".replace("{id}", &format!("{id}"))
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

    #[doc = "Perform a `PUT` request to `/api/config/{id}/`.\n\n**Parameters:**\n\n- `id: i64`: A unique integer value identifying this paperless application settings. (required)\n\n```rust,no_run\nasync fn example_config_update() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::ApplicationConfiguration = client\n        .config()\n        .update(\n            4 as i64,\n            &paperless_api_client::types::ApplicationConfigurationRequest {\n                user_args: Some(serde_json::Value::String(\"some-string\".to_string())),\n                barcode_tag_mapping: Some(serde_json::Value::String(\"some-string\".to_string())),\n                output_type: Some(paperless_api_client::types::OutputType::Pdfa2),\n                pages: Some(4 as i64),\n                language: Some(\"some-string\".to_string()),\n                mode: Some(paperless_api_client::types::Mode::SkipNoarchive),\n                skip_archive_file: Some(paperless_api_client::types::SkipArchiveFile::Always),\n                image_dpi: Some(4 as i64),\n                unpaper_clean: Some(paperless_api_client::types::UnpaperClean::None),\n                deskew: Some(true),\n                rotate_pages: Some(true),\n                rotate_pages_threshold: Some(3.14 as f64),\n                max_image_pixels: Some(3.14 as f64),\n                color_conversion_strategy: Some(paperless_api_client::types::ColorConversionStrategy::Gray),\n                app_title: Some(\"some-string\".to_string()),\n                app_logo: Some(bytes::Bytes::from(\"some-string\")),\n                barcodes_enabled: Some(true),\n                barcode_enable_tiff_support: Some(true),\n                barcode_string: Some(\"some-string\".to_string()),\n                barcode_retain_split_pages: Some(true),\n                barcode_enable_asn: Some(true),\n                barcode_asn_prefix: Some(\"some-string\".to_string()),\n                barcode_upscale: Some(3.14 as f64),\n                barcode_dpi: Some(4 as i64),\n                barcode_max_pages: Some(4 as i64),\n                barcode_enable_tag: Some(true),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn update<'a>(
        &'a self,
        id: i64,
        body: &crate::types::ApplicationConfigurationRequest,
    ) -> Result<crate::types::ApplicationConfiguration, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/config/{id}/".replace("{id}", &format!("{id}"))
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

    #[doc = "Perform a `DELETE` request to `/api/config/{id}/`.\n\n**Parameters:**\n\n- `id: i64`: A unique integer value identifying this paperless application settings. (required)\n\n```rust,no_run\nasync fn example_config_destroy() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    client.config().destroy(4 as i64).await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn destroy<'a>(&'a self, id: i64) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/config/{id}/".replace("{id}", &format!("{id}"))
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

    #[doc = "Perform a `PATCH` request to `/api/config/{id}/`.\n\n**Parameters:**\n\n- `id: i64`: A unique integer value identifying this paperless application settings. (required)\n\n```rust,no_run\nasync fn example_config_partial_update() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::ApplicationConfiguration = client\n        .config()\n        .partial_update(\n            4 as i64,\n            &paperless_api_client::types::PatchedApplicationConfigurationRequest {\n                user_args: Some(serde_json::Value::String(\"some-string\".to_string())),\n                barcode_tag_mapping: Some(serde_json::Value::String(\"some-string\".to_string())),\n                output_type: Some(paperless_api_client::types::OutputType::Pdfa2),\n                pages: Some(4 as i64),\n                language: Some(\"some-string\".to_string()),\n                mode: Some(paperless_api_client::types::Mode::SkipNoarchive),\n                skip_archive_file: Some(paperless_api_client::types::SkipArchiveFile::Always),\n                image_dpi: Some(4 as i64),\n                unpaper_clean: Some(paperless_api_client::types::UnpaperClean::None),\n                deskew: Some(true),\n                rotate_pages: Some(true),\n                rotate_pages_threshold: Some(3.14 as f64),\n                max_image_pixels: Some(3.14 as f64),\n                color_conversion_strategy: Some(paperless_api_client::types::ColorConversionStrategy::Gray),\n                app_title: Some(\"some-string\".to_string()),\n                app_logo: Some(bytes::Bytes::from(\"some-string\")),\n                barcodes_enabled: Some(true),\n                barcode_enable_tiff_support: Some(true),\n                barcode_string: Some(\"some-string\".to_string()),\n                barcode_retain_split_pages: Some(true),\n                barcode_enable_asn: Some(true),\n                barcode_asn_prefix: Some(\"some-string\".to_string()),\n                barcode_upscale: Some(3.14 as f64),\n                barcode_dpi: Some(4 as i64),\n                barcode_max_pages: Some(4 as i64),\n                barcode_enable_tag: Some(true),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn partial_update<'a>(
        &'a self,
        id: i64,
        body: &crate::types::PatchedApplicationConfigurationRequest,
    ) -> Result<crate::types::ApplicationConfiguration, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/config/{id}/".replace("{id}", &format!("{id}"))
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
