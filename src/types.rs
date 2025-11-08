#![doc = r" This module contains the generated types for the library."]
#[cfg(feature = "tabled")]
use tabled::Tabled;
pub mod base64 {
    #![doc = " Base64 data that encodes to url safe base64, but can decode from multiple"]
    #![doc = " base64 implementations to account for various clients and libraries. Compatible"]
    #![doc = " with serde and JsonSchema."]
    use serde::{
        de::{Error, Unexpected, Visitor},
        Deserialize, Deserializer, Serialize, Serializer,
    };
    use std::{convert::TryFrom, fmt};
    static ALLOWED_DECODING_FORMATS: &[data_encoding::Encoding] = &[
        data_encoding::BASE64,
        data_encoding::BASE64URL,
        data_encoding::BASE64URL_NOPAD,
        data_encoding::BASE64_MIME,
        data_encoding::BASE64_NOPAD,
    ];
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = " A container for binary that should be base64 encoded in serialisation. In reverse"]
    #[doc = " when deserializing, will decode from many different types of base64 possible."]
    pub struct Base64Data(pub Vec<u8>);
    impl Base64Data {
        #[doc = " Return is the data is empty."]
        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
    }

    impl fmt::Display for Base64Data {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", data_encoding::BASE64URL_NOPAD.encode(&self.0))
        }
    }

    impl From<Base64Data> for Vec<u8> {
        fn from(data: Base64Data) -> Vec<u8> {
            data.0
        }
    }

    impl From<Vec<u8>> for Base64Data {
        fn from(data: Vec<u8>) -> Base64Data {
            Base64Data(data)
        }
    }

    impl AsRef<[u8]> for Base64Data {
        fn as_ref(&self) -> &[u8] {
            &self.0
        }
    }

    impl TryFrom<&str> for Base64Data {
        type Error = anyhow::Error;
        fn try_from(v: &str) -> Result<Self, Self::Error> {
            for config in ALLOWED_DECODING_FORMATS {
                if let Ok(data) = config.decode(v.as_bytes()) {
                    return Ok(Base64Data(data));
                }
            }
            anyhow::bail!("Could not decode base64 data: {v}");
        }
    }

    struct Base64DataVisitor;
    impl Visitor<'_> for Base64DataVisitor {
        type Value = Base64Data;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a base64 encoded string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            for config in ALLOWED_DECODING_FORMATS {
                if let Ok(data) = config.decode(v.as_bytes()) {
                    return Ok(Base64Data(data));
                }
            }
            Err(serde::de::Error::invalid_value(Unexpected::Str(v), &self))
        }
    }

    impl<'de> Deserialize<'de> for Base64Data {
        fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_str(Base64DataVisitor)
        }
    }

    impl Serialize for Base64Data {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let encoded = data_encoding::BASE64URL_NOPAD.encode(&self.0);
            serializer.serialize_str(&encoded)
        }
    }

    impl schemars::JsonSchema for Base64Data {
        fn schema_name() -> String {
            "Base64Data".to_string()
        }

        fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
            let mut obj = gen.root_schema_for::<String>().schema;
            obj.format = Some("byte".to_string());
            schemars::schema::Schema::Object(obj)
        }

        fn is_referenceable() -> bool {
            false
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Base64Data;
        use std::convert::TryFrom;
        #[test]
        fn test_base64_try_from() {
            assert!(Base64Data::try_from("aGVsbG8=").is_ok());
            assert!(Base64Data::try_from("abcdefghij").is_err());
        }
    }
}

#[cfg(feature = "requests")]
pub mod multipart {
    #![doc = " Multipart form data types."]
    use std::path::PathBuf;
    #[doc = " An attachement to a multipart form."]
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Attachment {
        #[doc = " The name of the field."]
        pub name: String,
        #[doc = " The file path of the attachment."]
        pub filepath: Option<PathBuf>,
        #[doc = " The content type of the attachment."]
        pub content_type: Option<String>,
        #[doc = " The data of the attachment."]
        pub data: Vec<u8>,
    }

    impl std::convert::TryFrom<Attachment> for reqwest::multipart::Part {
        type Error = reqwest::Error;
        fn try_from(attachment: Attachment) -> Result<Self, Self::Error> {
            let mut part = reqwest::multipart::Part::bytes(attachment.data);
            if let Some(filepath) = attachment.filepath {
                part = part.file_name(filepath.to_string_lossy().to_string());
            }
            if let Some(content_type) = attachment.content_type {
                part = part.mime_str(&content_type)?;
            }
            Ok(part)
        }
    }

    impl std::convert::TryFrom<std::path::PathBuf> for Attachment {
        type Error = std::io::Error;
        fn try_from(path: std::path::PathBuf) -> Result<Self, Self::Error> {
            let content_type = mime_guess::from_path(&path).first_raw();
            let data = std::fs::read(&path)?;
            Ok(Attachment {
                name: "file".to_string(),
                filepath: Some(path),
                content_type: content_type.map(|s| s.to_string()),
                data,
            })
        }
    }
}

#[cfg(feature = "requests")]
pub mod paginate {
    #![doc = " Utility functions used for pagination."]
    use anyhow::Result;
    #[doc = " A trait for types that allow pagination."]
    pub trait Pagination {
        #[doc = " The item that is paginated."]
        type Item: serde::de::DeserializeOwned;
        #[doc = " Returns true if the response has more pages."]
        fn has_more_pages(&self) -> bool;
        #[doc = " Returns the next page token."]
        fn next_page_token(&self) -> Option<String>;
        #[doc = " Modify a request to get the next page."]
        fn next_page(
            &self,
            req: reqwest::Request,
        ) -> Result<reqwest::Request, crate::types::error::Error>;
        #[doc = " Get the items from a page."]
        fn items(&self) -> Vec<Self::Item>;
    }
}

pub mod phone_number {
    #![doc = " A library to implement phone numbers for our database and JSON serialization and deserialization."]
    use schemars::JsonSchema;
    use std::str::FromStr;
    #[doc = " A phone number."]
    #[derive(Debug, Default, Clone, PartialEq, Hash, Eq)]
    pub struct PhoneNumber(pub Option<phonenumber::PhoneNumber>);
    impl From<phonenumber::PhoneNumber> for PhoneNumber {
        fn from(id: phonenumber::PhoneNumber) -> PhoneNumber {
            PhoneNumber(Some(id))
        }
    }

    impl AsRef<Option<phonenumber::PhoneNumber>> for PhoneNumber {
        fn as_ref(&self) -> &Option<phonenumber::PhoneNumber> {
            &self.0
        }
    }

    impl std::ops::Deref for PhoneNumber {
        type Target = Option<phonenumber::PhoneNumber>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl serde::ser::Serialize for PhoneNumber {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
        {
            serializer.serialize_str(&self.to_string())
        }
    }

    impl<'de> serde::de::Deserialize<'de> for PhoneNumber {
        fn deserialize<D>(deserializer: D) -> Result<PhoneNumber, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            let s = String::deserialize(deserializer).unwrap_or_default();
            PhoneNumber::from_str(&s).map_err(serde::de::Error::custom)
        }
    }

