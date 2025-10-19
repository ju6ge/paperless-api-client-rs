use crate::Client;
use anyhow::Result;
#[derive(Clone, Debug)]
pub struct Documents {
    pub client: Client,
}

impl Documents {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Perform a `GET` request to `/api/documents/`.\n\nDocument views including search\n\n**Parameters:**\n\n- `added_date_gt: Option<chrono::NaiveDate>`\n- `added_date_gte: Option<chrono::NaiveDate>`\n- `added_date_lt: Option<chrono::NaiveDate>`\n- `added_date_lte: Option<chrono::NaiveDate>`\n- `added_day: Option<f64>`\n- `added_gt: Option<chrono::DateTime<chrono::Utc>>`\n- `added_gte: Option<chrono::DateTime<chrono::Utc>>`\n- `added_lt: Option<chrono::DateTime<chrono::Utc>>`\n- `added_lte: Option<chrono::DateTime<chrono::Utc>>`\n- `added_month: Option<f64>`\n- `added_year: Option<f64>`\n- `archive_serial_number: Option<i64>`\n- `archive_serial_number_gt: Option<i64>`\n- `archive_serial_number_gte: Option<i64>`\n- `archive_serial_number_isnull: Option<bool>`\n- `archive_serial_number_lt: Option<i64>`\n- `archive_serial_number_lte: Option<i64>`\n- `checksum_icontains: Option<String>`\n- `checksum_iendswith: Option<String>`\n- `checksum_iexact: Option<String>`\n- `checksum_istartswith: Option<String>`\n- `content_icontains: Option<String>`\n- `content_iendswith: Option<String>`\n- `content_iexact: Option<String>`\n- `content_istartswith: Option<String>`\n- `correspondent_id: Option<i64>`\n- `correspondent_id_in: Option<Vec<i64>>`: Multiple values may be separated by commas.\n- `correspondent_id_none: Option<i64>`\n- `correspondent_isnull: Option<bool>`\n- `correspondent_name_icontains: Option<String>`\n- `correspondent_name_iendswith: Option<String>`\n- `correspondent_name_iexact: Option<String>`\n- `correspondent_name_istartswith: Option<String>`\n- `created_date_gt: Option<chrono::NaiveDate>`\n- `created_date_gte: Option<chrono::NaiveDate>`\n- `created_date_lt: Option<chrono::NaiveDate>`\n- `created_date_lte: Option<chrono::NaiveDate>`\n- `created_day: Option<f64>`\n- `created_gt: Option<chrono::NaiveDate>`\n- `created_gte: Option<chrono::NaiveDate>`\n- `created_lt: Option<chrono::NaiveDate>`\n- `created_lte: Option<chrono::NaiveDate>`\n- `created_month: Option<f64>`\n- `created_year: Option<f64>`\n- `custom_field_query: Option<String>`\n- `custom_fields_icontains: Option<String>`\n- `custom_fields_id_all: Option<i64>`\n- `custom_fields_id_in: Option<i64>`\n- `custom_fields_id_none: Option<i64>`\n- `document_type_id: Option<i64>`\n- `document_type_id_in: Option<Vec<i64>>`: Multiple values may be separated by commas.\n- `document_type_id_none: Option<i64>`\n- `document_type_isnull: Option<bool>`\n- `document_type_name_icontains: Option<String>`\n- `document_type_name_iendswith: Option<String>`\n- `document_type_name_iexact: Option<String>`\n- `document_type_name_istartswith: Option<String>`\n- `fields: Option<Vec<String>>`\n- `full_perms: Option<bool>`\n- `has_custom_fields: Option<bool>`: Has custom field\n- `id: Option<i64>`\n- `id_in: Option<Vec<i64>>`: Multiple values may be separated by commas.\n- `is_in_inbox: Option<bool>`\n- `is_tagged: Option<bool>`: Is tagged\n- `mime_type: Option<String>`\n- `modified_date_gt: Option<chrono::NaiveDate>`\n- `modified_date_gte: Option<chrono::NaiveDate>`\n- `modified_date_lt: Option<chrono::NaiveDate>`\n- `modified_date_lte: Option<chrono::NaiveDate>`\n- `modified_day: Option<f64>`\n- `modified_gt: Option<chrono::DateTime<chrono::Utc>>`\n- `modified_gte: Option<chrono::DateTime<chrono::Utc>>`\n- `modified_lt: Option<chrono::DateTime<chrono::Utc>>`\n- `modified_lte: Option<chrono::DateTime<chrono::Utc>>`\n- `modified_month: Option<f64>`\n- `modified_year: Option<f64>`\n- `ordering: Option<String>`: Which field to use when ordering the results.\n- `original_filename_icontains: Option<String>`\n- `original_filename_iendswith: Option<String>`\n- `original_filename_iexact: Option<String>`\n- `original_filename_istartswith: Option<String>`\n- `owner_id: Option<i64>`\n- `owner_id_in: Option<Vec<i64>>`: Multiple values may be separated by commas.\n- `owner_id_none: Option<i64>`\n- `owner_isnull: Option<bool>`\n- `page: Option<i64>`: A page number within the paginated result set.\n- `page_size: Option<i64>`: Number of results to return per page.\n- `query: Option<String>`: Advanced search query string\n- `search: Option<String>`: A search term.\n- `shared_by_id: Option<bool>`\n- `storage_path_id: Option<i64>`\n- `storage_path_id_in: Option<Vec<i64>>`: Multiple values may be separated by commas.\n- `storage_path_id_none: Option<i64>`\n- `storage_path_isnull: Option<bool>`\n- `storage_path_name_icontains: Option<String>`\n- `storage_path_name_iendswith: Option<String>`\n- `storage_path_name_iexact: Option<String>`\n- `storage_path_name_istartswith: Option<String>`\n- `tags_id: Option<i64>`\n- `tags_id_all: Option<i64>`\n- `tags_id_in: Option<i64>`\n- `tags_id_none: Option<i64>`\n- `tags_name_icontains: Option<String>`\n- `tags_name_iendswith: Option<String>`\n- `tags_name_iexact: Option<String>`\n- `tags_name_istartswith: Option<String>`\n- `title_icontains: Option<String>`\n- `title_iendswith: Option<String>`\n- `title_iexact: Option<String>`\n- `title_istartswith: Option<String>`\n- `title_content: Option<String>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_documents_list_stream() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let mut documents = client.documents();\n    let mut stream = documents.list_stream(\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(3.14 as f64),\n        Some(chrono::Utc::now()),\n        Some(chrono::Utc::now()),\n        Some(chrono::Utc::now()),\n        Some(chrono::Utc::now()),\n        Some(3.14 as f64),\n        Some(3.14 as f64),\n        Some(4 as i64),\n        Some(4 as i64),\n        Some(4 as i64),\n        Some(true),\n        Some(4 as i64),\n        Some(4 as i64),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(4 as i64),\n        Some(vec![4 as i64]),\n        Some(4 as i64),\n        Some(true),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(3.14 as f64),\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(3.14 as f64),\n        Some(3.14 as f64),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(4 as i64),\n        Some(4 as i64),\n        Some(4 as i64),\n        Some(4 as i64),\n        Some(vec![4 as i64]),\n        Some(4 as i64),\n        Some(true),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(vec![\"some-string\".to_string()]),\n        Some(true),\n        Some(true),\n        Some(4 as i64),\n        Some(vec![4 as i64]),\n        Some(true),\n        Some(true),\n        Some(\"some-string\".to_string()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(3.14 as f64),\n        Some(chrono::Utc::now()),\n        Some(chrono::Utc::now()),\n        Some(chrono::Utc::now()),\n        Some(chrono::Utc::now()),\n        Some(3.14 as f64),\n        Some(3.14 as f64),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(4 as i64),\n        Some(vec![4 as i64]),\n        Some(4 as i64),\n        Some(true),\n        Some(4 as i64),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(true),\n        Some(4 as i64),\n        Some(vec![4 as i64]),\n        Some(4 as i64),\n        Some(true),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(4 as i64),\n        Some(4 as i64),\n        Some(4 as i64),\n        Some(4 as i64),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn list<'a>(
        &'a self,
        added_date_gt: Option<chrono::NaiveDate>,
        added_date_gte: Option<chrono::NaiveDate>,
        added_date_lt: Option<chrono::NaiveDate>,
        added_date_lte: Option<chrono::NaiveDate>,
        added_day: Option<f64>,
        added_gt: Option<chrono::DateTime<chrono::Utc>>,
        added_gte: Option<chrono::DateTime<chrono::Utc>>,
        added_lt: Option<chrono::DateTime<chrono::Utc>>,
        added_lte: Option<chrono::DateTime<chrono::Utc>>,
        added_month: Option<f64>,
        added_year: Option<f64>,
        archive_serial_number: Option<i64>,
        archive_serial_number_gt: Option<i64>,
        archive_serial_number_gte: Option<i64>,
        archive_serial_number_isnull: Option<bool>,
        archive_serial_number_lt: Option<i64>,
        archive_serial_number_lte: Option<i64>,
        checksum_icontains: Option<String>,
        checksum_iendswith: Option<String>,
        checksum_iexact: Option<String>,
        checksum_istartswith: Option<String>,
        content_icontains: Option<String>,
        content_iendswith: Option<String>,
        content_iexact: Option<String>,
        content_istartswith: Option<String>,
        correspondent_id: Option<i64>,
        correspondent_id_in: Option<Vec<i64>>,
        correspondent_id_none: Option<i64>,
        correspondent_isnull: Option<bool>,
        correspondent_name_icontains: Option<String>,
        correspondent_name_iendswith: Option<String>,
        correspondent_name_iexact: Option<String>,
        correspondent_name_istartswith: Option<String>,
        created_date_gt: Option<chrono::NaiveDate>,
        created_date_gte: Option<chrono::NaiveDate>,
        created_date_lt: Option<chrono::NaiveDate>,
        created_date_lte: Option<chrono::NaiveDate>,
        created_day: Option<f64>,
        created_gt: Option<chrono::NaiveDate>,
        created_gte: Option<chrono::NaiveDate>,
        created_lt: Option<chrono::NaiveDate>,
        created_lte: Option<chrono::NaiveDate>,
        created_month: Option<f64>,
        created_year: Option<f64>,
        custom_field_query: Option<String>,
        custom_fields_icontains: Option<String>,
        custom_fields_id_all: Option<i64>,
        custom_fields_id_in: Option<i64>,
        custom_fields_id_none: Option<i64>,
        document_type_id: Option<i64>,
        document_type_id_in: Option<Vec<i64>>,
        document_type_id_none: Option<i64>,
        document_type_isnull: Option<bool>,
        document_type_name_icontains: Option<String>,
        document_type_name_iendswith: Option<String>,
        document_type_name_iexact: Option<String>,
        document_type_name_istartswith: Option<String>,
        fields: Option<Vec<String>>,
        full_perms: Option<bool>,
        has_custom_fields: Option<bool>,
        id: Option<i64>,
        id_in: Option<Vec<i64>>,
        is_in_inbox: Option<bool>,
        is_tagged: Option<bool>,
        mime_type: Option<String>,
        modified_date_gt: Option<chrono::NaiveDate>,
        modified_date_gte: Option<chrono::NaiveDate>,
        modified_date_lt: Option<chrono::NaiveDate>,
        modified_date_lte: Option<chrono::NaiveDate>,
        modified_day: Option<f64>,
        modified_gt: Option<chrono::DateTime<chrono::Utc>>,
        modified_gte: Option<chrono::DateTime<chrono::Utc>>,
        modified_lt: Option<chrono::DateTime<chrono::Utc>>,
        modified_lte: Option<chrono::DateTime<chrono::Utc>>,
        modified_month: Option<f64>,
        modified_year: Option<f64>,
        ordering: Option<String>,
        original_filename_icontains: Option<String>,
        original_filename_iendswith: Option<String>,
        original_filename_iexact: Option<String>,
        original_filename_istartswith: Option<String>,
        owner_id: Option<i64>,
        owner_id_in: Option<Vec<i64>>,
        owner_id_none: Option<i64>,
        owner_isnull: Option<bool>,
        page: Option<i64>,
        page_size: Option<i64>,
        query: Option<String>,
        search: Option<String>,
        shared_by_id: Option<bool>,
        storage_path_id: Option<i64>,
        storage_path_id_in: Option<Vec<i64>>,
        storage_path_id_none: Option<i64>,
        storage_path_isnull: Option<bool>,
        storage_path_name_icontains: Option<String>,
        storage_path_name_iendswith: Option<String>,
        storage_path_name_iexact: Option<String>,
        storage_path_name_istartswith: Option<String>,
        tags_id: Option<i64>,
        tags_id_all: Option<i64>,
        tags_id_in: Option<i64>,
        tags_id_none: Option<i64>,
        tags_name_icontains: Option<String>,
        tags_name_iendswith: Option<String>,
        tags_name_iexact: Option<String>,
        tags_name_istartswith: Option<String>,
        title_icontains: Option<String>,
        title_iendswith: Option<String>,
        title_iexact: Option<String>,
        title_istartswith: Option<String>,
        title_content: Option<String>,
    ) -> Result<crate::types::PaginatedDocumentList, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "api/documents/"),
        );
        req = req.header("Authorization", format!("Token {}", &self.client.token));
        let mut query_params = vec![];
        if let Some(p) = added_date_gt {
            query_params.push(("added__date__gt", format!("{p}")));
        }

        if let Some(p) = added_date_gte {
            query_params.push(("added__date__gte", format!("{p}")));
        }

        if let Some(p) = added_date_lt {
            query_params.push(("added__date__lt", format!("{p}")));
        }

        if let Some(p) = added_date_lte {
            query_params.push(("added__date__lte", format!("{p}")));
        }

        if let Some(p) = added_day {
            query_params.push(("added__day", format!("{p}")));
        }

        if let Some(p) = added_gt {
            query_params.push(("added__gt", format!("{p}")));
        }

        if let Some(p) = added_gte {
            query_params.push(("added__gte", format!("{p}")));
        }

        if let Some(p) = added_lt {
            query_params.push(("added__lt", format!("{p}")));
        }

        if let Some(p) = added_lte {
            query_params.push(("added__lte", format!("{p}")));
        }

        if let Some(p) = added_month {
            query_params.push(("added__month", format!("{p}")));
        }

        if let Some(p) = added_year {
            query_params.push(("added__year", format!("{p}")));
        }

        if let Some(p) = archive_serial_number {
            query_params.push(("archive_serial_number", format!("{p}")));
        }

        if let Some(p) = archive_serial_number_gt {
            query_params.push(("archive_serial_number__gt", format!("{p}")));
        }

        if let Some(p) = archive_serial_number_gte {
            query_params.push(("archive_serial_number__gte", format!("{p}")));
        }

        if let Some(p) = archive_serial_number_isnull {
            query_params.push(("archive_serial_number__isnull", format!("{p}")));
        }

        if let Some(p) = archive_serial_number_lt {
            query_params.push(("archive_serial_number__lt", format!("{p}")));
        }

        if let Some(p) = archive_serial_number_lte {
            query_params.push(("archive_serial_number__lte", format!("{p}")));
        }

        if let Some(p) = checksum_icontains {
            query_params.push(("checksum__icontains", p));
        }

        if let Some(p) = checksum_iendswith {
            query_params.push(("checksum__iendswith", p));
        }

        if let Some(p) = checksum_iexact {
            query_params.push(("checksum__iexact", p));
        }

        if let Some(p) = checksum_istartswith {
            query_params.push(("checksum__istartswith", p));
        }

        if let Some(p) = content_icontains {
            query_params.push(("content__icontains", p));
        }

        if let Some(p) = content_iendswith {
            query_params.push(("content__iendswith", p));
        }

        if let Some(p) = content_iexact {
            query_params.push(("content__iexact", p));
        }

        if let Some(p) = content_istartswith {
            query_params.push(("content__istartswith", p));
        }

        if let Some(p) = correspondent_id {
            query_params.push(("correspondent__id", format!("{p}")));
        }

        if let Some(p) = correspondent_id_in {
            query_params.push(("correspondent__id__in", itertools::join(p, ",")));
        }

        if let Some(p) = correspondent_id_none {
            query_params.push(("correspondent__id__none", format!("{p}")));
        }

        if let Some(p) = correspondent_isnull {
            query_params.push(("correspondent__isnull", format!("{p}")));
        }

        if let Some(p) = correspondent_name_icontains {
            query_params.push(("correspondent__name__icontains", p));
        }

        if let Some(p) = correspondent_name_iendswith {
            query_params.push(("correspondent__name__iendswith", p));
        }

        if let Some(p) = correspondent_name_iexact {
            query_params.push(("correspondent__name__iexact", p));
        }

        if let Some(p) = correspondent_name_istartswith {
            query_params.push(("correspondent__name__istartswith", p));
        }

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

        if let Some(p) = custom_field_query {
            query_params.push(("custom_field_query", p));
        }

        if let Some(p) = custom_fields_icontains {
            query_params.push(("custom_fields__icontains", p));
        }

        if let Some(p) = custom_fields_id_all {
            query_params.push(("custom_fields__id__all", format!("{p}")));
        }

        if let Some(p) = custom_fields_id_in {
            query_params.push(("custom_fields__id__in", format!("{p}")));
        }

        if let Some(p) = custom_fields_id_none {
            query_params.push(("custom_fields__id__none", format!("{p}")));
        }

        if let Some(p) = document_type_id {
            query_params.push(("document_type__id", format!("{p}")));
        }

        if let Some(p) = document_type_id_in {
            query_params.push(("document_type__id__in", itertools::join(p, ",")));
        }

        if let Some(p) = document_type_id_none {
            query_params.push(("document_type__id__none", format!("{p}")));
        }

        if let Some(p) = document_type_isnull {
            query_params.push(("document_type__isnull", format!("{p}")));
        }

        if let Some(p) = document_type_name_icontains {
            query_params.push(("document_type__name__icontains", p));
        }

        if let Some(p) = document_type_name_iendswith {
            query_params.push(("document_type__name__iendswith", p));
        }

        if let Some(p) = document_type_name_iexact {
            query_params.push(("document_type__name__iexact", p));
        }

        if let Some(p) = document_type_name_istartswith {
            query_params.push(("document_type__name__istartswith", p));
        }

        if let Some(p) = fields {
            query_params.push(("fields", itertools::join(p, ",")));
        }

        if let Some(p) = full_perms {
            query_params.push(("full_perms", format!("{p}")));
        }

        if let Some(p) = has_custom_fields {
            query_params.push(("has_custom_fields", format!("{p}")));
        }

        if let Some(p) = id {
            query_params.push(("id", format!("{p}")));
        }

        if let Some(p) = id_in {
            query_params.push(("id__in", itertools::join(p, ",")));
        }

        if let Some(p) = is_in_inbox {
            query_params.push(("is_in_inbox", format!("{p}")));
        }

        if let Some(p) = is_tagged {
            query_params.push(("is_tagged", format!("{p}")));
        }

        if let Some(p) = mime_type {
            query_params.push(("mime_type", p));
        }

        if let Some(p) = modified_date_gt {
            query_params.push(("modified__date__gt", format!("{p}")));
        }

        if let Some(p) = modified_date_gte {
            query_params.push(("modified__date__gte", format!("{p}")));
        }

        if let Some(p) = modified_date_lt {
            query_params.push(("modified__date__lt", format!("{p}")));
        }

        if let Some(p) = modified_date_lte {
            query_params.push(("modified__date__lte", format!("{p}")));
        }

        if let Some(p) = modified_day {
            query_params.push(("modified__day", format!("{p}")));
        }

        if let Some(p) = modified_gt {
            query_params.push(("modified__gt", format!("{p}")));
        }

        if let Some(p) = modified_gte {
            query_params.push(("modified__gte", format!("{p}")));
        }

        if let Some(p) = modified_lt {
            query_params.push(("modified__lt", format!("{p}")));
        }

        if let Some(p) = modified_lte {
            query_params.push(("modified__lte", format!("{p}")));
        }

        if let Some(p) = modified_month {
            query_params.push(("modified__month", format!("{p}")));
        }

        if let Some(p) = modified_year {
            query_params.push(("modified__year", format!("{p}")));
        }

        if let Some(p) = ordering {
            query_params.push(("ordering", p));
        }

        if let Some(p) = original_filename_icontains {
            query_params.push(("original_filename__icontains", p));
        }

        if let Some(p) = original_filename_iendswith {
            query_params.push(("original_filename__iendswith", p));
        }

        if let Some(p) = original_filename_iexact {
            query_params.push(("original_filename__iexact", p));
        }

        if let Some(p) = original_filename_istartswith {
            query_params.push(("original_filename__istartswith", p));
        }

        if let Some(p) = owner_id {
            query_params.push(("owner__id", format!("{p}")));
        }

        if let Some(p) = owner_id_in {
            query_params.push(("owner__id__in", itertools::join(p, ",")));
        }

        if let Some(p) = owner_id_none {
            query_params.push(("owner__id__none", format!("{p}")));
        }

        if let Some(p) = owner_isnull {
            query_params.push(("owner__isnull", format!("{p}")));
        }

        if let Some(p) = page {
            query_params.push(("page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("page_size", format!("{p}")));
        }

        if let Some(p) = query {
            query_params.push(("query", p));
        }

        if let Some(p) = search {
            query_params.push(("search", p));
        }

        if let Some(p) = shared_by_id {
            query_params.push(("shared_by__id", format!("{p}")));
        }

        if let Some(p) = storage_path_id {
            query_params.push(("storage_path__id", format!("{p}")));
        }

        if let Some(p) = storage_path_id_in {
            query_params.push(("storage_path__id__in", itertools::join(p, ",")));
        }

        if let Some(p) = storage_path_id_none {
            query_params.push(("storage_path__id__none", format!("{p}")));
        }

        if let Some(p) = storage_path_isnull {
            query_params.push(("storage_path__isnull", format!("{p}")));
        }

        if let Some(p) = storage_path_name_icontains {
            query_params.push(("storage_path__name__icontains", p));
        }

        if let Some(p) = storage_path_name_iendswith {
            query_params.push(("storage_path__name__iendswith", p));
        }

        if let Some(p) = storage_path_name_iexact {
            query_params.push(("storage_path__name__iexact", p));
        }

        if let Some(p) = storage_path_name_istartswith {
            query_params.push(("storage_path__name__istartswith", p));
        }

        if let Some(p) = tags_id {
            query_params.push(("tags__id", format!("{p}")));
        }

        if let Some(p) = tags_id_all {
            query_params.push(("tags__id__all", format!("{p}")));
        }

        if let Some(p) = tags_id_in {
            query_params.push(("tags__id__in", format!("{p}")));
        }

        if let Some(p) = tags_id_none {
            query_params.push(("tags__id__none", format!("{p}")));
        }

        if let Some(p) = tags_name_icontains {
            query_params.push(("tags__name__icontains", p));
        }

        if let Some(p) = tags_name_iendswith {
            query_params.push(("tags__name__iendswith", p));
        }

        if let Some(p) = tags_name_iexact {
            query_params.push(("tags__name__iexact", p));
        }

        if let Some(p) = tags_name_istartswith {
            query_params.push(("tags__name__istartswith", p));
        }

        if let Some(p) = title_icontains {
            query_params.push(("title__icontains", p));
        }

        if let Some(p) = title_iendswith {
            query_params.push(("title__iendswith", p));
        }

        if let Some(p) = title_iexact {
            query_params.push(("title__iexact", p));
        }

        if let Some(p) = title_istartswith {
            query_params.push(("title__istartswith", p));
        }

        if let Some(p) = title_content {
            query_params.push(("title_content", p));
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

    #[doc = "Perform a `GET` request to `/api/documents/`.\n\nDocument views including search\n\n**Parameters:**\n\n- `added_date_gt: Option<chrono::NaiveDate>`\n- `added_date_gte: Option<chrono::NaiveDate>`\n- `added_date_lt: Option<chrono::NaiveDate>`\n- `added_date_lte: Option<chrono::NaiveDate>`\n- `added_day: Option<f64>`\n- `added_gt: Option<chrono::DateTime<chrono::Utc>>`\n- `added_gte: Option<chrono::DateTime<chrono::Utc>>`\n- `added_lt: Option<chrono::DateTime<chrono::Utc>>`\n- `added_lte: Option<chrono::DateTime<chrono::Utc>>`\n- `added_month: Option<f64>`\n- `added_year: Option<f64>`\n- `archive_serial_number: Option<i64>`\n- `archive_serial_number_gt: Option<i64>`\n- `archive_serial_number_gte: Option<i64>`\n- `archive_serial_number_isnull: Option<bool>`\n- `archive_serial_number_lt: Option<i64>`\n- `archive_serial_number_lte: Option<i64>`\n- `checksum_icontains: Option<String>`\n- `checksum_iendswith: Option<String>`\n- `checksum_iexact: Option<String>`\n- `checksum_istartswith: Option<String>`\n- `content_icontains: Option<String>`\n- `content_iendswith: Option<String>`\n- `content_iexact: Option<String>`\n- `content_istartswith: Option<String>`\n- `correspondent_id: Option<i64>`\n- `correspondent_id_in: Option<Vec<i64>>`: Multiple values may be separated by commas.\n- `correspondent_id_none: Option<i64>`\n- `correspondent_isnull: Option<bool>`\n- `correspondent_name_icontains: Option<String>`\n- `correspondent_name_iendswith: Option<String>`\n- `correspondent_name_iexact: Option<String>`\n- `correspondent_name_istartswith: Option<String>`\n- `created_date_gt: Option<chrono::NaiveDate>`\n- `created_date_gte: Option<chrono::NaiveDate>`\n- `created_date_lt: Option<chrono::NaiveDate>`\n- `created_date_lte: Option<chrono::NaiveDate>`\n- `created_day: Option<f64>`\n- `created_gt: Option<chrono::NaiveDate>`\n- `created_gte: Option<chrono::NaiveDate>`\n- `created_lt: Option<chrono::NaiveDate>`\n- `created_lte: Option<chrono::NaiveDate>`\n- `created_month: Option<f64>`\n- `created_year: Option<f64>`\n- `custom_field_query: Option<String>`\n- `custom_fields_icontains: Option<String>`\n- `custom_fields_id_all: Option<i64>`\n- `custom_fields_id_in: Option<i64>`\n- `custom_fields_id_none: Option<i64>`\n- `document_type_id: Option<i64>`\n- `document_type_id_in: Option<Vec<i64>>`: Multiple values may be separated by commas.\n- `document_type_id_none: Option<i64>`\n- `document_type_isnull: Option<bool>`\n- `document_type_name_icontains: Option<String>`\n- `document_type_name_iendswith: Option<String>`\n- `document_type_name_iexact: Option<String>`\n- `document_type_name_istartswith: Option<String>`\n- `fields: Option<Vec<String>>`\n- `full_perms: Option<bool>`\n- `has_custom_fields: Option<bool>`: Has custom field\n- `id: Option<i64>`\n- `id_in: Option<Vec<i64>>`: Multiple values may be separated by commas.\n- `is_in_inbox: Option<bool>`\n- `is_tagged: Option<bool>`: Is tagged\n- `mime_type: Option<String>`\n- `modified_date_gt: Option<chrono::NaiveDate>`\n- `modified_date_gte: Option<chrono::NaiveDate>`\n- `modified_date_lt: Option<chrono::NaiveDate>`\n- `modified_date_lte: Option<chrono::NaiveDate>`\n- `modified_day: Option<f64>`\n- `modified_gt: Option<chrono::DateTime<chrono::Utc>>`\n- `modified_gte: Option<chrono::DateTime<chrono::Utc>>`\n- `modified_lt: Option<chrono::DateTime<chrono::Utc>>`\n- `modified_lte: Option<chrono::DateTime<chrono::Utc>>`\n- `modified_month: Option<f64>`\n- `modified_year: Option<f64>`\n- `ordering: Option<String>`: Which field to use when ordering the results.\n- `original_filename_icontains: Option<String>`\n- `original_filename_iendswith: Option<String>`\n- `original_filename_iexact: Option<String>`\n- `original_filename_istartswith: Option<String>`\n- `owner_id: Option<i64>`\n- `owner_id_in: Option<Vec<i64>>`: Multiple values may be separated by commas.\n- `owner_id_none: Option<i64>`\n- `owner_isnull: Option<bool>`\n- `page: Option<i64>`: A page number within the paginated result set.\n- `page_size: Option<i64>`: Number of results to return per page.\n- `query: Option<String>`: Advanced search query string\n- `search: Option<String>`: A search term.\n- `shared_by_id: Option<bool>`\n- `storage_path_id: Option<i64>`\n- `storage_path_id_in: Option<Vec<i64>>`: Multiple values may be separated by commas.\n- `storage_path_id_none: Option<i64>`\n- `storage_path_isnull: Option<bool>`\n- `storage_path_name_icontains: Option<String>`\n- `storage_path_name_iendswith: Option<String>`\n- `storage_path_name_iexact: Option<String>`\n- `storage_path_name_istartswith: Option<String>`\n- `tags_id: Option<i64>`\n- `tags_id_all: Option<i64>`\n- `tags_id_in: Option<i64>`\n- `tags_id_none: Option<i64>`\n- `tags_name_icontains: Option<String>`\n- `tags_name_iendswith: Option<String>`\n- `tags_name_iexact: Option<String>`\n- `tags_name_istartswith: Option<String>`\n- `title_icontains: Option<String>`\n- `title_iendswith: Option<String>`\n- `title_iexact: Option<String>`\n- `title_istartswith: Option<String>`\n- `title_content: Option<String>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_documents_list_stream() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let mut documents = client.documents();\n    let mut stream = documents.list_stream(\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(3.14 as f64),\n        Some(chrono::Utc::now()),\n        Some(chrono::Utc::now()),\n        Some(chrono::Utc::now()),\n        Some(chrono::Utc::now()),\n        Some(3.14 as f64),\n        Some(3.14 as f64),\n        Some(4 as i64),\n        Some(4 as i64),\n        Some(4 as i64),\n        Some(true),\n        Some(4 as i64),\n        Some(4 as i64),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(4 as i64),\n        Some(vec![4 as i64]),\n        Some(4 as i64),\n        Some(true),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(3.14 as f64),\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(3.14 as f64),\n        Some(3.14 as f64),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(4 as i64),\n        Some(4 as i64),\n        Some(4 as i64),\n        Some(4 as i64),\n        Some(vec![4 as i64]),\n        Some(4 as i64),\n        Some(true),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(vec![\"some-string\".to_string()]),\n        Some(true),\n        Some(true),\n        Some(4 as i64),\n        Some(vec![4 as i64]),\n        Some(true),\n        Some(true),\n        Some(\"some-string\".to_string()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(chrono::Utc::now().date_naive()),\n        Some(3.14 as f64),\n        Some(chrono::Utc::now()),\n        Some(chrono::Utc::now()),\n        Some(chrono::Utc::now()),\n        Some(chrono::Utc::now()),\n        Some(3.14 as f64),\n        Some(3.14 as f64),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(4 as i64),\n        Some(vec![4 as i64]),\n        Some(4 as i64),\n        Some(true),\n        Some(4 as i64),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(true),\n        Some(4 as i64),\n        Some(vec![4 as i64]),\n        Some(4 as i64),\n        Some(true),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(4 as i64),\n        Some(4 as i64),\n        Some(4 as i64),\n        Some(4 as i64),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    #[allow(non_snake_case)]
    pub fn list_stream<'a>(
        &'a self,
        added__date__gt: Option<chrono::NaiveDate>,
        added__date__gte: Option<chrono::NaiveDate>,
        added__date__lt: Option<chrono::NaiveDate>,
        added__date__lte: Option<chrono::NaiveDate>,
        added__day: Option<f64>,
        added__gt: Option<chrono::DateTime<chrono::Utc>>,
        added__gte: Option<chrono::DateTime<chrono::Utc>>,
        added__lt: Option<chrono::DateTime<chrono::Utc>>,
        added__lte: Option<chrono::DateTime<chrono::Utc>>,
        added__month: Option<f64>,
        added__year: Option<f64>,
        archive_serial_number: Option<i64>,
        archive_serial_number__gt: Option<i64>,
        archive_serial_number__gte: Option<i64>,
        archive_serial_number__isnull: Option<bool>,
        archive_serial_number__lt: Option<i64>,
        archive_serial_number__lte: Option<i64>,
        checksum__icontains: Option<String>,
        checksum__iendswith: Option<String>,
        checksum__iexact: Option<String>,
        checksum__istartswith: Option<String>,
        content__icontains: Option<String>,
        content__iendswith: Option<String>,
        content__iexact: Option<String>,
        content__istartswith: Option<String>,
        correspondent__id: Option<i64>,
        correspondent__id__in: Option<Vec<i64>>,
        correspondent__id__none: Option<i64>,
        correspondent__isnull: Option<bool>,
        correspondent__name__icontains: Option<String>,
        correspondent__name__iendswith: Option<String>,
        correspondent__name__iexact: Option<String>,
        correspondent__name__istartswith: Option<String>,
        created__date__gt: Option<chrono::NaiveDate>,
        created__date__gte: Option<chrono::NaiveDate>,
        created__date__lt: Option<chrono::NaiveDate>,
        created__date__lte: Option<chrono::NaiveDate>,
        created__day: Option<f64>,
        created__gt: Option<chrono::NaiveDate>,
        created__gte: Option<chrono::NaiveDate>,
        created__lt: Option<chrono::NaiveDate>,
        created__lte: Option<chrono::NaiveDate>,
        created__month: Option<f64>,
        created__year: Option<f64>,
        custom_field_query: Option<String>,
        custom_fields__icontains: Option<String>,
        custom_fields__id__all: Option<i64>,
        custom_fields__id__in: Option<i64>,
        custom_fields__id__none: Option<i64>,
        document_type__id: Option<i64>,
        document_type__id__in: Option<Vec<i64>>,
        document_type__id__none: Option<i64>,
        document_type__isnull: Option<bool>,
        document_type__name__icontains: Option<String>,
        document_type__name__iendswith: Option<String>,
        document_type__name__iexact: Option<String>,
        document_type__name__istartswith: Option<String>,
        fields: Option<Vec<String>>,
        full_perms: Option<bool>,
        has_custom_fields: Option<bool>,
        id: Option<i64>,
        id__in: Option<Vec<i64>>,
        is_in_inbox: Option<bool>,
        is_tagged: Option<bool>,
        mime_type: Option<String>,
        modified__date__gt: Option<chrono::NaiveDate>,
        modified__date__gte: Option<chrono::NaiveDate>,
        modified__date__lt: Option<chrono::NaiveDate>,
        modified__date__lte: Option<chrono::NaiveDate>,
        modified__day: Option<f64>,
        modified__gt: Option<chrono::DateTime<chrono::Utc>>,
        modified__gte: Option<chrono::DateTime<chrono::Utc>>,
        modified__lt: Option<chrono::DateTime<chrono::Utc>>,
        modified__lte: Option<chrono::DateTime<chrono::Utc>>,
        modified__month: Option<f64>,
        modified__year: Option<f64>,
        ordering: Option<String>,
        original_filename__icontains: Option<String>,
        original_filename__iendswith: Option<String>,
        original_filename__iexact: Option<String>,
        original_filename__istartswith: Option<String>,
        owner__id: Option<i64>,
        owner__id__in: Option<Vec<i64>>,
        owner__id__none: Option<i64>,
        owner__isnull: Option<bool>,
        page_size: Option<i64>,
        query: Option<String>,
        search: Option<String>,
        shared_by__id: Option<bool>,
        storage_path__id: Option<i64>,
        storage_path__id__in: Option<Vec<i64>>,
        storage_path__id__none: Option<i64>,
        storage_path__isnull: Option<bool>,
        storage_path__name__icontains: Option<String>,
        storage_path__name__iendswith: Option<String>,
        storage_path__name__iexact: Option<String>,
        storage_path__name__istartswith: Option<String>,
        tags__id: Option<i64>,
        tags__id__all: Option<i64>,
        tags__id__in: Option<i64>,
        tags__id__none: Option<i64>,
        tags__name__icontains: Option<String>,
        tags__name__iendswith: Option<String>,
        tags__name__iexact: Option<String>,
        tags__name__istartswith: Option<String>,
        title__icontains: Option<String>,
        title__iendswith: Option<String>,
        title__iexact: Option<String>,
        title__istartswith: Option<String>,
        title_content: Option<String>,
    ) -> impl futures::Stream<Item = Result<crate::types::Document, crate::types::error::Error>>
           + Unpin
           + '_ {
        use crate::types::paginate::Pagination;
        use futures::{StreamExt, TryFutureExt, TryStreamExt};
        self.list(
            added__date__gt,
            added__date__gte,
            added__date__lt,
            added__date__lte,
            added__day,
            added__gt,
            added__gte,
            added__lt,
            added__lte,
            added__month,
            added__year,
            archive_serial_number,
            archive_serial_number__gt,
            archive_serial_number__gte,
            archive_serial_number__isnull,
            archive_serial_number__lt,
            archive_serial_number__lte,
            checksum__icontains,
            checksum__iendswith,
            checksum__iexact,
            checksum__istartswith,
            content__icontains,
            content__iendswith,
            content__iexact,
            content__istartswith,
            correspondent__id,
            correspondent__id__in,
            correspondent__id__none,
            correspondent__isnull,
            correspondent__name__icontains,
            correspondent__name__iendswith,
            correspondent__name__iexact,
            correspondent__name__istartswith,
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
            custom_field_query,
            custom_fields__icontains,
            custom_fields__id__all,
            custom_fields__id__in,
            custom_fields__id__none,
            document_type__id,
            document_type__id__in,
            document_type__id__none,
            document_type__isnull,
            document_type__name__icontains,
            document_type__name__iendswith,
            document_type__name__iexact,
            document_type__name__istartswith,
            fields,
            full_perms,
            has_custom_fields,
            id,
            id__in,
            is_in_inbox,
            is_tagged,
            mime_type,
            modified__date__gt,
            modified__date__gte,
            modified__date__lt,
            modified__date__lte,
            modified__day,
            modified__gt,
            modified__gte,
            modified__lt,
            modified__lte,
            modified__month,
            modified__year,
            ordering,
            original_filename__icontains,
            original_filename__iendswith,
            original_filename__iexact,
            original_filename__istartswith,
            owner__id,
            owner__id__in,
            owner__id__none,
            owner__isnull,
            None,
            page_size,
            query,
            search,
            shared_by__id,
            storage_path__id,
            storage_path__id__in,
            storage_path__id__none,
            storage_path__isnull,
            storage_path__name__icontains,
            storage_path__name__iendswith,
            storage_path__name__iexact,
            storage_path__name__istartswith,
            tags__id,
            tags__id__all,
            tags__id__in,
            tags__id__none,
            tags__name__icontains,
            tags__name__iendswith,
            tags__name__iexact,
            tags__name__istartswith,
            title__icontains,
            title__iendswith,
            title__iexact,
            title__istartswith,
            title_content,
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
                                format!("{}/{}", self.client.base_url, "api/documents/"),
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
                        .map_ok(|result: crate::types::PaginatedDocumentList| {
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

    #[doc = "Perform a `GET` request to `/api/documents/{doc_id}/`.\n\nRetrieve a single document\n\n**Parameters:**\n\n- `doc_id: i64`: A unique integer value identifying this document. (required)\n- `fields: Option<Vec<String>>`\n- `full_perms: Option<bool>`\n\n```rust,no_run\nasync fn example_documents_retrieve() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::Document = client\n        .documents()\n        .retrieve(4 as i64, Some(vec![\"some-string\".to_string()]), Some(true))\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn retrieve<'a>(
        &'a self,
        doc_id: i64,
        fields: Option<Vec<String>>,
        full_perms: Option<bool>,
    ) -> Result<crate::types::Document, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/documents/{doc_id}/".replace("{doc_id}", &format!("{doc_id}"))
            ),
        );
        req = req.header("Authorization", format!("Token {}", &self.client.token));
        let mut query_params = vec![];
        if let Some(p) = fields {
            query_params.push(("fields", itertools::join(p, ",")));
        }

        if let Some(p) = full_perms {
            query_params.push(("full_perms", format!("{p}")));
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

    #[doc = "Perform a `PUT` request to `/api/documents/{doc_id}/`.\n\nPass a user object to serializer\n\n**Parameters:**\n\n- `doc_id: i64`: A unique integer value identifying this document. (required)\n\n```rust,no_run\nasync fn example_documents_update() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::Document = client\n        .documents()\n        .update(\n            4 as i64,\n            &paperless_api_client::types::DocumentRequest {\n                correspondent: Some(4 as i64),\n                document_type: Some(4 as i64),\n                storage_path: Some(4 as i64),\n                title: Some(\"some-string\".to_string()),\n                content: Some(\"some-string\".to_string()),\n                tags: vec![4 as i64],\n                created: Some(chrono::Utc::now().date_naive()),\n                created_date: Some(chrono::Utc::now().date_naive()),\n                deleted_at: Some(chrono::Utc::now()),\n                archive_serial_number: Some(4 as i64),\n                owner: Some(4 as i64),\n                set_permissions: Some(paperless_api_client::types::SetPermissions {\n                    view: Some(paperless_api_client::types::View {\n                        users: Some(vec![4 as i64]),\n                        groups: Some(vec![4 as i64]),\n                    }),\n                    change: Some(paperless_api_client::types::Change {\n                        users: Some(vec![4 as i64]),\n                        groups: Some(vec![4 as i64]),\n                    }),\n                }),\n                custom_fields: Some(vec![paperless_api_client::types::CustomFieldInstanceRequest {\n                    value: Some(serde_json::Value::String(\"some-string\".to_string())),\n                    field: 4 as i64,\n                }]),\n                remove_inbox_tags: Some(true),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn update<'a>(
        &'a self,
        doc_id: i64,
        body: &crate::types::DocumentRequest,
    ) -> Result<crate::types::Document, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/documents/{doc_id}/".replace("{doc_id}", &format!("{doc_id}"))
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

    #[doc = "Perform a `DELETE` request to `/api/documents/{doc_id}/`.\n\nPass a user object to serializer\n\n**Parameters:**\n\n- `doc_id: i64`: A unique integer value identifying this document. (required)\n\n```rust,no_run\nasync fn example_documents_destroy() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    client.documents().destroy(4 as i64).await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn destroy<'a>(&'a self, doc_id: i64) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/documents/{doc_id}/".replace("{doc_id}", &format!("{doc_id}"))
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

    #[doc = "Perform a `PATCH` request to `/api/documents/{doc_id}/`.\n\nPass a user object to serializer\n\n**Parameters:**\n\n- `doc_id: i64`: A unique integer value identifying this document. (required)\n\n```rust,no_run\nasync fn example_documents_partial_update() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::Document = client\n        .documents()\n        .partial_update(\n            4 as i64,\n            &paperless_api_client::types::PatchedDocumentRequest {\n                correspondent: Some(4 as i64),\n                document_type: Some(4 as i64),\n                storage_path: Some(4 as i64),\n                title: Some(\"some-string\".to_string()),\n                content: Some(\"some-string\".to_string()),\n                tags: Some(vec![4 as i64]),\n                created: Some(chrono::Utc::now().date_naive()),\n                created_date: Some(chrono::Utc::now().date_naive()),\n                deleted_at: Some(chrono::Utc::now()),\n                archive_serial_number: Some(4 as i64),\n                owner: Some(4 as i64),\n                set_permissions: Some(paperless_api_client::types::SetPermissions {\n                    view: Some(paperless_api_client::types::View {\n                        users: Some(vec![4 as i64]),\n                        groups: Some(vec![4 as i64]),\n                    }),\n                    change: Some(paperless_api_client::types::Change {\n                        users: Some(vec![4 as i64]),\n                        groups: Some(vec![4 as i64]),\n                    }),\n                }),\n                custom_fields: Some(vec![paperless_api_client::types::CustomFieldInstanceRequest {\n                    value: Some(serde_json::Value::String(\"some-string\".to_string())),\n                    field: 4 as i64,\n                }]),\n                remove_inbox_tags: Some(true),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn partial_update<'a>(
        &'a self,
        doc_id: i64,
        body: &crate::types::PatchedDocumentRequest,
    ) -> Result<crate::types::Document, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/documents/{doc_id}/".replace("{doc_id}", &format!("{doc_id}"))
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

    #[doc = "Perform a `GET` request to `/api/documents/{doc_id}/download/`.\n\nDownload the document\n\n**Parameters:**\n\n- `doc_id: i64`: A unique integer value identifying this document. (required)\n- `original: Option<bool>`\n\n```rust,no_run\nasync fn example_documents_download_retrieve() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: bytes::Bytes = client\n        .documents()\n        .download_retrieve(4 as i64, Some(true))\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn download_retrieve<'a>(
        &'a self,
        doc_id: i64,
        original: Option<bool>,
    ) -> Result<bytes::Bytes, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/documents/{doc_id}/download/".replace("{doc_id}", &format!("{doc_id}"))
            ),
        );
        req = req.header("Authorization", format!("Token {}", &self.client.token));
        let mut query_params = vec![];
        if let Some(p) = original {
            query_params.push(("original", format!("{p}")));
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

    #[doc = "Perform a `POST` request to `/api/documents/{doc_id}/email/`.\n\nEmail the document to one or more recipients as an attachment.\n\n**Parameters:**\n\n- `doc_id: i64`: A unique integer value identifying this document. (required)\n\n```rust,no_run\nasync fn example_documents_email_create() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::EmailResponse = client\n        .documents()\n        .email_create(\n            4 as i64,\n            &paperless_api_client::types::EmailRequestRequest {\n                addresses: \"some-string\".to_string(),\n                subject: \"some-string\".to_string(),\n                message: \"some-string\".to_string(),\n                use_archive_version: true,\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn email_create<'a>(
        &'a self,
        doc_id: i64,
        body: &crate::types::EmailRequestRequest,
    ) -> Result<crate::types::EmailResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/documents/{doc_id}/email/".replace("{doc_id}", &format!("{doc_id}"))
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

    #[doc = "Perform a `GET` request to `/api/documents/{doc_id}/history/`.\n\nView the document history\n\n**Parameters:**\n\n- `doc_id: i64`: A unique integer value identifying this document. (required)\n- `page: Option<i64>`: A page number within the paginated result set.\n- `page_size: Option<i64>`: Number of results to return per page.\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_documents_history_list_stream() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let mut documents = client.documents();\n    let mut stream = documents.history_list_stream(4 as i64, Some(4 as i64));\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn history_list<'a>(
        &'a self,
        doc_id: i64,
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> Result<crate::types::PaginatedLogEntryList, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/documents/{doc_id}/history/".replace("{doc_id}", &format!("{doc_id}"))
            ),
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

    #[doc = "Perform a `GET` request to `/api/documents/{doc_id}/history/`.\n\nView the document history\n\n**Parameters:**\n\n- `doc_id: i64`: A unique integer value identifying this document. (required)\n- `page: Option<i64>`: A page number within the paginated result set.\n- `page_size: Option<i64>`: Number of results to return per page.\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_documents_history_list_stream() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let mut documents = client.documents();\n    let mut stream = documents.history_list_stream(4 as i64, Some(4 as i64));\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    #[allow(non_snake_case)]
    pub fn history_list_stream<'a>(
        &'a self,
        doc_id: i64,
        page_size: Option<i64>,
    ) -> impl futures::Stream<Item = Result<crate::types::LogEntry, crate::types::error::Error>>
           + Unpin
           + '_ {
        use crate::types::paginate::Pagination;
        use futures::{StreamExt, TryFutureExt, TryStreamExt};
        self.history_list(doc_id, None, page_size)
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
                                    format!(
                                        "{}/{}",
                                        self.client.base_url,
                                        "api/documents/{doc_id}/history/"
                                            .replace("{doc_id}", &format!("{}", doc_id))
                                    ),
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
                            .map_ok(|result: crate::types::PaginatedLogEntryList| {
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

    #[doc = "Perform a `GET` request to `/api/documents/{doc_id}/metadata/`.\n\nView the document metadata\n\n**Parameters:**\n\n- `doc_id: i64`: A unique integer value identifying this document. (required)\n\n```rust,no_run\nasync fn example_documents_metadata_retrieve() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::Metadata = client.documents().metadata_retrieve(4 as i64).await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn metadata_retrieve<'a>(
        &'a self,
        doc_id: i64,
    ) -> Result<crate::types::Metadata, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/documents/{doc_id}/metadata/".replace("{doc_id}", &format!("{doc_id}"))
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

    #[doc = "Perform a `GET` request to `/api/documents/{doc_id}/notes/`.\n\nView, add, or delete notes for the document\n\n**Parameters:**\n\n- `doc_id: i64`: A unique integer value identifying this document. (required)\n- `id: Option<i64>`: Note ID to delete (used only for DELETE requests)\n- `page: Option<i64>`: A page number within the paginated result set.\n- `page_size: Option<i64>`: Number of results to return per page.\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_documents_notes_list_stream() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let mut documents = client.documents();\n    let mut stream = documents.notes_list_stream(4 as i64, Some(4 as i64), Some(4 as i64));\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn notes_list<'a>(
        &'a self,
        doc_id: i64,
        id: Option<i64>,
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> Result<crate::types::PaginatedNotesList, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/documents/{doc_id}/notes/".replace("{doc_id}", &format!("{doc_id}"))
            ),
        );
        req = req.header("Authorization", format!("Token {}", &self.client.token));
        let mut query_params = vec![];
        if let Some(p) = id {
            query_params.push(("id", format!("{p}")));
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

    #[doc = "Perform a `GET` request to `/api/documents/{doc_id}/notes/`.\n\nView, add, or delete notes for the document\n\n**Parameters:**\n\n- `doc_id: i64`: A unique integer value identifying this document. (required)\n- `id: Option<i64>`: Note ID to delete (used only for DELETE requests)\n- `page: Option<i64>`: A page number within the paginated result set.\n- `page_size: Option<i64>`: Number of results to return per page.\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_documents_notes_list_stream() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let mut documents = client.documents();\n    let mut stream = documents.notes_list_stream(4 as i64, Some(4 as i64), Some(4 as i64));\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    #[allow(non_snake_case)]
    pub fn notes_list_stream<'a>(
        &'a self,
        doc_id: i64,
        id: Option<i64>,
        page_size: Option<i64>,
    ) -> impl futures::Stream<Item = Result<crate::types::Notes, crate::types::error::Error>> + Unpin + '_
    {
        use crate::types::paginate::Pagination;
        use futures::{StreamExt, TryFutureExt, TryStreamExt};
        self.notes_list(doc_id, id, None, page_size)
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
                                    format!(
                                        "{}/{}",
                                        self.client.base_url,
                                        "api/documents/{doc_id}/notes/"
                                            .replace("{doc_id}", &format!("{}", doc_id))
                                    ),
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
                            .map_ok(|result: crate::types::PaginatedNotesList| {
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

    #[doc = "Perform a `POST` request to `/api/documents/{doc_id}/notes/`.\n\nView, add, or delete notes for the document\n\n**Parameters:**\n\n- `doc_id: i64`: A unique integer value identifying this document. (required)\n- `id: Option<i64>`: Note ID to delete (used only for DELETE requests)\n- `page: Option<i64>`: A page number within the paginated result set.\n- `page_size: Option<i64>`: Number of results to return per page.\n\n```rust,no_run\nasync fn example_documents_notes_create() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::PaginatedNotesList = client\n        .documents()\n        .notes_create(\n            4 as i64,\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(4 as i64),\n            &paperless_api_client::types::NoteCreateRequestRequest {\n                note: \"some-string\".to_string(),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn notes_create<'a>(
        &'a self,
        doc_id: i64,
        id: Option<i64>,
        page: Option<i64>,
        page_size: Option<i64>,
        body: &crate::types::NoteCreateRequestRequest,
    ) -> Result<crate::types::PaginatedNotesList, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/documents/{doc_id}/notes/".replace("{doc_id}", &format!("{doc_id}"))
            ),
        );
        req = req.header("Authorization", format!("Token {}", &self.client.token));
        let mut query_params = vec![];
        if let Some(p) = id {
            query_params.push(("id", format!("{p}")));
        }

        if let Some(p) = page {
            query_params.push(("page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("page_size", format!("{p}")));
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

    #[doc = "Perform a `DELETE` request to `/api/documents/{doc_id}/notes/`.\n\nView, add, or delete notes for the document\n\n**Parameters:**\n\n- `doc_id: i64`: A unique integer value identifying this document. (required)\n- `id: Option<i64>`: Note ID to delete (used only for DELETE requests)\n- `page: Option<i64>`: A page number within the paginated result set.\n- `page_size: Option<i64>`: Number of results to return per page.\n\n```rust,no_run\nasync fn example_documents_notes_destroy() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::PaginatedNotesList = client\n        .documents()\n        .notes_destroy(4 as i64, Some(4 as i64), Some(4 as i64), Some(4 as i64))\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn notes_destroy<'a>(
        &'a self,
        doc_id: i64,
        id: Option<i64>,
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> Result<crate::types::PaginatedNotesList, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/documents/{doc_id}/notes/".replace("{doc_id}", &format!("{doc_id}"))
            ),
        );
        req = req.header("Authorization", format!("Token {}", &self.client.token));
        let mut query_params = vec![];
        if let Some(p) = id {
            query_params.push(("id", format!("{p}")));
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

    #[doc = "Perform a `GET` request to `/api/documents/{doc_id}/preview/`.\n\nView the document preview\n\n**Parameters:**\n\n- `doc_id: i64`: A unique integer value identifying this document. (required)\n\n```rust,no_run\nasync fn example_documents_preview_retrieve() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: bytes::Bytes = client.documents().preview_retrieve(4 as i64).await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn preview_retrieve<'a>(
        &'a self,
        doc_id: i64,
    ) -> Result<bytes::Bytes, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/documents/{doc_id}/preview/".replace("{doc_id}", &format!("{doc_id}"))
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

    #[doc = "Perform a `GET` request to `/api/documents/{doc_id}/share_links/`.\n\nView share links for the document\n\n**Parameters:**\n\n- `doc_id: &'astr` (required)\n\n```rust,no_run\nasync fn example_documents_share_links() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: Vec<paperless_api_client::types::DocumentShareLinksResponse> =\n        client.documents().share_links(\"some-string\").await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn share_links<'a>(
        &'a self,
        doc_id: &'a str,
    ) -> Result<Vec<crate::types::DocumentShareLinksResponse>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/documents/{doc_id}/share_links/".replace("{doc_id}", doc_id)
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

    #[doc = "Perform a `GET` request to `/api/documents/{doc_id}/suggestions/`.\n\nView suggestions for the document\n\n**Parameters:**\n\n- `doc_id: i64`: A unique integer value identifying this document. (required)\n\n```rust,no_run\nasync fn example_documents_suggestions_retrieve() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::Suggestions =\n        client.documents().suggestions_retrieve(4 as i64).await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn suggestions_retrieve<'a>(
        &'a self,
        doc_id: i64,
    ) -> Result<crate::types::Suggestions, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/documents/{doc_id}/suggestions/".replace("{doc_id}", &format!("{doc_id}"))
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

    #[doc = "Perform a `GET` request to `/api/documents/{doc_id}/thumb/`.\n\nView the document thumbnail\n\n**Parameters:**\n\n- `doc_id: i64`: A unique integer value identifying this document. (required)\n\n```rust,no_run\nasync fn example_documents_thumb_retrieve() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: bytes::Bytes = client.documents().thumb_retrieve(4 as i64).await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn thumb_retrieve<'a>(
        &'a self,
        doc_id: i64,
    ) -> Result<bytes::Bytes, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "api/documents/{doc_id}/thumb/".replace("{doc_id}", &format!("{doc_id}"))
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

    #[doc = "Perform a `POST` request to `/api/documents/bulk_download/`.\n\n```rust,no_run\nasync fn example_documents_bulk_download_create() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::BulkDownload = client\n        .documents()\n        .bulk_download_create(&paperless_api_client::types::BulkDownloadRequest {\n            documents: vec![4 as i64],\n            content: Some(paperless_api_client::types::ContentEnum::Originals),\n            compression: Some(paperless_api_client::types::CompressionEnum::Bzip2),\n            follow_formatting: true,\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn bulk_download_create<'a>(
        &'a self,
        body: &crate::types::BulkDownloadRequest,
    ) -> Result<crate::types::BulkDownload, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url, "api/documents/bulk_download/"
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

    #[doc = "Perform a `POST` request to `/api/documents/bulk_edit/`.\n\nPerform a bulk edit operation on a list of documents\n\nSee <https://docs.paperless-ngx.com/api/#bulk-editing|Further documentation> for more information.\n\n```rust,no_run\nasync fn example_documents_bulk_edit() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::BulkEditDocumentsResult = client\n        .documents()\n        .bulk_edit(&paperless_api_client::types::BulkEditRequest {\n            documents: vec![4 as i64],\n            method: paperless_api_client::types::MethodEnum::SetPermissions,\n            parameters: Some(std::collections::HashMap::from([(\n                \"some-key\".to_string(),\n                serde_json::Value::String(\"some-string\".to_string()),\n            )])),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn bulk_edit<'a>(
        &'a self,
        body: &crate::types::BulkEditRequest,
    ) -> Result<crate::types::BulkEditDocumentsResult, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "api/documents/bulk_edit/"),
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

    #[doc = "Perform a `GET` request to `/api/documents/next_asn/`.\n\nGet the next available Archive Serial Number (ASN) for a new document\n\n```rust,no_run\nasync fn example_documents_next_asn_retrieve() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: i64 = client.documents().next_asn_retrieve().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn next_asn_retrieve<'a>(&'a self) -> Result<i64, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "api/documents/next_asn/"),
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

    #[doc = "Perform a `POST` request to `/api/documents/post_document/`.\n\nUpload a document via the API\n\nSee <https://docs.paperless-ngx.com/api/#file-uploads|Further documentation> for more information.\n\n```rust,no_run\nasync fn example_documents_post_create() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: String = client\n        .documents()\n        .post_create(\n            vec![paperless_api_client::types::multipart::Attachment {\n                name: \"thing\".to_string(),\n                filepath: Some(\"myfile.json\".into()),\n                content_type: Some(\"application/json\".to_string()),\n                data: std::fs::read(\"myfile.json\").unwrap(),\n            }],\n            &paperless_api_client::types::PostDocumentRequest {\n                created: Some(chrono::Utc::now()),\n                document: bytes::Bytes::from(\"some-string\"),\n                title: Some(\"some-string\".to_string()),\n                correspondent: Some(4 as i64),\n                document_type: Some(4 as i64),\n                storage_path: Some(4 as i64),\n                tags: Some(vec![4 as i64]),\n                archive_serial_number: Some(4 as i64),\n                custom_fields: Some(vec![4 as i64]),\n                from_webui: Some(true),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn post_create<'a>(
        &'a self,
        attachments: Vec<crate::types::multipart::Attachment>,
        body: &crate::types::PostDocumentRequest,
    ) -> Result<String, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url, "api/documents/post_document/"
            ),
        );
        req = req.header("Authorization", format!("Token {}", &self.client.token));
        use std::convert::TryInto;
        let mut form = reqwest::multipart::Form::new();
        let mut json_part = reqwest::multipart::Part::text(serde_json::to_string(&body)?);
        json_part = json_part.file_name(format!("{}.json", "body"));
        json_part = json_part.mime_str("application/json")?;
        form = form.part("body", json_part);
        for attachment in attachments {
            form = form.part(attachment.name.clone(), attachment.try_into()?);
        }

        req = req.multipart(form);
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

    #[doc = "Perform a `POST` request to `/api/documents/selection_data/`.\n\nGet selection data for the selected documents\n\n```rust,no_run\nasync fn example_documents_selection_data_create() -> anyhow::Result<()> {\n    let client = paperless_api_client::Client::new_from_env();\n    let result: paperless_api_client::types::SelectionData = client\n        .documents()\n        .selection_data_create(\n            vec![paperless_api_client::types::multipart::Attachment {\n                name: \"thing\".to_string(),\n                filepath: Some(\"myfile.json\".into()),\n                content_type: Some(\"application/json\".to_string()),\n                data: std::fs::read(\"myfile.json\").unwrap(),\n            }],\n            &paperless_api_client::types::DocumentListRequest {\n                documents: vec![4 as i64],\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[allow(non_snake_case)]
    pub async fn selection_data_create<'a>(
        &'a self,
        attachments: Vec<crate::types::multipart::Attachment>,
        body: &crate::types::DocumentListRequest,
    ) -> Result<crate::types::SelectionData, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url, "api/documents/selection_data/"
            ),
        );
        req = req.header("Authorization", format!("Token {}", &self.client.token));
        use std::convert::TryInto;
        let mut form = reqwest::multipart::Form::new();
        let mut json_part = reqwest::multipart::Part::text(serde_json::to_string(&body)?);
        json_part = json_part.file_name(format!("{}.json", "body"));
        json_part = json_part.mime_str("application/json")?;
        form = form.part("body", json_part);
        for attachment in attachments {
            form = form.part(attachment.name.clone(), attachment.try_into()?);
        }

        req = req.multipart(form);
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
