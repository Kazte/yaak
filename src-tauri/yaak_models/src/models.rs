use chrono::NaiveDateTime;
use rusqlite::Row;
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, Default, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export, export_to = "../../../src-web/lib/gen/")]
pub struct Settings {
    pub id: String,
    #[ts(type = "\"settings\"")]
    pub model: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub theme: String,
    pub appearance: String,
    pub theme_dark: String,
    pub theme_light: String,
    pub update_channel: String,
    pub interface_font_size: i32,
    pub interface_scale: i32,
    pub editor_font_size: i32,
    pub editor_soft_wrap: bool,
    pub open_workspace_new_window: Option<bool>,
}

#[derive(Iden)]
pub enum SettingsIden {
    #[iden = "settings"]
    Table,
    Id,
    Model,
    CreatedAt,
    UpdatedAt,
    Theme,
    Appearance,
    UpdateChannel,
    ThemeDark,
    ThemeLight,
    InterfaceFontSize,
    InterfaceScale,
    EditorFontSize,
    EditorSoftWrap,
    OpenWorkspaceNewWindow,
}

impl<'s> TryFrom<&Row<'s>> for Settings {
    type Error = rusqlite::Error;

    fn try_from(r: &Row<'s>) -> Result<Self, Self::Error> {
        Ok(Settings {
            id: r.get("id")?,
            model: r.get("model")?,
            created_at: r.get("created_at")?,
            updated_at: r.get("updated_at")?,
            theme: r.get("theme")?,
            appearance: r.get("appearance")?,
            theme_dark: r.get("theme_dark")?,
            theme_light: r.get("theme_light")?,
            update_channel: r.get("update_channel")?,
            interface_font_size: r.get("interface_font_size")?,
            interface_scale: r.get("interface_scale")?,
            editor_font_size: r.get("editor_font_size")?,
            editor_soft_wrap: r.get("editor_soft_wrap")?,
            open_workspace_new_window: r.get("open_workspace_new_window")?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export, export_to = "../../../plugin-runtime-types/src/gen/")]
pub struct Workspace {
    pub id: String,
    #[ts(type = "\"workspace\"")]
    pub model: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name: String,
    pub description: String,
    pub variables: Vec<EnvironmentVariable>,

    // Settings
    #[serde(default = "default_true")]
    pub setting_validate_certificates: bool,
    #[serde(default = "default_true")]
    pub setting_follow_redirects: bool,
    pub setting_request_timeout: i32,
}

#[derive(Iden)]
pub enum WorkspaceIden {
    #[iden = "workspaces"]
    Table,
    Id,
    Model,
    CreatedAt,
    UpdatedAt,
    Name,
    Description,
    Variables,
    SettingValidateCertificates,
    SettingFollowRedirects,
    SettingRequestTimeout,
}

impl<'s> TryFrom<&Row<'s>> for Workspace {
    type Error = rusqlite::Error;

    fn try_from(r: &Row<'s>) -> Result<Self, Self::Error> {
        let variables: String = r.get("variables")?;
        Ok(Workspace {
            id: r.get("id")?,
            model: r.get("model")?,
            created_at: r.get("created_at")?,
            updated_at: r.get("updated_at")?,
            name: r.get("name")?,
            description: r.get("description")?,
            variables: serde_json::from_str(variables.as_str()).unwrap_or_default(),
            setting_validate_certificates: r.get("setting_validate_certificates")?,
            setting_follow_redirects: r.get("setting_follow_redirects")?,
            setting_request_timeout: r.get("setting_request_timeout")?,
        })
    }
}

impl Workspace {
    pub fn new(name: String) -> Self {
        Self {
            name,
            model: "workspace".to_string(),
            setting_validate_certificates: true,
            setting_follow_redirects: true,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../src-web/lib/gen/")]
enum CookieDomain {
    HostOnly(String),
    Suffix(String),
    NotPresent,
    Empty,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../src-web/lib/gen/")]
enum CookieExpires {
    AtUtc(String),
    SessionEnd,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../src-web/lib/gen/")]
pub struct Cookie {
    raw_cookie: String,
    domain: CookieDomain,
    expires: CookieExpires,
    path: (String, bool),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export, export_to = "../../../src-web/lib/gen/")]
pub struct CookieJar {
    pub id: String,
    #[ts(type = "\"cookie_jar\"")]
    pub model: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub workspace_id: String,
    pub name: String,
    pub cookies: Vec<Cookie>,
}

#[derive(Iden)]
pub enum CookieJarIden {
    #[iden = "cookie_jars"]
    Table,
    Id,
    Model,
    WorkspaceId,
    CreatedAt,
    UpdatedAt,
    Name,
    Cookies,
}

impl<'s> TryFrom<&Row<'s>> for CookieJar {
    type Error = rusqlite::Error;

    fn try_from(r: &Row<'s>) -> Result<Self, Self::Error> {
        let cookies: String = r.get("cookies")?;
        Ok(CookieJar {
            id: r.get("id")?,
            model: r.get("model")?,
            workspace_id: r.get("workspace_id")?,
            created_at: r.get("created_at")?,
            updated_at: r.get("updated_at")?,
            name: r.get("name")?,
            cookies: serde_json::from_str(cookies.as_str()).unwrap_or_default(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export, export_to = "../../../plugin-runtime-types/src/gen/")]
pub struct Environment {
    pub id: String,
    pub workspace_id: String,
    #[ts(type = "\"environment\"")]
    pub model: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name: String,
    pub variables: Vec<EnvironmentVariable>,
}

#[derive(Iden)]
pub enum EnvironmentIden {
    #[iden = "environments"]
    Table,
    Id,
    Model,
    WorkspaceId,
    CreatedAt,
    UpdatedAt,
    Name,
    Variables,
}

impl<'s> TryFrom<&Row<'s>> for Environment {
    type Error = rusqlite::Error;

    fn try_from(r: &Row<'s>) -> Result<Self, Self::Error> {
        let variables: String = r.get("variables")?;
        Ok(Environment {
            id: r.get("id")?,
            model: r.get("model")?,
            workspace_id: r.get("workspace_id")?,
            created_at: r.get("created_at")?,
            updated_at: r.get("updated_at")?,
            name: r.get("name")?,
            variables: serde_json::from_str(variables.as_str()).unwrap_or_default(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export, export_to = "../../../plugin-runtime-types/src/gen/")]
pub struct EnvironmentVariable {
    #[serde(default = "default_true")]
    #[ts(optional, as = "Option<bool>")]
    pub enabled: bool,
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export, export_to = "../../../plugin-runtime-types/src/gen/")]
pub struct Folder {
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub id: String,
    pub workspace_id: String,
    pub folder_id: Option<String>,
    #[ts(type = "\"folder\"")]
    pub model: String,
    pub name: String,
    pub sort_priority: f32,
}

#[derive(Iden)]
pub enum FolderIden {
    #[iden = "folders"]
    Table,
    Id,
    Model,
    WorkspaceId,
    FolderId,
    CreatedAt,
    UpdatedAt,
    Name,
    SortPriority,
}

impl<'s> TryFrom<&Row<'s>> for Folder {
    type Error = rusqlite::Error;

    fn try_from(r: &Row<'s>) -> Result<Self, Self::Error> {
        Ok(Folder {
            id: r.get("id")?,
            model: r.get("model")?,
            sort_priority: r.get("sort_priority")?,
            workspace_id: r.get("workspace_id")?,
            created_at: r.get("created_at")?,
            updated_at: r.get("updated_at")?,
            folder_id: r.get("folder_id")?,
            name: r.get("name")?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export, export_to = "../../../plugin-runtime-types/src/gen/")]
pub struct HttpRequestHeader {
    #[serde(default = "default_true")]
    #[ts(optional, as = "Option<bool>")]
    pub enabled: bool,
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export, export_to = "../../../plugin-runtime-types/src/gen/")]
pub struct HttpUrlParameter {
    #[serde(default = "default_true")]
    #[ts(optional, as = "Option<bool>")]
    pub enabled: bool,
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export, export_to = "../../../plugin-runtime-types/src/gen/")]
pub struct HttpRequest {
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub id: String,
    pub workspace_id: String,
    pub folder_id: Option<String>,
    #[ts(type = "\"http_request\"")]
    pub model: String,
    pub sort_priority: f32,
    pub name: String,
    pub url: String,
    pub url_parameters: Vec<HttpUrlParameter>,
    #[serde(default = "default_http_request_method")]
    pub method: String,
    #[ts(type = "Record<string, any>")] 
    pub body: HashMap<String, Value>,
    pub body_type: Option<String>,
    #[ts(type = "Record<string, any>")]
    pub authentication: HashMap<String, Value>,
    pub authentication_type: Option<String>,
    pub headers: Vec<HttpRequestHeader>,
}

#[derive(Iden)]
pub enum HttpRequestIden {
    #[iden = "http_requests"]
    Table,
    Id,
    Model,
    WorkspaceId,
    FolderId,
    CreatedAt,
    UpdatedAt,
    Name,
    SortPriority,
    Url,
    UrlParameters,
    Method,
    Body,
    BodyType,
    Authentication,
    AuthenticationType,
    Headers,
}

impl<'s> TryFrom<&Row<'s>> for HttpRequest {
    type Error = rusqlite::Error;

    fn try_from(r: &Row<'s>) -> Result<Self, Self::Error> {
        let url_parameters: String = r.get("url_parameters")?;
        let body: String = r.get("body")?;
        let authentication: String = r.get("authentication")?;
        let headers: String = r.get("headers")?;
        Ok(HttpRequest {
            id: r.get("id")?,
            model: r.get("model")?,
            sort_priority: r.get("sort_priority")?,
            workspace_id: r.get("workspace_id")?,
            created_at: r.get("created_at")?,
            updated_at: r.get("updated_at")?,
            url: r.get("url")?,
            url_parameters: serde_json::from_str(url_parameters.as_str()).unwrap_or_default(),
            method: r.get("method")?,
            body: serde_json::from_str(body.as_str()).unwrap_or_default(),
            body_type: r.get("body_type")?,
            authentication: serde_json::from_str(authentication.as_str()).unwrap_or_default(),
            authentication_type: r.get("authentication_type")?,
            headers: serde_json::from_str(headers.as_str()).unwrap_or_default(),
            folder_id: r.get("folder_id")?,
            name: r.get("name")?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export, export_to = "../../../plugin-runtime-types/src/gen/")]
pub struct HttpResponseHeader {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export, export_to = "../../../plugin-runtime-types/src/gen/")]
pub struct HttpResponse {
    pub id: String,
    #[ts(type = "\"http_response\"")]
    pub model: String,
    pub workspace_id: String,
    pub request_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub error: Option<String>,
    pub url: String,
    pub content_length: Option<i32>,
    pub version: Option<String>,
    pub elapsed: i32,
    pub elapsed_headers: i32,
    pub remote_addr: Option<String>,
    pub status: i32,
    pub status_reason: Option<String>,
    pub body_path: Option<String>,
    pub headers: Vec<HttpResponseHeader>,
}

#[derive(Iden)]
pub enum HttpResponseIden {
    #[iden = "http_responses"]
    Table,
    Id,
    Model,
    WorkspaceId,
    RequestId,
    CreatedAt,
    UpdatedAt,
    Error,
    Url,
    ContentLength,
    Version,
    Elapsed,
    ElapsedHeaders,
    RemoteAddr,
    Status,
    StatusReason,
    BodyPath,
    Headers,
}

impl<'s> TryFrom<&Row<'s>> for HttpResponse {
    type Error = rusqlite::Error;

    fn try_from(r: &Row<'s>) -> Result<Self, Self::Error> {
        let headers: String = r.get("headers")?;
        Ok(HttpResponse {
            id: r.get("id")?,
            model: r.get("model")?,
            workspace_id: r.get("workspace_id")?,
            request_id: r.get("request_id")?,
            created_at: r.get("created_at")?,
            updated_at: r.get("updated_at")?,
            error: r.get("error")?,
            url: r.get("url")?,
            content_length: r.get("content_length")?,
            version: r.get("version")?,
            elapsed: r.get("elapsed")?,
            elapsed_headers: r.get("elapsed_headers")?,
            remote_addr: r.get("remote_addr")?,
            status: r.get("status")?,
            status_reason: r.get("status_reason")?,
            body_path: r.get("body_path")?,
            headers: serde_json::from_str(headers.as_str()).unwrap_or_default(),
        })
    }
}

impl HttpResponse {
    pub fn new() -> Self {
        Self {
            model: "http_response".to_string(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export, export_to = "../../../plugin-runtime-types/src/gen/")]
pub struct GrpcMetadataEntry {
    #[serde(default = "default_true")]
    #[ts(optional, as = "Option<bool>")]
    pub enabled: bool,
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export, export_to = "../../../plugin-runtime-types/src/gen/")]
pub struct GrpcRequest {
    pub id: String,
    #[ts(type = "\"grpc_request\"")]
    pub model: String,
    pub workspace_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub folder_id: Option<String>,
    pub name: String,
    pub sort_priority: f32,
    pub url: String,
    pub service: Option<String>,
    pub method: Option<String>,
    pub message: String,
    pub authentication_type: Option<String>,
    #[ts(type = "Record<string, any>")]
    pub authentication: HashMap<String, Value>,
    pub metadata: Vec<GrpcMetadataEntry>,
}

#[derive(Iden)]
pub enum GrpcRequestIden {
    #[iden = "grpc_requests"]
    Table,
    Id,
    Model,
    WorkspaceId,
    CreatedAt,
    UpdatedAt,
    FolderId,
    Name,
    SortPriority,
    Url,
    Service,
    Method,
    Message,
    AuthenticationType,
    Authentication,
    Metadata,
}

impl<'s> TryFrom<&Row<'s>> for GrpcRequest {
    type Error = rusqlite::Error;

    fn try_from(r: &Row<'s>) -> Result<Self, Self::Error> {
        let authentication: String = r.get("authentication")?;
        let metadata: String = r.get("metadata")?;
        Ok(GrpcRequest {
            id: r.get("id")?,
            model: r.get("model")?,
            workspace_id: r.get("workspace_id")?,
            created_at: r.get("created_at")?,
            updated_at: r.get("updated_at")?,
            folder_id: r.get("folder_id")?,
            name: r.get("name")?,
            service: r.get("service")?,
            method: r.get("method")?,
            message: r.get("message")?,
            authentication_type: r.get("authentication_type")?,
            authentication: serde_json::from_str(authentication.as_str()).unwrap_or_default(),
            url: r.get("url")?,
            sort_priority: r.get("sort_priority")?,
            metadata: serde_json::from_str(metadata.as_str()).unwrap_or_default(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export, export_to = "../../../plugin-runtime-types/src/gen/")]
pub struct GrpcConnection {
    pub id: String,
    #[ts(type = "\"grpc_connection\"")]
    pub model: String,
    pub workspace_id: String,
    pub request_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub service: String,
    pub method: String,
    pub elapsed: i32,
    pub status: i32,
    pub url: String,
    pub error: Option<String>,
    pub trailers: HashMap<String, String>,
}

#[derive(Iden)]
pub enum GrpcConnectionIden {
    #[iden = "grpc_connections"]
    Table,
    Id,
    Model,
    WorkspaceId,
    CreatedAt,
    UpdatedAt,
    RequestId,
    Service,
    Method,
    Elapsed,
    Status,
    Url,
    Error,
    Trailers,
}

impl<'s> TryFrom<&Row<'s>> for GrpcConnection {
    type Error = rusqlite::Error;

    fn try_from(r: &Row<'s>) -> Result<Self, Self::Error> {
        let trailers: String = r.get("trailers")?;
        Ok(GrpcConnection {
            id: r.get("id")?,
            model: r.get("model")?,
            workspace_id: r.get("workspace_id")?,
            request_id: r.get("request_id")?,
            created_at: r.get("created_at")?,
            updated_at: r.get("updated_at")?,
            service: r.get("service")?,
            method: r.get("method")?,
            elapsed: r.get("elapsed")?,
            status: r.get("status")?,
            url: r.get("url")?,
            error: r.get("error")?,
            trailers: serde_json::from_str(trailers.as_str()).unwrap_or_default(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export, export_to = "../../../plugin-runtime-types/src/gen/")]
pub enum GrpcEventType {
    Info,
    Error,
    ClientMessage,
    ServerMessage,
    ConnectionStart,
    ConnectionEnd,
}

impl Default for GrpcEventType {
    fn default() -> Self {
        GrpcEventType::Info
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export, export_to = "../../../plugin-runtime-types/src/gen/")]
pub struct GrpcEvent {
    pub id: String,
    #[ts(type = "\"grpc_event\"")]
    pub model: String,
    pub workspace_id: String,
    pub request_id: String,
    pub connection_id: String,
    pub created_at: NaiveDateTime,
    pub content: String,
    pub event_type: GrpcEventType,
    pub metadata: HashMap<String, String>,
    pub status: Option<i32>,
    pub error: Option<String>,
}

#[derive(Iden)]
pub enum GrpcEventIden {
    #[iden = "grpc_events"]
    Table,
    Id,
    Model,
    WorkspaceId,
    RequestId,
    ConnectionId,
    CreatedAt,
    UpdatedAt,
    Content,
    EventType,
    Metadata,
    Status,
    Error,
}

impl<'s> TryFrom<&Row<'s>> for GrpcEvent {
    type Error = rusqlite::Error;

    fn try_from(r: &Row<'s>) -> Result<Self, Self::Error> {
        let event_type: String = r.get("event_type")?;
        let metadata: String = r.get("metadata")?;
        Ok(GrpcEvent {
            id: r.get("id")?,
            model: r.get("model")?,
            workspace_id: r.get("workspace_id")?,
            request_id: r.get("request_id")?,
            connection_id: r.get("connection_id")?,
            created_at: r.get("created_at")?,
            content: r.get("content")?,
            event_type: serde_json::from_str(event_type.as_str()).unwrap_or_default(),
            metadata: serde_json::from_str(metadata.as_str()).unwrap_or_default(),
            status: r.get("status")?,
            error: r.get("error")?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export, export_to = "../../../plugin-runtime-types/src/gen/")]
pub struct KeyValue {
    #[ts(type = "\"key_value\"")]
    pub model: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub namespace: String,
    pub key: String,
    pub value: String,
}

#[derive(Iden)]
pub enum KeyValueIden {
    #[iden = "key_values"]
    Table,
    Model,
    CreatedAt,
    UpdatedAt,
    Namespace,
    Key,
    Value,
}

impl<'s> TryFrom<&Row<'s>> for KeyValue {
    type Error = rusqlite::Error;

    fn try_from(r: &Row<'s>) -> Result<Self, Self::Error> {
        Ok(KeyValue {
            model: r.get("model")?,
            created_at: r.get("created_at")?,
            updated_at: r.get("updated_at")?,
            namespace: r.get("namespace")?,
            key: r.get("key")?,
            value: r.get("value")?,
        })
    }
}

fn default_true() -> bool {
    true
}

fn default_http_request_method() -> String {
    "GET".to_string()
}

pub enum ModelType {
    TypeCookieJar,
    TypeEnvironment,
    TypeFolder,
    TypeGrpcConnection,
    TypeGrpcEvent,
    TypeGrpcRequest,
    TypeHttpRequest,
    TypeHttpResponse,
    TypeWorkspace,
}

impl ModelType {
    pub fn id_prefix(&self) -> String {
        match self {
            ModelType::TypeCookieJar => "cj",
            ModelType::TypeEnvironment => "ev",
            ModelType::TypeFolder => "fl",
            ModelType::TypeGrpcConnection => "gc",
            ModelType::TypeGrpcEvent => "ge",
            ModelType::TypeGrpcRequest => "gr",
            ModelType::TypeHttpRequest => "rq",
            ModelType::TypeHttpResponse => "rs",
            ModelType::TypeWorkspace => "wk",
        }
        .to_string()
    }
}