    impl std::str::FromStr for PhoneNumber {
        type Err = anyhow::Error;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if s.trim().is_empty() {
                return Ok(PhoneNumber(None));
            }
            let s = if !s.trim().starts_with('+') {
                format!("+1{s}")
            } else {
                s.to_string()
            }
            .replace(['-', '(', ')', ' '], "");
            Ok(PhoneNumber(Some(phonenumber::parse(None, &s).map_err(
                |e| anyhow::anyhow!("invalid phone number `{s}`: {e}"),
            )?)))
        }
    }

    impl std::fmt::Display for PhoneNumber {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let s = if let Some(phone) = &self.0 {
                phone
                    .format()
                    .mode(phonenumber::Mode::International)
                    .to_string()
            } else {
                String::new()
            };
            write!(f, "{s}")
        }
    }

    impl JsonSchema for PhoneNumber {
        fn schema_name() -> String {
            "PhoneNumber".to_string()
        }

        fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
            let mut obj = gen.root_schema_for::<String>().schema;
            obj.format = Some("phone".to_string());
            schemars::schema::Schema::Object(obj)
        }

        fn is_referenceable() -> bool {
            false
        }
    }

    #[cfg(test)]
    mod test {
        use super::PhoneNumber;
        use pretty_assertions::assert_eq;
        #[test]
        fn test_parse_phone_number() {
            let mut phone = "+1-555-555-5555";
            let mut phone_parsed: PhoneNumber =
                serde_json::from_str(&format!(r#""{}""#, phone)).unwrap();
            let mut expected = PhoneNumber(Some(phonenumber::parse(None, phone).unwrap()));
            assert_eq!(phone_parsed, expected);
            let mut expected_str = "+1 555-555-5555";
            assert_eq!(expected_str, serde_json::json!(phone_parsed));
            phone = "555-555-5555";
            phone_parsed = serde_json::from_str(&format!(r#""{}""#, phone)).unwrap();
            assert_eq!(phone_parsed, expected);
            assert_eq!(expected_str, serde_json::json!(phone_parsed));
            phone = "+1 555-555-5555";
            phone_parsed = serde_json::from_str(&format!(r#""{}""#, phone)).unwrap();
            assert_eq!(phone_parsed, expected);
            assert_eq!(expected_str, serde_json::json!(phone_parsed));
            phone = "5555555555";
            phone_parsed = serde_json::from_str(&format!(r#""{}""#, phone)).unwrap();
            assert_eq!(phone_parsed, expected);
            assert_eq!(expected_str, serde_json::json!(phone_parsed));
            phone = "(510) 864-1234";
            phone_parsed = serde_json::from_str(&format!(r#""{}""#, phone)).unwrap();
            expected = PhoneNumber(Some(phonenumber::parse(None, "+15108641234").unwrap()));
            assert_eq!(phone_parsed, expected);
            expected_str = "+1 510-864-1234";
            assert_eq!(expected_str, serde_json::json!(phone_parsed));
            phone = "(510)8641234";
            phone_parsed = serde_json::from_str(&format!(r#""{}""#, phone)).unwrap();
            assert_eq!(phone_parsed, expected);
            expected_str = "+1 510-864-1234";
            assert_eq!(expected_str, serde_json::json!(phone_parsed));
            phone = "";
            phone_parsed = serde_json::from_str(&format!(r#""{}""#, phone)).unwrap();
            assert_eq!(phone_parsed, PhoneNumber(None));
            assert_eq!("", serde_json::json!(phone_parsed));
            phone = "+49 30  1234 1234";
            phone_parsed = serde_json::from_str(&format!(r#""{}""#, phone)).unwrap();
            expected = PhoneNumber(Some(phonenumber::parse(None, phone).unwrap()));
            assert_eq!(phone_parsed, expected);
            expected_str = "+49 30 12341234";
            assert_eq!(expected_str, serde_json::json!(phone_parsed));
        }
    }
}

#[cfg(feature = "requests")]
pub mod error {
    #![doc = " Error methods."]
    #[doc = " Error produced by generated client methods."]
    pub enum Error {
        #[doc = " The request did not conform to API requirements."]
        InvalidRequest(String),
        #[cfg(feature = "retry")]
        #[doc = " A server error either due to the data, or with the connection."]
        CommunicationError(reqwest_middleware::Error),
        #[doc = " A request error, caused when building the request."]
        RequestError(reqwest::Error),
        #[doc = " An expected response whose deserialization failed."]
        SerdeError {
            #[doc = " The error."]
            error: format_serde_error::SerdeError,
            #[doc = " The response status."]
            status: reqwest::StatusCode,
        },
        #[doc = " An expected error response."]
        InvalidResponsePayload {
            #[cfg(feature = "retry")]
            #[doc = " The error."]
            error: reqwest_middleware::Error,
            #[cfg(not(feature = "retry"))]
            #[doc = " The error."]
            error: reqwest::Error,
            #[doc = " The full response."]
            response: reqwest::Response,
        },
        #[doc = " An error from the server."]
        Server {
            #[doc = " The text from the body."]
            body: String,
            #[doc = " The response status."]
            status: reqwest::StatusCode,
        },
        #[doc = " A response not listed in the API description. This may represent a"]
        #[doc = " success or failure response; check `status().is_success()`."]
        UnexpectedResponse(reqwest::Response),
    }

    impl Error {
        #[doc = " Returns the status code, if the error was generated from a response."]
        pub fn status(&self) -> Option<reqwest::StatusCode> {
            match self {
                Error::InvalidRequest(_) => None,
                Error::RequestError(e) => e.status(),
                #[cfg(feature = "retry")]
                Error::CommunicationError(reqwest_middleware::Error::Reqwest(e)) => e.status(),
                #[cfg(feature = "retry")]
                Error::CommunicationError(reqwest_middleware::Error::Middleware(_)) => None,
                Error::SerdeError { error: _, status } => Some(*status),
                Error::InvalidResponsePayload { error: _, response } => Some(response.status()),
                Error::Server { body: _, status } => Some(*status),
                Error::UnexpectedResponse(r) => Some(r.status()),
            }
        }

        #[doc = " Creates a new error from a response status and a serde error."]
        pub fn from_serde_error(
            e: format_serde_error::SerdeError,
            status: reqwest::StatusCode,
        ) -> Self {
            Self::SerdeError { error: e, status }
        }
    }

    #[cfg(feature = "retry")]
    impl From<reqwest_middleware::Error> for Error {
        fn from(e: reqwest_middleware::Error) -> Self {
            Self::CommunicationError(e)
        }
    }

    impl From<reqwest::Error> for Error {
        fn from(e: reqwest::Error) -> Self {
            Self::RequestError(e)
        }
    }

    impl From<serde_json::Error> for Error {
        fn from(e: serde_json::Error) -> Self {
            Self::SerdeError {
                error: format_serde_error::SerdeError::new(String::new(), e),
                status: reqwest::StatusCode::INTERNAL_SERVER_ERROR,
            }
        }
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Error::InvalidRequest(s) => {
                    write!(f, "Invalid Request: {s}")
                }
                #[cfg(feature = "retry")]
                Error::CommunicationError(e) => {
                    write!(f, "Communication Error: {e}")
                }
                Error::RequestError(e) => {
                    write!(f, "Request Error: {e}")
                }
                Error::SerdeError { error, status: _ } => {
                    write!(f, "Serde Error: {error}")
                }
                Error::InvalidResponsePayload { error, response: _ } => {
                    write!(f, "Invalid Response Payload: {error}")
                }
                Error::Server { body, status } => {
                    write!(f, "Server Error: {status} {body}")
                }
                Error::UnexpectedResponse(r) => {
                    write!(f, "Unexpected Response: {r:?}")
                }
            }
        }
    }

    impl std::fmt::Debug for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Display::fmt(self, f)
        }
    }

    impl std::error::Error for Error {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                #[cfg(feature = "retry")]
                Error::CommunicationError(e) => Some(e),
                Error::SerdeError { error, status: _ } => Some(error),
                Error::InvalidResponsePayload { error, response: _ } => Some(error),
                _ => None,
            }
        }
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct AcknowledgeTasks {
    pub result: i64,
}

impl std::fmt::Display for AcknowledgeTasks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AcknowledgeTasks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.result).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["result".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct Actor {
    pub id: i64,
    pub username: String,
}

impl std::fmt::Display for Actor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Actor {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.username.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "username".into()]
    }
}

#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum OutputType {
    #[serde(rename = "pdf")]
    #[display("pdf")]
    Pdf,
    #[serde(rename = "pdfa")]
    #[display("pdfa")]
    Pdfa,
    #[serde(rename = "pdfa-1")]
    #[display("pdfa-1")]
    Pdfa1,
    #[serde(rename = "pdfa-2")]
    #[display("pdfa-2")]
    Pdfa2,
    #[serde(rename = "pdfa-3")]
    #[display("pdfa-3")]
    Pdfa3,
    #[serde(rename = "")]
    #[display("")]
    Empty,
}

#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum Mode {
    #[serde(rename = "skip")]
    #[display("skip")]
    Skip,
    #[serde(rename = "redo")]
    #[display("redo")]
    Redo,
    #[serde(rename = "force")]
    #[display("force")]
    Force,
    #[serde(rename = "skip_noarchive")]
    #[display("skip_noarchive")]
    SkipNoarchive,
    #[serde(rename = "")]
    #[display("")]
    Empty,
}

#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum SkipArchiveFile {
    #[serde(rename = "never")]
    #[display("never")]
    Never,
    #[serde(rename = "with_text")]
    #[display("with_text")]
    WithText,
    #[serde(rename = "always")]
    #[display("always")]
    Always,
    #[serde(rename = "")]
    #[display("")]
    Empty,
}

#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum UnpaperClean {
    #[serde(rename = "clean")]
    #[display("clean")]
    Clean,
    #[serde(rename = "clean-final")]
    #[display("clean-final")]
    CleanFinal,
    #[serde(rename = "none")]
    #[display("none")]
    None,
    #[serde(rename = "")]
    #[display("")]
    Empty,
}

#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum ColorConversionStrategy {
    LeaveColorUnchanged,
    #[serde(rename = "RGB")]
    #[display("RGB")]
    Rgb,
    UseDeviceIndependentColor,
    Gray,
    #[serde(rename = "CMYK")]
    #[display("CMYK")]
    Cmyk,
    #[serde(rename = "")]
    #[display("")]
    Empty,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct ApplicationConfiguration {
    pub id: i64,
    #[serde(default)]
    pub user_args: Option<serde_json::Value>,
    #[serde(default)]
    pub barcode_tag_mapping: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_type: Option<OutputType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pages: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skip_archive_file: Option<SkipArchiveFile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_dpi: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unpaper_clean: Option<UnpaperClean>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deskew: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotate_pages: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotate_pages_threshold: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_image_pixels: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color_conversion_strategy: Option<ColorConversionStrategy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_logo: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barcodes_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barcode_enable_tiff_support: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barcode_string: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barcode_retain_split_pages: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barcode_enable_asn: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barcode_asn_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barcode_upscale: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barcode_dpi: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barcode_max_pages: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barcode_enable_tag: Option<bool>,
}

impl std::fmt::Display for ApplicationConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApplicationConfiguration {
    const LENGTH: usize = 27;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            format!("{:?}", self.user_args).into(),
            format!("{:?}", self.barcode_tag_mapping).into(),
            if let Some(output_type) = &self.output_type {
                format!("{output_type:?}").into()
            } else {
                String::new().into()
            },
            if let Some(pages) = &self.pages {
                format!("{pages:?}").into()
            } else {
                String::new().into()
            },
            if let Some(language) = &self.language {
                format!("{language:?}").into()
            } else {
                String::new().into()
            },
            if let Some(mode) = &self.mode {
                format!("{mode:?}").into()
            } else {
                String::new().into()
            },
            if let Some(skip_archive_file) = &self.skip_archive_file {
                format!("{skip_archive_file:?}").into()
            } else {
                String::new().into()
            },
            if let Some(image_dpi) = &self.image_dpi {
                format!("{image_dpi:?}").into()
            } else {
                String::new().into()
            },
            if let Some(unpaper_clean) = &self.unpaper_clean {
                format!("{unpaper_clean:?}").into()
            } else {
                String::new().into()
            },
            if let Some(deskew) = &self.deskew {
                format!("{deskew:?}").into()
            } else {
                String::new().into()
            },
            if let Some(rotate_pages) = &self.rotate_pages {
                format!("{rotate_pages:?}").into()
            } else {
                String::new().into()
            },
            if let Some(rotate_pages_threshold) = &self.rotate_pages_threshold {
                format!("{rotate_pages_threshold:?}").into()
            } else {
                String::new().into()
            },
            if let Some(max_image_pixels) = &self.max_image_pixels {
                format!("{max_image_pixels:?}").into()
            } else {
                String::new().into()
            },
            if let Some(color_conversion_strategy) = &self.color_conversion_strategy {
                format!("{color_conversion_strategy:?}").into()
            } else {
                String::new().into()
            },
            if let Some(app_title) = &self.app_title {
                format!("{app_title:?}").into()
            } else {
                String::new().into()
            },
            if let Some(app_logo) = &self.app_logo {
                format!("{app_logo:?}").into()
            } else {
                String::new().into()
            },
            if let Some(barcodes_enabled) = &self.barcodes_enabled {
                format!("{barcodes_enabled:?}").into()
            } else {
                String::new().into()
            },
            if let Some(barcode_enable_tiff_support) = &self.barcode_enable_tiff_support {
                format!("{barcode_enable_tiff_support:?}").into()
            } else {
                String::new().into()
            },
            if let Some(barcode_string) = &self.barcode_string {
                format!("{barcode_string:?}").into()
            } else {
                String::new().into()
            },
            if let Some(barcode_retain_split_pages) = &self.barcode_retain_split_pages {
                format!("{barcode_retain_split_pages:?}").into()
            } else {
                String::new().into()
            },
            if let Some(barcode_enable_asn) = &self.barcode_enable_asn {
                format!("{barcode_enable_asn:?}").into()
            } else {
                String::new().into()
            },
            if let Some(barcode_asn_prefix) = &self.barcode_asn_prefix {
                format!("{barcode_asn_prefix:?}").into()
            } else {
                String::new().into()
            },
            if let Some(barcode_upscale) = &self.barcode_upscale {
                format!("{barcode_upscale:?}").into()
            } else {
                String::new().into()
            },
            if let Some(barcode_dpi) = &self.barcode_dpi {
                format!("{barcode_dpi:?}").into()
            } else {
                String::new().into()
            },
            if let Some(barcode_max_pages) = &self.barcode_max_pages {
                format!("{barcode_max_pages:?}").into()
            } else {
                String::new().into()
            },
            if let Some(barcode_enable_tag) = &self.barcode_enable_tag {
                format!("{barcode_enable_tag:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "user_args".into(),
            "barcode_tag_mapping".into(),
            "output_type".into(),
            "pages".into(),
            "language".into(),
            "mode".into(),
            "skip_archive_file".into(),
            "image_dpi".into(),
            "unpaper_clean".into(),
            "deskew".into(),
            "rotate_pages".into(),
            "rotate_pages_threshold".into(),
            "max_image_pixels".into(),
            "color_conversion_strategy".into(),
            "app_title".into(),
            "app_logo".into(),
            "barcodes_enabled".into(),
            "barcode_enable_tiff_support".into(),
            "barcode_string".into(),
            "barcode_retain_split_pages".into(),
            "barcode_enable_asn".into(),
            "barcode_asn_prefix".into(),
            "barcode_upscale".into(),
            "barcode_dpi".into(),
            "barcode_max_pages".into(),
            "barcode_enable_tag".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct ApplicationConfigurationRequest {
    #[serde(default)]
    pub user_args: Option<serde_json::Value>,
    #[serde(default)]
    pub barcode_tag_mapping: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_type: Option<OutputType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pages: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skip_archive_file: Option<SkipArchiveFile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_dpi: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unpaper_clean: Option<UnpaperClean>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deskew: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotate_pages: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotate_pages_threshold: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_image_pixels: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color_conversion_strategy: Option<ColorConversionStrategy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_logo: Option<bytes::Bytes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barcodes_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barcode_enable_tiff_support: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barcode_string: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barcode_retain_split_pages: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barcode_enable_asn: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barcode_asn_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barcode_upscale: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barcode_dpi: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barcode_max_pages: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barcode_enable_tag: Option<bool>,
}

impl std::fmt::Display for ApplicationConfigurationRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApplicationConfigurationRequest {
    const LENGTH: usize = 26;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.user_args).into(),
            format!("{:?}", self.barcode_tag_mapping).into(),
            if let Some(output_type) = &self.output_type {
                format!("{output_type:?}").into()
            } else {
                String::new().into()
            },
            if let Some(pages) = &self.pages {
                format!("{pages:?}").into()
            } else {
                String::new().into()
            },
            if let Some(language) = &self.language {
                format!("{language:?}").into()
            } else {
                String::new().into()
            },
            if let Some(mode) = &self.mode {
                format!("{mode:?}").into()
            } else {
                String::new().into()
            },
            if let Some(skip_archive_file) = &self.skip_archive_file {
                format!("{skip_archive_file:?}").into()
            } else {
                String::new().into()
            },
            if let Some(image_dpi) = &self.image_dpi {
                format!("{image_dpi:?}").into()
            } else {
                String::new().into()
            },
            if let Some(unpaper_clean) = &self.unpaper_clean {
                format!("{unpaper_clean:?}").into()
            } else {
                String::new().into()
            },
            if let Some(deskew) = &self.deskew {
                format!("{deskew:?}").into()
            } else {
                String::new().into()
            },
            if let Some(rotate_pages) = &self.rotate_pages {
                format!("{rotate_pages:?}").into()
            } else {
                String::new().into()
            },
            if let Some(rotate_pages_threshold) = &self.rotate_pages_threshold {
                format!("{rotate_pages_threshold:?}").into()
            } else {
                String::new().into()
            },
            if let Some(max_image_pixels) = &self.max_image_pixels {
                format!("{max_image_pixels:?}").into()
            } else {
                String::new().into()
            },
            if let Some(color_conversion_strategy) = &self.color_conversion_strategy {
                format!("{color_conversion_strategy:?}").into()
            } else {
                String::new().into()
            },
            if let Some(app_title) = &self.app_title {
                format!("{app_title:?}").into()
            } else {
                String::new().into()
            },
            if let Some(app_logo) = &self.app_logo {
                format!("{app_logo:?}").into()
            } else {
                String::new().into()
            },
            if let Some(barcodes_enabled) = &self.barcodes_enabled {
                format!("{barcodes_enabled:?}").into()
            } else {
                String::new().into()
            },
            if let Some(barcode_enable_tiff_support) = &self.barcode_enable_tiff_support {
                format!("{barcode_enable_tiff_support:?}").into()
            } else {
                String::new().into()
            },
            if let Some(barcode_string) = &self.barcode_string {
                format!("{barcode_string:?}").into()
            } else {
                String::new().into()
            },
            if let Some(barcode_retain_split_pages) = &self.barcode_retain_split_pages {
                format!("{barcode_retain_split_pages:?}").into()
            } else {
                String::new().into()
            },
            if let Some(barcode_enable_asn) = &self.barcode_enable_asn {
                format!("{barcode_enable_asn:?}").into()
            } else {
                String::new().into()
            },
            if let Some(barcode_asn_prefix) = &self.barcode_asn_prefix {
                format!("{barcode_asn_prefix:?}").into()
            } else {
                String::new().into()
            },
            if let Some(barcode_upscale) = &self.barcode_upscale {
                format!("{barcode_upscale:?}").into()
            } else {
                String::new().into()
            },
            if let Some(barcode_dpi) = &self.barcode_dpi {
                format!("{barcode_dpi:?}").into()
            } else {
                String::new().into()
            },
            if let Some(barcode_max_pages) = &self.barcode_max_pages {
                format!("{barcode_max_pages:?}").into()
            } else {
                String::new().into()
            },
            if let Some(barcode_enable_tag) = &self.barcode_enable_tag {
                format!("{barcode_enable_tag:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "user_args".into(),
            "barcode_tag_mapping".into(),
            "output_type".into(),
            "pages".into(),
            "language".into(),
            "mode".into(),
            "skip_archive_file".into(),
            "image_dpi".into(),
            "unpaper_clean".into(),
            "deskew".into(),
            "rotate_pages".into(),
            "rotate_pages_threshold".into(),
            "max_image_pixels".into(),
            "color_conversion_strategy".into(),
            "app_title".into(),
            "app_logo".into(),
            "barcodes_enabled".into(),
            "barcode_enable_tiff_support".into(),
            "barcode_string".into(),
            "barcode_retain_split_pages".into(),
            "barcode_enable_asn".into(),
            "barcode_asn_prefix".into(),
            "barcode_upscale".into(),
            "barcode_dpi".into(),
            "barcode_max_pages".into(),
            "barcode_enable_tag".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct BasicUser {
    pub id: i64,
    #[doc = "Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only."]
    pub username: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

impl std::fmt::Display for BasicUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BasicUser {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.username.clone().into(),
            if let Some(first_name) = &self.first_name {
                format!("{first_name:?}").into()
            } else {
                String::new().into()
            },
            if let Some(last_name) = &self.last_name {
                format!("{last_name:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "username".into(),
            "first_name".into(),
            "last_name".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct BasicUserRequest {
    #[doc = "Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only."]
    pub username: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

impl std::fmt::Display for BasicUserRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BasicUserRequest {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.username.clone().into(),
            if let Some(first_name) = &self.first_name {
                format!("{first_name:?}").into()
            } else {
                String::new().into()
            },
            if let Some(last_name) = &self.last_name {
                format!("{last_name:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["username".into(), "first_name".into(), "last_name".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct BulkDownload {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<ContentEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression: Option<CompressionEnum>,
    #[serde(default)]
    pub follow_formatting: bool,
}

impl std::fmt::Display for BulkDownload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BulkDownload {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(content) = &self.content {
                format!("{content:?}").into()
            } else {
                String::new().into()
            },
            if let Some(compression) = &self.compression {
                format!("{compression:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.follow_formatting).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "content".into(),
            "compression".into(),
            "follow_formatting".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct BulkDownloadRequest {
    pub documents: Vec<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<ContentEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression: Option<CompressionEnum>,
    #[serde(default)]
    pub follow_formatting: bool,
}

impl std::fmt::Display for BulkDownloadRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BulkDownloadRequest {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.documents).into(),
            if let Some(content) = &self.content {
                format!("{content:?}").into()
            } else {
                String::new().into()
            },
            if let Some(compression) = &self.compression {
                format!("{compression:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.follow_formatting).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "documents".into(),
            "content".into(),
            "compression".into(),
            "follow_formatting".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct BulkEditDocumentsResult {
    pub result: String,
}

impl std::fmt::Display for BulkEditDocumentsResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BulkEditDocumentsResult {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.result.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["result".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct BulkEditObjectsRequest {
    pub objects: Vec<i64>,
    pub object_type: ObjectTypeEnum,
    pub operation: OperationEnum,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub merge: bool,
}

impl std::fmt::Display for BulkEditObjectsRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BulkEditObjectsRequest {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.objects).into(),
            format!("{:?}", self.object_type).into(),
            format!("{:?}", self.operation).into(),
            if let Some(owner) = &self.owner {
                format!("{owner:?}").into()
            } else {
                String::new().into()
            },
            if let Some(permissions) = &self.permissions {
                format!("{permissions:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.merge).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "objects".into(),
            "object_type".into(),
            "operation".into(),
            "owner".into(),
            "permissions".into(),
            "merge".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct BulkEditRequest {
    pub documents: Vec<i64>,
    pub method: MethodEnum,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl std::fmt::Display for BulkEditRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BulkEditRequest {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.documents).into(),
            format!("{:?}", self.method).into(),
            if let Some(parameters) = &self.parameters {
                format!("{parameters:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["documents".into(), "method".into(), "parameters".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct BulkEditResult {
    pub result: String,
}

impl std::fmt::Display for BulkEditResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BulkEditResult {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.result.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["result".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct Classifier {
    pub status: String,
    pub error: String,
    pub last_trained: chrono::DateTime<chrono::Utc>,
}

impl std::fmt::Display for Classifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Classifier {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.status.clone().into(),
            self.error.clone().into(),
            format!("{:?}", self.last_trained).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["status".into(), "error".into(), "last_trained".into()]
    }
}

#[doc = "* `LeaveColorUnchanged` - LeaveColorUnchanged\n* `RGB` - RGB\n* `UseDeviceIndependentColor` - UseDeviceIndependentColor\n* `Gray` - Gray\n* `CMYK` - CMYK"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum ColorConversionStrategyEnum {
    LeaveColorUnchanged,
    #[serde(rename = "RGB")]
    #[display("RGB")]
    Rgb,
    UseDeviceIndependentColor,
    Gray,
    #[serde(rename = "CMYK")]
    #[display("CMYK")]
    Cmyk,
    #[serde(rename = "")]
    #[display("")]
    Empty,
}

#[doc = "* `none` - none\n* `deflated` - deflated\n* `bzip2` - bzip2\n* `lzma` - lzma"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum CompressionEnum {
    #[serde(rename = "none")]
    #[display("none")]
    None,
    #[serde(rename = "deflated")]
    #[display("deflated")]
    Deflated,
    #[serde(rename = "bzip2")]
    #[display("bzip2")]
    Bzip2,
    #[serde(rename = "lzma")]
    #[display("lzma")]
    Lzma,
}

#[doc = "* `archive` - archive\n* `originals` - originals\n* `both` - both"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum ContentEnum {
    #[serde(rename = "archive")]
    #[display("archive")]
    Archive,
    #[serde(rename = "originals")]
    #[display("originals")]
    Originals,
    #[serde(rename = "both")]
    #[display("both")]
    Both,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct View {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<i64>>,
}

impl std::fmt::Display for View {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for View {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(users) = &self.users {
                format!("{users:?}").into()
            } else {
                String::new().into()
            },
            if let Some(groups) = &self.groups {
                format!("{groups:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["users".into(), "groups".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct Change {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<i64>>,
}

impl std::fmt::Display for Change {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Change {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(users) = &self.users {
                format!("{users:?}").into()
            } else {
                String::new().into()
            },
            if let Some(groups) = &self.groups {
                format!("{groups:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["users".into(), "groups".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct Permissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view: Option<View>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change: Option<Change>,
}

impl std::fmt::Display for Permissions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Permissions {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(view) = &self.view {
                format!("{view:?}").into()
            } else {
                String::new().into()
            },
            if let Some(change) = &self.change {
                format!("{change:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["view".into(), "change".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct Correspondent {
    pub id: i64,
    pub slug: String,
    pub name: String,
    #[serde(rename = "match", default, skip_serializing_if = "Option::is_none")]
    pub match_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matching_algorithm: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_insensitive: Option<bool>,
    pub document_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_correspondence: Option<chrono::NaiveDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<i64>,
    pub permissions: Permissions,
    pub user_can_change: bool,
}

impl std::fmt::Display for Correspondent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Correspondent {
    const LENGTH: usize = 11;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.slug.clone().into(),
            self.name.clone().into(),
            if let Some(match_) = &self.match_ {
                format!("{match_:?}").into()
            } else {
                String::new().into()
            },
            if let Some(matching_algorithm) = &self.matching_algorithm {
                format!("{matching_algorithm:?}").into()
            } else {
                String::new().into()
            },
            if let Some(is_insensitive) = &self.is_insensitive {
                format!("{is_insensitive:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.document_count).into(),
            if let Some(last_correspondence) = &self.last_correspondence {
                format!("{last_correspondence:?}").into()
            } else {
                String::new().into()
            },
            if let Some(owner) = &self.owner {
                format!("{owner:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.permissions).into(),
            format!("{:?}", self.user_can_change).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "slug".into(),
            "name".into(),
            "match_".into(),
            "matching_algorithm".into(),
            "is_insensitive".into(),
            "document_count".into(),
            "last_correspondence".into(),
            "owner".into(),
            "permissions".into(),
            "user_can_change".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct CorrespondentCounts {
    pub id: i64,
    pub document_count: i64,
}

impl std::fmt::Display for CorrespondentCounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CorrespondentCounts {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            format!("{:?}", self.document_count).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "document_count".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct SetPermissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view: Option<View>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change: Option<Change>,
}

impl std::fmt::Display for SetPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SetPermissions {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(view) = &self.view {
                format!("{view:?}").into()
            } else {
                String::new().into()
            },
            if let Some(change) = &self.change {
                format!("{change:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["view".into(), "change".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct CorrespondentRequest {
    pub name: String,
    #[serde(rename = "match", default, skip_serializing_if = "Option::is_none")]
    pub match_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matching_algorithm: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_insensitive: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_permissions: Option<SetPermissions>,
}

impl std::fmt::Display for CorrespondentRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CorrespondentRequest {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.name.clone().into(),
            if let Some(match_) = &self.match_ {
                format!("{match_:?}").into()
            } else {
                String::new().into()
            },
            if let Some(matching_algorithm) = &self.matching_algorithm {
                format!("{matching_algorithm:?}").into()
            } else {
                String::new().into()
            },
            if let Some(is_insensitive) = &self.is_insensitive {
                format!("{is_insensitive:?}").into()
            } else {
                String::new().into()
            },
            if let Some(owner) = &self.owner {
                format!("{owner:?}").into()
            } else {
                String::new().into()
            },
            if let Some(set_permissions) = &self.set_permissions {
                format!("{set_permissions:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "match_".into(),
            "matching_algorithm".into(),
            "is_insensitive".into(),
            "owner".into(),
            "set_permissions".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct CustomField {
    pub id: i64,
    pub name: String,
    #[doc = "* `string` - string\n* `url` - url\n* `date` - date\n* `boolean` - boolean\n* `integer` - integer\n* `float` - float\n* `monetary` - monetary\n* `documentlink` - documentlink\n* `select` - select"]
    pub data_type: DataTypeEnum,
    #[doc = "Extra data for the custom field, such as select options"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_data: Option<serde_json::Value>,
    pub document_count: i64,
}

impl std::fmt::Display for CustomField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CustomField {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.name.clone().into(),
            format!("{:?}", self.data_type).into(),
            if let Some(extra_data) = &self.extra_data {
                format!("{extra_data:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.document_count).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "data_type".into(),
            "extra_data".into(),
            "document_count".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct CustomFieldCounts {
    pub id: i64,
    pub document_count: i64,
}

impl std::fmt::Display for CustomFieldCounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CustomFieldCounts {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            format!("{:?}", self.document_count).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "document_count".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct CustomFieldInstance {
    #[doc = "Given the *incoming* primitive data, return the value for this field\nthat should be validated and transformed to a native value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    pub field: i64,
}

impl std::fmt::Display for CustomFieldInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CustomFieldInstance {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(value) = &self.value {
                format!("{value:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.field).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["value".into(), "field".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct CustomFieldInstanceRequest {
    #[doc = "Given the *incoming* primitive data, return the value for this field\nthat should be validated and transformed to a native value."]
    #[serde(default)]
    pub value: Option<serde_json::Value>,
    pub field: i64,
}

impl std::fmt::Display for CustomFieldInstanceRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CustomFieldInstanceRequest {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.value).into(),
            format!("{:?}", self.field).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["value".into(), "field".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct CustomFieldRequest {
    pub name: String,
    #[doc = "* `string` - string\n* `url` - url\n* `date` - date\n* `boolean` - boolean\n* `integer` - integer\n* `float` - float\n* `monetary` - monetary\n* `documentlink` - documentlink\n* `select` - select"]
    pub data_type: DataTypeEnum,
    #[doc = "Extra data for the custom field, such as select options"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_data: Option<serde_json::Value>,
}

impl std::fmt::Display for CustomFieldRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CustomFieldRequest {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.name.clone().into(),
            format!("{:?}", self.data_type).into(),
            if let Some(extra_data) = &self.extra_data {
                format!("{extra_data:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "data_type".into(), "extra_data".into()]
    }
}

#[doc = "* `string` - string\n* `url` - url\n* `date` - date\n* `boolean` - boolean\n* `integer` - integer\n* `float` - float\n* `monetary` - monetary\n* `documentlink` - documentlink\n* `select` - select"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum DataTypeEnum {
    #[serde(rename = "string")]
    #[display("string")]
    String,
    #[serde(rename = "url")]
    #[display("url")]
    Url,
    #[serde(rename = "date")]
    #[display("date")]
    Date,
    #[serde(rename = "boolean")]
    #[display("boolean")]
    Boolean,
    #[serde(rename = "integer")]
    #[display("integer")]
    Integer,
    #[serde(rename = "float")]
    #[display("float")]
    Float,
    #[serde(rename = "monetary")]
    #[display("monetary")]
    Monetary,
    #[serde(rename = "documentlink")]
    #[display("documentlink")]
    Documentlink,
    #[serde(rename = "select")]
    #[display("select")]
    Select,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct Database {
    #[serde(rename = "type")]
    pub type_: String,
    pub url: String,
    pub status: String,
    pub error: String,
    pub migration_status: MigrationStatus,
}

impl std::fmt::Display for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Database {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.type_.clone().into(),
            self.url.clone().into(),
            self.status.clone().into(),
            self.error.clone().into(),
            format!("{:?}", self.migration_status).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "type_".into(),
            "url".into(),
            "status".into(),
            "error".into(),
            "migration_status".into(),
        ]
    }
}

#[doc = "* `table` - Table\n* `smallCards` - Small Cards\n* `largeCards` - Large Cards"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum DisplayModeEnum {
    #[serde(rename = "table")]
    #[display("table")]
    Table,
    #[serde(rename = "smallCards")]
    #[display("smallCards")]
    SmallCards,
    #[serde(rename = "largeCards")]
    #[display("largeCards")]
    LargeCards,
    #[serde(rename = "")]
    #[display("")]
    Empty,
}

#[doc = "Adds update nested feature"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct Document {
    pub id: i64,
    #[serde(default)]
    pub correspondent: Option<i64>,
    #[serde(default)]
    pub document_type: Option<i64>,
    #[serde(default)]
    pub storage_path: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The raw, text-only data of the document. This field is primarily used for searching."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    pub tags: Vec<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<chrono::NaiveDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[deprecated]
    pub created_date: Option<chrono::NaiveDate>,
    pub modified: chrono::DateTime<chrono::Utc>,
    pub added: chrono::DateTime<chrono::Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The position of this document in your physical document archive."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive_serial_number: Option<i64>,
    #[serde(default)]
    pub original_file_name: Option<String>,
    #[serde(default)]
    pub archived_file_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    #[serde(default)]
    pub user_can_change: Option<bool>,
    #[serde(default)]
    pub is_shared_by_requester: Option<bool>,
    pub notes: Vec<Notes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldInstance>>,
    #[serde(default)]
    pub page_count: Option<i64>,
    pub mime_type: String,
}

impl std::fmt::Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Document {
    const LENGTH: usize = 23;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            format!("{:?}", self.correspondent).into(),
            format!("{:?}", self.document_type).into(),
            format!("{:?}", self.storage_path).into(),
            if let Some(title) = &self.title {
                format!("{title:?}").into()
            } else {
                String::new().into()
            },
            if let Some(content) = &self.content {
                format!("{content:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.tags).into(),
            if let Some(created) = &self.created {
                format!("{created:?}").into()
            } else {
                String::new().into()
            },
            if let Some(created_date) = &self.created_date {
                format!("{created_date:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.modified).into(),
            format!("{:?}", self.added).into(),
            if let Some(deleted_at) = &self.deleted_at {
                format!("{deleted_at:?}").into()
            } else {
                String::new().into()
            },
            if let Some(archive_serial_number) = &self.archive_serial_number {
                format!("{archive_serial_number:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.original_file_name).into(),
            format!("{:?}", self.archived_file_name).into(),
            if let Some(owner) = &self.owner {
                format!("{owner:?}").into()
            } else {
                String::new().into()
            },
            if let Some(permissions) = &self.permissions {
                format!("{permissions:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.user_can_change).into(),
            format!("{:?}", self.is_shared_by_requester).into(),
            format!("{:?}", self.notes).into(),
            if let Some(custom_fields) = &self.custom_fields {
                format!("{custom_fields:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.page_count).into(),
            self.mime_type.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "correspondent".into(),
            "document_type".into(),
            "storage_path".into(),
            "title".into(),
            "content".into(),
            "tags".into(),
            "created".into(),
            "created_date".into(),
            "modified".into(),
            "added".into(),
            "deleted_at".into(),
            "archive_serial_number".into(),
            "original_file_name".into(),
            "archived_file_name".into(),
            "owner".into(),
            "permissions".into(),
            "user_can_change".into(),
            "is_shared_by_requester".into(),
            "notes".into(),
            "custom_fields".into(),
            "page_count".into(),
            "mime_type".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct DocumentListRequest {
    pub documents: Vec<i64>,
}

impl std::fmt::Display for DocumentListRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for DocumentListRequest {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.documents).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["documents".into()]
    }
}

#[doc = "Adds update nested feature"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct DocumentRequest {
    #[serde(default)]
    pub correspondent: Option<i64>,
    #[serde(default)]
    pub document_type: Option<i64>,
    #[serde(default)]
    pub storage_path: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The raw, text-only data of the document. This field is primarily used for searching."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    pub tags: Vec<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<chrono::NaiveDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[deprecated]
    pub created_date: Option<chrono::NaiveDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The position of this document in your physical document archive."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive_serial_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_permissions: Option<SetPermissions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldInstanceRequest>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_inbox_tags: Option<bool>,
}

impl std::fmt::Display for DocumentRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for DocumentRequest {
    const LENGTH: usize = 14;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.correspondent).into(),
            format!("{:?}", self.document_type).into(),
            format!("{:?}", self.storage_path).into(),
            if let Some(title) = &self.title {
                format!("{title:?}").into()
            } else {
                String::new().into()
            },
            if let Some(content) = &self.content {
                format!("{content:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.tags).into(),
            if let Some(created) = &self.created {
                format!("{created:?}").into()
            } else {
                String::new().into()
            },
            if let Some(created_date) = &self.created_date {
                format!("{created_date:?}").into()
            } else {
                String::new().into()
            },
            if let Some(deleted_at) = &self.deleted_at {
                format!("{deleted_at:?}").into()
            } else {
                String::new().into()
            },
            if let Some(archive_serial_number) = &self.archive_serial_number {
                format!("{archive_serial_number:?}").into()
            } else {
                String::new().into()
            },
            if let Some(owner) = &self.owner {
                format!("{owner:?}").into()
            } else {
                String::new().into()
            },
            if let Some(set_permissions) = &self.set_permissions {
                format!("{set_permissions:?}").into()
            } else {
                String::new().into()
            },
            if let Some(custom_fields) = &self.custom_fields {
                format!("{custom_fields:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_inbox_tags) = &self.remove_inbox_tags {
                format!("{remove_inbox_tags:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "correspondent".into(),
            "document_type".into(),
            "storage_path".into(),
            "title".into(),
            "content".into(),
            "tags".into(),
            "created".into(),
            "created_date".into(),
            "deleted_at".into(),
            "archive_serial_number".into(),
            "owner".into(),
            "set_permissions".into(),
            "custom_fields".into(),
            "remove_inbox_tags".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct DocumentType {
    pub id: i64,
    pub slug: String,
    pub name: String,
    #[serde(rename = "match", default, skip_serializing_if = "Option::is_none")]
    pub match_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matching_algorithm: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_insensitive: Option<bool>,
    pub document_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<i64>,
    pub permissions: Permissions,
    pub user_can_change: bool,
}

impl std::fmt::Display for DocumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for DocumentType {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.slug.clone().into(),
            self.name.clone().into(),
            if let Some(match_) = &self.match_ {
                format!("{match_:?}").into()
            } else {
                String::new().into()
            },
            if let Some(matching_algorithm) = &self.matching_algorithm {
                format!("{matching_algorithm:?}").into()
            } else {
                String::new().into()
            },
            if let Some(is_insensitive) = &self.is_insensitive {
                format!("{is_insensitive:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.document_count).into(),
            if let Some(owner) = &self.owner {
                format!("{owner:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.permissions).into(),
            format!("{:?}", self.user_can_change).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "slug".into(),
            "name".into(),
            "match_".into(),
            "matching_algorithm".into(),
            "is_insensitive".into(),
            "document_count".into(),
            "owner".into(),
            "permissions".into(),
            "user_can_change".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct DocumentTypeCounts {
    pub id: i64,
    pub document_count: i64,
}

impl std::fmt::Display for DocumentTypeCounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for DocumentTypeCounts {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            format!("{:?}", self.document_count).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "document_count".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct DocumentTypeRequest {
    pub name: String,
    #[serde(rename = "match", default, skip_serializing_if = "Option::is_none")]
    pub match_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matching_algorithm: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_insensitive: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_permissions: Option<SetPermissions>,
}

impl std::fmt::Display for DocumentTypeRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for DocumentTypeRequest {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.name.clone().into(),
            if let Some(match_) = &self.match_ {
                format!("{match_:?}").into()
            } else {
                String::new().into()
            },
            if let Some(matching_algorithm) = &self.matching_algorithm {
                format!("{matching_algorithm:?}").into()
            } else {
                String::new().into()
            },
            if let Some(is_insensitive) = &self.is_insensitive {
                format!("{is_insensitive:?}").into()
            } else {
                String::new().into()
            },
            if let Some(owner) = &self.owner {
                format!("{owner:?}").into()
            } else {
                String::new().into()
            },
            if let Some(set_permissions) = &self.set_permissions {
                format!("{set_permissions:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "match_".into(),
            "matching_algorithm".into(),
            "is_insensitive".into(),
            "owner".into(),
            "set_permissions".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct EmailRequestRequest {
    pub addresses: String,
    pub subject: String,
    pub message: String,
    #[serde(default)]
    pub use_archive_version: bool,
}

impl std::fmt::Display for EmailRequestRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for EmailRequestRequest {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.addresses.clone().into(),
            self.subject.clone().into(),
            self.message.clone().into(),
            format!("{:?}", self.use_archive_version).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "addresses".into(),
            "subject".into(),
            "message".into(),
            "use_archive_version".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct EmailResponse {
    pub message: String,
}

impl std::fmt::Display for EmailResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for EmailResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.message.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["message".into()]
    }
}

#[doc = "* `archive` - Archive\n* `original` - Original"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum FileVersionEnum {
    #[serde(rename = "archive")]
    #[display("archive")]
    Archive,
    #[serde(rename = "original")]
    #[display("original")]
    Original,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct Group {
    pub id: i64,
    pub name: String,
    pub permissions: Vec<String>,
}

impl std::fmt::Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Group {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.name.clone().into(),
            format!("{:?}", self.permissions).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "name".into(), "permissions".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct GroupRequest {
    pub name: String,
    pub permissions: Vec<String>,
}

impl std::fmt::Display for GroupRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GroupRequest {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.name.clone().into(),
            format!("{:?}", self.permissions).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "permissions".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct Index {
    pub status: String,
    pub error: String,
    pub last_modified: chrono::DateTime<chrono::Utc>,
}

impl std::fmt::Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Index {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.status.clone().into(),
            self.error.clone().into(),
            format!("{:?}", self.last_modified).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["status".into(), "error".into(), "last_modified".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct LogEntry {
    pub id: i64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub action: String,
    pub changes: std::collections::HashMap<String, serde_json::Value>,
    pub actor: Actor,
}

impl std::fmt::Display for LogEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for LogEntry {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            format!("{:?}", self.timestamp).into(),
            self.action.clone().into(),
            format!("{:?}", self.changes).into(),
            format!("{:?}", self.actor).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "timestamp".into(),
            "action".into(),
            "changes".into(),
            "actor".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct MailAccount {
    pub id: i64,
    pub name: String,
    pub imap_server: String,
    #[doc = "This is usually 143 for unencrypted and STARTTLS connections, and 993 for SSL connections."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imap_port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imap_security: Option<i64>,
    pub username: String,
    pub password: String,
    #[doc = "The character set to use when communicating with the mail server, such as 'UTF-8' or 'US-ASCII'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub character_set: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_token: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<i64>,
    pub user_can_change: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_type: Option<i64>,
    #[doc = "The expiration date of the refresh token. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration: Option<chrono::DateTime<chrono::Utc>>,
}

impl std::fmt::Display for MailAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for MailAccount {
    const LENGTH: usize = 13;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.name.clone().into(),
            self.imap_server.clone().into(),
            if let Some(imap_port) = &self.imap_port {
                format!("{imap_port:?}").into()
            } else {
                String::new().into()
            },
            if let Some(imap_security) = &self.imap_security {
                format!("{imap_security:?}").into()
            } else {
                String::new().into()
            },
            self.username.clone().into(),
            self.password.clone().into(),
            if let Some(character_set) = &self.character_set {
                format!("{character_set:?}").into()
            } else {
                String::new().into()
            },
            if let Some(is_token) = &self.is_token {
                format!("{is_token:?}").into()
            } else {
                String::new().into()
            },
            if let Some(owner) = &self.owner {
                format!("{owner:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.user_can_change).into(),
            if let Some(account_type) = &self.account_type {
                format!("{account_type:?}").into()
            } else {
                String::new().into()
            },
            if let Some(expiration) = &self.expiration {
                format!("{expiration:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "imap_server".into(),
            "imap_port".into(),
            "imap_security".into(),
            "username".into(),
            "password".into(),
            "character_set".into(),
            "is_token".into(),
            "owner".into(),
            "user_can_change".into(),
            "account_type".into(),
            "expiration".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct MailAccountProcessResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

impl std::fmt::Display for MailAccountProcessResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for MailAccountProcessResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(result) = &self.result {
            format!("{result:?}").into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["result".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct MailAccountRequest {
    pub name: String,
    pub imap_server: String,
    #[doc = "This is usually 143 for unencrypted and STARTTLS connections, and 993 for SSL connections."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imap_port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imap_security: Option<i64>,
    pub username: String,
    pub password: String,
    #[doc = "The character set to use when communicating with the mail server, such as 'UTF-8' or 'US-ASCII'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub character_set: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_token: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_permissions: Option<SetPermissions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_type: Option<i64>,
    #[doc = "The expiration date of the refresh token. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration: Option<chrono::DateTime<chrono::Utc>>,
}

impl std::fmt::Display for MailAccountRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for MailAccountRequest {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.name.clone().into(),
            self.imap_server.clone().into(),
            if let Some(imap_port) = &self.imap_port {
                format!("{imap_port:?}").into()
            } else {
                String::new().into()
            },
            if let Some(imap_security) = &self.imap_security {
                format!("{imap_security:?}").into()
            } else {
                String::new().into()
            },
            self.username.clone().into(),
            self.password.clone().into(),
            if let Some(character_set) = &self.character_set {
                format!("{character_set:?}").into()
            } else {
                String::new().into()
            },
            if let Some(is_token) = &self.is_token {
                format!("{is_token:?}").into()
            } else {
                String::new().into()
            },
            if let Some(owner) = &self.owner {
                format!("{owner:?}").into()
            } else {
                String::new().into()
            },
            if let Some(set_permissions) = &self.set_permissions {
                format!("{set_permissions:?}").into()
            } else {
                String::new().into()
            },
            if let Some(account_type) = &self.account_type {
                format!("{account_type:?}").into()
            } else {
                String::new().into()
            },
            if let Some(expiration) = &self.expiration {
                format!("{expiration:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "imap_server".into(),
            "imap_port".into(),
            "imap_security".into(),
            "username".into(),
            "password".into(),
            "character_set".into(),
            "is_token".into(),
            "owner".into(),
            "set_permissions".into(),
            "account_type".into(),
            "expiration".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct MailAccountTestResponse {
    pub success: bool,
}

impl std::fmt::Display for MailAccountTestResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for MailAccountTestResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.success).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["success".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct MailRule {
    pub id: i64,
    pub name: String,
    pub account: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Subfolders must be separated by a delimiter, often a dot ('.') or slash ('/'), but it varies by mail server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_from: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_body: Option<String>,
    #[doc = "Only consume documents which entirely match this filename if specified. Wildcards such as *.pdf or *invoice* are allowed. Case insensitive."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_attachment_filename_include: Option<String>,
    #[doc = "Do not consume documents which entirely match this filename if specified. Wildcards such as *.pdf or *invoice* are allowed. Case insensitive."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_attachment_filename_exclude: Option<String>,
    #[doc = "Specified in days."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_age: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_parameter: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_title_from: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_tags: Option<Vec<Option<i64>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_correspondent_from: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_correspondent: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_document_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_owner_from_rule: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
    #[doc = "Inline attachments include embedded images, so it's best to combine this option with a filename filter.\n\n* `1` - Only process attachments.\n* `2` - Process all files, including 'inline' attachments."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consumption_scope: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pdf_layout: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<i64>,
    pub user_can_change: bool,
}

impl std::fmt::Display for MailRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for MailRule {
    const LENGTH: usize = 26;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.name.clone().into(),
            format!("{:?}", self.account).into(),
            if let Some(enabled) = &self.enabled {
                format!("{enabled:?}").into()
            } else {
                String::new().into()
            },
            if let Some(folder) = &self.folder {
                format!("{folder:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_from) = &self.filter_from {
                format!("{filter_from:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_to) = &self.filter_to {
                format!("{filter_to:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_subject) = &self.filter_subject {
                format!("{filter_subject:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_body) = &self.filter_body {
                format!("{filter_body:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_attachment_filename_include) =
                &self.filter_attachment_filename_include
            {
                format!("{filter_attachment_filename_include:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_attachment_filename_exclude) =
                &self.filter_attachment_filename_exclude
            {
                format!("{filter_attachment_filename_exclude:?}").into()
            } else {
                String::new().into()
            },
            if let Some(maximum_age) = &self.maximum_age {
                format!("{maximum_age:?}").into()
            } else {
                String::new().into()
            },
            if let Some(action) = &self.action {
                format!("{action:?}").into()
            } else {
                String::new().into()
            },
            if let Some(action_parameter) = &self.action_parameter {
                format!("{action_parameter:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_title_from) = &self.assign_title_from {
                format!("{assign_title_from:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_tags) = &self.assign_tags {
                format!("{assign_tags:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_correspondent_from) = &self.assign_correspondent_from {
                format!("{assign_correspondent_from:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_correspondent) = &self.assign_correspondent {
                format!("{assign_correspondent:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_document_type) = &self.assign_document_type {
                format!("{assign_document_type:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_owner_from_rule) = &self.assign_owner_from_rule {
                format!("{assign_owner_from_rule:?}").into()
            } else {
                String::new().into()
            },
            if let Some(order) = &self.order {
                format!("{order:?}").into()
            } else {
                String::new().into()
            },
            if let Some(attachment_type) = &self.attachment_type {
                format!("{attachment_type:?}").into()
            } else {
                String::new().into()
            },
            if let Some(consumption_scope) = &self.consumption_scope {
                format!("{consumption_scope:?}").into()
            } else {
                String::new().into()
            },
            if let Some(pdf_layout) = &self.pdf_layout {
                format!("{pdf_layout:?}").into()
            } else {
                String::new().into()
            },
            if let Some(owner) = &self.owner {
                format!("{owner:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.user_can_change).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "account".into(),
            "enabled".into(),
            "folder".into(),
            "filter_from".into(),
            "filter_to".into(),
            "filter_subject".into(),
            "filter_body".into(),
            "filter_attachment_filename_include".into(),
            "filter_attachment_filename_exclude".into(),
            "maximum_age".into(),
            "action".into(),
            "action_parameter".into(),
            "assign_title_from".into(),
            "assign_tags".into(),
            "assign_correspondent_from".into(),
            "assign_correspondent".into(),
            "assign_document_type".into(),
            "assign_owner_from_rule".into(),
            "order".into(),
            "attachment_type".into(),
            "consumption_scope".into(),
            "pdf_layout".into(),
            "owner".into(),
            "user_can_change".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct MailRuleRequest {
    pub name: String,
    pub account: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Subfolders must be separated by a delimiter, often a dot ('.') or slash ('/'), but it varies by mail server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_from: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_body: Option<String>,
    #[doc = "Only consume documents which entirely match this filename if specified. Wildcards such as *.pdf or *invoice* are allowed. Case insensitive."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_attachment_filename_include: Option<String>,
    #[doc = "Do not consume documents which entirely match this filename if specified. Wildcards such as *.pdf or *invoice* are allowed. Case insensitive."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_attachment_filename_exclude: Option<String>,
    #[doc = "Specified in days."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_age: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_parameter: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_title_from: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_tags: Option<Vec<Option<i64>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_correspondent_from: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_correspondent: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_document_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_owner_from_rule: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
    #[doc = "Inline attachments include embedded images, so it's best to combine this option with a filename filter.\n\n* `1` - Only process attachments.\n* `2` - Process all files, including 'inline' attachments."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consumption_scope: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pdf_layout: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_permissions: Option<SetPermissions>,
}

impl std::fmt::Display for MailRuleRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for MailRuleRequest {
    const LENGTH: usize = 25;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.name.clone().into(),
            format!("{:?}", self.account).into(),
            if let Some(enabled) = &self.enabled {
                format!("{enabled:?}").into()
            } else {
                String::new().into()
            },
            if let Some(folder) = &self.folder {
                format!("{folder:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_from) = &self.filter_from {
                format!("{filter_from:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_to) = &self.filter_to {
                format!("{filter_to:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_subject) = &self.filter_subject {
                format!("{filter_subject:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_body) = &self.filter_body {
                format!("{filter_body:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_attachment_filename_include) =
                &self.filter_attachment_filename_include
            {
                format!("{filter_attachment_filename_include:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_attachment_filename_exclude) =
                &self.filter_attachment_filename_exclude
            {
                format!("{filter_attachment_filename_exclude:?}").into()
            } else {
                String::new().into()
            },
            if let Some(maximum_age) = &self.maximum_age {
                format!("{maximum_age:?}").into()
            } else {
                String::new().into()
            },
            if let Some(action) = &self.action {
                format!("{action:?}").into()
            } else {
                String::new().into()
            },
            if let Some(action_parameter) = &self.action_parameter {
                format!("{action_parameter:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_title_from) = &self.assign_title_from {
                format!("{assign_title_from:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_tags) = &self.assign_tags {
                format!("{assign_tags:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_correspondent_from) = &self.assign_correspondent_from {
                format!("{assign_correspondent_from:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_correspondent) = &self.assign_correspondent {
                format!("{assign_correspondent:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_document_type) = &self.assign_document_type {
                format!("{assign_document_type:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_owner_from_rule) = &self.assign_owner_from_rule {
                format!("{assign_owner_from_rule:?}").into()
            } else {
                String::new().into()
            },
            if let Some(order) = &self.order {
                format!("{order:?}").into()
            } else {
                String::new().into()
            },
            if let Some(attachment_type) = &self.attachment_type {
                format!("{attachment_type:?}").into()
            } else {
                String::new().into()
            },
            if let Some(consumption_scope) = &self.consumption_scope {
                format!("{consumption_scope:?}").into()
            } else {
                String::new().into()
            },
            if let Some(pdf_layout) = &self.pdf_layout {
                format!("{pdf_layout:?}").into()
            } else {
                String::new().into()
            },
            if let Some(owner) = &self.owner {
                format!("{owner:?}").into()
            } else {
                String::new().into()
            },
            if let Some(set_permissions) = &self.set_permissions {
                format!("{set_permissions:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "account".into(),
            "enabled".into(),
            "folder".into(),
            "filter_from".into(),
            "filter_to".into(),
            "filter_subject".into(),
            "filter_body".into(),
            "filter_attachment_filename_include".into(),
            "filter_attachment_filename_exclude".into(),
            "maximum_age".into(),
            "action".into(),
            "action_parameter".into(),
            "assign_title_from".into(),
            "assign_tags".into(),
            "assign_correspondent_from".into(),
            "assign_correspondent".into(),
            "assign_document_type".into(),
            "assign_owner_from_rule".into(),
            "order".into(),
            "attachment_type".into(),
            "consumption_scope".into(),
            "pdf_layout".into(),
            "owner".into(),
            "set_permissions".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct Metadata {
    pub original_checksum: String,
    pub original_size: i64,
    pub original_mime_type: String,
    pub media_filename: String,
    pub has_archive_version: bool,
    pub original_metadata: std::collections::HashMap<String, serde_json::Value>,
    pub archive_checksum: String,
    pub archive_media_filename: String,
    pub original_filename: String,
    pub archive_size: i64,
    pub archive_metadata: std::collections::HashMap<String, serde_json::Value>,
    pub lang: String,
}

impl std::fmt::Display for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Metadata {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.original_checksum.clone().into(),
            format!("{:?}", self.original_size).into(),
            self.original_mime_type.clone().into(),
            self.media_filename.clone().into(),
            format!("{:?}", self.has_archive_version).into(),
            format!("{:?}", self.original_metadata).into(),
            self.archive_checksum.clone().into(),
            self.archive_media_filename.clone().into(),
            self.original_filename.clone().into(),
            format!("{:?}", self.archive_size).into(),
            format!("{:?}", self.archive_metadata).into(),
            self.lang.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "original_checksum".into(),
            "original_size".into(),
            "original_mime_type".into(),
            "media_filename".into(),
            "has_archive_version".into(),
            "original_metadata".into(),
            "archive_checksum".into(),
            "archive_media_filename".into(),
            "original_filename".into(),
            "archive_size".into(),
            "archive_metadata".into(),
            "lang".into(),
        ]
    }
}

#[doc = "* `set_correspondent` - set_correspondent\n* `set_document_type` - set_document_type\n* `set_storage_path` - set_storage_path\n* `add_tag` - add_tag\n* `remove_tag` - remove_tag\n* `modify_tags` - modify_tags\n* `modify_custom_fields` - modify_custom_fields\n* `delete` - delete\n* `reprocess` - reprocess\n* `set_permissions` - set_permissions\n* `rotate` - rotate\n* `merge` - merge\n* `split` - split\n* `delete_pages` - delete_pages\n* `edit_pdf` - edit_pdf"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum MethodEnum {
    #[serde(rename = "set_correspondent")]
    #[display("set_correspondent")]
    SetCorrespondent,
    #[serde(rename = "set_document_type")]
    #[display("set_document_type")]
    SetDocumentType,
    #[serde(rename = "set_storage_path")]
    #[display("set_storage_path")]
    SetStoragePath,
    #[serde(rename = "add_tag")]
    #[display("add_tag")]
    AddTag,
    #[serde(rename = "remove_tag")]
    #[display("remove_tag")]
    RemoveTag,
    #[serde(rename = "modify_tags")]
    #[display("modify_tags")]
    ModifyTags,
    #[serde(rename = "modify_custom_fields")]
    #[display("modify_custom_fields")]
    ModifyCustomFields,
    #[serde(rename = "delete")]
    #[display("delete")]
    Delete,
    #[serde(rename = "reprocess")]
    #[display("reprocess")]
    Reprocess,
    #[serde(rename = "set_permissions")]
    #[display("set_permissions")]
    SetPermissions,
    #[serde(rename = "rotate")]
    #[display("rotate")]
    Rotate,
    #[serde(rename = "merge")]
    #[display("merge")]
    Merge,
    #[serde(rename = "split")]
    #[display("split")]
    Split,
    #[serde(rename = "delete_pages")]
    #[display("delete_pages")]
    DeletePages,
    #[serde(rename = "edit_pdf")]
    #[display("edit_pdf")]
    EditPdf,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct MigrationStatus {
    pub latest_migration: String,
    pub unapplied_migrations: Vec<String>,
}

impl std::fmt::Display for MigrationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for MigrationStatus {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.latest_migration.clone().into(),
            format!("{:?}", self.unapplied_migrations).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["latest_migration".into(), "unapplied_migrations".into()]
    }
}

#[doc = "* `skip` - skip\n* `redo` - redo\n* `force` - force\n* `skip_noarchive` - skip_noarchive"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum ModeEnum {
    #[serde(rename = "skip")]
    #[display("skip")]
    Skip,
    #[serde(rename = "redo")]
    #[display("redo")]
    Redo,
    #[serde(rename = "force")]
    #[display("force")]
    Force,
    #[serde(rename = "skip_noarchive")]
    #[display("skip_noarchive")]
    SkipNoarchive,
    #[serde(rename = "")]
    #[display("")]
    Empty,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct NoteCreateRequestRequest {
    pub note: String,
}

impl std::fmt::Display for NoteCreateRequestRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for NoteCreateRequestRequest {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.note.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["note".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct Notes {
    pub id: i64,
    #[doc = "Note for the document"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    pub user: BasicUser,
}

impl std::fmt::Display for Notes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Notes {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            if let Some(note) = &self.note {
                format!("{note:?}").into()
            } else {
                String::new().into()
            },
            if let Some(created) = &self.created {
                format!("{created:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.user).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "note".into(), "created".into(), "user".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct NotesRequest {
    #[doc = "Note for the document"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
}

impl std::fmt::Display for NotesRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for NotesRequest {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(note) = &self.note {
                format!("{note:?}").into()
            } else {
                String::new().into()
            },
            if let Some(created) = &self.created {
                format!("{created:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["note".into(), "created".into()]
    }
}

#[doc = "* `tags` - tags\n* `correspondents` - correspondents\n* `document_types` - document_types\n* `storage_paths` - storage_paths"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum ObjectTypeEnum {
    #[serde(rename = "tags")]
    #[display("tags")]
    Tags,
    #[serde(rename = "correspondents")]
    #[display("correspondents")]
    Correspondents,
    #[serde(rename = "document_types")]
    #[display("document_types")]
    DocumentTypes,
    #[serde(rename = "storage_paths")]
    #[display("storage_paths")]
    StoragePaths,
}

#[doc = "* `set_permissions` - set_permissions\n* `delete` - delete"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum OperationEnum {
    #[serde(rename = "set_permissions")]
    #[display("set_permissions")]
    SetPermissions,
    #[serde(rename = "delete")]
    #[display("delete")]
    Delete,
}

#[doc = "* `pdf` - pdf\n* `pdfa` - pdfa\n* `pdfa-1` - pdfa-1\n* `pdfa-2` - pdfa-2\n* `pdfa-3` - pdfa-3"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum OutputTypeEnum {
    #[serde(rename = "pdf")]
    #[display("pdf")]
    Pdf,
    #[serde(rename = "pdfa")]
    #[display("pdfa")]
    Pdfa,
    #[serde(rename = "pdfa-1")]
    #[display("pdfa-1")]
    Pdfa1,
    #[serde(rename = "pdfa-2")]
    #[display("pdfa-2")]
    Pdfa2,
    #[serde(rename = "pdfa-3")]
    #[display("pdfa-3")]
    Pdfa3,
    #[serde(rename = "")]
    #[display("")]
    Empty,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PaginatedCorrespondentList {
    pub count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    pub results: Vec<Correspondent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all: Option<Vec<i64>>,
}

impl std::fmt::Display for PaginatedCorrespondentList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for PaginatedCorrespondentList {
    type Item = Correspondent;
    fn has_more_pages(&self) -> bool {
        self.next.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {req:?}"
            ))
        })?;
        *req.url_mut() = url::Url::parse(self.next.as_deref().unwrap_or("")).map_err(|_| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to parse url: {:?}",
                self.next
            ))
        })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedCorrespondentList {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.count).into(),
            if let Some(next) = &self.next {
                format!("{next:?}").into()
            } else {
                String::new().into()
            },
            if let Some(previous) = &self.previous {
                format!("{previous:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(all) = &self.all {
                format!("{all:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "count".into(),
            "next".into(),
            "previous".into(),
            "results".into(),
            "all".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PaginatedCustomFieldList {
    pub count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    pub results: Vec<CustomField>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all: Option<Vec<i64>>,
}

impl std::fmt::Display for PaginatedCustomFieldList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for PaginatedCustomFieldList {
    type Item = CustomField;
    fn has_more_pages(&self) -> bool {
        self.next.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {req:?}"
            ))
        })?;
        *req.url_mut() = url::Url::parse(self.next.as_deref().unwrap_or("")).map_err(|_| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to parse url: {:?}",
                self.next
            ))
        })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedCustomFieldList {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.count).into(),
            if let Some(next) = &self.next {
                format!("{next:?}").into()
            } else {
                String::new().into()
            },
            if let Some(previous) = &self.previous {
                format!("{previous:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(all) = &self.all {
                format!("{all:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "count".into(),
            "next".into(),
            "previous".into(),
            "results".into(),
            "all".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PaginatedDocumentList {
    pub count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    pub results: Vec<Document>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all: Option<Vec<i64>>,
}

impl std::fmt::Display for PaginatedDocumentList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for PaginatedDocumentList {
    type Item = Document;
    fn has_more_pages(&self) -> bool {
        self.next.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {req:?}"
            ))
        })?;
        *req.url_mut() = url::Url::parse(self.next.as_deref().unwrap_or("")).map_err(|_| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to parse url: {:?}",
                self.next
            ))
        })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedDocumentList {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.count).into(),
            if let Some(next) = &self.next {
                format!("{next:?}").into()
            } else {
                String::new().into()
            },
            if let Some(previous) = &self.previous {
                format!("{previous:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(all) = &self.all {
                format!("{all:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "count".into(),
            "next".into(),
            "previous".into(),
            "results".into(),
            "all".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PaginatedDocumentTypeList {
    pub count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    pub results: Vec<DocumentType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all: Option<Vec<i64>>,
}

impl std::fmt::Display for PaginatedDocumentTypeList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for PaginatedDocumentTypeList {
    type Item = DocumentType;
    fn has_more_pages(&self) -> bool {
        self.next.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {req:?}"
            ))
        })?;
        *req.url_mut() = url::Url::parse(self.next.as_deref().unwrap_or("")).map_err(|_| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to parse url: {:?}",
                self.next
            ))
        })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedDocumentTypeList {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.count).into(),
            if let Some(next) = &self.next {
                format!("{next:?}").into()
            } else {
                String::new().into()
            },
            if let Some(previous) = &self.previous {
                format!("{previous:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(all) = &self.all {
                format!("{all:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "count".into(),
            "next".into(),
            "previous".into(),
            "results".into(),
            "all".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PaginatedGroupList {
    pub count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    pub results: Vec<Group>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all: Option<Vec<i64>>,
}

impl std::fmt::Display for PaginatedGroupList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for PaginatedGroupList {
    type Item = Group;
    fn has_more_pages(&self) -> bool {
        self.next.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {req:?}"
            ))
        })?;
        *req.url_mut() = url::Url::parse(self.next.as_deref().unwrap_or("")).map_err(|_| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to parse url: {:?}",
                self.next
            ))
        })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedGroupList {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.count).into(),
            if let Some(next) = &self.next {
                format!("{next:?}").into()
            } else {
                String::new().into()
            },
            if let Some(previous) = &self.previous {
                format!("{previous:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(all) = &self.all {
                format!("{all:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "count".into(),
            "next".into(),
            "previous".into(),
            "results".into(),
            "all".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PaginatedLogEntryList {
    pub count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    pub results: Vec<LogEntry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all: Option<Vec<i64>>,
}

impl std::fmt::Display for PaginatedLogEntryList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for PaginatedLogEntryList {
    type Item = LogEntry;
    fn has_more_pages(&self) -> bool {
        self.next.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {req:?}"
            ))
        })?;
        *req.url_mut() = url::Url::parse(self.next.as_deref().unwrap_or("")).map_err(|_| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to parse url: {:?}",
                self.next
            ))
        })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedLogEntryList {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.count).into(),
            if let Some(next) = &self.next {
                format!("{next:?}").into()
            } else {
                String::new().into()
            },
            if let Some(previous) = &self.previous {
                format!("{previous:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(all) = &self.all {
                format!("{all:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "count".into(),
            "next".into(),
            "previous".into(),
            "results".into(),
            "all".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PaginatedMailAccountList {
    pub count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    pub results: Vec<MailAccount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all: Option<Vec<i64>>,
}

impl std::fmt::Display for PaginatedMailAccountList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for PaginatedMailAccountList {
    type Item = MailAccount;
    fn has_more_pages(&self) -> bool {
        self.next.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {req:?}"
            ))
        })?;
        *req.url_mut() = url::Url::parse(self.next.as_deref().unwrap_or("")).map_err(|_| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to parse url: {:?}",
                self.next
            ))
        })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedMailAccountList {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.count).into(),
            if let Some(next) = &self.next {
                format!("{next:?}").into()
            } else {
                String::new().into()
            },
            if let Some(previous) = &self.previous {
                format!("{previous:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(all) = &self.all {
                format!("{all:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "count".into(),
            "next".into(),
            "previous".into(),
            "results".into(),
            "all".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PaginatedMailRuleList {
    pub count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    pub results: Vec<MailRule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all: Option<Vec<i64>>,
}

impl std::fmt::Display for PaginatedMailRuleList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for PaginatedMailRuleList {
    type Item = MailRule;
    fn has_more_pages(&self) -> bool {
        self.next.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {req:?}"
            ))
        })?;
        *req.url_mut() = url::Url::parse(self.next.as_deref().unwrap_or("")).map_err(|_| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to parse url: {:?}",
                self.next
            ))
        })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedMailRuleList {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.count).into(),
            if let Some(next) = &self.next {
                format!("{next:?}").into()
            } else {
                String::new().into()
            },
            if let Some(previous) = &self.previous {
                format!("{previous:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(all) = &self.all {
                format!("{all:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "count".into(),
            "next".into(),
            "previous".into(),
            "results".into(),
            "all".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PaginatedNotesList {
    pub count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    pub results: Vec<Notes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all: Option<Vec<i64>>,
}

impl std::fmt::Display for PaginatedNotesList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for PaginatedNotesList {
    type Item = Notes;
    fn has_more_pages(&self) -> bool {
        self.next.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {req:?}"
            ))
        })?;
        *req.url_mut() = url::Url::parse(self.next.as_deref().unwrap_or("")).map_err(|_| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to parse url: {:?}",
                self.next
            ))
        })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedNotesList {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.count).into(),
            if let Some(next) = &self.next {
                format!("{next:?}").into()
            } else {
                String::new().into()
            },
            if let Some(previous) = &self.previous {
                format!("{previous:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(all) = &self.all {
                format!("{all:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "count".into(),
            "next".into(),
            "previous".into(),
            "results".into(),
            "all".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PaginatedSavedViewList {
    pub count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    pub results: Vec<SavedView>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all: Option<Vec<i64>>,
}

impl std::fmt::Display for PaginatedSavedViewList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for PaginatedSavedViewList {
    type Item = SavedView;
    fn has_more_pages(&self) -> bool {
        self.next.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {req:?}"
            ))
        })?;
        *req.url_mut() = url::Url::parse(self.next.as_deref().unwrap_or("")).map_err(|_| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to parse url: {:?}",
                self.next
            ))
        })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedSavedViewList {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.count).into(),
            if let Some(next) = &self.next {
                format!("{next:?}").into()
            } else {
                String::new().into()
            },
            if let Some(previous) = &self.previous {
                format!("{previous:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(all) = &self.all {
                format!("{all:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "count".into(),
            "next".into(),
            "previous".into(),
            "results".into(),
            "all".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PaginatedShareLinkList {
    pub count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    pub results: Vec<ShareLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all: Option<Vec<i64>>,
}

impl std::fmt::Display for PaginatedShareLinkList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for PaginatedShareLinkList {
    type Item = ShareLink;
    fn has_more_pages(&self) -> bool {
        self.next.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {req:?}"
            ))
        })?;
        *req.url_mut() = url::Url::parse(self.next.as_deref().unwrap_or("")).map_err(|_| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to parse url: {:?}",
                self.next
            ))
        })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedShareLinkList {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.count).into(),
            if let Some(next) = &self.next {
                format!("{next:?}").into()
            } else {
                String::new().into()
            },
            if let Some(previous) = &self.previous {
                format!("{previous:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(all) = &self.all {
                format!("{all:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "count".into(),
            "next".into(),
            "previous".into(),
            "results".into(),
            "all".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PaginatedStoragePathList {
    pub count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    pub results: Vec<StoragePath>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all: Option<Vec<i64>>,
}

impl std::fmt::Display for PaginatedStoragePathList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for PaginatedStoragePathList {
    type Item = StoragePath;
    fn has_more_pages(&self) -> bool {
        self.next.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {req:?}"
            ))
        })?;
        *req.url_mut() = url::Url::parse(self.next.as_deref().unwrap_or("")).map_err(|_| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to parse url: {:?}",
                self.next
            ))
        })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedStoragePathList {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.count).into(),
            if let Some(next) = &self.next {
                format!("{next:?}").into()
            } else {
                String::new().into()
            },
            if let Some(previous) = &self.previous {
                format!("{previous:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(all) = &self.all {
                format!("{all:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "count".into(),
            "next".into(),
            "previous".into(),
            "results".into(),
            "all".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PaginatedTagList {
    pub count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    pub results: Vec<Tag>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all: Option<Vec<i64>>,
}

impl std::fmt::Display for PaginatedTagList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for PaginatedTagList {
    type Item = Tag;
    fn has_more_pages(&self) -> bool {
        self.next.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {req:?}"
            ))
        })?;
        *req.url_mut() = url::Url::parse(self.next.as_deref().unwrap_or("")).map_err(|_| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to parse url: {:?}",
                self.next
            ))
        })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedTagList {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.count).into(),
            if let Some(next) = &self.next {
                format!("{next:?}").into()
            } else {
                String::new().into()
            },
            if let Some(previous) = &self.previous {
                format!("{previous:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(all) = &self.all {
                format!("{all:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "count".into(),
            "next".into(),
            "previous".into(),
            "results".into(),
            "all".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PaginatedUserList {
    pub count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    pub results: Vec<User>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all: Option<Vec<i64>>,
}

impl std::fmt::Display for PaginatedUserList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for PaginatedUserList {
    type Item = User;
    fn has_more_pages(&self) -> bool {
        self.next.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {req:?}"
            ))
        })?;
        *req.url_mut() = url::Url::parse(self.next.as_deref().unwrap_or("")).map_err(|_| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to parse url: {:?}",
                self.next
            ))
        })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedUserList {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.count).into(),
            if let Some(next) = &self.next {
                format!("{next:?}").into()
            } else {
                String::new().into()
            },
            if let Some(previous) = &self.previous {
                format!("{previous:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(all) = &self.all {
                format!("{all:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "count".into(),
            "next".into(),
            "previous".into(),
            "results".into(),
            "all".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PaginatedWorkflowActionList {
    pub count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    pub results: Vec<WorkflowAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all: Option<Vec<i64>>,
}

impl std::fmt::Display for PaginatedWorkflowActionList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for PaginatedWorkflowActionList {
    type Item = WorkflowAction;
    fn has_more_pages(&self) -> bool {
        self.next.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {req:?}"
            ))
        })?;
        *req.url_mut() = url::Url::parse(self.next.as_deref().unwrap_or("")).map_err(|_| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to parse url: {:?}",
                self.next
            ))
        })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedWorkflowActionList {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.count).into(),
            if let Some(next) = &self.next {
                format!("{next:?}").into()
            } else {
                String::new().into()
            },
            if let Some(previous) = &self.previous {
                format!("{previous:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(all) = &self.all {
                format!("{all:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "count".into(),
            "next".into(),
            "previous".into(),
            "results".into(),
            "all".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PaginatedWorkflowList {
    pub count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    pub results: Vec<Workflow>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all: Option<Vec<i64>>,
}

impl std::fmt::Display for PaginatedWorkflowList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for PaginatedWorkflowList {
    type Item = Workflow;
    fn has_more_pages(&self) -> bool {
        self.next.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {req:?}"
            ))
        })?;
        *req.url_mut() = url::Url::parse(self.next.as_deref().unwrap_or("")).map_err(|_| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to parse url: {:?}",
                self.next
            ))
        })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedWorkflowList {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.count).into(),
            if let Some(next) = &self.next {
                format!("{next:?}").into()
            } else {
                String::new().into()
            },
            if let Some(previous) = &self.previous {
                format!("{previous:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(all) = &self.all {
                format!("{all:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "count".into(),
            "next".into(),
            "previous".into(),
            "results".into(),
            "all".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PaginatedWorkflowTriggerList {
    pub count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    pub results: Vec<WorkflowTrigger>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all: Option<Vec<i64>>,
}

impl std::fmt::Display for PaginatedWorkflowTriggerList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for PaginatedWorkflowTriggerList {
    type Item = WorkflowTrigger;
    fn has_more_pages(&self) -> bool {
        self.next.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {req:?}"
            ))
        })?;
        *req.url_mut() = url::Url::parse(self.next.as_deref().unwrap_or("")).map_err(|_| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to parse url: {:?}",
                self.next
            ))
        })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedWorkflowTriggerList {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.count).into(),
            if let Some(next) = &self.next {
                format!("{next:?}").into()
            } else {
                String::new().into()
            },
            if let Some(previous) = &self.previous {
                format!("{previous:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(all) = &self.all {
                format!("{all:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "count".into(),
            "next".into(),
            "previous".into(),
            "results".into(),
            "all".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PaperlessAuthToken {
    pub token: String,
}

impl std::fmt::Display for PaperlessAuthToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaperlessAuthToken {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.token.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["token".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PaperlessAuthTokenRequest {
    pub username: String,
    pub password: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl std::fmt::Display for PaperlessAuthTokenRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaperlessAuthTokenRequest {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.username.clone().into(),
            self.password.clone().into(),
            if let Some(code) = &self.code {
                format!("{code:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["username".into(), "password".into(), "code".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PatchedApplicationConfigurationRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_args: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barcode_tag_mapping: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_type: Option<OutputType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pages: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skip_archive_file: Option<SkipArchiveFile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_dpi: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unpaper_clean: Option<UnpaperClean>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deskew: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotate_pages: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotate_pages_threshold: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_image_pixels: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color_conversion_strategy: Option<ColorConversionStrategy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_logo: Option<bytes::Bytes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barcodes_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barcode_enable_tiff_support: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barcode_string: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barcode_retain_split_pages: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barcode_enable_asn: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barcode_asn_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barcode_upscale: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barcode_dpi: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barcode_max_pages: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barcode_enable_tag: Option<bool>,
}

impl std::fmt::Display for PatchedApplicationConfigurationRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PatchedApplicationConfigurationRequest {
    const LENGTH: usize = 26;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(user_args) = &self.user_args {
                format!("{user_args:?}").into()
            } else {
                String::new().into()
            },
            if let Some(barcode_tag_mapping) = &self.barcode_tag_mapping {
                format!("{barcode_tag_mapping:?}").into()
            } else {
                String::new().into()
            },
            if let Some(output_type) = &self.output_type {
                format!("{output_type:?}").into()
            } else {
                String::new().into()
            },
            if let Some(pages) = &self.pages {
                format!("{pages:?}").into()
            } else {
                String::new().into()
            },
            if let Some(language) = &self.language {
                format!("{language:?}").into()
            } else {
                String::new().into()
            },
            if let Some(mode) = &self.mode {
                format!("{mode:?}").into()
            } else {
                String::new().into()
            },
            if let Some(skip_archive_file) = &self.skip_archive_file {
                format!("{skip_archive_file:?}").into()
            } else {
                String::new().into()
            },
            if let Some(image_dpi) = &self.image_dpi {
                format!("{image_dpi:?}").into()
            } else {
                String::new().into()
            },
            if let Some(unpaper_clean) = &self.unpaper_clean {
                format!("{unpaper_clean:?}").into()
            } else {
                String::new().into()
            },
            if let Some(deskew) = &self.deskew {
                format!("{deskew:?}").into()
            } else {
                String::new().into()
            },
            if let Some(rotate_pages) = &self.rotate_pages {
                format!("{rotate_pages:?}").into()
            } else {
                String::new().into()
            },
            if let Some(rotate_pages_threshold) = &self.rotate_pages_threshold {
                format!("{rotate_pages_threshold:?}").into()
            } else {
                String::new().into()
            },
            if let Some(max_image_pixels) = &self.max_image_pixels {
                format!("{max_image_pixels:?}").into()
            } else {
                String::new().into()
            },
            if let Some(color_conversion_strategy) = &self.color_conversion_strategy {
                format!("{color_conversion_strategy:?}").into()
            } else {
                String::new().into()
            },
            if let Some(app_title) = &self.app_title {
                format!("{app_title:?}").into()
            } else {
                String::new().into()
            },
            if let Some(app_logo) = &self.app_logo {
                format!("{app_logo:?}").into()
            } else {
                String::new().into()
            },
            if let Some(barcodes_enabled) = &self.barcodes_enabled {
                format!("{barcodes_enabled:?}").into()
            } else {
                String::new().into()
            },
            if let Some(barcode_enable_tiff_support) = &self.barcode_enable_tiff_support {
                format!("{barcode_enable_tiff_support:?}").into()
            } else {
                String::new().into()
            },
            if let Some(barcode_string) = &self.barcode_string {
                format!("{barcode_string:?}").into()
            } else {
                String::new().into()
            },
            if let Some(barcode_retain_split_pages) = &self.barcode_retain_split_pages {
                format!("{barcode_retain_split_pages:?}").into()
            } else {
                String::new().into()
            },
            if let Some(barcode_enable_asn) = &self.barcode_enable_asn {
                format!("{barcode_enable_asn:?}").into()
            } else {
                String::new().into()
            },
            if let Some(barcode_asn_prefix) = &self.barcode_asn_prefix {
                format!("{barcode_asn_prefix:?}").into()
            } else {
                String::new().into()
            },
            if let Some(barcode_upscale) = &self.barcode_upscale {
                format!("{barcode_upscale:?}").into()
            } else {
                String::new().into()
            },
            if let Some(barcode_dpi) = &self.barcode_dpi {
                format!("{barcode_dpi:?}").into()
            } else {
                String::new().into()
            },
            if let Some(barcode_max_pages) = &self.barcode_max_pages {
                format!("{barcode_max_pages:?}").into()
            } else {
                String::new().into()
            },
            if let Some(barcode_enable_tag) = &self.barcode_enable_tag {
                format!("{barcode_enable_tag:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "user_args".into(),
            "barcode_tag_mapping".into(),
            "output_type".into(),
            "pages".into(),
            "language".into(),
            "mode".into(),
            "skip_archive_file".into(),
            "image_dpi".into(),
            "unpaper_clean".into(),
            "deskew".into(),
            "rotate_pages".into(),
            "rotate_pages_threshold".into(),
            "max_image_pixels".into(),
            "color_conversion_strategy".into(),
            "app_title".into(),
            "app_logo".into(),
            "barcodes_enabled".into(),
            "barcode_enable_tiff_support".into(),
            "barcode_string".into(),
            "barcode_retain_split_pages".into(),
            "barcode_enable_asn".into(),
            "barcode_asn_prefix".into(),
            "barcode_upscale".into(),
            "barcode_dpi".into(),
            "barcode_max_pages".into(),
            "barcode_enable_tag".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PatchedCorrespondentRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "match", default, skip_serializing_if = "Option::is_none")]
    pub match_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matching_algorithm: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_insensitive: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_permissions: Option<SetPermissions>,
}

impl std::fmt::Display for PatchedCorrespondentRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PatchedCorrespondentRequest {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{name:?}").into()
            } else {
                String::new().into()
            },
            if let Some(match_) = &self.match_ {
                format!("{match_:?}").into()
            } else {
                String::new().into()
            },
            if let Some(matching_algorithm) = &self.matching_algorithm {
                format!("{matching_algorithm:?}").into()
            } else {
                String::new().into()
            },
            if let Some(is_insensitive) = &self.is_insensitive {
                format!("{is_insensitive:?}").into()
            } else {
                String::new().into()
            },
            if let Some(owner) = &self.owner {
                format!("{owner:?}").into()
            } else {
                String::new().into()
            },
            if let Some(set_permissions) = &self.set_permissions {
                format!("{set_permissions:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "match_".into(),
            "matching_algorithm".into(),
            "is_insensitive".into(),
            "owner".into(),
            "set_permissions".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PatchedCustomFieldRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "* `string` - string\n* `url` - url\n* `date` - date\n* `boolean` - boolean\n* `integer` - integer\n* `float` - float\n* `monetary` - monetary\n* `documentlink` - documentlink\n* `select` - select"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<DataTypeEnum>,
    #[doc = "Extra data for the custom field, such as select options"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_data: Option<serde_json::Value>,
}

impl std::fmt::Display for PatchedCustomFieldRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PatchedCustomFieldRequest {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{name:?}").into()
            } else {
                String::new().into()
            },
            if let Some(data_type) = &self.data_type {
                format!("{data_type:?}").into()
            } else {
                String::new().into()
            },
            if let Some(extra_data) = &self.extra_data {
                format!("{extra_data:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "data_type".into(), "extra_data".into()]
    }
}

#[doc = "Adds update nested feature"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PatchedDocumentRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correspondent: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage_path: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The raw, text-only data of the document. This field is primarily used for searching."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<chrono::NaiveDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[deprecated]
    pub created_date: Option<chrono::NaiveDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The position of this document in your physical document archive."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive_serial_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_permissions: Option<SetPermissions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldInstanceRequest>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_inbox_tags: Option<bool>,
}

impl std::fmt::Display for PatchedDocumentRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PatchedDocumentRequest {
    const LENGTH: usize = 14;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(correspondent) = &self.correspondent {
                format!("{correspondent:?}").into()
            } else {
                String::new().into()
            },
            if let Some(document_type) = &self.document_type {
                format!("{document_type:?}").into()
            } else {
                String::new().into()
            },
            if let Some(storage_path) = &self.storage_path {
                format!("{storage_path:?}").into()
            } else {
                String::new().into()
            },
            if let Some(title) = &self.title {
                format!("{title:?}").into()
            } else {
                String::new().into()
            },
            if let Some(content) = &self.content {
                format!("{content:?}").into()
            } else {
                String::new().into()
            },
            if let Some(tags) = &self.tags {
                format!("{tags:?}").into()
            } else {
                String::new().into()
            },
            if let Some(created) = &self.created {
                format!("{created:?}").into()
            } else {
                String::new().into()
            },
            if let Some(created_date) = &self.created_date {
                format!("{created_date:?}").into()
            } else {
                String::new().into()
            },
            if let Some(deleted_at) = &self.deleted_at {
                format!("{deleted_at:?}").into()
            } else {
                String::new().into()
            },
            if let Some(archive_serial_number) = &self.archive_serial_number {
                format!("{archive_serial_number:?}").into()
            } else {
                String::new().into()
            },
            if let Some(owner) = &self.owner {
                format!("{owner:?}").into()
            } else {
                String::new().into()
            },
            if let Some(set_permissions) = &self.set_permissions {
                format!("{set_permissions:?}").into()
            } else {
                String::new().into()
            },
            if let Some(custom_fields) = &self.custom_fields {
                format!("{custom_fields:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_inbox_tags) = &self.remove_inbox_tags {
                format!("{remove_inbox_tags:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "correspondent".into(),
            "document_type".into(),
            "storage_path".into(),
            "title".into(),
            "content".into(),
            "tags".into(),
            "created".into(),
            "created_date".into(),
            "deleted_at".into(),
            "archive_serial_number".into(),
            "owner".into(),
            "set_permissions".into(),
            "custom_fields".into(),
            "remove_inbox_tags".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PatchedDocumentTypeRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "match", default, skip_serializing_if = "Option::is_none")]
    pub match_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matching_algorithm: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_insensitive: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_permissions: Option<SetPermissions>,
}

impl std::fmt::Display for PatchedDocumentTypeRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PatchedDocumentTypeRequest {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{name:?}").into()
            } else {
                String::new().into()
            },
            if let Some(match_) = &self.match_ {
                format!("{match_:?}").into()
            } else {
                String::new().into()
            },
            if let Some(matching_algorithm) = &self.matching_algorithm {
                format!("{matching_algorithm:?}").into()
            } else {
                String::new().into()
            },
            if let Some(is_insensitive) = &self.is_insensitive {
                format!("{is_insensitive:?}").into()
            } else {
                String::new().into()
            },
            if let Some(owner) = &self.owner {
                format!("{owner:?}").into()
            } else {
                String::new().into()
            },
            if let Some(set_permissions) = &self.set_permissions {
                format!("{set_permissions:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "match_".into(),
            "matching_algorithm".into(),
            "is_insensitive".into(),
            "owner".into(),
            "set_permissions".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PatchedGroupRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

impl std::fmt::Display for PatchedGroupRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PatchedGroupRequest {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{name:?}").into()
            } else {
                String::new().into()
            },
            if let Some(permissions) = &self.permissions {
                format!("{permissions:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "permissions".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PatchedMailAccountRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imap_server: Option<String>,
    #[doc = "This is usually 143 for unencrypted and STARTTLS connections, and 993 for SSL connections."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imap_port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imap_security: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "The character set to use when communicating with the mail server, such as 'UTF-8' or 'US-ASCII'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub character_set: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_token: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_permissions: Option<SetPermissions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_type: Option<i64>,
    #[doc = "The expiration date of the refresh token. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration: Option<chrono::DateTime<chrono::Utc>>,
}

impl std::fmt::Display for PatchedMailAccountRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PatchedMailAccountRequest {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{name:?}").into()
            } else {
                String::new().into()
            },
            if let Some(imap_server) = &self.imap_server {
                format!("{imap_server:?}").into()
            } else {
                String::new().into()
            },
            if let Some(imap_port) = &self.imap_port {
                format!("{imap_port:?}").into()
            } else {
                String::new().into()
            },
            if let Some(imap_security) = &self.imap_security {
                format!("{imap_security:?}").into()
            } else {
                String::new().into()
            },
            if let Some(username) = &self.username {
                format!("{username:?}").into()
            } else {
                String::new().into()
            },
            if let Some(password) = &self.password {
                format!("{password:?}").into()
            } else {
                String::new().into()
            },
            if let Some(character_set) = &self.character_set {
                format!("{character_set:?}").into()
            } else {
                String::new().into()
            },
            if let Some(is_token) = &self.is_token {
                format!("{is_token:?}").into()
            } else {
                String::new().into()
            },
            if let Some(owner) = &self.owner {
                format!("{owner:?}").into()
            } else {
                String::new().into()
            },
            if let Some(set_permissions) = &self.set_permissions {
                format!("{set_permissions:?}").into()
            } else {
                String::new().into()
            },
            if let Some(account_type) = &self.account_type {
                format!("{account_type:?}").into()
            } else {
                String::new().into()
            },
            if let Some(expiration) = &self.expiration {
                format!("{expiration:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "imap_server".into(),
            "imap_port".into(),
            "imap_security".into(),
            "username".into(),
            "password".into(),
            "character_set".into(),
            "is_token".into(),
            "owner".into(),
            "set_permissions".into(),
            "account_type".into(),
            "expiration".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PatchedMailRuleRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Subfolders must be separated by a delimiter, often a dot ('.') or slash ('/'), but it varies by mail server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_from: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_body: Option<String>,
    #[doc = "Only consume documents which entirely match this filename if specified. Wildcards such as *.pdf or *invoice* are allowed. Case insensitive."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_attachment_filename_include: Option<String>,
    #[doc = "Do not consume documents which entirely match this filename if specified. Wildcards such as *.pdf or *invoice* are allowed. Case insensitive."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_attachment_filename_exclude: Option<String>,
    #[doc = "Specified in days."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_age: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_parameter: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_title_from: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_tags: Option<Vec<Option<i64>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_correspondent_from: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_correspondent: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_document_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_owner_from_rule: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
    #[doc = "Inline attachments include embedded images, so it's best to combine this option with a filename filter.\n\n* `1` - Only process attachments.\n* `2` - Process all files, including 'inline' attachments."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consumption_scope: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pdf_layout: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_permissions: Option<SetPermissions>,
}

impl std::fmt::Display for PatchedMailRuleRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PatchedMailRuleRequest {
    const LENGTH: usize = 25;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{name:?}").into()
            } else {
                String::new().into()
            },
            if let Some(account) = &self.account {
                format!("{account:?}").into()
            } else {
                String::new().into()
            },
            if let Some(enabled) = &self.enabled {
                format!("{enabled:?}").into()
            } else {
                String::new().into()
            },
            if let Some(folder) = &self.folder {
                format!("{folder:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_from) = &self.filter_from {
                format!("{filter_from:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_to) = &self.filter_to {
                format!("{filter_to:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_subject) = &self.filter_subject {
                format!("{filter_subject:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_body) = &self.filter_body {
                format!("{filter_body:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_attachment_filename_include) =
                &self.filter_attachment_filename_include
            {
                format!("{filter_attachment_filename_include:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_attachment_filename_exclude) =
                &self.filter_attachment_filename_exclude
            {
                format!("{filter_attachment_filename_exclude:?}").into()
            } else {
                String::new().into()
            },
            if let Some(maximum_age) = &self.maximum_age {
                format!("{maximum_age:?}").into()
            } else {
                String::new().into()
            },
            if let Some(action) = &self.action {
                format!("{action:?}").into()
            } else {
                String::new().into()
            },
            if let Some(action_parameter) = &self.action_parameter {
                format!("{action_parameter:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_title_from) = &self.assign_title_from {
                format!("{assign_title_from:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_tags) = &self.assign_tags {
                format!("{assign_tags:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_correspondent_from) = &self.assign_correspondent_from {
                format!("{assign_correspondent_from:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_correspondent) = &self.assign_correspondent {
                format!("{assign_correspondent:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_document_type) = &self.assign_document_type {
                format!("{assign_document_type:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_owner_from_rule) = &self.assign_owner_from_rule {
                format!("{assign_owner_from_rule:?}").into()
            } else {
                String::new().into()
            },
            if let Some(order) = &self.order {
                format!("{order:?}").into()
            } else {
                String::new().into()
            },
            if let Some(attachment_type) = &self.attachment_type {
                format!("{attachment_type:?}").into()
            } else {
                String::new().into()
            },
            if let Some(consumption_scope) = &self.consumption_scope {
                format!("{consumption_scope:?}").into()
            } else {
                String::new().into()
            },
            if let Some(pdf_layout) = &self.pdf_layout {
                format!("{pdf_layout:?}").into()
            } else {
                String::new().into()
            },
            if let Some(owner) = &self.owner {
                format!("{owner:?}").into()
            } else {
                String::new().into()
            },
            if let Some(set_permissions) = &self.set_permissions {
                format!("{set_permissions:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "account".into(),
            "enabled".into(),
            "folder".into(),
            "filter_from".into(),
            "filter_to".into(),
            "filter_subject".into(),
            "filter_body".into(),
            "filter_attachment_filename_include".into(),
            "filter_attachment_filename_exclude".into(),
            "maximum_age".into(),
            "action".into(),
            "action_parameter".into(),
            "assign_title_from".into(),
            "assign_tags".into(),
            "assign_correspondent_from".into(),
            "assign_correspondent".into(),
            "assign_document_type".into(),
            "assign_owner_from_rule".into(),
            "order".into(),
            "attachment_type".into(),
            "consumption_scope".into(),
            "pdf_layout".into(),
            "owner".into(),
            "set_permissions".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PatchedProfileRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

impl std::fmt::Display for PatchedProfileRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PatchedProfileRequest {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(email) = &self.email {
                format!("{email:?}").into()
            } else {
                String::new().into()
            },
            if let Some(password) = &self.password {
                format!("{password:?}").into()
            } else {
                String::new().into()
            },
            if let Some(first_name) = &self.first_name {
                format!("{first_name:?}").into()
            } else {
                String::new().into()
            },
            if let Some(last_name) = &self.last_name {
                format!("{last_name:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "email".into(),
            "password".into(),
            "first_name".into(),
            "last_name".into(),
        ]
    }
}

#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum DisplayMode {
    #[serde(rename = "table")]
    #[display("table")]
    Table,
    #[serde(rename = "smallCards")]
    #[display("smallCards")]
    SmallCards,
    #[serde(rename = "largeCards")]
    #[display("largeCards")]
    LargeCards,
    #[serde(rename = "")]
    #[display("")]
    Empty,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PatchedSavedViewRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_on_dashboard: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_in_sidebar: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_reverse: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_rules: Option<Vec<SavedViewFilterRuleRequest>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_mode: Option<DisplayMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_fields: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<i64>,
}

impl std::fmt::Display for PatchedSavedViewRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PatchedSavedViewRequest {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{name:?}").into()
            } else {
                String::new().into()
            },
            if let Some(show_on_dashboard) = &self.show_on_dashboard {
                format!("{show_on_dashboard:?}").into()
            } else {
                String::new().into()
            },
            if let Some(show_in_sidebar) = &self.show_in_sidebar {
                format!("{show_in_sidebar:?}").into()
            } else {
                String::new().into()
            },
            if let Some(sort_field) = &self.sort_field {
                format!("{sort_field:?}").into()
            } else {
                String::new().into()
            },
            if let Some(sort_reverse) = &self.sort_reverse {
                format!("{sort_reverse:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_rules) = &self.filter_rules {
                format!("{filter_rules:?}").into()
            } else {
                String::new().into()
            },
            if let Some(page_size) = &self.page_size {
                format!("{page_size:?}").into()
            } else {
                String::new().into()
            },
            if let Some(display_mode) = &self.display_mode {
                format!("{display_mode:?}").into()
            } else {
                String::new().into()
            },
            if let Some(display_fields) = &self.display_fields {
                format!("{display_fields:?}").into()
            } else {
                String::new().into()
            },
            if let Some(owner) = &self.owner {
                format!("{owner:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "show_on_dashboard".into(),
            "show_in_sidebar".into(),
            "sort_field".into(),
            "sort_reverse".into(),
            "filter_rules".into(),
            "page_size".into(),
            "display_mode".into(),
            "display_fields".into(),
            "owner".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PatchedShareLinkRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<i64>,
    #[doc = "* `archive` - Archive\n* `original` - Original"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_version: Option<FileVersionEnum>,
}

impl std::fmt::Display for PatchedShareLinkRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PatchedShareLinkRequest {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(expiration) = &self.expiration {
                format!("{expiration:?}").into()
            } else {
                String::new().into()
            },
            if let Some(document) = &self.document {
                format!("{document:?}").into()
            } else {
                String::new().into()
            },
            if let Some(file_version) = &self.file_version {
                format!("{file_version:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "expiration".into(),
            "document".into(),
            "file_version".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PatchedStoragePathRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "match", default, skip_serializing_if = "Option::is_none")]
    pub match_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matching_algorithm: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_insensitive: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_permissions: Option<SetPermissions>,
}

impl std::fmt::Display for PatchedStoragePathRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PatchedStoragePathRequest {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{name:?}").into()
            } else {
                String::new().into()
            },
            if let Some(path) = &self.path {
                format!("{path:?}").into()
            } else {
                String::new().into()
            },
            if let Some(match_) = &self.match_ {
                format!("{match_:?}").into()
            } else {
                String::new().into()
            },
            if let Some(matching_algorithm) = &self.matching_algorithm {
                format!("{matching_algorithm:?}").into()
            } else {
                String::new().into()
            },
            if let Some(is_insensitive) = &self.is_insensitive {
                format!("{is_insensitive:?}").into()
            } else {
                String::new().into()
            },
            if let Some(owner) = &self.owner {
                format!("{owner:?}").into()
            } else {
                String::new().into()
            },
            if let Some(set_permissions) = &self.set_permissions {
                format!("{set_permissions:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "path".into(),
            "match_".into(),
            "matching_algorithm".into(),
            "is_insensitive".into(),
            "owner".into(),
            "set_permissions".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PatchedTagRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "match", default, skip_serializing_if = "Option::is_none")]
    pub match_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matching_algorithm: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_insensitive: Option<bool>,
    #[doc = "Marks this tag as an inbox tag: All newly consumed documents will be tagged with inbox tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_inbox_tag: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_permissions: Option<SetPermissions>,
}

impl std::fmt::Display for PatchedTagRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PatchedTagRequest {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{name:?}").into()
            } else {
                String::new().into()
            },
            if let Some(color) = &self.color {
                format!("{color:?}").into()
            } else {
                String::new().into()
            },
            if let Some(match_) = &self.match_ {
                format!("{match_:?}").into()
            } else {
                String::new().into()
            },
            if let Some(matching_algorithm) = &self.matching_algorithm {
                format!("{matching_algorithm:?}").into()
            } else {
                String::new().into()
            },
            if let Some(is_insensitive) = &self.is_insensitive {
                format!("{is_insensitive:?}").into()
            } else {
                String::new().into()
            },
            if let Some(is_inbox_tag) = &self.is_inbox_tag {
                format!("{is_inbox_tag:?}").into()
            } else {
                String::new().into()
            },
            if let Some(owner) = &self.owner {
                format!("{owner:?}").into()
            } else {
                String::new().into()
            },
            if let Some(set_permissions) = &self.set_permissions {
                format!("{set_permissions:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "color".into(),
            "match_".into(),
            "matching_algorithm".into(),
            "is_insensitive".into(),
            "is_inbox_tag".into(),
            "owner".into(),
            "set_permissions".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PatchedUserRequest {
    #[doc = "Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_joined: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Designates whether the user can log into this admin site."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_staff: Option<bool>,
    #[doc = "Designates whether this user should be treated as active. Unselect this instead of deleting accounts."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[doc = "Designates that this user has all permissions without explicitly assigning them."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_superuser: Option<bool>,
    #[doc = "The groups this user belongs to. A user will get all permissions granted to each of their groups."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_permissions: Option<Vec<String>>,
}

impl std::fmt::Display for PatchedUserRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PatchedUserRequest {
    const LENGTH: usize = 11;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(username) = &self.username {
                format!("{username:?}").into()
            } else {
                String::new().into()
            },
            if let Some(email) = &self.email {
                format!("{email:?}").into()
            } else {
                String::new().into()
            },
            if let Some(password) = &self.password {
                format!("{password:?}").into()
            } else {
                String::new().into()
            },
            if let Some(first_name) = &self.first_name {
                format!("{first_name:?}").into()
            } else {
                String::new().into()
            },
            if let Some(last_name) = &self.last_name {
                format!("{last_name:?}").into()
            } else {
                String::new().into()
            },
            if let Some(date_joined) = &self.date_joined {
                format!("{date_joined:?}").into()
            } else {
                String::new().into()
            },
            if let Some(is_staff) = &self.is_staff {
                format!("{is_staff:?}").into()
            } else {
                String::new().into()
            },
            if let Some(is_active) = &self.is_active {
                format!("{is_active:?}").into()
            } else {
                String::new().into()
            },
            if let Some(is_superuser) = &self.is_superuser {
                format!("{is_superuser:?}").into()
            } else {
                String::new().into()
            },
            if let Some(groups) = &self.groups {
                format!("{groups:?}").into()
            } else {
                String::new().into()
            },
            if let Some(user_permissions) = &self.user_permissions {
                format!("{user_permissions:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "username".into(),
            "email".into(),
            "password".into(),
            "first_name".into(),
            "last_name".into(),
            "date_joined".into(),
            "is_staff".into(),
            "is_active".into(),
            "is_superuser".into(),
            "groups".into(),
            "user_permissions".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PatchedWorkflowActionRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<i64>,
    #[doc = "Assign a document title, can include some placeholders, see documentation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_tags: Option<Vec<Option<i64>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_correspondent: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_document_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_storage_path: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_owner: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_view_users: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_view_groups: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_change_users: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_change_groups: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_custom_fields: Option<Vec<i64>>,
    #[doc = "Optional values to assign to the custom fields."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_custom_fields_values: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_all_tags: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_tags: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_all_correspondents: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_correspondents: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_all_document_types: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_document_types: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_all_storage_paths: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_storage_paths: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_custom_fields: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_all_custom_fields: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_all_owners: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_owners: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_all_permissions: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_view_users: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_view_groups: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_change_users: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_change_groups: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<WorkflowActionEmailRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<WorkflowActionWebhookRequest>,
}

impl std::fmt::Display for PatchedWorkflowActionRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PatchedWorkflowActionRequest {
    const LENGTH: usize = 33;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{id:?}").into()
            } else {
                String::new().into()
            },
            if let Some(type_) = &self.type_ {
                format!("{type_:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_title) = &self.assign_title {
                format!("{assign_title:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_tags) = &self.assign_tags {
                format!("{assign_tags:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_correspondent) = &self.assign_correspondent {
                format!("{assign_correspondent:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_document_type) = &self.assign_document_type {
                format!("{assign_document_type:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_storage_path) = &self.assign_storage_path {
                format!("{assign_storage_path:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_owner) = &self.assign_owner {
                format!("{assign_owner:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_view_users) = &self.assign_view_users {
                format!("{assign_view_users:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_view_groups) = &self.assign_view_groups {
                format!("{assign_view_groups:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_change_users) = &self.assign_change_users {
                format!("{assign_change_users:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_change_groups) = &self.assign_change_groups {
                format!("{assign_change_groups:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_custom_fields) = &self.assign_custom_fields {
                format!("{assign_custom_fields:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_custom_fields_values) = &self.assign_custom_fields_values {
                format!("{assign_custom_fields_values:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_all_tags) = &self.remove_all_tags {
                format!("{remove_all_tags:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_tags) = &self.remove_tags {
                format!("{remove_tags:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_all_correspondents) = &self.remove_all_correspondents {
                format!("{remove_all_correspondents:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_correspondents) = &self.remove_correspondents {
                format!("{remove_correspondents:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_all_document_types) = &self.remove_all_document_types {
                format!("{remove_all_document_types:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_document_types) = &self.remove_document_types {
                format!("{remove_document_types:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_all_storage_paths) = &self.remove_all_storage_paths {
                format!("{remove_all_storage_paths:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_storage_paths) = &self.remove_storage_paths {
                format!("{remove_storage_paths:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_custom_fields) = &self.remove_custom_fields {
                format!("{remove_custom_fields:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_all_custom_fields) = &self.remove_all_custom_fields {
                format!("{remove_all_custom_fields:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_all_owners) = &self.remove_all_owners {
                format!("{remove_all_owners:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_owners) = &self.remove_owners {
                format!("{remove_owners:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_all_permissions) = &self.remove_all_permissions {
                format!("{remove_all_permissions:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_view_users) = &self.remove_view_users {
                format!("{remove_view_users:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_view_groups) = &self.remove_view_groups {
                format!("{remove_view_groups:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_change_users) = &self.remove_change_users {
                format!("{remove_change_users:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_change_groups) = &self.remove_change_groups {
                format!("{remove_change_groups:?}").into()
            } else {
                String::new().into()
            },
            if let Some(email) = &self.email {
                format!("{email:?}").into()
            } else {
                String::new().into()
            },
            if let Some(webhook) = &self.webhook {
                format!("{webhook:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "type_".into(),
            "assign_title".into(),
            "assign_tags".into(),
            "assign_correspondent".into(),
            "assign_document_type".into(),
            "assign_storage_path".into(),
            "assign_owner".into(),
            "assign_view_users".into(),
            "assign_view_groups".into(),
            "assign_change_users".into(),
            "assign_change_groups".into(),
            "assign_custom_fields".into(),
            "assign_custom_fields_values".into(),
            "remove_all_tags".into(),
            "remove_tags".into(),
            "remove_all_correspondents".into(),
            "remove_correspondents".into(),
            "remove_all_document_types".into(),
            "remove_document_types".into(),
            "remove_all_storage_paths".into(),
            "remove_storage_paths".into(),
            "remove_custom_fields".into(),
            "remove_all_custom_fields".into(),
            "remove_all_owners".into(),
            "remove_owners".into(),
            "remove_all_permissions".into(),
            "remove_view_users".into(),
            "remove_view_groups".into(),
            "remove_change_users".into(),
            "remove_change_groups".into(),
            "email".into(),
            "webhook".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PatchedWorkflowRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<WorkflowTriggerRequest>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<WorkflowActionRequest>>,
}

impl std::fmt::Display for PatchedWorkflowRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PatchedWorkflowRequest {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{name:?}").into()
            } else {
                String::new().into()
            },
            if let Some(order) = &self.order {
                format!("{order:?}").into()
            } else {
                String::new().into()
            },
            if let Some(enabled) = &self.enabled {
                format!("{enabled:?}").into()
            } else {
                String::new().into()
            },
            if let Some(triggers) = &self.triggers {
                format!("{triggers:?}").into()
            } else {
                String::new().into()
            },
            if let Some(actions) = &self.actions {
                format!("{actions:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "order".into(),
            "enabled".into(),
            "triggers".into(),
            "actions".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PatchedWorkflowTriggerRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default)]
    pub sources: Vec<i64>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<i64>,
    #[doc = "Only consume documents with a path that matches this if specified. Wildcards specified as * are allowed. Case insensitive."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_path: Option<String>,
    #[doc = "Only consume documents which entirely match this filename if specified. Wildcards such as *.pdf or *invoice* are allowed. Case insensitive."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_filename: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_mailrule: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matching_algorithm: Option<i64>,
    #[serde(rename = "match", default, skip_serializing_if = "Option::is_none")]
    pub match_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_insensitive: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_has_tags: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_has_correspondent: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_has_document_type: Option<i64>,
    #[doc = "The number of days to offset the schedule trigger by."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_offset_days: Option<i64>,
    #[doc = "If the schedule should be recurring."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_is_recurring: Option<bool>,
    #[doc = "The number of days between recurring schedule triggers."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_recurring_interval_days: Option<i64>,
    #[doc = "The field to check for a schedule trigger.\n\n* `added` - Added\n* `created` - Created\n* `modified` - Modified\n* `custom_field` - Custom Field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_date_field: Option<ScheduleDateFieldEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_date_custom_field: Option<i64>,
}

impl std::fmt::Display for PatchedWorkflowTriggerRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PatchedWorkflowTriggerRequest {
    const LENGTH: usize = 17;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{id:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.sources).into(),
            if let Some(type_) = &self.type_ {
                format!("{type_:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_path) = &self.filter_path {
                format!("{filter_path:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_filename) = &self.filter_filename {
                format!("{filter_filename:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_mailrule) = &self.filter_mailrule {
                format!("{filter_mailrule:?}").into()
            } else {
                String::new().into()
            },
            if let Some(matching_algorithm) = &self.matching_algorithm {
                format!("{matching_algorithm:?}").into()
            } else {
                String::new().into()
            },
            if let Some(match_) = &self.match_ {
                format!("{match_:?}").into()
            } else {
                String::new().into()
            },
            if let Some(is_insensitive) = &self.is_insensitive {
                format!("{is_insensitive:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_has_tags) = &self.filter_has_tags {
                format!("{filter_has_tags:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_has_correspondent) = &self.filter_has_correspondent {
                format!("{filter_has_correspondent:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_has_document_type) = &self.filter_has_document_type {
                format!("{filter_has_document_type:?}").into()
            } else {
                String::new().into()
            },
            if let Some(schedule_offset_days) = &self.schedule_offset_days {
                format!("{schedule_offset_days:?}").into()
            } else {
                String::new().into()
            },
            if let Some(schedule_is_recurring) = &self.schedule_is_recurring {
                format!("{schedule_is_recurring:?}").into()
            } else {
                String::new().into()
            },
            if let Some(schedule_recurring_interval_days) = &self.schedule_recurring_interval_days {
                format!("{schedule_recurring_interval_days:?}").into()
            } else {
                String::new().into()
            },
            if let Some(schedule_date_field) = &self.schedule_date_field {
                format!("{schedule_date_field:?}").into()
            } else {
                String::new().into()
            },
            if let Some(schedule_date_custom_field) = &self.schedule_date_custom_field {
                format!("{schedule_date_custom_field:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "sources".into(),
            "type_".into(),
            "filter_path".into(),
            "filter_filename".into(),
            "filter_mailrule".into(),
            "matching_algorithm".into(),
            "match_".into(),
            "is_insensitive".into(),
            "filter_has_tags".into(),
            "filter_has_correspondent".into(),
            "filter_has_document_type".into(),
            "schedule_offset_days".into(),
            "schedule_is_recurring".into(),
            "schedule_recurring_interval_days".into(),
            "schedule_date_field".into(),
            "schedule_date_custom_field".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct PostDocumentRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    pub document: bytes::Bytes,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correspondent: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage_path: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive_serial_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_webui: Option<bool>,
}

impl std::fmt::Display for PostDocumentRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostDocumentRequest {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(created) = &self.created {
                format!("{created:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.document).into(),
            if let Some(title) = &self.title {
                format!("{title:?}").into()
            } else {
                String::new().into()
            },
            if let Some(correspondent) = &self.correspondent {
                format!("{correspondent:?}").into()
            } else {
                String::new().into()
            },
            if let Some(document_type) = &self.document_type {
                format!("{document_type:?}").into()
            } else {
                String::new().into()
            },
            if let Some(storage_path) = &self.storage_path {
                format!("{storage_path:?}").into()
            } else {
                String::new().into()
            },
            if let Some(tags) = &self.tags {
                format!("{tags:?}").into()
            } else {
                String::new().into()
            },
            if let Some(archive_serial_number) = &self.archive_serial_number {
                format!("{archive_serial_number:?}").into()
            } else {
                String::new().into()
            },
            if let Some(custom_fields) = &self.custom_fields {
                format!("{custom_fields:?}").into()
            } else {
                String::new().into()
            },
            if let Some(from_webui) = &self.from_webui {
                format!("{from_webui:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "created".into(),
            "document".into(),
            "title".into(),
            "correspondent".into(),
            "document_type".into(),
            "storage_path".into(),
            "tags".into(),
            "archive_serial_number".into(),
            "custom_fields".into(),
            "from_webui".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct Profile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    pub auth_token: String,
    pub social_accounts: Vec<SocialAccount>,
    pub has_usable_password: bool,
    pub is_mfa_enabled: bool,
}

impl std::fmt::Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Profile {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(email) = &self.email {
                format!("{email:?}").into()
            } else {
                String::new().into()
            },
            if let Some(password) = &self.password {
                format!("{password:?}").into()
            } else {
                String::new().into()
            },
            if let Some(first_name) = &self.first_name {
                format!("{first_name:?}").into()
            } else {
                String::new().into()
            },
            if let Some(last_name) = &self.last_name {
                format!("{last_name:?}").into()
            } else {
                String::new().into()
            },
            self.auth_token.clone().into(),
            format!("{:?}", self.social_accounts).into(),
            format!("{:?}", self.has_usable_password).into(),
            format!("{:?}", self.is_mfa_enabled).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "email".into(),
            "password".into(),
            "first_name".into(),
            "last_name".into(),
            "auth_token".into(),
            "social_accounts".into(),
            "has_usable_password".into(),
            "is_mfa_enabled".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct SanityCheck {
    pub status: String,
    pub error: String,
    pub last_run: chrono::DateTime<chrono::Utc>,
}

impl std::fmt::Display for SanityCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SanityCheck {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.status.clone().into(),
            self.error.clone().into(),
            format!("{:?}", self.last_run).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["status".into(), "error".into(), "last_run".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct SavedView {
    pub id: i64,
    pub name: String,
    pub show_on_dashboard: bool,
    pub show_in_sidebar: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_reverse: Option<bool>,
    pub filter_rules: Vec<SavedViewFilterRule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_mode: Option<DisplayMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_fields: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<i64>,
    pub user_can_change: bool,
}

impl std::fmt::Display for SavedView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SavedView {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.name.clone().into(),
            format!("{:?}", self.show_on_dashboard).into(),
            format!("{:?}", self.show_in_sidebar).into(),
            if let Some(sort_field) = &self.sort_field {
                format!("{sort_field:?}").into()
            } else {
                String::new().into()
            },
            if let Some(sort_reverse) = &self.sort_reverse {
                format!("{sort_reverse:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.filter_rules).into(),
            if let Some(page_size) = &self.page_size {
                format!("{page_size:?}").into()
            } else {
                String::new().into()
            },
            if let Some(display_mode) = &self.display_mode {
                format!("{display_mode:?}").into()
            } else {
                String::new().into()
            },
            if let Some(display_fields) = &self.display_fields {
                format!("{display_fields:?}").into()
            } else {
                String::new().into()
            },
            if let Some(owner) = &self.owner {
                format!("{owner:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.user_can_change).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "show_on_dashboard".into(),
            "show_in_sidebar".into(),
            "sort_field".into(),
            "sort_reverse".into(),
            "filter_rules".into(),
            "page_size".into(),
            "display_mode".into(),
            "display_fields".into(),
            "owner".into(),
            "user_can_change".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct SavedViewFilterRule {
    pub rule_type: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl std::fmt::Display for SavedViewFilterRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SavedViewFilterRule {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.rule_type).into(),
            if let Some(value) = &self.value {
                format!("{value:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["rule_type".into(), "value".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct SavedViewFilterRuleRequest {
    pub rule_type: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl std::fmt::Display for SavedViewFilterRuleRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SavedViewFilterRuleRequest {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.rule_type).into(),
            if let Some(value) = &self.value {
                format!("{value:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["rule_type".into(), "value".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct SavedViewRequest {
    pub name: String,
    pub show_on_dashboard: bool,
    pub show_in_sidebar: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_reverse: Option<bool>,
    pub filter_rules: Vec<SavedViewFilterRuleRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_mode: Option<DisplayMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_fields: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<i64>,
}

impl std::fmt::Display for SavedViewRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SavedViewRequest {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.name.clone().into(),
            format!("{:?}", self.show_on_dashboard).into(),
            format!("{:?}", self.show_in_sidebar).into(),
            if let Some(sort_field) = &self.sort_field {
                format!("{sort_field:?}").into()
            } else {
                String::new().into()
            },
            if let Some(sort_reverse) = &self.sort_reverse {
                format!("{sort_reverse:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.filter_rules).into(),
            if let Some(page_size) = &self.page_size {
                format!("{page_size:?}").into()
            } else {
                String::new().into()
            },
            if let Some(display_mode) = &self.display_mode {
                format!("{display_mode:?}").into()
            } else {
                String::new().into()
            },
            if let Some(display_fields) = &self.display_fields {
                format!("{display_fields:?}").into()
            } else {
                String::new().into()
            },
            if let Some(owner) = &self.owner {
                format!("{owner:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "show_on_dashboard".into(),
            "show_in_sidebar".into(),
            "sort_field".into(),
            "sort_reverse".into(),
            "filter_rules".into(),
            "page_size".into(),
            "display_mode".into(),
            "display_fields".into(),
            "owner".into(),
        ]
    }
}

#[doc = "* `added` - Added\n* `created` - Created\n* `modified` - Modified\n* `custom_field` - Custom Field"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum ScheduleDateFieldEnum {
    #[serde(rename = "added")]
    #[display("added")]
    Added,
    #[serde(rename = "created")]
    #[display("created")]
    Created,
    #[serde(rename = "modified")]
    #[display("modified")]
    Modified,
    #[serde(rename = "custom_field")]
    #[display("custom_field")]
    CustomField,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct SearchResult {
    pub total: i64,
    pub documents: Vec<Document>,
    pub saved_views: Vec<SavedView>,
    pub tags: Vec<Tag>,
    pub correspondents: Vec<Correspondent>,
    pub document_types: Vec<DocumentType>,
    pub storage_paths: Vec<StoragePath>,
    pub users: Vec<User>,
    pub groups: Vec<Group>,
    pub mail_rules: Vec<MailRule>,
    pub mail_accounts: Vec<MailAccount>,
    pub workflows: Vec<Workflow>,
    pub custom_fields: Vec<CustomField>,
}

impl std::fmt::Display for SearchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SearchResult {
    const LENGTH: usize = 13;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.total).into(),
            format!("{:?}", self.documents).into(),
            format!("{:?}", self.saved_views).into(),
            format!("{:?}", self.tags).into(),
            format!("{:?}", self.correspondents).into(),
            format!("{:?}", self.document_types).into(),
            format!("{:?}", self.storage_paths).into(),
            format!("{:?}", self.users).into(),
            format!("{:?}", self.groups).into(),
            format!("{:?}", self.mail_rules).into(),
            format!("{:?}", self.mail_accounts).into(),
            format!("{:?}", self.workflows).into(),
            format!("{:?}", self.custom_fields).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "total".into(),
            "documents".into(),
            "saved_views".into(),
            "tags".into(),
            "correspondents".into(),
            "document_types".into(),
            "storage_paths".into(),
            "users".into(),
            "groups".into(),
            "mail_rules".into(),
            "mail_accounts".into(),
            "workflows".into(),
            "custom_fields".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct SelectionData {
    pub selected_correspondents: Vec<CorrespondentCounts>,
    pub selected_tags: Vec<TagCounts>,
    pub selected_document_types: Vec<DocumentTypeCounts>,
    pub selected_storage_paths: Vec<StoragePathCounts>,
    pub selected_custom_fields: Vec<CustomFieldCounts>,
}

impl std::fmt::Display for SelectionData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SelectionData {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.selected_correspondents).into(),
            format!("{:?}", self.selected_tags).into(),
            format!("{:?}", self.selected_document_types).into(),
            format!("{:?}", self.selected_storage_paths).into(),
            format!("{:?}", self.selected_custom_fields).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "selected_correspondents".into(),
            "selected_tags".into(),
            "selected_document_types".into(),
            "selected_storage_paths".into(),
            "selected_custom_fields".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct ShareLink {
    pub id: i64,
    pub created: chrono::DateTime<chrono::Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration: Option<chrono::DateTime<chrono::Utc>>,
    pub slug: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<i64>,
    #[doc = "* `archive` - Archive\n* `original` - Original"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_version: Option<FileVersionEnum>,
}

impl std::fmt::Display for ShareLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ShareLink {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            format!("{:?}", self.created).into(),
            if let Some(expiration) = &self.expiration {
                format!("{expiration:?}").into()
            } else {
                String::new().into()
            },
            self.slug.clone().into(),
            if let Some(document) = &self.document {
                format!("{document:?}").into()
            } else {
                String::new().into()
            },
            if let Some(file_version) = &self.file_version {
                format!("{file_version:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "created".into(),
            "expiration".into(),
            "slug".into(),
            "document".into(),
            "file_version".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct ShareLinkRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<i64>,
    #[doc = "* `archive` - Archive\n* `original` - Original"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_version: Option<FileVersionEnum>,
}

impl std::fmt::Display for ShareLinkRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ShareLinkRequest {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(expiration) = &self.expiration {
                format!("{expiration:?}").into()
            } else {
                String::new().into()
            },
            if let Some(document) = &self.document {
                format!("{document:?}").into()
            } else {
                String::new().into()
            },
            if let Some(file_version) = &self.file_version {
                format!("{file_version:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "expiration".into(),
            "document".into(),
            "file_version".into(),
        ]
    }
}

#[doc = "* `never` - never\n* `with_text` - with_text\n* `always` - always"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum SkipArchiveFileEnum {
    #[serde(rename = "never")]
    #[display("never")]
    Never,
    #[serde(rename = "with_text")]
    #[display("with_text")]
    WithText,
    #[serde(rename = "always")]
    #[display("always")]
    Always,
    #[serde(rename = "")]
    #[display("")]
    Empty,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct SocialAccount {
    pub id: i64,
    pub provider: String,
    pub name: String,
}

impl std::fmt::Display for SocialAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SocialAccount {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.provider.clone().into(),
            self.name.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "provider".into(), "name".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct SocialAccountRequest {
    pub provider: String,
}

impl std::fmt::Display for SocialAccountRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SocialAccountRequest {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.provider.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["provider".into()]
    }
}

#[doc = "* `FAILURE` - FAILURE\n* `PENDING` - PENDING\n* `RECEIVED` - RECEIVED\n* `RETRY` - RETRY\n* `REVOKED` - REVOKED\n* `STARTED` - STARTED\n* `SUCCESS` - SUCCESS"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum StatusEnum {
    #[serde(rename = "FAILURE")]
    #[display("FAILURE")]
    Failure,
    #[serde(rename = "PENDING")]
    #[display("PENDING")]
    Pending,
    #[serde(rename = "RECEIVED")]
    #[display("RECEIVED")]
    Received,
    #[serde(rename = "RETRY")]
    #[display("RETRY")]
    Retry,
    #[serde(rename = "REVOKED")]
    #[display("REVOKED")]
    Revoked,
    #[serde(rename = "STARTED")]
    #[display("STARTED")]
    Started,
    #[serde(rename = "SUCCESS")]
    #[display("SUCCESS")]
    Success,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct Storage {
    pub total: i64,
    pub available: i64,
}

impl std::fmt::Display for Storage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Storage {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.total).into(),
            format!("{:?}", self.available).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["total".into(), "available".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct StoragePath {
    pub id: i64,
    pub slug: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "match", default, skip_serializing_if = "Option::is_none")]
    pub match_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matching_algorithm: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_insensitive: Option<bool>,
    pub document_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<i64>,
    pub user_can_change: bool,
}

impl std::fmt::Display for StoragePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for StoragePath {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.slug.clone().into(),
            self.name.clone().into(),
            self.path.clone().into(),
            if let Some(match_) = &self.match_ {
                format!("{match_:?}").into()
            } else {
                String::new().into()
            },
            if let Some(matching_algorithm) = &self.matching_algorithm {
                format!("{matching_algorithm:?}").into()
            } else {
                String::new().into()
            },
            if let Some(is_insensitive) = &self.is_insensitive {
                format!("{is_insensitive:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.document_count).into(),
            if let Some(owner) = &self.owner {
                format!("{owner:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.user_can_change).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "slug".into(),
            "name".into(),
            "path".into(),
            "match_".into(),
            "matching_algorithm".into(),
            "is_insensitive".into(),
            "document_count".into(),
            "owner".into(),
            "user_can_change".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct StoragePathCounts {
    pub id: i64,
    pub document_count: i64,
}

impl std::fmt::Display for StoragePathCounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for StoragePathCounts {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            format!("{:?}", self.document_count).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "document_count".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct StoragePathRequest {
    pub name: String,
    pub path: String,
    #[serde(rename = "match", default, skip_serializing_if = "Option::is_none")]
    pub match_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matching_algorithm: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_insensitive: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_permissions: Option<SetPermissions>,
}

impl std::fmt::Display for StoragePathRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for StoragePathRequest {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.name.clone().into(),
            self.path.clone().into(),
            if let Some(match_) = &self.match_ {
                format!("{match_:?}").into()
            } else {
                String::new().into()
            },
            if let Some(matching_algorithm) = &self.matching_algorithm {
                format!("{matching_algorithm:?}").into()
            } else {
                String::new().into()
            },
            if let Some(is_insensitive) = &self.is_insensitive {
                format!("{is_insensitive:?}").into()
            } else {
                String::new().into()
            },
            if let Some(owner) = &self.owner {
                format!("{owner:?}").into()
            } else {
                String::new().into()
            },
            if let Some(set_permissions) = &self.set_permissions {
                format!("{set_permissions:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "path".into(),
            "match_".into(),
            "matching_algorithm".into(),
            "is_insensitive".into(),
            "owner".into(),
            "set_permissions".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct Suggestions {
    pub correspondents: Vec<i64>,
    pub tags: Vec<i64>,
    pub document_types: Vec<i64>,
    pub storage_paths: Vec<i64>,
    pub dates: Vec<String>,
}

impl std::fmt::Display for Suggestions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Suggestions {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.correspondents).into(),
            format!("{:?}", self.tags).into(),
            format!("{:?}", self.document_types).into(),
            format!("{:?}", self.storage_paths).into(),
            format!("{:?}", self.dates).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "correspondents".into(),
            "tags".into(),
            "document_types".into(),
            "storage_paths".into(),
            "dates".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct SystemStatus {
    pub pngx_version: String,
    pub server_os: String,
    pub install_type: String,
    pub storage: Storage,
    pub database: Database,
    pub tasks: Tasks,
    pub index: Index,
    pub classifier: Classifier,
    pub sanity_check: SanityCheck,
}

impl std::fmt::Display for SystemStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SystemStatus {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.pngx_version.clone().into(),
            self.server_os.clone().into(),
            self.install_type.clone().into(),
            format!("{:?}", self.storage).into(),
            format!("{:?}", self.database).into(),
            format!("{:?}", self.tasks).into(),
            format!("{:?}", self.index).into(),
            format!("{:?}", self.classifier).into(),
            format!("{:?}", self.sanity_check).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "pngx_version".into(),
            "server_os".into(),
            "install_type".into(),
            "storage".into(),
            "database".into(),
            "tasks".into(),
            "index".into(),
            "classifier".into(),
            "sanity_check".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct Tag {
    pub id: i64,
    pub slug: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    pub text_color: String,
    #[serde(rename = "match", default, skip_serializing_if = "Option::is_none")]
    pub match_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matching_algorithm: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_insensitive: Option<bool>,
    #[doc = "Marks this tag as an inbox tag: All newly consumed documents will be tagged with inbox tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_inbox_tag: Option<bool>,
    #[serde(default)]
    pub document_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<i64>,
    pub user_can_change: bool,
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Tag {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.slug.clone().into(),
            self.name.clone().into(),
            if let Some(color) = &self.color {
                format!("{color:?}").into()
            } else {
                String::new().into()
            },
            self.text_color.clone().into(),
            if let Some(match_) = &self.match_ {
                format!("{match_:?}").into()
            } else {
                String::new().into()
            },
            if let Some(matching_algorithm) = &self.matching_algorithm {
                format!("{matching_algorithm:?}").into()
            } else {
                String::new().into()
            },
            if let Some(is_insensitive) = &self.is_insensitive {
                format!("{is_insensitive:?}").into()
            } else {
                String::new().into()
            },
            if let Some(is_inbox_tag) = &self.is_inbox_tag {
                format!("{is_inbox_tag:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.document_count).into(),
            if let Some(owner) = &self.owner {
                format!("{owner:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.user_can_change).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "slug".into(),
            "name".into(),
            "color".into(),
            "text_color".into(),
            "match_".into(),
            "matching_algorithm".into(),
            "is_insensitive".into(),
            "is_inbox_tag".into(),
            "document_count".into(),
            "owner".into(),
            "user_can_change".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct TagCounts {
    pub id: i64,
    pub document_count: i64,
}

impl std::fmt::Display for TagCounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TagCounts {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            format!("{:?}", self.document_count).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "document_count".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct TagRequest {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "match", default, skip_serializing_if = "Option::is_none")]
    pub match_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matching_algorithm: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_insensitive: Option<bool>,
    #[doc = "Marks this tag as an inbox tag: All newly consumed documents will be tagged with inbox tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_inbox_tag: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_permissions: Option<SetPermissions>,
}

impl std::fmt::Display for TagRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TagRequest {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.name.clone().into(),
            if let Some(color) = &self.color {
                format!("{color:?}").into()
            } else {
                String::new().into()
            },
            if let Some(match_) = &self.match_ {
                format!("{match_:?}").into()
            } else {
                String::new().into()
            },
            if let Some(matching_algorithm) = &self.matching_algorithm {
                format!("{matching_algorithm:?}").into()
            } else {
                String::new().into()
            },
            if let Some(is_insensitive) = &self.is_insensitive {
                format!("{is_insensitive:?}").into()
            } else {
                String::new().into()
            },
            if let Some(is_inbox_tag) = &self.is_inbox_tag {
                format!("{is_inbox_tag:?}").into()
            } else {
                String::new().into()
            },
            if let Some(owner) = &self.owner {
                format!("{owner:?}").into()
            } else {
                String::new().into()
            },
            if let Some(set_permissions) = &self.set_permissions {
                format!("{set_permissions:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "color".into(),
            "match_".into(),
            "matching_algorithm".into(),
            "is_insensitive".into(),
            "is_inbox_tag".into(),
            "owner".into(),
            "set_permissions".into(),
        ]
    }
}

#[doc = "* `consume_file` - Consume File\n* `train_classifier` - Train Classifier\n* `check_sanity` - Check Sanity\n* `index_optimize` - Index Optimize"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum TaskNameEnum {
    #[serde(rename = "consume_file")]
    #[display("consume_file")]
    ConsumeFile,
    #[serde(rename = "train_classifier")]
    #[display("train_classifier")]
    TrainClassifier,
    #[serde(rename = "check_sanity")]
    #[display("check_sanity")]
    CheckSanity,
    #[serde(rename = "index_optimize")]
    #[display("index_optimize")]
    IndexOptimize,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct Tasks {
    pub redis_url: String,
    pub redis_status: String,
    pub redis_error: String,
    pub celery_status: String,
}

impl std::fmt::Display for Tasks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Tasks {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.redis_url.clone().into(),
            self.redis_status.clone().into(),
            self.redis_error.clone().into(),
            self.celery_status.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "redis_url".into(),
            "redis_status".into(),
            "redis_error".into(),
            "celery_status".into(),
        ]
    }
}

#[doc = "Name of the task that was run\n\n* `consume_file` - Consume File\n* `train_classifier` - Train Classifier\n* `check_sanity` - Check Sanity\n* `index_optimize` - Index Optimize"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum TaskName {
    #[serde(rename = "consume_file")]
    #[display("consume_file")]
    ConsumeFile,
    #[serde(rename = "train_classifier")]
    #[display("train_classifier")]
    TrainClassifier,
    #[serde(rename = "check_sanity")]
    #[display("check_sanity")]
    CheckSanity,
    #[serde(rename = "index_optimize")]
    #[display("index_optimize")]
    IndexOptimize,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct TasksView {
    pub id: i64,
    #[doc = "Celery ID for the Task that was run"]
    pub task_id: String,
    #[doc = "Name of the task that was run\n\n* `consume_file` - Consume File\n* `train_classifier` - Train Classifier\n* `check_sanity` - Check Sanity\n* `index_optimize` - Index Optimize"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_name: Option<TaskName>,
    #[doc = "Name of the file which the Task was run for"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_file_name: Option<String>,
    #[doc = "Datetime field when the task result was created in UTC"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_created: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Datetime field when the task was completed in UTC"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_done: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The type of task that was run\n\n* `auto_task` - Auto Task\n* `scheduled_task` - Scheduled Task\n* `manual_task` - Manual Task"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<TasksViewTypeEnum>,
    #[doc = "Current state of the task being run\n\n* `FAILURE` - FAILURE\n* `PENDING` - PENDING\n* `RECEIVED` - RECEIVED\n* `RETRY` - RETRY\n* `REVOKED` - REVOKED\n* `STARTED` - STARTED\n* `SUCCESS` - SUCCESS"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StatusEnum>,
    #[doc = "The data returned by the task"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[doc = "If the task is acknowledged via the frontend or API"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acknowledged: Option<bool>,
    #[serde(default)]
    pub related_document: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<i64>,
}

impl std::fmt::Display for TasksView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TasksView {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.task_id.clone().into(),
            if let Some(task_name) = &self.task_name {
                format!("{task_name:?}").into()
            } else {
                String::new().into()
            },
            if let Some(task_file_name) = &self.task_file_name {
                format!("{task_file_name:?}").into()
            } else {
                String::new().into()
            },
            if let Some(date_created) = &self.date_created {
                format!("{date_created:?}").into()
            } else {
                String::new().into()
            },
            if let Some(date_done) = &self.date_done {
                format!("{date_done:?}").into()
            } else {
                String::new().into()
            },
            if let Some(type_) = &self.type_ {
                format!("{type_:?}").into()
            } else {
                String::new().into()
            },
            if let Some(status) = &self.status {
                format!("{status:?}").into()
            } else {
                String::new().into()
            },
            if let Some(result) = &self.result {
                format!("{result:?}").into()
            } else {
                String::new().into()
            },
            if let Some(acknowledged) = &self.acknowledged {
                format!("{acknowledged:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.related_document).into(),
            if let Some(owner) = &self.owner {
                format!("{owner:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "task_id".into(),
            "task_name".into(),
            "task_file_name".into(),
            "date_created".into(),
            "date_done".into(),
            "type_".into(),
            "status".into(),
            "result".into(),
            "acknowledged".into(),
            "related_document".into(),
            "owner".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct TasksViewRequest {
    #[doc = "Celery ID for the Task that was run"]
    pub task_id: String,
    #[doc = "Name of the task that was run\n\n* `consume_file` - Consume File\n* `train_classifier` - Train Classifier\n* `check_sanity` - Check Sanity\n* `index_optimize` - Index Optimize"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_name: Option<TaskName>,
    #[doc = "Name of the file which the Task was run for"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_file_name: Option<String>,
    #[doc = "Datetime field when the task result was created in UTC"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_created: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Datetime field when the task was completed in UTC"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_done: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The type of task that was run\n\n* `auto_task` - Auto Task\n* `scheduled_task` - Scheduled Task\n* `manual_task` - Manual Task"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<TasksViewTypeEnum>,
    #[doc = "Current state of the task being run\n\n* `FAILURE` - FAILURE\n* `PENDING` - PENDING\n* `RECEIVED` - RECEIVED\n* `RETRY` - RETRY\n* `REVOKED` - REVOKED\n* `STARTED` - STARTED\n* `SUCCESS` - SUCCESS"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StatusEnum>,
    #[doc = "The data returned by the task"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[doc = "If the task is acknowledged via the frontend or API"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acknowledged: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<i64>,
}

impl std::fmt::Display for TasksViewRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TasksViewRequest {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.task_id.clone().into(),
            if let Some(task_name) = &self.task_name {
                format!("{task_name:?}").into()
            } else {
                String::new().into()
            },
            if let Some(task_file_name) = &self.task_file_name {
                format!("{task_file_name:?}").into()
            } else {
                String::new().into()
            },
            if let Some(date_created) = &self.date_created {
                format!("{date_created:?}").into()
            } else {
                String::new().into()
            },
            if let Some(date_done) = &self.date_done {
                format!("{date_done:?}").into()
            } else {
                String::new().into()
            },
            if let Some(type_) = &self.type_ {
                format!("{type_:?}").into()
            } else {
                String::new().into()
            },
            if let Some(status) = &self.status {
                format!("{status:?}").into()
            } else {
                String::new().into()
            },
            if let Some(result) = &self.result {
                format!("{result:?}").into()
            } else {
                String::new().into()
            },
            if let Some(acknowledged) = &self.acknowledged {
                format!("{acknowledged:?}").into()
            } else {
                String::new().into()
            },
            if let Some(owner) = &self.owner {
                format!("{owner:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "task_id".into(),
            "task_name".into(),
            "task_file_name".into(),
            "date_created".into(),
            "date_done".into(),
            "type_".into(),
            "status".into(),
            "result".into(),
            "acknowledged".into(),
            "owner".into(),
        ]
    }
}

#[doc = "* `auto_task` - Auto Task\n* `scheduled_task` - Scheduled Task\n* `manual_task` - Manual Task"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum TasksViewTypeEnum {
    #[serde(rename = "auto_task")]
    #[display("auto_task")]
    AutoTask,
    #[serde(rename = "scheduled_task")]
    #[display("scheduled_task")]
    ScheduledTask,
    #[serde(rename = "manual_task")]
    #[display("manual_task")]
    ManualTask,
}

#[doc = "* `restore` - restore\n* `empty` - empty"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum TrashActionEnum {
    #[serde(rename = "restore")]
    #[display("restore")]
    Restore,
    #[serde(rename = "empty")]
    #[display("empty")]
    Empty,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct TrashRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<i64>>,
    pub action: TrashActionEnum,
}

impl std::fmt::Display for TrashRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TrashRequest {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(documents) = &self.documents {
                format!("{documents:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.action).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["documents".into(), "action".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct UiSettingsView {
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl std::fmt::Display for UiSettingsView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UiSettingsView {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            if let Some(settings) = &self.settings {
                format!("{settings:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "settings".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct UiSettingsViewRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl std::fmt::Display for UiSettingsViewRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UiSettingsViewRequest {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(settings) = &self.settings {
            format!("{settings:?}").into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["settings".into()]
    }
}

#[doc = "* `clean` - clean\n* `clean-final` - clean-final\n* `none` - none"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum UnpaperCleanEnum {
    #[serde(rename = "clean")]
    #[display("clean")]
    Clean,
    #[serde(rename = "clean-final")]
    #[display("clean-final")]
    CleanFinal,
    #[serde(rename = "none")]
    #[display("none")]
    None,
    #[serde(rename = "")]
    #[display("")]
    Empty,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct User {
    pub id: i64,
    #[doc = "Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only."]
    pub username: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_joined: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Designates whether the user can log into this admin site."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_staff: Option<bool>,
    #[doc = "Designates whether this user should be treated as active. Unselect this instead of deleting accounts."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[doc = "Designates that this user has all permissions without explicitly assigning them."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_superuser: Option<bool>,
    #[doc = "The groups this user belongs to. A user will get all permissions granted to each of their groups."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_permissions: Option<Vec<String>>,
    pub inherited_permissions: Vec<String>,
    pub is_mfa_enabled: bool,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for User {
    const LENGTH: usize = 14;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.username.clone().into(),
            if let Some(email) = &self.email {
                format!("{email:?}").into()
            } else {
                String::new().into()
            },
            if let Some(password) = &self.password {
                format!("{password:?}").into()
            } else {
                String::new().into()
            },
            if let Some(first_name) = &self.first_name {
                format!("{first_name:?}").into()
            } else {
                String::new().into()
            },
            if let Some(last_name) = &self.last_name {
                format!("{last_name:?}").into()
            } else {
                String::new().into()
            },
            if let Some(date_joined) = &self.date_joined {
                format!("{date_joined:?}").into()
            } else {
                String::new().into()
            },
            if let Some(is_staff) = &self.is_staff {
                format!("{is_staff:?}").into()
            } else {
                String::new().into()
            },
            if let Some(is_active) = &self.is_active {
                format!("{is_active:?}").into()
            } else {
                String::new().into()
            },
            if let Some(is_superuser) = &self.is_superuser {
                format!("{is_superuser:?}").into()
            } else {
                String::new().into()
            },
            if let Some(groups) = &self.groups {
                format!("{groups:?}").into()
            } else {
                String::new().into()
            },
            if let Some(user_permissions) = &self.user_permissions {
                format!("{user_permissions:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.inherited_permissions).into(),
            format!("{:?}", self.is_mfa_enabled).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "username".into(),
            "email".into(),
            "password".into(),
            "first_name".into(),
            "last_name".into(),
            "date_joined".into(),
            "is_staff".into(),
            "is_active".into(),
            "is_superuser".into(),
            "groups".into(),
            "user_permissions".into(),
            "inherited_permissions".into(),
            "is_mfa_enabled".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct UserRequest {
    #[doc = "Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only."]
    pub username: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_joined: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Designates whether the user can log into this admin site."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_staff: Option<bool>,
    #[doc = "Designates whether this user should be treated as active. Unselect this instead of deleting accounts."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[doc = "Designates that this user has all permissions without explicitly assigning them."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_superuser: Option<bool>,
    #[doc = "The groups this user belongs to. A user will get all permissions granted to each of their groups."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_permissions: Option<Vec<String>>,
}

impl std::fmt::Display for UserRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UserRequest {
    const LENGTH: usize = 11;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.username.clone().into(),
            if let Some(email) = &self.email {
                format!("{email:?}").into()
            } else {
                String::new().into()
            },
            if let Some(password) = &self.password {
                format!("{password:?}").into()
            } else {
                String::new().into()
            },
            if let Some(first_name) = &self.first_name {
                format!("{first_name:?}").into()
            } else {
                String::new().into()
            },
            if let Some(last_name) = &self.last_name {
                format!("{last_name:?}").into()
            } else {
                String::new().into()
            },
            if let Some(date_joined) = &self.date_joined {
                format!("{date_joined:?}").into()
            } else {
                String::new().into()
            },
            if let Some(is_staff) = &self.is_staff {
                format!("{is_staff:?}").into()
            } else {
                String::new().into()
            },
            if let Some(is_active) = &self.is_active {
                format!("{is_active:?}").into()
            } else {
                String::new().into()
            },
            if let Some(is_superuser) = &self.is_superuser {
                format!("{is_superuser:?}").into()
            } else {
                String::new().into()
            },
            if let Some(groups) = &self.groups {
                format!("{groups:?}").into()
            } else {
                String::new().into()
            },
            if let Some(user_permissions) = &self.user_permissions {
                format!("{user_permissions:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "username".into(),
            "email".into(),
            "password".into(),
            "first_name".into(),
            "last_name".into(),
            "date_joined".into(),
            "is_staff".into(),
            "is_active".into(),
            "is_superuser".into(),
            "groups".into(),
            "user_permissions".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct Workflow {
    pub id: i64,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    pub triggers: Vec<WorkflowTrigger>,
    pub actions: Vec<WorkflowAction>,
}

impl std::fmt::Display for Workflow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Workflow {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.name.clone().into(),
            if let Some(order) = &self.order {
                format!("{order:?}").into()
            } else {
                String::new().into()
            },
            if let Some(enabled) = &self.enabled {
                format!("{enabled:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.triggers).into(),
            format!("{:?}", self.actions).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "order".into(),
            "enabled".into(),
            "triggers".into(),
            "actions".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct WorkflowAction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<i64>,
    #[doc = "Assign a document title, can include some placeholders, see documentation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_tags: Option<Vec<Option<i64>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_correspondent: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_document_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_storage_path: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_owner: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_view_users: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_view_groups: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_change_users: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_change_groups: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_custom_fields: Option<Vec<i64>>,
    #[doc = "Optional values to assign to the custom fields."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_custom_fields_values: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_all_tags: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_tags: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_all_correspondents: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_correspondents: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_all_document_types: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_document_types: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_all_storage_paths: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_storage_paths: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_custom_fields: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_all_custom_fields: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_all_owners: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_owners: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_all_permissions: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_view_users: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_view_groups: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_change_users: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_change_groups: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<WorkflowActionEmail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<WorkflowActionWebhook>,
}

impl std::fmt::Display for WorkflowAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for WorkflowAction {
    const LENGTH: usize = 33;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{id:?}").into()
            } else {
                String::new().into()
            },
            if let Some(type_) = &self.type_ {
                format!("{type_:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_title) = &self.assign_title {
                format!("{assign_title:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_tags) = &self.assign_tags {
                format!("{assign_tags:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_correspondent) = &self.assign_correspondent {
                format!("{assign_correspondent:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_document_type) = &self.assign_document_type {
                format!("{assign_document_type:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_storage_path) = &self.assign_storage_path {
                format!("{assign_storage_path:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_owner) = &self.assign_owner {
                format!("{assign_owner:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_view_users) = &self.assign_view_users {
                format!("{assign_view_users:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_view_groups) = &self.assign_view_groups {
                format!("{assign_view_groups:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_change_users) = &self.assign_change_users {
                format!("{assign_change_users:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_change_groups) = &self.assign_change_groups {
                format!("{assign_change_groups:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_custom_fields) = &self.assign_custom_fields {
                format!("{assign_custom_fields:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_custom_fields_values) = &self.assign_custom_fields_values {
                format!("{assign_custom_fields_values:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_all_tags) = &self.remove_all_tags {
                format!("{remove_all_tags:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_tags) = &self.remove_tags {
                format!("{remove_tags:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_all_correspondents) = &self.remove_all_correspondents {
                format!("{remove_all_correspondents:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_correspondents) = &self.remove_correspondents {
                format!("{remove_correspondents:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_all_document_types) = &self.remove_all_document_types {
                format!("{remove_all_document_types:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_document_types) = &self.remove_document_types {
                format!("{remove_document_types:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_all_storage_paths) = &self.remove_all_storage_paths {
                format!("{remove_all_storage_paths:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_storage_paths) = &self.remove_storage_paths {
                format!("{remove_storage_paths:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_custom_fields) = &self.remove_custom_fields {
                format!("{remove_custom_fields:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_all_custom_fields) = &self.remove_all_custom_fields {
                format!("{remove_all_custom_fields:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_all_owners) = &self.remove_all_owners {
                format!("{remove_all_owners:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_owners) = &self.remove_owners {
                format!("{remove_owners:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_all_permissions) = &self.remove_all_permissions {
                format!("{remove_all_permissions:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_view_users) = &self.remove_view_users {
                format!("{remove_view_users:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_view_groups) = &self.remove_view_groups {
                format!("{remove_view_groups:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_change_users) = &self.remove_change_users {
                format!("{remove_change_users:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_change_groups) = &self.remove_change_groups {
                format!("{remove_change_groups:?}").into()
            } else {
                String::new().into()
            },
            if let Some(email) = &self.email {
                format!("{email:?}").into()
            } else {
                String::new().into()
            },
            if let Some(webhook) = &self.webhook {
                format!("{webhook:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "type_".into(),
            "assign_title".into(),
            "assign_tags".into(),
            "assign_correspondent".into(),
            "assign_document_type".into(),
            "assign_storage_path".into(),
            "assign_owner".into(),
            "assign_view_users".into(),
            "assign_view_groups".into(),
            "assign_change_users".into(),
            "assign_change_groups".into(),
            "assign_custom_fields".into(),
            "assign_custom_fields_values".into(),
            "remove_all_tags".into(),
            "remove_tags".into(),
            "remove_all_correspondents".into(),
            "remove_correspondents".into(),
            "remove_all_document_types".into(),
            "remove_document_types".into(),
            "remove_all_storage_paths".into(),
            "remove_storage_paths".into(),
            "remove_custom_fields".into(),
            "remove_all_custom_fields".into(),
            "remove_all_owners".into(),
            "remove_owners".into(),
            "remove_all_permissions".into(),
            "remove_view_users".into(),
            "remove_view_groups".into(),
            "remove_change_users".into(),
            "remove_change_groups".into(),
            "email".into(),
            "webhook".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct WorkflowActionEmail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "The subject of the email, can include some placeholders, see documentation."]
    pub subject: String,
    #[doc = "The body (message) of the email, can include some placeholders, see documentation."]
    pub body: String,
    #[doc = "The destination email addresses, comma separated."]
    pub to: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_document: Option<bool>,
}

impl std::fmt::Display for WorkflowActionEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for WorkflowActionEmail {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{id:?}").into()
            } else {
                String::new().into()
            },
            self.subject.clone().into(),
            self.body.clone().into(),
            self.to.clone().into(),
            if let Some(include_document) = &self.include_document {
                format!("{include_document:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "subject".into(),
            "body".into(),
            "to".into(),
            "include_document".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct WorkflowActionEmailRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "The subject of the email, can include some placeholders, see documentation."]
    pub subject: String,
    #[doc = "The body (message) of the email, can include some placeholders, see documentation."]
    pub body: String,
    #[doc = "The destination email addresses, comma separated."]
    pub to: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_document: Option<bool>,
}

impl std::fmt::Display for WorkflowActionEmailRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for WorkflowActionEmailRequest {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{id:?}").into()
            } else {
                String::new().into()
            },
            self.subject.clone().into(),
            self.body.clone().into(),
            self.to.clone().into(),
            if let Some(include_document) = &self.include_document {
                format!("{include_document:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "subject".into(),
            "body".into(),
            "to".into(),
            "include_document".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct WorkflowActionRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<i64>,
    #[doc = "Assign a document title, can include some placeholders, see documentation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_tags: Option<Vec<Option<i64>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_correspondent: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_document_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_storage_path: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_owner: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_view_users: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_view_groups: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_change_users: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_change_groups: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_custom_fields: Option<Vec<i64>>,
    #[doc = "Optional values to assign to the custom fields."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assign_custom_fields_values: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_all_tags: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_tags: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_all_correspondents: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_correspondents: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_all_document_types: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_document_types: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_all_storage_paths: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_storage_paths: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_custom_fields: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_all_custom_fields: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_all_owners: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_owners: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_all_permissions: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_view_users: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_view_groups: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_change_users: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_change_groups: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<WorkflowActionEmailRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<WorkflowActionWebhookRequest>,
}

impl std::fmt::Display for WorkflowActionRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for WorkflowActionRequest {
    const LENGTH: usize = 33;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{id:?}").into()
            } else {
                String::new().into()
            },
            if let Some(type_) = &self.type_ {
                format!("{type_:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_title) = &self.assign_title {
                format!("{assign_title:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_tags) = &self.assign_tags {
                format!("{assign_tags:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_correspondent) = &self.assign_correspondent {
                format!("{assign_correspondent:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_document_type) = &self.assign_document_type {
                format!("{assign_document_type:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_storage_path) = &self.assign_storage_path {
                format!("{assign_storage_path:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_owner) = &self.assign_owner {
                format!("{assign_owner:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_view_users) = &self.assign_view_users {
                format!("{assign_view_users:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_view_groups) = &self.assign_view_groups {
                format!("{assign_view_groups:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_change_users) = &self.assign_change_users {
                format!("{assign_change_users:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_change_groups) = &self.assign_change_groups {
                format!("{assign_change_groups:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_custom_fields) = &self.assign_custom_fields {
                format!("{assign_custom_fields:?}").into()
            } else {
                String::new().into()
            },
            if let Some(assign_custom_fields_values) = &self.assign_custom_fields_values {
                format!("{assign_custom_fields_values:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_all_tags) = &self.remove_all_tags {
                format!("{remove_all_tags:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_tags) = &self.remove_tags {
                format!("{remove_tags:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_all_correspondents) = &self.remove_all_correspondents {
                format!("{remove_all_correspondents:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_correspondents) = &self.remove_correspondents {
                format!("{remove_correspondents:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_all_document_types) = &self.remove_all_document_types {
                format!("{remove_all_document_types:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_document_types) = &self.remove_document_types {
                format!("{remove_document_types:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_all_storage_paths) = &self.remove_all_storage_paths {
                format!("{remove_all_storage_paths:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_storage_paths) = &self.remove_storage_paths {
                format!("{remove_storage_paths:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_custom_fields) = &self.remove_custom_fields {
                format!("{remove_custom_fields:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_all_custom_fields) = &self.remove_all_custom_fields {
                format!("{remove_all_custom_fields:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_all_owners) = &self.remove_all_owners {
                format!("{remove_all_owners:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_owners) = &self.remove_owners {
                format!("{remove_owners:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_all_permissions) = &self.remove_all_permissions {
                format!("{remove_all_permissions:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_view_users) = &self.remove_view_users {
                format!("{remove_view_users:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_view_groups) = &self.remove_view_groups {
                format!("{remove_view_groups:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_change_users) = &self.remove_change_users {
                format!("{remove_change_users:?}").into()
            } else {
                String::new().into()
            },
            if let Some(remove_change_groups) = &self.remove_change_groups {
                format!("{remove_change_groups:?}").into()
            } else {
                String::new().into()
            },
            if let Some(email) = &self.email {
                format!("{email:?}").into()
            } else {
                String::new().into()
            },
            if let Some(webhook) = &self.webhook {
                format!("{webhook:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "type_".into(),
            "assign_title".into(),
            "assign_tags".into(),
            "assign_correspondent".into(),
            "assign_document_type".into(),
            "assign_storage_path".into(),
            "assign_owner".into(),
            "assign_view_users".into(),
            "assign_view_groups".into(),
            "assign_change_users".into(),
            "assign_change_groups".into(),
            "assign_custom_fields".into(),
            "assign_custom_fields_values".into(),
            "remove_all_tags".into(),
            "remove_tags".into(),
            "remove_all_correspondents".into(),
            "remove_correspondents".into(),
            "remove_all_document_types".into(),
            "remove_document_types".into(),
            "remove_all_storage_paths".into(),
            "remove_storage_paths".into(),
            "remove_custom_fields".into(),
            "remove_all_custom_fields".into(),
            "remove_all_owners".into(),
            "remove_owners".into(),
            "remove_all_permissions".into(),
            "remove_view_users".into(),
            "remove_view_groups".into(),
            "remove_change_users".into(),
            "remove_change_groups".into(),
            "email".into(),
            "webhook".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct WorkflowActionWebhook {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "The destination URL for the notification."]
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_params: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub as_json: Option<bool>,
    #[doc = "The parameters to send with the webhook URL if body not used."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
    #[doc = "The body to send with the webhook URL if parameters not used."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[doc = "The headers to send with the webhook URL."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_document: Option<bool>,
}

impl std::fmt::Display for WorkflowActionWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for WorkflowActionWebhook {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{id:?}").into()
            } else {
                String::new().into()
            },
            self.url.clone().into(),
            if let Some(use_params) = &self.use_params {
                format!("{use_params:?}").into()
            } else {
                String::new().into()
            },
            if let Some(as_json) = &self.as_json {
                format!("{as_json:?}").into()
            } else {
                String::new().into()
            },
            if let Some(params) = &self.params {
                format!("{params:?}").into()
            } else {
                String::new().into()
            },
            if let Some(body) = &self.body {
                format!("{body:?}").into()
            } else {
                String::new().into()
            },
            if let Some(headers) = &self.headers {
                format!("{headers:?}").into()
            } else {
                String::new().into()
            },
            if let Some(include_document) = &self.include_document {
                format!("{include_document:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "url".into(),
            "use_params".into(),
            "as_json".into(),
            "params".into(),
            "body".into(),
            "headers".into(),
            "include_document".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct WorkflowActionWebhookRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "The destination URL for the notification."]
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_params: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub as_json: Option<bool>,
    #[doc = "The parameters to send with the webhook URL if body not used."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
    #[doc = "The body to send with the webhook URL if parameters not used."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[doc = "The headers to send with the webhook URL."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_document: Option<bool>,
}

impl std::fmt::Display for WorkflowActionWebhookRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for WorkflowActionWebhookRequest {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{id:?}").into()
            } else {
                String::new().into()
            },
            self.url.clone().into(),
            if let Some(use_params) = &self.use_params {
                format!("{use_params:?}").into()
            } else {
                String::new().into()
            },
            if let Some(as_json) = &self.as_json {
                format!("{as_json:?}").into()
            } else {
                String::new().into()
            },
            if let Some(params) = &self.params {
                format!("{params:?}").into()
            } else {
                String::new().into()
            },
            if let Some(body) = &self.body {
                format!("{body:?}").into()
            } else {
                String::new().into()
            },
            if let Some(headers) = &self.headers {
                format!("{headers:?}").into()
            } else {
                String::new().into()
            },
            if let Some(include_document) = &self.include_document {
                format!("{include_document:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "url".into(),
            "use_params".into(),
            "as_json".into(),
            "params".into(),
            "body".into(),
            "headers".into(),
            "include_document".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct WorkflowRequest {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    pub triggers: Vec<WorkflowTriggerRequest>,
    pub actions: Vec<WorkflowActionRequest>,
}

impl std::fmt::Display for WorkflowRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for WorkflowRequest {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.name.clone().into(),
            if let Some(order) = &self.order {
                format!("{order:?}").into()
            } else {
                String::new().into()
            },
            if let Some(enabled) = &self.enabled {
                format!("{enabled:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.triggers).into(),
            format!("{:?}", self.actions).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "order".into(),
            "enabled".into(),
            "triggers".into(),
            "actions".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct WorkflowTrigger {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default)]
    pub sources: Vec<i64>,
    #[serde(rename = "type")]
    pub type_: i64,
    #[doc = "Only consume documents with a path that matches this if specified. Wildcards specified as * are allowed. Case insensitive."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_path: Option<String>,
    #[doc = "Only consume documents which entirely match this filename if specified. Wildcards such as *.pdf or *invoice* are allowed. Case insensitive."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_filename: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_mailrule: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matching_algorithm: Option<i64>,
    #[serde(rename = "match", default, skip_serializing_if = "Option::is_none")]
    pub match_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_insensitive: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_has_tags: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_has_correspondent: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_has_document_type: Option<i64>,
    #[doc = "The number of days to offset the schedule trigger by."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_offset_days: Option<i64>,
    #[doc = "If the schedule should be recurring."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_is_recurring: Option<bool>,
    #[doc = "The number of days between recurring schedule triggers."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_recurring_interval_days: Option<i64>,
    #[doc = "The field to check for a schedule trigger.\n\n* `added` - Added\n* `created` - Created\n* `modified` - Modified\n* `custom_field` - Custom Field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_date_field: Option<ScheduleDateFieldEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_date_custom_field: Option<i64>,
}

impl std::fmt::Display for WorkflowTrigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for WorkflowTrigger {
    const LENGTH: usize = 17;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{id:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.sources).into(),
            format!("{:?}", self.type_).into(),
            if let Some(filter_path) = &self.filter_path {
                format!("{filter_path:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_filename) = &self.filter_filename {
                format!("{filter_filename:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_mailrule) = &self.filter_mailrule {
                format!("{filter_mailrule:?}").into()
            } else {
                String::new().into()
            },
            if let Some(matching_algorithm) = &self.matching_algorithm {
                format!("{matching_algorithm:?}").into()
            } else {
                String::new().into()
            },
            if let Some(match_) = &self.match_ {
                format!("{match_:?}").into()
            } else {
                String::new().into()
            },
            if let Some(is_insensitive) = &self.is_insensitive {
                format!("{is_insensitive:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_has_tags) = &self.filter_has_tags {
                format!("{filter_has_tags:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_has_correspondent) = &self.filter_has_correspondent {
                format!("{filter_has_correspondent:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_has_document_type) = &self.filter_has_document_type {
                format!("{filter_has_document_type:?}").into()
            } else {
                String::new().into()
            },
            if let Some(schedule_offset_days) = &self.schedule_offset_days {
                format!("{schedule_offset_days:?}").into()
            } else {
                String::new().into()
            },
            if let Some(schedule_is_recurring) = &self.schedule_is_recurring {
                format!("{schedule_is_recurring:?}").into()
            } else {
                String::new().into()
            },
            if let Some(schedule_recurring_interval_days) = &self.schedule_recurring_interval_days {
                format!("{schedule_recurring_interval_days:?}").into()
            } else {
                String::new().into()
            },
            if let Some(schedule_date_field) = &self.schedule_date_field {
                format!("{schedule_date_field:?}").into()
            } else {
                String::new().into()
            },
            if let Some(schedule_date_custom_field) = &self.schedule_date_custom_field {
                format!("{schedule_date_custom_field:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "sources".into(),
            "type_".into(),
            "filter_path".into(),
            "filter_filename".into(),
            "filter_mailrule".into(),
            "matching_algorithm".into(),
            "match_".into(),
            "is_insensitive".into(),
            "filter_has_tags".into(),
            "filter_has_correspondent".into(),
            "filter_has_document_type".into(),
            "schedule_offset_days".into(),
            "schedule_is_recurring".into(),
            "schedule_recurring_interval_days".into(),
            "schedule_date_field".into(),
            "schedule_date_custom_field".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct WorkflowTriggerRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default)]
    pub sources: Vec<i64>,
    #[serde(rename = "type")]
    pub type_: i64,
    #[doc = "Only consume documents with a path that matches this if specified. Wildcards specified as * are allowed. Case insensitive."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_path: Option<String>,
    #[doc = "Only consume documents which entirely match this filename if specified. Wildcards such as *.pdf or *invoice* are allowed. Case insensitive."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_filename: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_mailrule: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matching_algorithm: Option<i64>,
    #[serde(rename = "match", default, skip_serializing_if = "Option::is_none")]
    pub match_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_insensitive: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_has_tags: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_has_correspondent: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_has_document_type: Option<i64>,
    #[doc = "The number of days to offset the schedule trigger by."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_offset_days: Option<i64>,
    #[doc = "If the schedule should be recurring."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_is_recurring: Option<bool>,
    #[doc = "The number of days between recurring schedule triggers."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_recurring_interval_days: Option<i64>,
    #[doc = "The field to check for a schedule trigger.\n\n* `added` - Added\n* `created` - Created\n* `modified` - Modified\n* `custom_field` - Custom Field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_date_field: Option<ScheduleDateFieldEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_date_custom_field: Option<i64>,
}

impl std::fmt::Display for WorkflowTriggerRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for WorkflowTriggerRequest {
    const LENGTH: usize = 17;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{id:?}").into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.sources).into(),
            format!("{:?}", self.type_).into(),
            if let Some(filter_path) = &self.filter_path {
                format!("{filter_path:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_filename) = &self.filter_filename {
                format!("{filter_filename:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_mailrule) = &self.filter_mailrule {
                format!("{filter_mailrule:?}").into()
            } else {
                String::new().into()
            },
            if let Some(matching_algorithm) = &self.matching_algorithm {
                format!("{matching_algorithm:?}").into()
            } else {
                String::new().into()
            },
            if let Some(match_) = &self.match_ {
                format!("{match_:?}").into()
            } else {
                String::new().into()
            },
            if let Some(is_insensitive) = &self.is_insensitive {
                format!("{is_insensitive:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_has_tags) = &self.filter_has_tags {
                format!("{filter_has_tags:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_has_correspondent) = &self.filter_has_correspondent {
                format!("{filter_has_correspondent:?}").into()
            } else {
                String::new().into()
            },
            if let Some(filter_has_document_type) = &self.filter_has_document_type {
                format!("{filter_has_document_type:?}").into()
            } else {
                String::new().into()
            },
            if let Some(schedule_offset_days) = &self.schedule_offset_days {
                format!("{schedule_offset_days:?}").into()
            } else {
                String::new().into()
            },
            if let Some(schedule_is_recurring) = &self.schedule_is_recurring {
                format!("{schedule_is_recurring:?}").into()
            } else {
                String::new().into()
            },
            if let Some(schedule_recurring_interval_days) = &self.schedule_recurring_interval_days {
                format!("{schedule_recurring_interval_days:?}").into()
            } else {
                String::new().into()
            },
            if let Some(schedule_date_field) = &self.schedule_date_field {
                format!("{schedule_date_field:?}").into()
            } else {
                String::new().into()
            },
            if let Some(schedule_date_custom_field) = &self.schedule_date_custom_field {
                format!("{schedule_date_custom_field:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "sources".into(),
            "type_".into(),
            "filter_path".into(),
            "filter_filename".into(),
            "filter_mailrule".into(),
            "matching_algorithm".into(),
            "match_".into(),
            "is_insensitive".into(),
            "filter_has_tags".into(),
            "filter_has_correspondent".into(),
            "filter_has_document_type".into(),
            "schedule_offset_days".into(),
            "schedule_is_recurring".into(),
            "schedule_recurring_interval_days".into(),
            "schedule_date_field".into(),
            "schedule_date_custom_field".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct DocumentShareLinksResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

impl std::fmt::Display for DocumentShareLinksResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for DocumentShareLinksResponse {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{id:?}").into()
            } else {
                String::new().into()
            },
            if let Some(created) = &self.created {
                format!("{created:?}").into()
            } else {
                String::new().into()
            },
            if let Some(expiration) = &self.expiration {
                format!("{expiration:?}").into()
            } else {
                String::new().into()
            },
            if let Some(slug) = &self.slug {
                format!("{slug:?}").into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "created".into(),
            "expiration".into(),
            "slug".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct ProfileDisconnectSocialAccountCreateRequestBody {
    pub id: i64,
}

impl std::fmt::Display for ProfileDisconnectSocialAccountCreateRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ProfileDisconnectSocialAccountCreateRequestBody {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.id).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct ProfileTotpCreateRequestBody {
    pub secret: String,
    pub code: String,
}

impl std::fmt::Display for ProfileTotpCreateRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ProfileTotpCreateRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.secret.clone().into(), self.code.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["secret".into(), "code".into()]
    }
}

#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum Status {
    #[serde(rename = "FAILURE")]
    #[display("FAILURE")]
    Failure,
    #[serde(rename = "PENDING")]
    #[display("PENDING")]
    Pending,
    #[serde(rename = "RECEIVED")]
    #[display("RECEIVED")]
    Received,
    #[serde(rename = "RETRY")]
    #[display("RETRY")]
    Retry,
    #[serde(rename = "REVOKED")]
    #[display("REVOKED")]
    Revoked,
    #[serde(rename = "STARTED")]
    #[display("STARTED")]
    Started,
    #[serde(rename = "SUCCESS")]
    #[display("SUCCESS")]
    Success,
}

#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum ListTaskName {
    #[serde(rename = "check_sanity")]
    #[display("check_sanity")]
    CheckSanity,
    #[serde(rename = "consume_file")]
    #[display("consume_file")]
    ConsumeFile,
    #[serde(rename = "index_optimize")]
    #[display("index_optimize")]
    IndexOptimize,
    #[serde(rename = "train_classifier")]
    #[display("train_classifier")]
    TrainClassifier,
}

#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum Type {
    #[serde(rename = "auto_task")]
    #[display("auto_task")]
    AutoTask,
    #[serde(rename = "manual_task")]
    #[display("manual_task")]
    ManualTask,
    #[serde(rename = "scheduled_task")]
    #[display("scheduled_task")]
    ScheduledTask,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[allow(non_snake_case)]
pub struct AcknowledgeTasksRequestBody {
    pub tasks: Vec<i64>,
}

impl std::fmt::Display for AcknowledgeTasksRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AcknowledgeTasksRequestBody {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.tasks).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["tasks".into()]
    }
}
