//! Paperless-ngx API client
//!
//! [![docs.rs](https://docs.rs/paperless-api-client/badge.svg)](https://docs.rs/paperless-api-client)
//!
//! ## API Details
//!
//! OpenAPI Spec for Paperless-ngx
//!
//!
//!
//!
//!
//!
//! ## Client Details
//!
//!
//!
//! The documentation for the crate is generated
//! along with the code to make this library easy to use.
//!
//!
//! To install the library, add the following to your `Cargo.toml` file.
//!
//! ```toml
//! [dependencies]
//! paperless-api-client = "6.0.0"
//! ```
//!
//! ## Basic example
//!
//! Typical use will require intializing a `Client`. This requires
//! a user agent string and set of credentials.
//!
//! ```rust,no_run
//! use paperless_api_client::Client;
//!
//! let client = Client::new(
//!     String::from("api-key"),
//! );
//! ```
//!
//! Alternatively, the library can search for most of the variables required for
//! the client in the environment:
//!
//! - `PAPERLESS_API_CLIENT_API_TOKEN`
//!
//!
//! And then you can create a client from the environment.
//!
//! ```rust,no_run
//! use paperless_api_client::Client;
//!
//! let client = Client::new_from_env();
//! ```
//!
#![allow(mismatched_lifetime_syntaxes)]
#![allow(missing_docs)]
#![allow(unused_imports)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::too_many_arguments)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "requests")]
pub mod bulk_edit_objects;
#[cfg(feature = "requests")]
pub mod config;
#[cfg(feature = "requests")]
pub mod correspondents;
#[cfg(feature = "requests")]
pub mod custom_fields;
#[cfg(feature = "requests")]
pub mod document_types;
#[cfg(feature = "requests")]
pub mod documents;
#[cfg(feature = "requests")]
pub mod groups;
#[cfg(feature = "requests")]
pub mod logs;
#[cfg(feature = "requests")]
pub mod mail_accounts;
#[cfg(feature = "requests")]
pub mod mail_rules;
mod methods;
#[cfg(feature = "requests")]
pub mod oauth;
#[cfg(feature = "requests")]
pub mod profile;
#[cfg(feature = "requests")]
pub mod remote_version;
#[cfg(feature = "requests")]
pub mod saved_views;
#[cfg(feature = "requests")]
pub mod search;
#[cfg(feature = "requests")]
pub mod share_links;
#[cfg(feature = "requests")]
pub mod statistics;
#[cfg(feature = "requests")]
pub mod status;
#[cfg(feature = "requests")]
pub mod storage_paths;
#[cfg(feature = "requests")]
pub mod tags;
#[cfg(feature = "requests")]
pub mod tasks;
#[cfg(test)]
mod tests;
#[cfg(feature = "requests")]
pub mod token;
#[cfg(feature = "requests")]
pub mod trash;
pub mod types;
#[cfg(feature = "requests")]
pub mod ui_settings;
#[cfg(feature = "requests")]
pub mod users;
#[cfg(feature = "requests")]
pub mod workflow_actions;
#[cfg(feature = "requests")]
pub mod workflow_triggers;
#[cfg(feature = "requests")]
pub mod workflows;

#[cfg(feature = "requests")]
use std::env;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "requests")]
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), ".rs/", env!("CARGO_PKG_VERSION"),);

/// Entrypoint for interacting with the API client.
#[derive(Clone, Debug)]
#[cfg(feature = "requests")]
pub struct Client {
    token: String,
    base_url: String,

    #[cfg(feature = "retry")]
    client: reqwest_middleware::ClientWithMiddleware,
    #[cfg(feature = "retry")]
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(dead_code)]
    client_http1_only: reqwest_middleware::ClientWithMiddleware,

    #[cfg(not(feature = "retry"))]
    client: reqwest::Client,
    #[cfg(not(feature = "retry"))]
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(dead_code)]
    client_http1_only: reqwest::Client,
}

