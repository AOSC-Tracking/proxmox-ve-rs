use std::fmt::Debug;

use proxmox_sdn_types::net::Net;

use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

use crate::ser::FrrWord;
use crate::ser::FrrWordError;

/// The name of a OpenFabric router. Is an FrrWord.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OpenfabricRouterName(FrrWord);

impl From<FrrWord> for OpenfabricRouterName {
    fn from(value: FrrWord) -> Self {
        Self(value)
    }
}

impl OpenfabricRouterName {
    pub fn new(name: FrrWord) -> Self {
        Self(name)
    }
}

impl std::fmt::Display for OpenfabricRouterName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0.as_ref())
    }
}

/// All the properties a OpenFabric router can hold.
///
/// These can serialized with a " " space prefix as they are in the `router openfabric` block.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OpenfabricRouter {
    /// The NET address
    pub net: Net,
}

impl OpenfabricRouter {
    pub fn new(net: Net) -> Self {
        Self { net }
    }

    pub fn net(&self) -> &Net {
        &self.net
    }
}

/// The OpenFabric properties.
///
/// This struct holds all the OpenFabric interface properties. The most important one here is the
/// fabric_id, which ties the interface to a fabric. When serialized these properties all get
/// prefixed with a space (" ") as they are inside the interface block.
///
/// The is_ipv4 and is_ipv6 properties decide if we need to add `ip router openfabric`, `ipv6
/// router openfabric`, or both. An interface can only be part of a single fabric.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OpenfabricInterface {
    // Note: an interface can only be a part of a single fabric (so no vec needed here)
    pub fabric_id: OpenfabricRouterName,
    #[serde(
        default,
        deserialize_with = "proxmox_serde::perl::deserialize_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub passive: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hello_interval: Option<proxmox_sdn_types::openfabric::HelloInterval>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csnp_interval: Option<proxmox_sdn_types::openfabric::CsnpInterval>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hello_multiplier: Option<proxmox_sdn_types::openfabric::HelloMultiplier>,
    #[serde(deserialize_with = "proxmox_serde::perl::deserialize_bool")]
    pub is_ipv4: bool,
    #[serde(deserialize_with = "proxmox_serde::perl::deserialize_bool")]
    pub is_ipv6: bool,
}

#[derive(Error, Debug)]
pub enum OpenfabricInterfaceError {
    #[error("Unknown error converting to OpenFabricInterface")]
    UnknownError,
    #[error("Error parsing frr word")]
    FrrWordParse(#[from] FrrWordError),
}
