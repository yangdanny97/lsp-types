use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    GlobPattern, LSPObject, StaticRegistrationOptions, TextDocumentContentChangeEvent,
    TextDocumentIdentifier, TextDocumentItem, VersionedTextDocumentIdentifier,
};

/// A notebook document.
///
/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocument {
    /// The notebook document's URI.
    pub uri: Url,

    /// The type of the notebook.
    pub notebook_type: String,

    /// The version number of this document (it will strictly increase after each
    /// change, including undo/redo).
    pub version: i32,

    /// Additional metadata stored with the notebook document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<LSPObject>,

    /// The cells of a notebook.
    pub cells: Vec<NotebookCell>,
}

/// A notebook cell.
///
/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookCell {
    /// The cell's kind.
    pub kind: NotebookCellKind,

    /// The URI of the cell's text document content.
    pub document: Url,

    /// Additional metadata stored with the cell.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<LSPObject>,

    /// Additional execution summary information if supported by the client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_summary: Option<ExecutionSummary>,
}

/// A notebook cell kind.
///
/// @since 3.17.0
#[derive(Eq, PartialEq, Clone, Copy, Deserialize, Serialize)]
#[serde(transparent)]
pub struct NotebookCellKind(i32);
lsp_enum! {
impl NotebookCellKind {
    /// A markup cell, e.g. a Markdown cell in a Jupyter notebook.
    pub const MARKUP: NotebookCellKind = NotebookCellKind(1);
    /// A code cell, e.g. source code in a Jupyter notebook.
    pub const CODE: NotebookCellKind = NotebookCellKind(2);
}
}