/// A request builder.
#[cfg(feature = "retry")]
#[cfg(feature = "requests")]
pub struct RequestBuilder(pub reqwest_middleware::RequestBuilder);
#[cfg(not(feature = "retry"))]
#[cfg(feature = "requests")]
pub struct RequestBuilder(pub reqwest::RequestBuilder);

#[cfg(feature = "requests")]
impl Client {
    /// Create a new Client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key your requests will work.
    /// Also takes reqwest client builders, for customizing the client's behaviour.
    #[tracing::instrument(skip(token))]
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new_from_reqwest<T>(
        token: T,
        builder_http: reqwest::ClientBuilder,
        builder_websocket: reqwest::ClientBuilder,
    ) -> Self
    where
        T: ToString + std::fmt::Debug,
    {
        #[cfg(feature = "retry")]
        {
            // Retry up to 3 times with increasing intervals between attempts.
            let retry_policy =
                reqwest_retry::policies::ExponentialBackoff::builder().build_with_max_retries(3);
            match (builder_http.build(), builder_websocket.build()) {
                (Ok(c), Ok(c1)) => {
                    let client = reqwest_middleware::ClientBuilder::new(c)
                        // Trace HTTP requests. See the tracing crate to make use of these traces.
                        .with(reqwest_tracing::TracingMiddleware::default())
                        // Retry failed requests.
                        .with(reqwest_conditional_middleware::ConditionalMiddleware::new(
                            reqwest_retry::RetryTransientMiddleware::new_with_policy(retry_policy),
                            |req: &reqwest::Request| req.try_clone().is_some(),
                        ))
                        .build();
                    let client_http1_only = reqwest_middleware::ClientBuilder::new(c1)
                        .with(reqwest_tracing::TracingMiddleware::default())
                        .with(reqwest_conditional_middleware::ConditionalMiddleware::new(
                            reqwest_retry::RetryTransientMiddleware::new_with_policy(retry_policy),
                            |req: &reqwest::Request| req.try_clone().is_some(),
                        ))
                        .build();
                    Client {
                        token: token.to_string(),
                        base_url: "https://your-paperles.url/api".to_string(),

                        client,
                        client_http1_only,
                    }
                }
                (Err(e), _) | (_, Err(e)) => panic!("creating reqwest client failed: {e:?}"),
            }
        }
        #[cfg(not(feature = "retry"))]
        {
            match (builder_http.build(), builder_websocket.build()) {
                (Ok(c), Ok(c1)) => Client {
                    token: token.to_string(),
                    base_url: "https://your-paperles.url/api".to_string(),

                    client: c,
                    client_http1_only: c1,
                },
                (Err(e), _) | (_, Err(e)) => panic!("creating reqwest client failed: {:?}", e),
            }
        }
    }

    /// Create a new Client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key your requests will work.
    /// Also takes reqwest client builders, for customizing the client's behaviour.
    #[tracing::instrument(skip(token))]
    #[cfg(target_arch = "wasm32")]
    pub fn new_from_reqwest<T>(token: T, builder_http: reqwest::ClientBuilder) -> Self
    where
        T: ToString + std::fmt::Debug,
    {
        #[cfg(feature = "retry")]
        {
            // Retry up to 3 times with increasing intervals between attempts.
            let retry_policy =
                reqwest_retry::policies::ExponentialBackoff::builder().build_with_max_retries(3);
            match builder_http.build() {
                Ok(c) => {
                    let client = reqwest_middleware::ClientBuilder::new(c)
                        // Trace HTTP requests. See the tracing crate to make use of these traces.
                        .with(reqwest_tracing::TracingMiddleware::default())
                        // Retry failed requests.
                        .with(reqwest_conditional_middleware::ConditionalMiddleware::new(
                            reqwest_retry::RetryTransientMiddleware::new_with_policy(retry_policy),
                            |req: &reqwest::Request| req.try_clone().is_some(),
                        ))
                        .build();
                    Client {
                        token: token.to_string(),
                        base_url: "https://your-paperles.url/api".to_string(),

                        client,
                    }
                }
                Err(e) => panic!("creating reqwest client failed: {:?}", e),
            }
        }
        #[cfg(not(feature = "retry"))]
        {
            match builder_http.build() {
                Ok(c) => Client {
                    token: token.to_string(),
                    base_url: "https://your-paperles.url/api".to_string(),

                    client: c,
                },
                Err(e) => panic!("creating reqwest client failed: {:?}", e),
            }
        }
    }

