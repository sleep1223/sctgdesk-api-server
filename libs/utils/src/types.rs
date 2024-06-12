// Copyright (c) 2024 Ronan LE MEILLAT for SCTG Development
//
// This file is part of the SCTGDesk project.
//
// SCTGDesk is free software: you can redistribute it and/or modify
// it under the terms of the Affero General Public License version 3 as
// published by the Free Software Foundation.
//
// SCTGDesk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// Affero General Public License for more details.
//
// You should have received a copy of the Affero General Public License
// along with SCTGDesk. If not, see <https://www.gnu.org/licenses/agpl-3.0.html>.
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use oauth2::oauth_provider::OAuthProvider;
use rocket_okapi::okapi::schemars;
use rocket_okapi::JsonSchema;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize};

use crate::Token;

pub type SessionId = u64;
pub type UserId = Vec<u8>;

struct BoolVisitor;

impl<'de> Visitor<'de> for BoolVisitor {
    type Value = Option<bool>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a boolean or a string 'true', 'false', '0', '1'")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Some(value))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value {
            "true" | "1" => Ok(Some(true)),
            "false" | "0" => Ok(Some(false)),
            _ => Err(serde::de::Error::custom(
                "expected 'true', 'false', '0', '1'",
            )),
        }
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(None)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(None)
    }
}

fn from_str_to_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(BoolVisitor)
}

