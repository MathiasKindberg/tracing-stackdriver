use serde::Deserialize;
use time::OffsetDateTime;

#[derive(Clone, Debug, Deserialize)]
pub struct MockSourceLocation {
    pub file: String,
    pub line: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct MockDefaultEvent {
    #[serde(deserialize_with = "time::serde::rfc3339::deserialize")]
    pub time: OffsetDateTime,
    pub target: String,
    pub severity: String,
    #[serde(rename = "logging.googleapis.com/sourceLocation")]
    pub source_location: MockSourceLocation,
}

#[derive(Debug, Deserialize)]
pub struct MockSpan {
    pub name: String,
    pub foo: String,
}

#[derive(Debug, Deserialize)]
pub struct MockEventWithSpan {
    pub span: MockSpan,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MockHttpRequest {
    pub request_method: String,
    pub latency: String,
    pub remote_ip: String,
    pub status: u16,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MockHttpEvent {
    pub http_request: MockHttpRequest,
}