    /// Create a new Client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key your requests will work.
    #[tracing::instrument(skip(token))]
    pub fn new<T>(token: T) -> Self
    where
        T: ToString + std::fmt::Debug,
    {
        #[cfg(not(target_arch = "wasm32"))]
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            // For file conversions we need this to be long.
            .timeout(std::time::Duration::from_secs(600))
            .connect_timeout(std::time::Duration::from_secs(60));
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::Client::builder();
        #[cfg(not(target_arch = "wasm32"))]
        let client_http1 = reqwest::Client::builder()
            // For file conversions we need this to be long.
            .user_agent(APP_USER_AGENT)
            .timeout(std::time::Duration::from_secs(600))
            .connect_timeout(std::time::Duration::from_secs(60))
            .http1_only();
        #[cfg(not(target_arch = "wasm32"))]
        return Self::new_from_reqwest(token, client, client_http1);
        #[cfg(target_arch = "wasm32")]
        Self::new_from_reqwest(token, client)
    }

    /// Set the base URL for the client to something other than the default: <https://your-paperles.url/api>.
    #[tracing::instrument]
    pub fn set_base_url<H>(&mut self, base_url: H)
    where
        H: Into<String> + std::fmt::Display + std::fmt::Debug,
    {
        self.base_url = base_url.to_string().trim_end_matches('/').to_string();
    }

    /// Create a new Client struct from the environment variable: `ENV_VARIABLE_PREFIX_API_TOKEN`.
    #[tracing::instrument]
    pub fn new_from_env() -> Self {
        let token = env::var("PAPERLESS_API_CLIENT_API_TOKEN")
            .expect("must set PAPERLESS_API_CLIENT_API_TOKEN");
        let base_url = env::var("PAPERLESS_API_CLIENT_HOST")
            .unwrap_or("https://your-paperles.url/api".to_string());

        let mut c = Client::new(token);
        c.set_base_url(base_url);
        c
    }

