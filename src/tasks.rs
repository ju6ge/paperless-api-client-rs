use crate::Client;
use anyhow::Result;
#[derive(Clone, Debug)]
pub struct Tasks {
    pub client: Client,
}

impl Tasks {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Perform a `GET` request to `/api/tasks/`.\n\n**Parameters:**\n\n- `acknowledged: Option<bool>`: Acknowledged\n- `ordering: Option<String>`: Which field to use when ordering the results.\n- `status: Option<crate::types::Status>`: Current state of the task being run\n\n* `FAILURE` - FAILURE\n* `PENDING` - PENDING\n* `RECEIVED` - RECEIVED\n* `RETRY` - RETRY\n* `REVOKED` - REVOKED\n* `STARTED` - STARTED\n* `SUCCESS` - SUCCESS\n- `task_id: Option<String>`: Filter tasks by Celery UUID\n- `task_name: Option<crate::types::ListTaskName>`: Name of the task that was run\n\n* `consume_file` - Consume File\n* `train_classifier` - Train Classifier\n* `check_sanity` - Check Sanity\n* `index_optimize` - Index Optimize\n- `type_: Option<crate::types::Type>`: The type of task that was run\n\n* `auto_task` - Auto Task\n* `scheduled_task` - Scheduled Task\n* `manual_task` - Manual Task\n\n```rust,no_run\nasync fn example_tasks_list() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: Vec<paperless_api_client::types::TasksView> = client\n        .tasks()\n        .list(\n            Some(true),\n            Some(\"some-string\".to_string()),\n            Some(paperless_api_client::types::Status::Revoked),\n            Some(\"some-string\".to_string()),\n            Some(paperless_api_client::types::OptionListTaskName::IndexOptimize),\n            Some(paperless_api_client::types::Type::ManualTask),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn list<'a>(
        &'a self,
        acknowledged: Option<bool>,
        ordering: Option<String>,
        status: Option<crate::types::Status>,
        task_id: Option<String>,
        task_name: Option<crate::types::ListTaskName>,
        type_: Option<crate::types::Type>,
    ) -> Result<Vec<crate::types::TasksView>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "api/tasks/"),
        );
        req = req.header("Authorization", format!("Token {}", &self.client.token));
        let mut query_params = vec![];
        if let Some(p) = acknowledged {
            query_params.push(("acknowledged", format!("{p}")));
        }

        if let Some(p) = ordering {
            query_params.push(("ordering", p));
        }

        if let Some(p) = status {
            query_params.push(("status", format!("{p}")));
        }

        if let Some(p) = task_id {
            query_params.push(("task_id", p));
        }

        if let Some(p) = task_name {
            query_params.push(("task_name", format!("{p}")));
        }

        if let Some(p) = type_ {
            query_params.push(("type", format!("{p}")));
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

    #[doc = "Perform a `GET` request to `/api/tasks/{id}/`.\n\n**Parameters:**\n\n- `id: i64`: A unique integer value identifying this paperless task. (required)\n- `task_id: Option<String>`: Filter tasks by Celery UUID\n\n```rust,no_run\nasync fn example_tasks_retrieve() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::TasksView = client\n        .tasks()\n        .retrieve(4 as i64, Some(\"some-string\".to_string()))\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn retrieve<'a>(
        &'a self,
        id: i64,
        task_id: Option<String>,
    ) -> Result<crate::types::TasksView, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/tasks/{id}/".replace("{id}", &format!("{id}"))
            ),
        );
        req = req.header("Authorization", format!("Token {}", &self.client.token));
        let mut query_params = vec![];
        if let Some(p) = task_id {
            query_params.push(("task_id", p));
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

    #[doc = "Perform a `POST` request to `/api/tasks/acknowledge/`.\n\nAcknowledge a list of tasks\n\n**Parameters:**\n\n- `task_id: Option<String>`: Filter tasks by Celery UUID\n\n```rust,no_run\nasync fn example_tasks_acknowledge() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::AcknowledgeTasks = client\n        .tasks()\n        .acknowledge(\n            Some(\"some-string\".to_string()),\n            &paperless_api_client::types::AcknowledgeTasksRequestBody {\n                tasks: vec![4 as i64],\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn acknowledge<'a>(
        &'a self,
        task_id: Option<String>,
        body: &crate::types::AcknowledgeTasksRequestBody,
    ) -> Result<crate::types::AcknowledgeTasks, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "api/tasks/acknowledge/"),
        );
        req = req.header("Authorization", format!("Token {}", &self.client.token));
        let mut query_params = vec![];
        if let Some(p) = task_id {
            query_params.push(("task_id", p));
        }

        req = req.query(&query_params);
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

    #[doc = "Perform a `POST` request to `/api/tasks/run/`.\n\n**Parameters:**\n\n- `task_id: Option<String>`: Filter tasks by Celery UUID\n\n```rust,no_run\nasync fn example_tasks_run_create() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::TasksView = client\n        .tasks()\n        .run_create(\n            Some(\"some-string\".to_string()),\n            &paperless_api_client::types::TasksViewRequest {\n                task_id: \"some-string\".to_string(),\n                task_name: Some(paperless_api_client::types::TaskName::CheckSanity),\n                task_file_name: Some(\"some-string\".to_string()),\n                date_created: Some(chrono::Utc::now()),\n                date_done: Some(chrono::Utc::now()),\n                type_: Some(paperless_api_client::types::TasksViewTypeEnum::ScheduledTask),\n                status: Some(paperless_api_client::types::StatusEnum::Revoked),\n                result: Some(\"some-string\".to_string()),\n                acknowledged: Some(true),\n                owner: Some(4 as i64),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn run_create<'a>(
        &'a self,
        task_id: Option<String>,
        body: &crate::types::TasksViewRequest,
    ) -> Result<crate::types::TasksView, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "api/tasks/run/"),
        );
        req = req.header("Authorization", format!("Token {}", &self.client.token));
        let mut query_params = vec![];
        if let Some(p) = task_id {
            query_params.push(("task_id", p));
        }

        req = req.query(&query_params);
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
