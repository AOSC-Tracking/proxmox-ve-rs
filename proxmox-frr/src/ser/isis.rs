use std::fmt::Debug;

use proxmox_sdn_types::net::Net;
use serde::Deserialize;
use serde::Serialize;

use crate::ser::FrrWord;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct IsisRouterName(FrrWord);

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum IsisLevel {
    #[serde(rename = "level-1")]
    Level1,
    #[serde(rename = "level-2")]
    Level2,
    #[serde(rename = "level-1-2")]
    Level12,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Redistribute {
    ipv4_connected: IsisLevel,
    ipv6_connected: IsisLevel,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct IsisRouter {
    pub net: Net,
    #[serde(default, deserialize_with = "proxmox_serde::perl::deserialize_bool")]
    pub log_adjacency_changes: Option<bool>,
    pub redistribute: Option<Redistribute>,
    #[serde(default)]
    pub custom_frr_config: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct IsisInterface {
    pub domain: IsisRouterName,
    #[serde(deserialize_with = "proxmox_serde::perl::deserialize_bool")]
    pub is_ipv4: bool,
    #[serde(deserialize_with = "proxmox_serde::perl::deserialize_bool")]
    pub is_ipv6: bool,
    #[serde(default)]
    pub custom_frr_config: Vec<String>,
}