    /// Create a raw request to our API.
    #[tracing::instrument]
    pub async fn request_raw(
        &self,
        method: reqwest::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> anyhow::Result<RequestBuilder> {
        let u = if uri.starts_with("https://") || uri.starts_with("http://") {
            uri.to_string()
        } else {
            format!("{}/{}", self.base_url, uri.trim_start_matches('/'))
        };

        let mut req = self.client.request(method, &u);

        // Add in our authentication.
        req = req.bearer_auth(&self.token);

        // Set the default headers.
        req = req.header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        req = req.header(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        if let Some(body) = body {
            req = req.body(body);
        }

        Ok(RequestBuilder(req))
    }

    /// Return a reference to an interface that provides access to bulk_edit_objects operations.
    pub fn bulk_edit_objects(&self) -> bulk_edit_objects::BulkEditObjects {
        bulk_edit_objects::BulkEditObjects::new(self.clone())
    }

    /// Return a reference to an interface that provides access to config operations.
    pub fn config(&self) -> config::Config {
        config::Config::new(self.clone())
    }

    /// Return a reference to an interface that provides access to correspondents operations.
    pub fn correspondents(&self) -> correspondents::Correspondents {
        correspondents::Correspondents::new(self.clone())
    }

    /// Return a reference to an interface that provides access to custom_fields operations.
    pub fn custom_fields(&self) -> custom_fields::CustomFields {
        custom_fields::CustomFields::new(self.clone())
    }

    /// Return a reference to an interface that provides access to document_types operations.
    pub fn document_types(&self) -> document_types::DocumentTypes {
        document_types::DocumentTypes::new(self.clone())
    }

    /// Return a reference to an interface that provides access to documents operations.
    pub fn documents(&self) -> documents::Documents {
        documents::Documents::new(self.clone())
    }

    /// Return a reference to an interface that provides access to groups operations.
    pub fn groups(&self) -> groups::Groups {
        groups::Groups::new(self.clone())
    }

    /// Return a reference to an interface that provides access to logs operations.
    pub fn logs(&self) -> logs::Logs {
        logs::Logs::new(self.clone())
    }

    /// Return a reference to an interface that provides access to mail_accounts operations.
    pub fn mail_accounts(&self) -> mail_accounts::MailAccounts {
        mail_accounts::MailAccounts::new(self.clone())
    }

    /// Return a reference to an interface that provides access to mail_rules operations.
    pub fn mail_rules(&self) -> mail_rules::MailRules {
        mail_rules::MailRules::new(self.clone())
    }

    /// Return a reference to an interface that provides access to oauth operations.
    pub fn oauth(&self) -> oauth::Oauth {
        oauth::Oauth::new(self.clone())
    }

    /// Return a reference to an interface that provides access to profile operations.
    pub fn profile(&self) -> profile::Profile {
        profile::Profile::new(self.clone())
    }

    /// Return a reference to an interface that provides access to remote_version operations.
    pub fn remote_version(&self) -> remote_version::RemoteVersion {
        remote_version::RemoteVersion::new(self.clone())
    }

    /// Return a reference to an interface that provides access to saved_views operations.
    pub fn saved_views(&self) -> saved_views::SavedViews {
        saved_views::SavedViews::new(self.clone())
    }

    /// Return a reference to an interface that provides access to search operations.
    pub fn search(&self) -> search::Search {
        search::Search::new(self.clone())
    }

    /// Return a reference to an interface that provides access to share_links operations.
    pub fn share_links(&self) -> share_links::ShareLinks {
        share_links::ShareLinks::new(self.clone())
    }

    /// Return a reference to an interface that provides access to statistics operations.
    pub fn statistics(&self) -> statistics::Statistics {
        statistics::Statistics::new(self.clone())
    }

    /// Return a reference to an interface that provides access to status operations.
    pub fn status(&self) -> status::Status {
        status::Status::new(self.clone())
    }

    /// Return a reference to an interface that provides access to storage_paths operations.
    pub fn storage_paths(&self) -> storage_paths::StoragePaths {
        storage_paths::StoragePaths::new(self.clone())
    }

    /// Return a reference to an interface that provides access to tags operations.
    pub fn tags(&self) -> tags::Tags {
        tags::Tags::new(self.clone())
    }

    /// Return a reference to an interface that provides access to tasks operations.
    pub fn tasks(&self) -> tasks::Tasks {
        tasks::Tasks::new(self.clone())
    }

    /// Return a reference to an interface that provides access to token operations.
    pub fn token(&self) -> token::Token {
        token::Token::new(self.clone())
    }

    /// Return a reference to an interface that provides access to trash operations.
    pub fn trash(&self) -> trash::Trash {
        trash::Trash::new(self.clone())
    }

    /// Return a reference to an interface that provides access to ui_settings operations.
    pub fn ui_settings(&self) -> ui_settings::UiSettings {
        ui_settings::UiSettings::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users operations.
    pub fn users(&self) -> users::Users {
        users::Users::new(self.clone())
    }

    /// Return a reference to an interface that provides access to workflow_actions operations.
    pub fn workflow_actions(&self) -> workflow_actions::WorkflowActions {
        workflow_actions::WorkflowActions::new(self.clone())
    }

    /// Return a reference to an interface that provides access to workflow_triggers operations.
    pub fn workflow_triggers(&self) -> workflow_triggers::WorkflowTriggers {
        workflow_triggers::WorkflowTriggers::new(self.clone())
    }

    /// Return a reference to an interface that provides access to workflows operations.
    pub fn workflows(&self) -> workflows::Workflows {
        workflows::Workflows::new(self.clone())
    }
}