fn from_bool_to_str<S>(val: &Option<bool>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match val {
        Some(true) => serializer.serialize_str("true"),
        Some(false) => serializer.serialize_str("false"),
        None => serializer.serialize_none(),
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub struct AddressBook {
    pub ab: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<u32>,
}

impl AddressBook {
    pub fn empty() -> Self {
        Self {
            ab: "{}".to_string(),
            name: None,
            owner: None,
            rule: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SystemInfo {
    pub cpu: Option<String>,
    pub hostname: Option<String>,
    pub id: Option<String>,
    pub memory: Option<String>,
    pub os: Option<String>,
    pub username: Option<String>,
    pub uuid: Option<String>,
    pub version: Option<String>,
    pub ip: Option<String>,
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub id: String,
    pub uuid: String,
}

#[derive(Deserialize, Serialize, Debug, JsonSchema, Clone, Default)]
pub struct UserInfo {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub admin: bool
}

#[derive(Serialize, Debug, JsonSchema)]
pub struct LoginReply {
    #[serde(rename = "type")]
    pub response_type: String,
    pub user: UserInfo,
    pub access_token: Token,
}
#[derive(Serialize, Debug, JsonSchema)]
pub struct LogoutReply {
    pub data: String,
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct CurrentUserRequest {
    pub id: String,
    pub uuid: String,
}

#[derive(Serialize, Debug, JsonSchema)]
pub struct CurrentUserResponse {
    pub error: bool,
    #[serde(flatten)]
    pub data: UserInfo,
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct AbRequest {
    pub data: String,
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct AuditRequest {
    #[serde(default)]
    #[serde(rename = "Id")]
    pub id_: usize,
    #[serde(default)]
    pub action: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub ip: String,
    #[serde(default)]
    pub uuid: String,
}

// {
//    peers: [{id: "abcd", username: "", hostname: "", platform: "", alias: "", tags: ["", "", ...]}, ...],
//    tags: [],
// }

#[derive(Serialize, Debug, JsonSchema)]
pub struct Ab {
    pub tags: Vec<String>,
    pub peers: Vec<AbPeer>,
}

#[derive(Serialize, Debug, JsonSchema)]
pub struct AbGetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    pub data: String,
}

#[derive(Serialize, Debug, JsonSchema)]
pub struct AbPersonal {
    pub guid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct AbSettingsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub max_peer_one_ab: u32,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct AbProfile {
    pub guid: String,
    pub name: String,
    pub owner: String,
    pub note: Option<String>,
    pub rule: u32,
}

impl Default for AbProfile {
    fn default() -> Self {
        AbProfile {
            guid: "".to_string(),
            name: "".to_string(),
            owner: "".to_string(),
            note: None,
            rule: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct AbSharedProfilesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub total: u32,
    pub data: Vec<AbProfile>,
}

impl Default for AbSharedProfilesResponse {
    fn default() -> Self {
        AbSharedProfilesResponse {
            error: None,
            total: 0,
            data: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct AbPeer {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(
        rename = "forceAlwaysRelay",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "from_str_to_bool",
        serialize_with = "from_bool_to_str"
    )]
    pub force_always_relay: Option<bool>,
    #[serde(rename = "rdpPort")]
    pub rdp_port: Option<String>,
    #[serde(rename = "rdpUsername", skip_serializing_if = "Option::is_none")]
    pub rdp_username: Option<String>,
    #[serde(rename = "loginName", skip_serializing_if = "Option::is_none")]
    pub login_name: Option<String>, //login username
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "from_str_to_bool",
        serialize_with = "from_bool_to_str"
    )]
    pub same_server: Option<bool>,
}
impl Default for AbPeer {
    fn default() -> Self {
        AbPeer {
            id: "".to_string(),
            hash: Some("".to_string()),
            password: Some("".to_string()),
            username: Some("".to_string()),
            hostname: Some("".to_string()),
            platform: Some("".to_string()),
            alias: Some("".to_string()),
            tags: Some(Vec::new()),
            force_always_relay: Some(false),
            rdp_port: Some("".to_string()),
            rdp_username: Some("".to_string()),
            login_name: Some("".to_string()),
            same_server: None,
        }
    }
}

impl AbPeer {
    pub fn default_test() -> Self {
        AbPeer {
            id: "123456789".to_string(),
            hash: Some("0".to_string()),
            password: Some("none".to_string()),
            username: Some("someone".to_string()),
            hostname: Some("unknown".to_string()),
            platform: Some("windows".to_string()),
            alias: Some("Test peer".to_string()),
            tags: Some(Vec::new()),
            force_always_relay: Some(false),
            rdp_port: Some("".to_string()),
            rdp_username: Some("".to_string()),
            login_name: Some("user".to_string()),
            same_server: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct AbTag {
    pub name: String,
    pub color: u32,
}
impl Default for AbTag {
    fn default() -> Self {
        AbTag {
            name: "TAG1".to_string(),
            color: 4288585374,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct AbTagRenameRequest {
    pub old: String,
    pub new: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct AbPeersResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub total: u32,
    pub data: Vec<AbPeer>,
}
impl Default for AbPeersResponse {
    fn default() -> Self {
        AbPeersResponse {
            error: None,
            total: 0,
            data: Vec::new(),
        }
    }
}
impl AbPeersResponse {
    pub fn default_test() -> Self {
        AbPeersResponse {
            error: None,
            total: 1,
            data: vec![AbPeer::default_test()],
        }
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct HeartbeatRequest {
    pub id: String,
    pub modified_at: u64,
    pub uuid: String,
    pub ver: u32,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SystemInfoRequest {
    pub cpu: String,
    pub hostname: String,
    pub id: String,
    pub memory: String,
    pub os: String,
    pub username: String,
    pub uuid: String,
    pub version: String,
}

#[derive(Serialize, Debug, JsonSchema)]
pub struct UsersResponse {
    pub msg: String,
    pub total: u32,
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct User {
    name: String,
    password: String,
    #[serde(rename = "confirm-password")]
    confirm_password: String,
    email: String,
    is_admin: bool,
    #[serde(rename = "group_name")]
    group_name: String,
    note: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct UpdateUserRequest {
    pub uuid: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default, rename = "confirm-password", skip_serializing_if = "Option::is_none")]
    pub confirm_password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_admin: Option<bool>,
    #[serde(default, rename = "group_name", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}
impl Default for UpdateUserRequest {
    fn default() -> Self {
        UpdateUserRequest {
            uuid: uuid::Uuid::new_v4().to_string(),
            name: None,
            password: None,
            confirm_password: None,
            email: None,
            is_admin: None,
            group_name: None,
            note: None,
            status: None,
        }
    }
}
#[derive(Serialize, Deserialize, Clone, JsonSchema, Debug)]
pub struct OidcDeviceInfo {
    pub name: String,
    pub os: String,
    pub r#type: String,
}

impl Default for OidcDeviceInfo {
    fn default() -> Self {
        OidcDeviceInfo {
            name: "".to_string(),
            os: "".to_string(),
            r#type: "".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct OidcAuthRequest {
    #[serde(rename = "deviceInfo")]
    pub device_info: OidcDeviceInfo,
    pub id: String,
    pub op: String,
    pub uuid: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct OidcAuthUrl {
    pub code: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct AuthQueryParams {
    pub code: String,
    pub id: String,
    pub uuid: String,
}

// OIDC response
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[repr(i32)]
pub enum OidcUserStatus {
    Disabled = 0,
    Normal = 1,
    Unverified = -1,
}
impl Default for OidcUserStatus {
    fn default() -> Self {
        OidcUserStatus::Normal
    }
}
impl Into<i32> for OidcUserStatus {
    fn into(self) -> i32 {
        self as i32
    }
}

impl Into<i64> for OidcUserStatus {
    fn into(self) -> i64 {
        self as i64
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct OidcResponse {
    pub access_token: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub tfa_type: String,
    pub secret: String,
    pub user: OidcUser,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct OidcUser {
    pub name: String,
    pub email: String,
    pub note: String,
    pub status: i64,
    pub info: OidcUserInfo,
    pub is_admin: bool,
    pub third_auth_type: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct OidcUserInfo {
    pub email_verification: bool,
    pub email_alarm_notification: bool,
    pub login_device_whitelist: Vec<String>,
    pub other: HashMap<String, String>,
}

#[derive(Clone)]
pub struct OidcState {
    pub id: String,           // is id of the Rustdesk client
    pub uuid: String,         // is uuid of the Rustdesk client
    pub code: Option<String>, // is openid_code
    pub auth_token: Option<String>,
    pub redirect_url: Option<String>,
    pub callback_url: Option<String>,
    pub provider: Option<Arc<dyn OAuthProvider>>,
    pub name: Option<String>,
    pub email: Option<String>,
}
impl Default for OidcState {
    fn default() -> Self {
        OidcState {
            id: "".to_string(),
            uuid: "".to_string(),
            code: None,
            auth_token: None,
            redirect_url: None,
            callback_url: None,
            provider: None,
            name: None,
            email: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct OidcTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub id_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
struct JwtClaims {
    sub: String,
    email: String,
    name: String,
    iat: i64,
    exp: i64,
    iss: String,
    aud: String,
}

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct AddUserRequest {
    pub name: String,
    pub password: String,
    #[serde(rename = "confirm-password")]
    pub confirm_password: String,
    pub email: String,
    pub is_admin: bool,
    pub group_name: String,
}

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct EnableUserRequest {
    pub rows: Vec<String>,
    pub disable: bool,
}

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct Provider {
    name: String,
    order_index: u32,
    enabled: bool,
    client_id: String,
    client_secret: String,
    authorization_endpoint: String,
    token_endpoint: String,
    userinfo_endpoint: String,
}

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct OidcSettingsResponse {
    max_auth_count: u32,
    callback_url: String,
    providers: Vec<Provider>,
}

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct UserListResponse {
    pub guid: String,
    pub name: String,
    pub email: String,
    pub note: Option<String>,
    pub status: i32,
    pub group_name: String,
    pub is_admin: bool,
}

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct UserList {
    pub msg: String,
    pub total: u32,
    pub data: Vec<UserListResponse>,
}

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct SoftwareResponse {
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct SoftwareVersionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct PeersResponse {
    pub msg: String,
    pub total: u32,
    pub data: Vec<Peer>,
}

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct PeersCountResponse {
    pub total: u32,
}
#[derive(Serialize, Deserialize, Clone, JsonSchema, Debug, Default)]
pub struct PeerInfo {
    pub cpu: Option<String>,
    pub hostname: Option<String>,
    pub id: Option<String>,
    pub memory: Option<String>,
    pub os: Option<String>,
    pub username: Option<String>,
    pub uuid: Option<String>,
    pub version: Option<String>,
    pub ip: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct Peer{
    pub guid: String,
    pub id: String,
    pub status: i32,
    pub strategy_name: String,
    pub last_online: String,
    pub info: PeerInfo,
}

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct GroupsResponse {
    pub msg: String,
    pub total: u32,
    pub data: Vec<Group>,
}

pub type GroupInfo = String;
#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct Group {
    pub guid: String,
    pub name: String,
    pub team: String,
    pub created_at: String,
    pub access_to: Vec<String>,
    pub accessed_from: Vec<String>,
    pub note: Option<String>,
    pub info: GroupInfo,
}

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct AbSharedAddRequest {
    pub name: String,
    pub note: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct AbRule {
    pub guid: String,
    pub user: Option<String>,
    pub group: Option<String>,
    pub rule: u32,
}

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct AbRulesResponse {
    pub msg: String,
    pub total: u32,
    pub data: Vec<AbRule>,
}

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct AbRuleAddRequest {
    pub guid: String, // address book guid
    pub user: Option<String>, // user=None and group=None means all users
    pub group: Option<String>,
    pub rule: u32,
}

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct AbRuleDeleteRequest {
    pub guid: String, // rule guid
}

pub enum Platform {
    Windows,MacOS,Linux,Android,All
}

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct CpuCount {
    pub cpu: String,
    pub total: u32,
}