/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionSummary {
    /// A strict monotonically increasing value indicating the execution order of a cell.
    pub execution_order: u32,

    /// Whether the execution was successful or not if known by the client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

/// A notebook document filter denotes a notebook document by different properties.
///
/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum NotebookDocumentFilter {
    ByType(NotebookDocumentFilterByType),
    ByScheme(NotebookDocumentFilterByScheme),
    ByPattern(NotebookDocumentFilterByPattern),
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocumentFilterByType {
    /// The type of the enclosing notebook.
    pub notebook_type: String,
    /// A Uri scheme, like `file` or `untitled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    /// A glob pattern.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<GlobPattern>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocumentFilterByScheme {
    /// The type of the enclosing notebook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_type: Option<String>,
    /// A Uri scheme, like `file` or `untitled`.
    pub scheme: String,
    /// A glob pattern.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<GlobPattern>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocumentFilterByPattern {
    /// The type of the enclosing notebook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_type: Option<String>,
    /// A Uri scheme, like `file` or `untitled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    /// A glob pattern.
    pub pattern: GlobPattern,
}

/// A notebook cell text document filter denotes a cell text document by
/// different properties.
///
/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct NotebookCellTextDocumentFilter {
    /// A filter that matches against the notebook containing the notebook cell.
    pub notebook: NotebookSelector,

    /// A language id like `python`. Will be matched against the language id of
    /// the notebook cell's document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

/// A notebook selector: either a notebook type string or a `NotebookDocumentFilter`.
///
/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum NotebookSelector {
    String(String),
    Filter(NotebookDocumentFilter),
}

/// Notebook document sync client capabilities.
///
/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocumentSyncClientCapabilities {
    /// Whether implementation supports dynamic registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_registration: Option<bool>,

    /// The client supports sending execution summary data per cell.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_summary_support: Option<bool>,
}

/// Notebook document specific client capabilities.
///
/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocumentClientCapabilities {
    /// Capabilities specific to notebook document synchronization.
    ///
    /// @since 3.17.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synchronization: Option<NotebookDocumentSyncClientCapabilities>,
}

/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookCellLanguage {
    pub language: String,
}

/// A notebook document filter where the notebook type is required.
///
/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocumentFilterWithNotebook {
    /// The notebook to be synced.
    pub notebook: NotebookSelector,

    /// The cells of the matching notebook to be synced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cells: Option<Vec<NotebookCellLanguage>>,
}

/// A notebook document filter where the cells are required.
///
/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocumentFilterWithCells {
    /// The notebook to be synced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook: Option<NotebookSelector>,

    /// The cells of the matching notebook to be synced.
    pub cells: Vec<NotebookCellLanguage>,
}

/// A notebook document sync filter.
///
/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum NotebookDocumentSyncFilter {
    WithNotebook(NotebookDocumentFilterWithNotebook),
    WithCells(NotebookDocumentFilterWithCells),
}

/// Options specific to a notebook plus its cells to be synced to the server.
///
/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocumentSyncOptions {
    /// The notebooks to be synced.
    pub notebook_selector: Vec<NotebookDocumentSyncFilter>,

    /// Whether save notifications should be forwarded to the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save: Option<bool>,
}

/// Registration options specific to a notebook.
///
/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocumentSyncRegistrationOptions {
    #[serde(flatten)]
    pub notebook_document_sync_options: NotebookDocumentSyncOptions,

    #[serde(flatten)]
    pub static_registration_options: StaticRegistrationOptions,
}

/// A literal to identify a notebook document in the client.
///
/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct NotebookDocumentIdentifier {
    /// The notebook document's URI.
    pub uri: Url,
}

/// A versioned notebook document identifier.
///
/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct VersionedNotebookDocumentIdentifier {
    /// The version number of this notebook document.
    pub version: i32,

    /// The notebook document's URI.
    pub uri: Url,
}

/// The params sent in a `notebookDocument/didOpen` notification.
///
/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DidOpenNotebookDocumentParams {
    /// The notebook document that got opened.
    pub notebook_document: NotebookDocument,

    /// The text documents that represent the content of a notebook cell.
    pub cell_text_documents: Vec<TextDocumentItem>,
}

/// A change describing a cell array change.
///
/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookCellArrayChange {
    /// The start offset of the cell that changed.
    pub start: u32,

    /// The deleted cells.
    pub delete_count: u32,

    /// The new cells, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cells: Option<Vec<NotebookCell>>,
}

/// A structural change to cells in a notebook document.
///
/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocumentCellChangeStructure {
    /// The change to the cell array.
    pub array: NotebookCellArrayChange,

    /// Additional opened cell text documents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did_open: Option<Vec<TextDocumentItem>>,

    /// Additional closed cell text documents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did_close: Option<Vec<TextDocumentIdentifier>>,
}

/// Content changes to a cell's text document.
///
/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocumentCellContentChanges {
    pub document: VersionedTextDocumentIdentifier,

    pub changes: Vec<TextDocumentContentChangeEvent>,
}

/// Cell changes in a notebook document.
///
/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocumentCellChanges {
    /// Changes to the cell structure to add or remove cells.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure: Option<NotebookDocumentCellChangeStructure>,

    /// Changes to notebook cells properties like its kind, execution summary, or metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<NotebookCell>>,

    /// Changes to the text content of notebook cells.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_content: Option<Vec<NotebookDocumentCellContentChanges>>,
}

/// A change event for a notebook document.
///
/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocumentChangeEvent {
    /// The changed metadata, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<LSPObject>,

    /// Changes to cells.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cells: Option<NotebookDocumentCellChanges>,
}

/// The params sent in a `notebookDocument/didChange` notification.
///
/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeNotebookDocumentParams {
    /// The notebook document that did change.
    pub notebook_document: VersionedNotebookDocumentIdentifier,

    /// The actual changes to the notebook document.
    pub change: NotebookDocumentChangeEvent,
}

/// The params sent in a `notebookDocument/didSave` notification.
///
/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DidSaveNotebookDocumentParams {
    /// The notebook document that got saved.
    pub notebook_document: NotebookDocumentIdentifier,
}

/// The params sent in a `notebookDocument/didClose` notification.
///
/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DidCloseNotebookDocumentParams {
    /// The notebook document that got closed.
    pub notebook_document: NotebookDocumentIdentifier,

    /// The text documents that represent the content of a notebook cell that
    /// got closed.
    pub cell_text_documents: Vec<TextDocumentIdentifier>,
}
