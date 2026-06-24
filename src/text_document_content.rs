use serde::{Deserialize, Serialize};
use url::Url;

use crate::StaticRegistrationOptions;

/// Client capabilities for the `workspace/textDocumentContent` request.
///
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentContentClientCapabilities {
    /// Whether implementation supports dynamic registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_registration: Option<bool>,
}

/// Options for the `workspace/textDocumentContent` request.
///
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentContentOptions {
    /// The URI schemes the server can provide content for.
    pub schemes: Vec<String>,
}

/// Registration options for the `workspace/textDocumentContent` request.
///
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentContentRegistrationOptions {
    #[serde(flatten)]
    pub text_document_content_options: TextDocumentContentOptions,

    #[serde(flatten)]
    pub static_registration_options: StaticRegistrationOptions,
}

/// Parameters of the `workspace/textDocumentContent` request.
///
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct TextDocumentContentParams {
    /// The URI of the text document.
    pub uri: Url,
}

/// Result of the `workspace/textDocumentContent` request.
///
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct TextDocumentContentResult {
    /// The text content of the document.
    pub text: String,
}

/// Parameters of the `workspace/textDocumentContent/refresh` request.
///
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct TextDocumentContentRefreshParams {
    /// The URI of the document to refresh.
    pub uri: Url,
}
